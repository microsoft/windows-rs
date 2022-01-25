pub trait IAccIdentity_Impl: Sized {
    fn GetIdentityString(&mut self, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::Result<()>;
}
impl IAccIdentity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccIdentity_Impl, const OFFSET: isize>() -> IAccIdentity_Vtbl {
        unsafe extern "system" fn GetIdentityString<Identity: ::windows::core::IUnknownImpl, Impl: IAccIdentity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIdentityString(::core::mem::transmute_copy(&dwidchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetIdentityString: GetIdentityString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAccPropServer_Impl: Sized {
    fn GetPropValue(&mut self, pidstring: *const u8, dwidstringlen: u32, idprop: &::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAccPropServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServer_Impl, const OFFSET: isize>() -> IAccPropServer_Vtbl {
        unsafe extern "system" fn GetPropValue<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropValue(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&pfhasprop)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetPropValue: GetPropValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccPropServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IAccPropServices_Impl: Sized {
    fn SetPropValue(&mut self, pidstring: *const u8, dwidstringlen: u32, idprop: &::windows::core::GUID, var: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetPropServer(&mut self, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: &::core::option::Option<IAccPropServer>, annoscope: AnnoScope) -> ::windows::core::Result<()>;
    fn ClearProps(&mut self, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::Result<()>;
    fn SetHwndProp(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: &::windows::core::GUID, var: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetHwndPropStr(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: &::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHwndPropServer(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: &::core::option::Option<IAccPropServer>, annoscope: AnnoScope) -> ::windows::core::Result<()>;
    fn ClearHwndProps(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::Result<()>;
    fn ComposeHwndIdentityString(&mut self, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::Result<()>;
    fn DecomposeHwndIdentityString(&mut self, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows::core::Result<()>;
    fn SetHmenuProp(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: &::windows::core::GUID, var: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetHmenuPropStr(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: &::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetHmenuPropServer(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: &::core::option::Option<IAccPropServer>, annoscope: AnnoScope) -> ::windows::core::Result<()>;
    fn ClearHmenuProps(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::Result<()>;
    fn ComposeHmenuIdentityString(&mut self, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::Result<()>;
    fn DecomposeHmenuIdentityString(&mut self, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IAccPropServices_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>() -> IAccPropServices_Vtbl {
        unsafe extern "system" fn SetPropValue<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPropValue(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn SetPropServer<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPropServer(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::core::mem::transmute(&pserver), ::core::mem::transmute_copy(&annoscope)).into()
        }
        unsafe extern "system" fn ClearProps<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearProps(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into()
        }
        unsafe extern "system" fn SetHwndProp<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHwndProp(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn SetHwndPropStr<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHwndPropStr(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&str)).into()
        }
        unsafe extern "system" fn SetHwndPropServer<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHwndPropServer(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::core::mem::transmute(&pserver), ::core::mem::transmute_copy(&annoscope)).into()
        }
        unsafe extern "system" fn ClearHwndProps<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearHwndProps(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into()
        }
        unsafe extern "system" fn ComposeHwndIdentityString<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ComposeHwndIdentityString(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into()
        }
        unsafe extern "system" fn DecomposeHwndIdentityString<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DecomposeHwndIdentityString(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pidobject), ::core::mem::transmute_copy(&pidchild)).into()
        }
        unsafe extern "system" fn SetHmenuProp<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHmenuProp(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&var)).into()
        }
        unsafe extern "system" fn SetHmenuPropStr<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHmenuPropStr(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&idprop), ::core::mem::transmute_copy(&str)).into()
        }
        unsafe extern "system" fn SetHmenuPropServer<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHmenuPropServer(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::core::mem::transmute(&pserver), ::core::mem::transmute_copy(&annoscope)).into()
        }
        unsafe extern "system" fn ClearHmenuProps<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearHmenuProps(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into()
        }
        unsafe extern "system" fn ComposeHmenuIdentityString<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ComposeHmenuIdentityString(::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into()
        }
        unsafe extern "system" fn DecomposeHmenuIdentityString<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServices_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DecomposeHmenuIdentityString(::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&phmenu), ::core::mem::transmute_copy(&pidchild)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetPropValue: SetPropValue::<Identity, Impl, OFFSET>,
            SetPropServer: SetPropServer::<Identity, Impl, OFFSET>,
            ClearProps: ClearProps::<Identity, Impl, OFFSET>,
            SetHwndProp: SetHwndProp::<Identity, Impl, OFFSET>,
            SetHwndPropStr: SetHwndPropStr::<Identity, Impl, OFFSET>,
            SetHwndPropServer: SetHwndPropServer::<Identity, Impl, OFFSET>,
            ClearHwndProps: ClearHwndProps::<Identity, Impl, OFFSET>,
            ComposeHwndIdentityString: ComposeHwndIdentityString::<Identity, Impl, OFFSET>,
            DecomposeHwndIdentityString: DecomposeHwndIdentityString::<Identity, Impl, OFFSET>,
            SetHmenuProp: SetHmenuProp::<Identity, Impl, OFFSET>,
            SetHmenuPropStr: SetHmenuPropStr::<Identity, Impl, OFFSET>,
            SetHmenuPropServer: SetHmenuPropServer::<Identity, Impl, OFFSET>,
            ClearHmenuProps: ClearHmenuProps::<Identity, Impl, OFFSET>,
            ComposeHmenuIdentityString: ComposeHmenuIdentityString::<Identity, Impl, OFFSET>,
            DecomposeHmenuIdentityString: DecomposeHmenuIdentityString::<Identity, Impl, OFFSET>,
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
    fn accChild(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn accName(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accValue(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accDescription(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accRole(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accState(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accHelp(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accHelpTopic(&mut self, pszhelpfile: *mut super::super::Foundation::BSTR, varchild: &super::super::System::Com::VARIANT, pidtopic: *mut i32) -> ::windows::core::Result<()>;
    fn accKeyboardShortcut(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accFocus(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accSelection(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accDefaultAction(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn accSelect(&mut self, flagsselect: i32, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn accLocation(&mut self, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn accNavigate(&mut self, navdir: i32, varstart: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accHitTest(&mut self, xleft: i32, ytop: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn accDoDefaultAction(&mut self, varchild: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetaccName(&mut self, varchild: &super::super::System::Com::VARIANT, szname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetaccValue(&mut self, varchild: &super::super::System::Com::VARIANT, szvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAccessible_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>() -> IAccessible_Vtbl {
        unsafe extern "system" fn accParent<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdispparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accParent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdispparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accChildCount<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcountchildren: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accChildCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcountchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accChild<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppdispchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accChild(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdispchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accName<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accName(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accValue<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accValue(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDescription<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accDescription(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accRole<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarrole: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accRole(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accState<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarstate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accState(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHelp<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accHelp(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszhelp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHelpTopic<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpfile: *mut super::super::Foundation::BSTR, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pidtopic: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).accHelpTopic(::core::mem::transmute_copy(&pszhelpfile), ::core::mem::transmute_copy(&varchild), ::core::mem::transmute_copy(&pidtopic)).into()
        }
        unsafe extern "system" fn accKeyboardShortcut<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accKeyboardShortcut(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszkeyboardshortcut = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accFocus<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accSelection<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchildren: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDefaultAction<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accDefaultAction(::core::mem::transmute_copy(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszdefaultaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accSelect<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).accSelect(::core::mem::transmute_copy(&flagsselect), ::core::mem::transmute_copy(&varchild)).into()
        }
        unsafe extern "system" fn accLocation<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).accLocation(::core::mem::transmute_copy(&pxleft), ::core::mem::transmute_copy(&pytop), ::core::mem::transmute_copy(&pcxwidth), ::core::mem::transmute_copy(&pcyheight), ::core::mem::transmute_copy(&varchild)).into()
        }
        unsafe extern "system" fn accNavigate<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, navdir: i32, varstart: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarendupat: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accNavigate(::core::mem::transmute_copy(&navdir), ::core::mem::transmute_copy(&varstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarendupat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHitTest<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xleft: i32, ytop: i32, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).accHitTest(::core::mem::transmute_copy(&xleft), ::core::mem::transmute_copy(&ytop)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarchild = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDoDefaultAction<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).accDoDefaultAction(::core::mem::transmute_copy(&varchild)).into()
        }
        unsafe extern "system" fn SetaccName<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, szname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetaccName(::core::mem::transmute_copy(&varchild), ::core::mem::transmute_copy(&szname)).into()
        }
        unsafe extern "system" fn SetaccValue<Identity: ::windows::core::IUnknownImpl, Impl: IAccessible_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, szvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetaccValue(::core::mem::transmute_copy(&varchild), ::core::mem::transmute_copy(&szvalue)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            accParent: accParent::<Identity, Impl, OFFSET>,
            accChildCount: accChildCount::<Identity, Impl, OFFSET>,
            accChild: accChild::<Identity, Impl, OFFSET>,
            accName: accName::<Identity, Impl, OFFSET>,
            accValue: accValue::<Identity, Impl, OFFSET>,
            accDescription: accDescription::<Identity, Impl, OFFSET>,
            accRole: accRole::<Identity, Impl, OFFSET>,
            accState: accState::<Identity, Impl, OFFSET>,
            accHelp: accHelp::<Identity, Impl, OFFSET>,
            accHelpTopic: accHelpTopic::<Identity, Impl, OFFSET>,
            accKeyboardShortcut: accKeyboardShortcut::<Identity, Impl, OFFSET>,
            accFocus: accFocus::<Identity, Impl, OFFSET>,
            accSelection: accSelection::<Identity, Impl, OFFSET>,
            accDefaultAction: accDefaultAction::<Identity, Impl, OFFSET>,
            accSelect: accSelect::<Identity, Impl, OFFSET>,
            accLocation: accLocation::<Identity, Impl, OFFSET>,
            accNavigate: accNavigate::<Identity, Impl, OFFSET>,
            accHitTest: accHitTest::<Identity, Impl, OFFSET>,
            accDoDefaultAction: accDoDefaultAction::<Identity, Impl, OFFSET>,
            SetaccName: SetaccName::<Identity, Impl, OFFSET>,
            SetaccValue: SetaccValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessible as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleEx_Impl: Sized {
    fn GetObjectForChild(&mut self, idchild: i32) -> ::windows::core::Result<IAccessibleEx>;
    fn GetIAccessiblePair(&mut self, ppacc: *mut ::core::option::Option<IAccessible>, pidchild: *mut i32) -> ::windows::core::Result<()>;
    fn GetRuntimeId(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ConvertReturnedElement(&mut self, pin: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<IAccessibleEx>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleEx_Impl, const OFFSET: isize>() -> IAccessibleEx_Vtbl {
        unsafe extern "system" fn GetObjectForChild<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idchild: i32, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObjectForChild(::core::mem::transmute_copy(&idchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIAccessiblePair<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacc: *mut ::windows::core::RawPtr, pidchild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetIAccessiblePair(::core::mem::transmute_copy(&ppacc), ::core::mem::transmute_copy(&pidchild)).into()
        }
        unsafe extern "system" fn GetRuntimeId<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertReturnedElement<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, ppretvalout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConvertReturnedElement(::core::mem::transmute(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppretvalout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetObjectForChild: GetObjectForChild::<Identity, Impl, OFFSET>,
            GetIAccessiblePair: GetIAccessiblePair::<Identity, Impl, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, Impl, OFFSET>,
            ConvertReturnedElement: ConvertReturnedElement::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHandler_Impl, const OFFSET: isize>() -> IAccessibleHandler_Vtbl {
        unsafe extern "system" fn AccessibleObjectFromID<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, lobjectid: i32, piaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AccessibleObjectFromID(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&lobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *piaccessible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AccessibleObjectFromID: AccessibleObjectFromID::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibleHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleHostingElementProviders_Impl: Sized {
    fn GetEmbeddedFragmentRoots(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetObjectIdForProvider(&mut self, pprovider: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleHostingElementProviders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>() -> IAccessibleHostingElementProviders_Vtbl {
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEmbeddedFragmentRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectIdForProvider<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, pidobject: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetObjectIdForProvider(::core::mem::transmute(&pprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *pidobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Identity, Impl, OFFSET>,
            GetObjectIdForProvider: GetObjectIdForProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibleHostingElementProviders as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleWindowlessSite_Impl: Sized {
    fn AcquireObjectIdRange(&mut self, rangesize: i32, prangeowner: &::core::option::Option<IAccessibleHandler>) -> ::windows::core::Result<i32>;
    fn ReleaseObjectIdRange(&mut self, rangebase: i32, prangeowner: &::core::option::Option<IAccessibleHandler>) -> ::windows::core::Result<()>;
    fn QueryObjectIdRanges(&mut self, prangesowner: &::core::option::Option<IAccessibleHandler>) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetParentAccessible(&mut self) -> ::windows::core::Result<IAccessible>;
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleWindowlessSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>() -> IAccessibleWindowlessSite_Vtbl {
        unsafe extern "system" fn AcquireObjectIdRange<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangesize: i32, prangeowner: ::windows::core::RawPtr, prangebase: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AcquireObjectIdRange(::core::mem::transmute_copy(&rangesize), ::core::mem::transmute(&prangeowner)) {
                ::core::result::Result::Ok(ok__) => {
                    *prangebase = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseObjectIdRange<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangebase: i32, prangeowner: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseObjectIdRange(::core::mem::transmute_copy(&rangebase), ::core::mem::transmute(&prangeowner)).into()
        }
        unsafe extern "system" fn QueryObjectIdRanges<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangesowner: ::windows::core::RawPtr, psaranges: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryObjectIdRanges(::core::mem::transmute(&prangesowner)) {
                ::core::result::Result::Ok(ok__) => {
                    *psaranges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentAccessible<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParentAccessible() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AcquireObjectIdRange: AcquireObjectIdRange::<Identity, Impl, OFFSET>,
            ReleaseObjectIdRange: ReleaseObjectIdRange::<Identity, Impl, OFFSET>,
            QueryObjectIdRanges: QueryObjectIdRanges::<Identity, Impl, OFFSET>,
            GetParentAccessible: GetParentAccessible::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>() -> IAnnotationProvider_Vtbl {
        unsafe extern "system" fn AnnotationTypeId<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnnotationTypeName<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTime<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Target<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AnnotationTypeId: AnnotationTypeId::<Identity, Impl, OFFSET>,
            AnnotationTypeName: AnnotationTypeName::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            DateTime: DateTime::<Identity, Impl, OFFSET>,
            Target: Target::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomNavigationProvider_Impl, const OFFSET: isize>() -> ICustomNavigationProvider_Vtbl {
        unsafe extern "system" fn Navigate<Identity: ::windows::core::IUnknownImpl, Impl: ICustomNavigationProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Navigate(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Navigate: Navigate::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockProvider_Impl, const OFFSET: isize>() -> IDockProvider_Vtbl {
        unsafe extern "system" fn SetDockPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDockProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockposition: DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDockPosition(::core::mem::transmute_copy(&dockposition)).into()
        }
        unsafe extern "system" fn DockPosition<Identity: ::windows::core::IUnknownImpl, Impl: IDockProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDockPosition: SetDockPosition::<Identity, Impl, OFFSET>,
            DockPosition: DockPosition::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>() -> IDragProvider_Vtbl {
        unsafe extern "system" fn IsGrabbed<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffects<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGrabbedItems<Identity: ::windows::core::IUnknownImpl, Impl: IDragProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsGrabbed: IsGrabbed::<Identity, Impl, OFFSET>,
            DropEffect: DropEffect::<Identity, Impl, OFFSET>,
            DropEffects: DropEffects::<Identity, Impl, OFFSET>,
            GetGrabbedItems: GetGrabbedItems::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProvider_Impl, const OFFSET: isize>() -> IDropTargetProvider_Vtbl {
        unsafe extern "system" fn DropTargetEffect<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DropTargetEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropTargetEffects<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DropTargetEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DropTargetEffect: DropTargetEffect::<Identity, Impl, OFFSET>,
            DropTargetEffects: DropTargetEffects::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>() -> IExpandCollapseProvider_Vtbl {
        unsafe extern "system" fn Expand<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Expand().into()
        }
        unsafe extern "system" fn Collapse<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Collapse().into()
        }
        unsafe extern "system" fn ExpandCollapseState<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Expand: Expand::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            ExpandCollapseState: ExpandCollapseState::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>() -> IGridItemProvider_Vtbl {
        unsafe extern "system" fn Row<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Row() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Column<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Column() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowSpan<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnSpan<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainingGrid<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Row: Row::<Identity, Impl, OFFSET>,
            Column: Column::<Identity, Impl, OFFSET>,
            RowSpan: RowSpan::<Identity, Impl, OFFSET>,
            ColumnSpan: ColumnSpan::<Identity, Impl, OFFSET>,
            ContainingGrid: ContainingGrid::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridProvider_Impl, const OFFSET: isize>() -> IGridProvider_Vtbl {
        unsafe extern "system" fn GetItem<Identity: ::windows::core::IUnknownImpl, Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowCount<Identity: ::windows::core::IUnknownImpl, Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RowCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnCount<Identity: ::windows::core::IUnknownImpl, Impl: IGridProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            RowCount: RowCount::<Identity, Impl, OFFSET>,
            ColumnCount: ColumnCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInvokeProvider_Impl, const OFFSET: isize>() -> IInvokeProvider_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IInvokeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInvokeProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IItemContainerProvider_Impl: Sized {
    fn FindItemByProperty(&mut self, pstartafter: &::core::option::Option<IRawElementProviderSimple>, propertyid: i32, value: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IItemContainerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerProvider_Impl, const OFFSET: isize>() -> IItemContainerProvider_Vtbl {
        unsafe extern "system" fn FindItemByProperty<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: ::windows::core::RawPtr, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindItemByProperty(::core::mem::transmute(&pstartafter), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfound = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), FindItemByProperty: FindItemByProperty::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>() -> ILegacyIAccessibleProvider_Vtbl {
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select(::core::mem::transmute_copy(&flagsselect)).into()
        }
        unsafe extern "system" fn DoDefaultAction<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DoDefaultAction().into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&szvalue)).into()
        }
        unsafe extern "system" fn GetIAccessible<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIAccessible() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaccessible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildId<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChildId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Role<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Role() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Help<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Help() {
                ::core::result::Result::Ok(ok__) => {
                    *pszhelp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardShortcut<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).KeyboardShortcut() {
                ::core::result::Result::Ok(ok__) => {
                    *pszkeyboardshortcut = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselectedchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultAction<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdefaultaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, Impl, OFFSET>,
            DoDefaultAction: DoDefaultAction::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetIAccessible: GetIAccessible::<Identity, Impl, OFFSET>,
            ChildId: ChildId::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Role: Role::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Help: Help::<Identity, Impl, OFFSET>,
            KeyboardShortcut: KeyboardShortcut::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            DefaultAction: DefaultAction::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>() -> IMultipleViewProvider_Vtbl {
        unsafe extern "system" fn GetViewName<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetViewName(::core::mem::transmute_copy(&viewid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCurrentView(::core::mem::transmute_copy(&viewid)).into()
        }
        unsafe extern "system" fn CurrentView<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedViews<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetViewName: GetViewName::<Identity, Impl, OFFSET>,
            SetCurrentView: SetCurrentView::<Identity, Impl, OFFSET>,
            CurrentView: CurrentView::<Identity, Impl, OFFSET>,
            GetSupportedViews: GetSupportedViews::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectModelProvider_Impl, const OFFSET: isize>() -> IObjectModelProvider_Vtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Identity: ::windows::core::IUnknownImpl, Impl: IObjectModelProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUnderlyingObjectModel() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectModelProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProxyProviderWinEventHandler_Impl: Sized {
    fn RespondToWinEvent(&mut self, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: &::core::option::Option<IProxyProviderWinEventSink>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProxyProviderWinEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventHandler_Impl, const OFFSET: isize>() -> IProxyProviderWinEventHandler_Vtbl {
        unsafe extern "system" fn RespondToWinEvent<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RespondToWinEvent(::core::mem::transmute_copy(&idwinevent), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&psink)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), RespondToWinEvent: RespondToWinEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProxyProviderWinEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProxyProviderWinEventSink_Impl: Sized {
    fn AddAutomationPropertyChangedEvent(&mut self, pprovider: &::core::option::Option<IRawElementProviderSimple>, id: i32, newvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddAutomationEvent(&mut self, pprovider: &::core::option::Option<IRawElementProviderSimple>, id: i32) -> ::windows::core::Result<()>;
    fn AddStructureChangedEvent(&mut self, pprovider: &::core::option::Option<IRawElementProviderSimple>, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProxyProviderWinEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>() -> IProxyProviderWinEventSink_Vtbl {
        unsafe extern "system" fn AddAutomationPropertyChangedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, id: i32, newvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAutomationPropertyChangedEvent(::core::mem::transmute(&pprovider), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&newvalue)).into()
        }
        unsafe extern "system" fn AddAutomationEvent<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAutomationEvent(::core::mem::transmute(&pprovider), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn AddStructureChangedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStructureChangedEvent(::core::mem::transmute(&pprovider), ::core::mem::transmute_copy(&structurechangetype), ::core::mem::transmute_copy(&runtimeid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddAutomationPropertyChangedEvent: AddAutomationPropertyChangedEvent::<Identity, Impl, OFFSET>,
            AddAutomationEvent: AddAutomationEvent::<Identity, Impl, OFFSET>,
            AddStructureChangedEvent: AddStructureChangedEvent::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>() -> IRangeValueProvider_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maximum<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Maximum() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minimum<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Minimum() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeChange<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmallChange<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            Maximum: Maximum::<Identity, Impl, OFFSET>,
            Minimum: Minimum::<Identity, Impl, OFFSET>,
            LargeChange: LargeChange::<Identity, Impl, OFFSET>,
            SmallChange: SmallChange::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>() -> IRawElementProviderAdviseEvents_Vtbl {
        unsafe extern "system" fn AdviseEventAdded<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AdviseEventAdded(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyids)).into()
        }
        unsafe extern "system" fn AdviseEventRemoved<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AdviseEventRemoved(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyids)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AdviseEventAdded: AdviseEventAdded::<Identity, Impl, OFFSET>,
            AdviseEventRemoved: AdviseEventRemoved::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>() -> IRawElementProviderFragment_Vtbl {
        unsafe extern "system" fn Navigate<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Navigate(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeId<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut UiaRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEmbeddedFragmentRoots() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFocus().into()
        }
        unsafe extern "system" fn FragmentRoot<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FragmentRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Navigate: Navigate::<Identity, Impl, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, Impl, OFFSET>,
            BoundingRectangle: BoundingRectangle::<Identity, Impl, OFFSET>,
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Identity, Impl, OFFSET>,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            FragmentRoot: FragmentRoot::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>() -> IRawElementProviderFragmentRoot_Vtbl {
        unsafe extern "system" fn ElementProviderFromPoint<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElementProviderFromPoint(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ElementProviderFromPoint: ElementProviderFromPoint::<Identity, Impl, OFFSET>,
            GetFocus: GetFocus::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHostingAccessibles_Impl, const OFFSET: isize>() -> IRawElementProviderHostingAccessibles_Vtbl {
        unsafe extern "system" fn GetEmbeddedAccessibles<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHostingAccessibles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEmbeddedAccessibles() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetEmbeddedAccessibles: GetEmbeddedAccessibles::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHwndOverride_Impl, const OFFSET: isize>() -> IRawElementProviderHwndOverride_Vtbl {
        unsafe extern "system" fn GetOverrideProviderForHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHwndOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOverrideProviderForHwnd(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetOverrideProviderForHwnd: GetOverrideProviderForHwnd::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>() -> IRawElementProviderSimple_Vtbl {
        unsafe extern "system" fn ProviderOptions<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ProviderOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProviderOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatternProvider<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPatternProvider(::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertyValue(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostRawElementProvider<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HostRawElementProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProviderOptions: ProviderOptions::<Identity, Impl, OFFSET>,
            GetPatternProvider: GetPatternProvider::<Identity, Impl, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, Impl, OFFSET>,
            HostRawElementProvider: HostRawElementProvider::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple2_Impl, const OFFSET: isize>() -> IRawElementProviderSimple2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        Self { base: IRawElementProviderSimple_Vtbl::new::<Identity, Impl, OFFSET>(), ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderSimple2 as ::windows::core::Interface>::IID || iid == &<IRawElementProviderSimple as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawElementProviderSimple3_Impl: Sized + IRawElementProviderSimple_Impl + IRawElementProviderSimple2_Impl {
    fn GetMetadataValue(&mut self, targetid: i32, metadataid: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawElementProviderSimple3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple3_Impl, const OFFSET: isize>() -> IRawElementProviderSimple3_Vtbl {
        unsafe extern "system" fn GetMetadataValue<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: i32, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataValue(::core::mem::transmute_copy(&targetid), ::core::mem::transmute_copy(&metadataid)) {
                ::core::result::Result::Ok(ok__) => {
                    *returnval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IRawElementProviderSimple2_Vtbl::new::<Identity, Impl, OFFSET>(), GetMetadataValue: GetMetadataValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderSimple3 as ::windows::core::Interface>::IID || iid == &<IRawElementProviderSimple as ::windows::core::Interface>::IID || iid == &<IRawElementProviderSimple2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderWindowlessSite_Impl: Sized {
    fn GetAdjacentFragment(&mut self, direction: NavigateDirection) -> ::windows::core::Result<IRawElementProviderFragment>;
    fn GetRuntimeIdPrefix(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderWindowlessSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>() -> IRawElementProviderWindowlessSite_Vtbl {
        unsafe extern "system" fn GetAdjacentFragment<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAdjacentFragment(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeIdPrefix<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRuntimeIdPrefix() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAdjacentFragment: GetAdjacentFragment::<Identity, Impl, OFFSET>,
            GetRuntimeIdPrefix: GetRuntimeIdPrefix::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditUiaInformation_Impl, const OFFSET: isize>() -> IRichEditUiaInformation_Vtbl {
        unsafe extern "system" fn GetBoundaryRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiarect: *mut UiaRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBoundaryRectangle(::core::mem::transmute_copy(&puiarect)).into()
        }
        unsafe extern "system" fn IsVisible<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsVisible().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBoundaryRectangle: GetBoundaryRectangle::<Identity, Impl, OFFSET>,
            IsVisible: IsVisible::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditUiaInformation as ::windows::core::Interface>::IID
    }
}
pub trait IRicheditWindowlessAccessibility_Impl: Sized {
    fn CreateProvider(&mut self, psite: &::core::option::Option<IRawElementProviderWindowlessSite>) -> ::windows::core::Result<IRawElementProviderSimple>;
}
impl IRicheditWindowlessAccessibility_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditWindowlessAccessibility_Impl, const OFFSET: isize>() -> IRicheditWindowlessAccessibility_Vtbl {
        unsafe extern "system" fn CreateProvider<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditWindowlessAccessibility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psite: ::windows::core::RawPtr, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateProvider(::core::mem::transmute(&psite)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprovider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateProvider: CreateProvider::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRicheditWindowlessAccessibility as ::windows::core::Interface>::IID
    }
}
pub trait IScrollItemProvider_Impl: Sized {
    fn ScrollIntoView(&mut self) -> ::windows::core::Result<()>;
}
impl IScrollItemProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollItemProvider_Impl, const OFFSET: isize>() -> IScrollItemProvider_Vtbl {
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows::core::IUnknownImpl, Impl: IScrollItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScrollIntoView().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>() -> IScrollProvider_Vtbl {
        unsafe extern "system" fn Scroll<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Scroll(::core::mem::transmute_copy(&horizontalamount), ::core::mem::transmute_copy(&verticalamount)).into()
        }
        unsafe extern "system" fn SetScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScrollPercent(::core::mem::transmute_copy(&horizontalpercent), ::core::mem::transmute_copy(&verticalpercent)).into()
        }
        unsafe extern "system" fn HorizontalScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalViewSize<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalViewSize<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontallyScrollable<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticallyScrollable<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            SetScrollPercent: SetScrollPercent::<Identity, Impl, OFFSET>,
            HorizontalScrollPercent: HorizontalScrollPercent::<Identity, Impl, OFFSET>,
            VerticalScrollPercent: VerticalScrollPercent::<Identity, Impl, OFFSET>,
            HorizontalViewSize: HorizontalViewSize::<Identity, Impl, OFFSET>,
            VerticalViewSize: VerticalViewSize::<Identity, Impl, OFFSET>,
            HorizontallyScrollable: HorizontallyScrollable::<Identity, Impl, OFFSET>,
            VerticallyScrollable: VerticallyScrollable::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>() -> ISelectionItemProvider_Vtbl {
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn IsSelected<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContainer<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            SelectionContainer: SelectionContainer::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider_Impl, const OFFSET: isize>() -> ISelectionProvider_Vtbl {
        unsafe extern "system" fn GetSelection<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanSelectMultiple<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelectionRequired<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            CanSelectMultiple: CanSelectMultiple::<Identity, Impl, OFFSET>,
            IsSelectionRequired: IsSelectionRequired::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider2_Impl, const OFFSET: isize>() -> ISelectionProvider2_Vtbl {
        unsafe extern "system" fn FirstSelectedItem<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FirstSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastSelectedItem<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LastSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSelectedItem<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISelectionProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            FirstSelectedItem: FirstSelectedItem::<Identity, Impl, OFFSET>,
            LastSelectedItem: LastSelectedItem::<Identity, Impl, OFFSET>,
            CurrentSelectedItem: CurrentSelectedItem::<Identity, Impl, OFFSET>,
            ItemCount: ItemCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectionProvider2 as ::windows::core::Interface>::IID || iid == &<ISelectionProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>() -> ISpreadsheetItemProvider_Vtbl {
        unsafe extern "system" fn Formula<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Formula() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationObjects<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationTypes<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Formula: Formula::<Identity, Impl, OFFSET>,
            GetAnnotationObjects: GetAnnotationObjects::<Identity, Impl, OFFSET>,
            GetAnnotationTypes: GetAnnotationTypes::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetProvider_Impl, const OFFSET: isize>() -> ISpreadsheetProvider_Vtbl {
        unsafe extern "system" fn GetItemByName<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetItemByName: GetItemByName::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>() -> IStylesProvider_Vtbl {
        unsafe extern "system" fn StyleId<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StyleId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleName<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StyleName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillColor<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternStyle<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternColor<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StyleId: StyleId::<Identity, Impl, OFFSET>,
            StyleName: StyleName::<Identity, Impl, OFFSET>,
            FillColor: FillColor::<Identity, Impl, OFFSET>,
            FillPatternStyle: FillPatternStyle::<Identity, Impl, OFFSET>,
            Shape: Shape::<Identity, Impl, OFFSET>,
            FillPatternColor: FillPatternColor::<Identity, Impl, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>() -> ISynchronizedInputProvider_Vtbl {
        unsafe extern "system" fn StartListening<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartListening(::core::mem::transmute_copy(&inputtype)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StartListening: StartListening::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProvider_Impl, const OFFSET: isize>() -> ITableItemProvider_Vtbl {
        unsafe extern "system" fn GetRowHeaderItems<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaderItems<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRowHeaderItems: GetRowHeaderItems::<Identity, Impl, OFFSET>,
            GetColumnHeaderItems: GetColumnHeaderItems::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableProvider_Impl, const OFFSET: isize>() -> ITableProvider_Vtbl {
        unsafe extern "system" fn GetRowHeaders<Identity: ::windows::core::IUnknownImpl, Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: ::windows::core::IUnknownImpl, Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowOrColumnMajor<Identity: ::windows::core::IUnknownImpl, Impl: ITableProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRowHeaders: GetRowHeaders::<Identity, Impl, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, Impl, OFFSET>,
            RowOrColumnMajor: RowOrColumnMajor::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProvider_Impl, const OFFSET: isize>() -> ITextChildProvider_Vtbl {
        unsafe extern "system" fn TextContainer<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TextContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TextRange() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TextContainer: TextContainer::<Identity, Impl, OFFSET>,
            TextRange: TextRange::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProvider_Impl, const OFFSET: isize>() -> ITextEditProvider_Vtbl {
        unsafe extern "system" fn GetActiveComposition<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActiveComposition() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConversionTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITextProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Identity, Impl, OFFSET>,
            GetConversionTarget: GetConversionTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextEditProvider as ::windows::core::Interface>::IID || iid == &<ITextProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextProvider_Impl: Sized {
    fn GetSelection(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleRanges(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn RangeFromChild(&mut self, childelement: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn RangeFromPoint(&mut self, point: &UiaPoint) -> ::windows::core::Result<ITextRangeProvider>;
    fn DocumentRange(&mut self) -> ::windows::core::Result<ITextRangeProvider>;
    fn SupportedTextSelection(&mut self) -> ::windows::core::Result<SupportedTextSelection>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITextProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>() -> ITextProvider_Vtbl {
        unsafe extern "system" fn GetSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVisibleRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromChild<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childelement: ::windows::core::RawPtr, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromChild(::core::mem::transmute(&childelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: UiaPoint, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromPoint(::core::mem::transmute_copy(&point)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DocumentRange() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedTextSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Identity, Impl, OFFSET>,
            RangeFromChild: RangeFromChild::<Identity, Impl, OFFSET>,
            RangeFromPoint: RangeFromPoint::<Identity, Impl, OFFSET>,
            DocumentRange: DocumentRange::<Identity, Impl, OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITextProvider2_Impl: Sized + ITextProvider_Impl {
    fn RangeFromAnnotation(&mut self, annotationelement: &::core::option::Option<IRawElementProviderSimple>) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetCaretRange(&mut self, isactive: *mut super::super::Foundation::BOOL, pretval: *mut ::core::option::Option<ITextRangeProvider>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITextProvider2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2_Impl, const OFFSET: isize>() -> ITextProvider2_Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationelement: ::windows::core::RawPtr, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromAnnotation(::core::mem::transmute(&annotationelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaretRange(::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&pretval)).into()
        }
        Self {
            base: ITextProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Identity, Impl, OFFSET>,
            GetCaretRange: GetCaretRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextProvider2 as ::windows::core::Interface>::IID || iid == &<ITextProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITextRangeProvider_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<ITextRangeProvider>;
    fn Compare(&mut self, range: &::core::option::Option<ITextRangeProvider>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CompareEndpoints(&mut self, endpoint: TextPatternRangeEndpoint, targetrange: &::core::option::Option<ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<i32>;
    fn ExpandToEnclosingUnit(&mut self, unit: TextUnit) -> ::windows::core::Result<()>;
    fn FindAttribute(&mut self, attributeid: i32, val: &super::super::System::Com::VARIANT, backward: super::super::Foundation::BOOL) -> ::windows::core::Result<ITextRangeProvider>;
    fn FindText(&mut self, text: &super::super::Foundation::BSTR, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL) -> ::windows::core::Result<ITextRangeProvider>;
    fn GetAttributeValue(&mut self, attributeid: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetBoundingRectangles(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEnclosingElement(&mut self) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn GetText(&mut self, maxlength: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Move(&mut self, unit: TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByUnit(&mut self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByRange(&mut self, endpoint: TextPatternRangeEndpoint, targetrange: &::core::option::Option<ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>;
    fn Select(&mut self) -> ::windows::core::Result<()>;
    fn AddToSelection(&mut self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&mut self) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&mut self, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetChildren(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRangeProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>() -> ITextRangeProvider_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Compare(::core::mem::transmute(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompareEndpoints(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute(&targetrange), ::core::mem::transmute_copy(&targetendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExpandToEnclosingUnit(::core::mem::transmute_copy(&unit)).into()
        }
        unsafe extern "system" fn FindAttribute<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, val: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, backward: super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindAttribute(::core::mem::transmute_copy(&attributeid), ::core::mem::transmute_copy(&val), ::core::mem::transmute_copy(&backward)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindText(::core::mem::transmute_copy(&text), ::core::mem::transmute_copy(&backward), ::core::mem::transmute_copy(&ignorecase)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeValue(::core::mem::transmute_copy(&attributeid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBoundingRectangles() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnclosingElement() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&maxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Move(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveEndpointByUnit(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveEndpointByRange(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute(&targetrange), ::core::mem::transmute_copy(&targetendpoint)).into()
        }
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScrollIntoView(::core::mem::transmute_copy(&aligntotop)).into()
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Compare: Compare::<Identity, Impl, OFFSET>,
            CompareEndpoints: CompareEndpoints::<Identity, Impl, OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Identity, Impl, OFFSET>,
            FindAttribute: FindAttribute::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, Impl, OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Identity, Impl, OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Identity, Impl, OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider2_Impl, const OFFSET: isize>() -> ITextRangeProvider2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        Self { base: ITextRangeProvider_Vtbl::new::<Identity, Impl, OFFSET>(), ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRangeProvider2 as ::windows::core::Interface>::IID || iid == &<ITextRangeProvider as ::windows::core::Interface>::IID
    }
}
pub trait IToggleProvider_Impl: Sized {
    fn Toggle(&mut self) -> ::windows::core::Result<()>;
    fn ToggleState(&mut self) -> ::windows::core::Result<ToggleState>;
}
impl IToggleProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProvider_Impl, const OFFSET: isize>() -> IToggleProvider_Vtbl {
        unsafe extern "system" fn Toggle<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Toggle().into()
        }
        unsafe extern "system" fn ToggleState<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Toggle: Toggle::<Identity, Impl, OFFSET>,
            ToggleState: ToggleState::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>() -> ITransformProvider_Vtbl {
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Resize<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rotate(::core::mem::transmute_copy(&degrees)).into()
        }
        unsafe extern "system" fn CanMove<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanMove() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanResize<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanResize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRotate<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Move: Move::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            CanMove: CanMove::<Identity, Impl, OFFSET>,
            CanResize: CanResize::<Identity, Impl, OFFSET>,
            CanRotate: CanRotate::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>() -> ITransformProvider2_Vtbl {
        unsafe extern "system" fn Zoom<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Zoom(::core::mem::transmute_copy(&zoom)).into()
        }
        unsafe extern "system" fn CanZoom<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomMinimum<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ZoomMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomMaximum<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ZoomMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomByUnit<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ZoomByUnit(::core::mem::transmute_copy(&zoomunit)).into()
        }
        Self {
            base: ITransformProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            Zoom: Zoom::<Identity, Impl, OFFSET>,
            CanZoom: CanZoom::<Identity, Impl, OFFSET>,
            ZoomLevel: ZoomLevel::<Identity, Impl, OFFSET>,
            ZoomMinimum: ZoomMinimum::<Identity, Impl, OFFSET>,
            ZoomMaximum: ZoomMaximum::<Identity, Impl, OFFSET>,
            ZoomByUnit: ZoomByUnit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransformProvider2 as ::windows::core::Interface>::IID || iid == &<ITransformProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation_Impl: Sized {
    fn CompareElements(&mut self, el1: &::core::option::Option<IUIAutomationElement>, el2: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CompareRuntimeIds(&mut self, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetRootElement(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromHandle(&mut self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromPoint(&mut self, pt: &super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetFocusedElement(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetRootElementBuildCache(&mut self, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromHandleBuildCache(&mut self, hwnd: super::super::Foundation::HWND, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromPointBuildCache(&mut self, pt: &super::super::Foundation::POINT, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetFocusedElementBuildCache(&mut self, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn CreateTreeWalker(&mut self, pcondition: &::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationTreeWalker>;
    fn ControlViewWalker(&mut self) -> ::windows::core::Result<IUIAutomationTreeWalker>;
    fn ContentViewWalker(&mut self) -> ::windows::core::Result<IUIAutomationTreeWalker>;
    fn RawViewWalker(&mut self) -> ::windows::core::Result<IUIAutomationTreeWalker>;
    fn RawViewCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn ControlViewCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn ContentViewCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateCacheRequest(&mut self) -> ::windows::core::Result<IUIAutomationCacheRequest>;
    fn CreateTrueCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateFalseCondition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreatePropertyCondition(&mut self, propertyid: i32, value: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreatePropertyConditionEx(&mut self, propertyid: i32, value: &super::super::System::Com::VARIANT, flags: PropertyConditionFlags) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateAndCondition(&mut self, condition1: &::core::option::Option<IUIAutomationCondition>, condition2: &::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromArray(&mut self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromNativeArray(&mut self, conditions: *const ::core::option::Option<IUIAutomationCondition>, conditioncount: i32) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateOrCondition(&mut self, condition1: &::core::option::Option<IUIAutomationCondition>, condition2: &::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromArray(&mut self, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromNativeArray(&mut self, conditions: *const ::core::option::Option<IUIAutomationCondition>, conditioncount: i32) -> ::windows::core::Result<IUIAutomationCondition>;
    fn CreateNotCondition(&mut self, condition: &::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationCondition>;
    fn AddAutomationEventHandler(&mut self, eventid: i32, element: &::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveAutomationEventHandler(&mut self, eventid: i32, element: &::core::option::Option<IUIAutomationElement>, handler: &::core::option::Option<IUIAutomationEventHandler>) -> ::windows::core::Result<()>;
    fn AddPropertyChangedEventHandlerNativeArray(&mut self, element: &::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const i32, propertycount: i32) -> ::windows::core::Result<()>;
    fn AddPropertyChangedEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RemovePropertyChangedEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, handler: &::core::option::Option<IUIAutomationPropertyChangedEventHandler>) -> ::windows::core::Result<()>;
    fn AddStructureChangedEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationStructureChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveStructureChangedEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, handler: &::core::option::Option<IUIAutomationStructureChangedEventHandler>) -> ::windows::core::Result<()>;
    fn AddFocusChangedEventHandler(&mut self, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationFocusChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveFocusChangedEventHandler(&mut self, handler: &::core::option::Option<IUIAutomationFocusChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveAllEventHandlers(&mut self) -> ::windows::core::Result<()>;
    fn IntNativeArrayToSafeArray(&mut self, array: *const i32, arraycount: i32) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn IntSafeArrayToNativeArray(&mut self, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::Result<()>;
    fn RectToVariant(&mut self, rc: &super::super::Foundation::RECT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn VariantToRect(&mut self, var: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SafeArrayToRectNativeArray(&mut self, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::Result<()>;
    fn CreateProxyFactoryEntry(&mut self, factory: &::core::option::Option<IUIAutomationProxyFactory>) -> ::windows::core::Result<IUIAutomationProxyFactoryEntry>;
    fn ProxyFactoryMapping(&mut self) -> ::windows::core::Result<IUIAutomationProxyFactoryMapping>;
    fn GetPropertyProgrammaticName(&mut self, property: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPatternProgrammaticName(&mut self, pattern: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PollForPotentialSupportedPatterns(&mut self, pelement: &::core::option::Option<IUIAutomationElement>, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn PollForPotentialSupportedProperties(&mut self, pelement: &::core::option::Option<IUIAutomationElement>, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn CheckNotSupported(&mut self, value: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ReservedNotSupportedValue(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ReservedMixedAttributeValue(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ElementFromIAccessible(&mut self, accessible: &::core::option::Option<IAccessible>, childid: i32) -> ::windows::core::Result<IUIAutomationElement>;
    fn ElementFromIAccessibleBuildCache(&mut self, accessible: &::core::option::Option<IAccessible>, childid: i32, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>() -> IUIAutomation_Vtbl {
        unsafe extern "system" fn CompareElements<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, el1: ::windows::core::RawPtr, el2: ::windows::core::RawPtr, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompareElements(::core::mem::transmute(&el1), ::core::mem::transmute(&el2)) {
                ::core::result::Result::Ok(ok__) => {
                    *aresame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareRuntimeIds<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompareRuntimeIds(::core::mem::transmute_copy(&runtimeid1), ::core::mem::transmute_copy(&runtimeid2)) {
                ::core::result::Result::Ok(ok__) => {
                    *aresame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRootElement() {
                ::core::result::Result::Ok(ok__) => {
                    *root = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromHandle<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElementFromHandle(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromPoint<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElementFromPoint(::core::mem::transmute_copy(&pt)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootElementBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRootElementBuildCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *root = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromHandleBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElementFromHandleBuildCache(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromPointBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElementFromPointBuildCache(::core::mem::transmute_copy(&pt), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElementBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFocusedElementBuildCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTreeWalker<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcondition: ::windows::core::RawPtr, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTreeWalker(::core::mem::transmute(&pcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *walker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlViewWalker<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ControlViewWalker() {
                ::core::result::Result::Ok(ok__) => {
                    *walker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentViewWalker<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentViewWalker() {
                ::core::result::Result::Ok(ok__) => {
                    *walker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawViewWalker<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RawViewWalker() {
                ::core::result::Result::Ok(ok__) => {
                    *walker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawViewCondition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RawViewCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlViewCondition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ControlViewCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentViewCondition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentViewCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCacheRequest<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCacheRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *cacherequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrueCondition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTrueCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFalseCondition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFalseCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyCondition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePropertyCondition(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyConditionEx<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, flags: PropertyConditionFlags, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePropertyConditionEx(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndCondition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: ::windows::core::RawPtr, condition2: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAndCondition(::core::mem::transmute(&condition1), ::core::mem::transmute(&condition2)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndConditionFromArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAndConditionFromArray(::core::mem::transmute_copy(&conditions)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndConditionFromNativeArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const ::windows::core::RawPtr, conditioncount: i32, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAndConditionFromNativeArray(::core::mem::transmute_copy(&conditions), ::core::mem::transmute_copy(&conditioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrCondition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: ::windows::core::RawPtr, condition2: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateOrCondition(::core::mem::transmute(&condition1), ::core::mem::transmute(&condition2)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrConditionFromArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateOrConditionFromArray(::core::mem::transmute_copy(&conditions)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrConditionFromNativeArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const ::windows::core::RawPtr, conditioncount: i32, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateOrConditionFromNativeArray(::core::mem::transmute_copy(&conditions), ::core::mem::transmute_copy(&conditioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNotCondition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateNotCondition(::core::mem::transmute(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *newcondition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAutomationEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAutomationEventHandler(::core::mem::transmute_copy(&eventid), ::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveAutomationEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAutomationEventHandler(::core::mem::transmute_copy(&eventid), ::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandlerNativeArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const i32, propertycount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyChangedEventHandlerNativeArray(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler), ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler), ::core::mem::transmute_copy(&propertyarray)).into()
        }
        unsafe extern "system" fn RemovePropertyChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePropertyChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStructureChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveStructureChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStructureChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddFocusChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddFocusChangedEventHandler(::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveFocusChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFocusChangedEventHandler(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveAllEventHandlers<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAllEventHandlers().into()
        }
        unsafe extern "system" fn IntNativeArrayToSafeArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, array: *const i32, arraycount: i32, safearray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IntNativeArrayToSafeArray(::core::mem::transmute_copy(&array), ::core::mem::transmute_copy(&arraycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *safearray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntSafeArrayToNativeArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IntSafeArrayToNativeArray(::core::mem::transmute_copy(&intarray), ::core::mem::transmute_copy(&array), ::core::mem::transmute_copy(&arraycount)).into()
        }
        unsafe extern "system" fn RectToVariant<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, var: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RectToVariant(::core::mem::transmute_copy(&rc)) {
                ::core::result::Result::Ok(ok__) => {
                    *var = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VariantToRect<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, rc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).VariantToRect(::core::mem::transmute_copy(&var)) {
                ::core::result::Result::Ok(ok__) => {
                    *rc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SafeArrayToRectNativeArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SafeArrayToRectNativeArray(::core::mem::transmute_copy(&rects), ::core::mem::transmute_copy(&rectarray), ::core::mem::transmute_copy(&rectarraycount)).into()
        }
        unsafe extern "system" fn CreateProxyFactoryEntry<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, factoryentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateProxyFactoryEntry(::core::mem::transmute(&factory)) {
                ::core::result::Result::Ok(ok__) => {
                    *factoryentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyFactoryMapping<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorymapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyFactoryMapping() {
                ::core::result::Result::Ok(ok__) => {
                    *factorymapping = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyProgrammaticName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertyProgrammaticName(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatternProgrammaticName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPatternProgrammaticName(::core::mem::transmute_copy(&pattern)) {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedPatterns<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PollForPotentialSupportedPatterns(::core::mem::transmute(&pelement), ::core::mem::transmute_copy(&patternids), ::core::mem::transmute_copy(&patternnames)).into()
        }
        unsafe extern "system" fn PollForPotentialSupportedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PollForPotentialSupportedProperties(::core::mem::transmute(&pelement), ::core::mem::transmute_copy(&propertyids), ::core::mem::transmute_copy(&propertynames)).into()
        }
        unsafe extern "system" fn CheckNotSupported<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, isnotsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckNotSupported(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *isnotsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReservedNotSupportedValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notsupportedvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReservedNotSupportedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *notsupportedvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReservedMixedAttributeValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mixedattributevalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReservedMixedAttributeValue() {
                ::core::result::Result::Ok(ok__) => {
                    *mixedattributevalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromIAccessible<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: ::windows::core::RawPtr, childid: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElementFromIAccessible(::core::mem::transmute(&accessible), ::core::mem::transmute_copy(&childid)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromIAccessibleBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: ::windows::core::RawPtr, childid: i32, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ElementFromIAccessibleBuildCache(::core::mem::transmute(&accessible), ::core::mem::transmute_copy(&childid), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CompareElements: CompareElements::<Identity, Impl, OFFSET>,
            CompareRuntimeIds: CompareRuntimeIds::<Identity, Impl, OFFSET>,
            GetRootElement: GetRootElement::<Identity, Impl, OFFSET>,
            ElementFromHandle: ElementFromHandle::<Identity, Impl, OFFSET>,
            ElementFromPoint: ElementFromPoint::<Identity, Impl, OFFSET>,
            GetFocusedElement: GetFocusedElement::<Identity, Impl, OFFSET>,
            GetRootElementBuildCache: GetRootElementBuildCache::<Identity, Impl, OFFSET>,
            ElementFromHandleBuildCache: ElementFromHandleBuildCache::<Identity, Impl, OFFSET>,
            ElementFromPointBuildCache: ElementFromPointBuildCache::<Identity, Impl, OFFSET>,
            GetFocusedElementBuildCache: GetFocusedElementBuildCache::<Identity, Impl, OFFSET>,
            CreateTreeWalker: CreateTreeWalker::<Identity, Impl, OFFSET>,
            ControlViewWalker: ControlViewWalker::<Identity, Impl, OFFSET>,
            ContentViewWalker: ContentViewWalker::<Identity, Impl, OFFSET>,
            RawViewWalker: RawViewWalker::<Identity, Impl, OFFSET>,
            RawViewCondition: RawViewCondition::<Identity, Impl, OFFSET>,
            ControlViewCondition: ControlViewCondition::<Identity, Impl, OFFSET>,
            ContentViewCondition: ContentViewCondition::<Identity, Impl, OFFSET>,
            CreateCacheRequest: CreateCacheRequest::<Identity, Impl, OFFSET>,
            CreateTrueCondition: CreateTrueCondition::<Identity, Impl, OFFSET>,
            CreateFalseCondition: CreateFalseCondition::<Identity, Impl, OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Identity, Impl, OFFSET>,
            CreatePropertyConditionEx: CreatePropertyConditionEx::<Identity, Impl, OFFSET>,
            CreateAndCondition: CreateAndCondition::<Identity, Impl, OFFSET>,
            CreateAndConditionFromArray: CreateAndConditionFromArray::<Identity, Impl, OFFSET>,
            CreateAndConditionFromNativeArray: CreateAndConditionFromNativeArray::<Identity, Impl, OFFSET>,
            CreateOrCondition: CreateOrCondition::<Identity, Impl, OFFSET>,
            CreateOrConditionFromArray: CreateOrConditionFromArray::<Identity, Impl, OFFSET>,
            CreateOrConditionFromNativeArray: CreateOrConditionFromNativeArray::<Identity, Impl, OFFSET>,
            CreateNotCondition: CreateNotCondition::<Identity, Impl, OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Identity, Impl, OFFSET>,
            RemoveAutomationEventHandler: RemoveAutomationEventHandler::<Identity, Impl, OFFSET>,
            AddPropertyChangedEventHandlerNativeArray: AddPropertyChangedEventHandlerNativeArray::<Identity, Impl, OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Identity, Impl, OFFSET>,
            RemovePropertyChangedEventHandler: RemovePropertyChangedEventHandler::<Identity, Impl, OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveStructureChangedEventHandler: RemoveStructureChangedEventHandler::<Identity, Impl, OFFSET>,
            AddFocusChangedEventHandler: AddFocusChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveFocusChangedEventHandler: RemoveFocusChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveAllEventHandlers: RemoveAllEventHandlers::<Identity, Impl, OFFSET>,
            IntNativeArrayToSafeArray: IntNativeArrayToSafeArray::<Identity, Impl, OFFSET>,
            IntSafeArrayToNativeArray: IntSafeArrayToNativeArray::<Identity, Impl, OFFSET>,
            RectToVariant: RectToVariant::<Identity, Impl, OFFSET>,
            VariantToRect: VariantToRect::<Identity, Impl, OFFSET>,
            SafeArrayToRectNativeArray: SafeArrayToRectNativeArray::<Identity, Impl, OFFSET>,
            CreateProxyFactoryEntry: CreateProxyFactoryEntry::<Identity, Impl, OFFSET>,
            ProxyFactoryMapping: ProxyFactoryMapping::<Identity, Impl, OFFSET>,
            GetPropertyProgrammaticName: GetPropertyProgrammaticName::<Identity, Impl, OFFSET>,
            GetPatternProgrammaticName: GetPatternProgrammaticName::<Identity, Impl, OFFSET>,
            PollForPotentialSupportedPatterns: PollForPotentialSupportedPatterns::<Identity, Impl, OFFSET>,
            PollForPotentialSupportedProperties: PollForPotentialSupportedProperties::<Identity, Impl, OFFSET>,
            CheckNotSupported: CheckNotSupported::<Identity, Impl, OFFSET>,
            ReservedNotSupportedValue: ReservedNotSupportedValue::<Identity, Impl, OFFSET>,
            ReservedMixedAttributeValue: ReservedMixedAttributeValue::<Identity, Impl, OFFSET>,
            ElementFromIAccessible: ElementFromIAccessible::<Identity, Impl, OFFSET>,
            ElementFromIAccessibleBuildCache: ElementFromIAccessibleBuildCache::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2_Impl, const OFFSET: isize>() -> IUIAutomation2_Vtbl {
        unsafe extern "system" fn AutoSetFocus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoSetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *autosetfocus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoSetFocus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoSetFocus(::core::mem::transmute_copy(&autosetfocus)).into()
        }
        unsafe extern "system" fn ConnectionTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectionTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *timeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnectionTimeout(::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn TransactionTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TransactionTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *timeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransactionTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransactionTimeout(::core::mem::transmute_copy(&timeout)).into()
        }
        Self {
            base: IUIAutomation_Vtbl::new::<Identity, Impl, OFFSET>(),
            AutoSetFocus: AutoSetFocus::<Identity, Impl, OFFSET>,
            SetAutoSetFocus: SetAutoSetFocus::<Identity, Impl, OFFSET>,
            ConnectionTimeout: ConnectionTimeout::<Identity, Impl, OFFSET>,
            SetConnectionTimeout: SetConnectionTimeout::<Identity, Impl, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, Impl, OFFSET>,
            SetTransactionTimeout: SetTransactionTimeout::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation2 as ::windows::core::Interface>::IID || iid == &<IUIAutomation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation3_Impl: Sized + IUIAutomation_Impl + IUIAutomation2_Impl {
    fn AddTextEditTextChangedEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationTextEditTextChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveTextEditTextChangedEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, handler: &::core::option::Option<IUIAutomationTextEditTextChangedEventHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation3_Impl, const OFFSET: isize>() -> IUIAutomation3_Vtbl {
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTextEditTextChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&texteditchangetype), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveTextEditTextChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveTextEditTextChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: IUIAutomation2_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveTextEditTextChangedEventHandler: RemoveTextEditTextChangedEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation3 as ::windows::core::Interface>::IID || iid == &<IUIAutomation as ::windows::core::Interface>::IID || iid == &<IUIAutomation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation4_Impl: Sized + IUIAutomation_Impl + IUIAutomation2_Impl + IUIAutomation3_Impl {
    fn AddChangesEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationChangesEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveChangesEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, handler: &::core::option::Option<IUIAutomationChangesEventHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation4_Impl, const OFFSET: isize>() -> IUIAutomation4_Vtbl {
        unsafe extern "system" fn AddChangesEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddChangesEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&changetypes), ::core::mem::transmute_copy(&changescount), ::core::mem::transmute(&pcacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveChangesEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveChangesEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: IUIAutomation3_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddChangesEventHandler: AddChangesEventHandler::<Identity, Impl, OFFSET>,
            RemoveChangesEventHandler: RemoveChangesEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation4 as ::windows::core::Interface>::IID || iid == &<IUIAutomation as ::windows::core::Interface>::IID || iid == &<IUIAutomation2 as ::windows::core::Interface>::IID || iid == &<IUIAutomation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation5_Impl: Sized + IUIAutomation_Impl + IUIAutomation2_Impl + IUIAutomation3_Impl + IUIAutomation4_Impl {
    fn AddNotificationEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationNotificationEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveNotificationEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, handler: &::core::option::Option<IUIAutomationNotificationEventHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation5_Impl, const OFFSET: isize>() -> IUIAutomation5_Vtbl {
        unsafe extern "system" fn AddNotificationEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddNotificationEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveNotificationEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveNotificationEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: IUIAutomation4_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddNotificationEventHandler: AddNotificationEventHandler::<Identity, Impl, OFFSET>,
            RemoveNotificationEventHandler: RemoveNotificationEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation5 as ::windows::core::Interface>::IID || iid == &<IUIAutomation as ::windows::core::Interface>::IID || iid == &<IUIAutomation2 as ::windows::core::Interface>::IID || iid == &<IUIAutomation3 as ::windows::core::Interface>::IID || iid == &<IUIAutomation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation6_Impl: Sized + IUIAutomation_Impl + IUIAutomation2_Impl + IUIAutomation3_Impl + IUIAutomation4_Impl + IUIAutomation5_Impl {
    fn CreateEventHandlerGroup(&mut self) -> ::windows::core::Result<IUIAutomationEventHandlerGroup>;
    fn AddEventHandlerGroup(&mut self, element: &::core::option::Option<IUIAutomationElement>, handlergroup: &::core::option::Option<IUIAutomationEventHandlerGroup>) -> ::windows::core::Result<()>;
    fn RemoveEventHandlerGroup(&mut self, element: &::core::option::Option<IUIAutomationElement>, handlergroup: &::core::option::Option<IUIAutomationEventHandlerGroup>) -> ::windows::core::Result<()>;
    fn ConnectionRecoveryBehavior(&mut self) -> ::windows::core::Result<ConnectionRecoveryBehaviorOptions>;
    fn SetConnectionRecoveryBehavior(&mut self, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows::core::Result<()>;
    fn CoalesceEvents(&mut self) -> ::windows::core::Result<CoalesceEventsOptions>;
    fn SetCoalesceEvents(&mut self, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows::core::Result<()>;
    fn AddActiveTextPositionChangedEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows::core::Result<()>;
    fn RemoveActiveTextPositionChangedEventHandler(&mut self, element: &::core::option::Option<IUIAutomationElement>, handler: &::core::option::Option<IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>() -> IUIAutomation6_Vtbl {
        unsafe extern "system" fn CreateEventHandlerGroup<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handlergroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEventHandlerGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *handlergroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEventHandlerGroup<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handlergroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddEventHandlerGroup(::core::mem::transmute(&element), ::core::mem::transmute(&handlergroup)).into()
        }
        unsafe extern "system" fn RemoveEventHandlerGroup<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handlergroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveEventHandlerGroup(::core::mem::transmute(&element), ::core::mem::transmute(&handlergroup)).into()
        }
        unsafe extern "system" fn ConnectionRecoveryBehavior<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: *mut ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectionRecoveryBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *connectionrecoverybehavioroptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionRecoveryBehavior<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnectionRecoveryBehavior(::core::mem::transmute_copy(&connectionrecoverybehavioroptions)).into()
        }
        unsafe extern "system" fn CoalesceEvents<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: *mut CoalesceEventsOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CoalesceEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *coalesceeventsoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoalesceEvents<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCoalesceEvents(::core::mem::transmute_copy(&coalesceeventsoptions)).into()
        }
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddActiveTextPositionChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn RemoveActiveTextPositionChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveActiveTextPositionChangedEventHandler(::core::mem::transmute(&element), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: IUIAutomation5_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateEventHandlerGroup: CreateEventHandlerGroup::<Identity, Impl, OFFSET>,
            AddEventHandlerGroup: AddEventHandlerGroup::<Identity, Impl, OFFSET>,
            RemoveEventHandlerGroup: RemoveEventHandlerGroup::<Identity, Impl, OFFSET>,
            ConnectionRecoveryBehavior: ConnectionRecoveryBehavior::<Identity, Impl, OFFSET>,
            SetConnectionRecoveryBehavior: SetConnectionRecoveryBehavior::<Identity, Impl, OFFSET>,
            CoalesceEvents: CoalesceEvents::<Identity, Impl, OFFSET>,
            SetCoalesceEvents: SetCoalesceEvents::<Identity, Impl, OFFSET>,
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveActiveTextPositionChangedEventHandler: RemoveActiveTextPositionChangedEventHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation6 as ::windows::core::Interface>::IID || iid == &<IUIAutomation as ::windows::core::Interface>::IID || iid == &<IUIAutomation2 as ::windows::core::Interface>::IID || iid == &<IUIAutomation3 as ::windows::core::Interface>::IID || iid == &<IUIAutomation4 as ::windows::core::Interface>::IID || iid == &<IUIAutomation5 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationActiveTextPositionChangedEventHandler_Impl: Sized {
    fn HandleActiveTextPositionChangedEvent(&mut self, sender: &::core::option::Option<IUIAutomationElement>, range: &::core::option::Option<IUIAutomationTextRange>) -> ::windows::core::Result<()>;
}
impl IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleActiveTextPositionChangedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, range: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandleActiveTextPositionChangedEvent(::core::mem::transmute(&sender), ::core::mem::transmute(&range)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            HandleActiveTextPositionChangedEvent: HandleActiveTextPositionChangedEvent::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>() -> IUIAutomationAndCondition_Vtbl {
        unsafe extern "system" fn ChildCount<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChildCount() {
                ::core::result::Result::Ok(ok__) => {
                    *childcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::windows::core::RawPtr, childarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChildrenAsNativeArray(::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)).into()
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *childarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(),
            ChildCount: ChildCount::<Identity, Impl, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationAndCondition as ::windows::core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>() -> IUIAutomationAnnotationPattern_Vtbl {
        unsafe extern "system" fn CurrentAnnotationTypeId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypeName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAuthor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAuthor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDateTime<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentTarget<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAnnotationTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAnnotationTypeName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAuthor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAuthor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDateTime<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedTarget<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CurrentAnnotationTypeId: CurrentAnnotationTypeId::<Identity, Impl, OFFSET>,
            CurrentAnnotationTypeName: CurrentAnnotationTypeName::<Identity, Impl, OFFSET>,
            CurrentAuthor: CurrentAuthor::<Identity, Impl, OFFSET>,
            CurrentDateTime: CurrentDateTime::<Identity, Impl, OFFSET>,
            CurrentTarget: CurrentTarget::<Identity, Impl, OFFSET>,
            CachedAnnotationTypeId: CachedAnnotationTypeId::<Identity, Impl, OFFSET>,
            CachedAnnotationTypeName: CachedAnnotationTypeName::<Identity, Impl, OFFSET>,
            CachedAuthor: CachedAuthor::<Identity, Impl, OFFSET>,
            CachedDateTime: CachedDateTime::<Identity, Impl, OFFSET>,
            CachedTarget: CachedTarget::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationBoolCondition_Impl, const OFFSET: isize>() -> IUIAutomationBoolCondition_Vtbl {
        unsafe extern "system" fn BooleanValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationBoolCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boolval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BooleanValue() {
                ::core::result::Result::Ok(ok__) => {
                    *boolval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(), BooleanValue: BooleanValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationBoolCondition as ::windows::core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationCacheRequest_Impl: Sized {
    fn AddProperty(&mut self, propertyid: i32) -> ::windows::core::Result<()>;
    fn AddPattern(&mut self, patternid: i32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IUIAutomationCacheRequest>;
    fn TreeScope(&mut self) -> ::windows::core::Result<TreeScope>;
    fn SetTreeScope(&mut self, scope: TreeScope) -> ::windows::core::Result<()>;
    fn TreeFilter(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
    fn SetTreeFilter(&mut self, filter: &::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<()>;
    fn AutomationElementMode(&mut self) -> ::windows::core::Result<AutomationElementMode>;
    fn SetAutomationElementMode(&mut self, mode: AutomationElementMode) -> ::windows::core::Result<()>;
}
impl IUIAutomationCacheRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>() -> IUIAutomationCacheRequest_Vtbl {
        unsafe extern "system" fn AddProperty<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddProperty(::core::mem::transmute_copy(&propertyid)).into()
        }
        unsafe extern "system" fn AddPattern<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPattern(::core::mem::transmute_copy(&patternid)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *clonedrequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TreeScope<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut TreeScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TreeScope() {
                ::core::result::Result::Ok(ok__) => {
                    *scope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreeScope<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTreeScope(::core::mem::transmute_copy(&scope)).into()
        }
        unsafe extern "system" fn TreeFilter<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TreeFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *filter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreeFilter<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTreeFilter(::core::mem::transmute(&filter)).into()
        }
        unsafe extern "system" fn AutomationElementMode<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut AutomationElementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutomationElementMode() {
                ::core::result::Result::Ok(ok__) => {
                    *mode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomationElementMode<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: AutomationElementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutomationElementMode(::core::mem::transmute_copy(&mode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddProperty: AddProperty::<Identity, Impl, OFFSET>,
            AddPattern: AddPattern::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            TreeScope: TreeScope::<Identity, Impl, OFFSET>,
            SetTreeScope: SetTreeScope::<Identity, Impl, OFFSET>,
            TreeFilter: TreeFilter::<Identity, Impl, OFFSET>,
            SetTreeFilter: SetTreeFilter::<Identity, Impl, OFFSET>,
            AutomationElementMode: AutomationElementMode::<Identity, Impl, OFFSET>,
            SetAutomationElementMode: SetAutomationElementMode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationCacheRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationChangesEventHandler_Impl: Sized {
    fn HandleChangesEvent(&mut self, sender: &::core::option::Option<IUIAutomationElement>, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationChangesEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationChangesEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationChangesEventHandler_Vtbl {
        unsafe extern "system" fn HandleChangesEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationChangesEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandleChangesEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&uiachanges), ::core::mem::transmute_copy(&changescount)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HandleChangesEvent: HandleChangesEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationChangesEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationCondition_Impl: Sized {}
impl IUIAutomationCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCondition_Impl, const OFFSET: isize>() -> IUIAutomationCondition_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationCondition as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationCustomNavigationPattern_Impl: Sized {
    fn Navigate(&mut self, direction: NavigateDirection) -> ::windows::core::Result<IUIAutomationElement>;
}
impl IUIAutomationCustomNavigationPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: isize>() -> IUIAutomationCustomNavigationPattern_Vtbl {
        unsafe extern "system" fn Navigate<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Navigate(::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Navigate: Navigate::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>() -> IUIAutomationDockPattern_Vtbl {
        unsafe extern "system" fn SetDockPosition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockpos: DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDockPosition(::core::mem::transmute_copy(&dockpos)).into()
        }
        unsafe extern "system" fn CurrentDockPosition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDockPosition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedDockPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDockPosition: SetDockPosition::<Identity, Impl, OFFSET>,
            CurrentDockPosition: CurrentDockPosition::<Identity, Impl, OFFSET>,
            CachedDockPosition: CachedDockPosition::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>() -> IUIAutomationDragPattern_Vtbl {
        unsafe extern "system" fn CurrentIsGrabbed<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsGrabbed<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsGrabbed() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropEffect<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropEffect<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedDropEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropEffects<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropEffects<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedDropEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentGrabbedItems<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedGrabbedItems<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedGrabbedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CurrentIsGrabbed: CurrentIsGrabbed::<Identity, Impl, OFFSET>,
            CachedIsGrabbed: CachedIsGrabbed::<Identity, Impl, OFFSET>,
            CurrentDropEffect: CurrentDropEffect::<Identity, Impl, OFFSET>,
            CachedDropEffect: CachedDropEffect::<Identity, Impl, OFFSET>,
            CurrentDropEffects: CurrentDropEffects::<Identity, Impl, OFFSET>,
            CachedDropEffects: CachedDropEffects::<Identity, Impl, OFFSET>,
            GetCurrentGrabbedItems: GetCurrentGrabbedItems::<Identity, Impl, OFFSET>,
            GetCachedGrabbedItems: GetCachedGrabbedItems::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>() -> IUIAutomationDropTargetPattern_Vtbl {
        unsafe extern "system" fn CurrentDropTargetEffect<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDropTargetEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropTargetEffect<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedDropTargetEffect() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropTargetEffects<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDropTargetEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropTargetEffects<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedDropTargetEffects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CurrentDropTargetEffect: CurrentDropTargetEffect::<Identity, Impl, OFFSET>,
            CachedDropTargetEffect: CachedDropTargetEffect::<Identity, Impl, OFFSET>,
            CurrentDropTargetEffects: CurrentDropTargetEffects::<Identity, Impl, OFFSET>,
            CachedDropTargetEffects: CachedDropTargetEffects::<Identity, Impl, OFFSET>,
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
    fn FindFirst(&mut self, scope: TreeScope, condition: &::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationElement>;
    fn FindAll(&mut self, scope: TreeScope, condition: &::core::option::Option<IUIAutomationCondition>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn FindFirstBuildCache(&mut self, scope: TreeScope, condition: &::core::option::Option<IUIAutomationCondition>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn FindAllBuildCache(&mut self, scope: TreeScope, condition: &::core::option::Option<IUIAutomationCondition>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn BuildUpdatedCache(&mut self, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>() -> IUIAutomationElement_Vtbl {
        unsafe extern "system" fn SetFocus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFocus().into()
        }
        unsafe extern "system" fn GetRuntimeId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    *runtimeid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirst<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindFirst(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAll<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindAll(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindFirstBuildCache(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindAllBuildCache(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildUpdatedCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, updatedelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BuildUpdatedCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *updatedelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentPropertyValue(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValueEx<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentPropertyValueEx(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ignoredefaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPropertyValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedPropertyValue(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPropertyValueEx<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedPropertyValueEx(::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ignoredefaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPatternAs<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentPatternAs(::core::mem::transmute_copy(&patternid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&patternobject)).into()
        }
        unsafe extern "system" fn GetCachedPatternAs<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCachedPatternAs(::core::mem::transmute_copy(&patternid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&patternobject)).into()
        }
        unsafe extern "system" fn GetCurrentPattern<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentPattern(::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    *patternobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPattern<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedPattern(::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    *patternobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedParent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedParent() {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedChildren<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProcessId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentControlType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLocalizedControlType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentLocalizedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAcceleratorKey<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAcceleratorKey() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAccessKey<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHasKeyboardFocus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentHasKeyboardFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsKeyboardFocusable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsKeyboardFocusable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAutomationId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAutomationId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentClassName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHelpText<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCulture<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCulture() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsControlElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsControlElement() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsContentElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsContentElement() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsPassword<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsPassword() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentNativeWindowHandle<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentNativeWindowHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsOffscreen<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsOffscreen() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentOrientation<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFrameworkId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFrameworkId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsRequiredForForm<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsRequiredForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemStatus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentItemStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentBoundingRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLabeledBy<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentLabeledBy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAriaRole<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAriaRole() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAriaProperties<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAriaProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsDataValidForForm<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsDataValidForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentControllerFor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentControllerFor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDescribedBy<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDescribedBy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFlowsTo<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFlowsTo() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProviderDescription<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentProviderDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedProcessId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedControlType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLocalizedControlType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedLocalizedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAcceleratorKey<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAcceleratorKey() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAccessKey<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHasKeyboardFocus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedHasKeyboardFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsKeyboardFocusable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsKeyboardFocusable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAutomationId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAutomationId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedClassName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHelpText<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCulture<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCulture() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsControlElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsControlElement() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsContentElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsContentElement() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsPassword<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsPassword() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedNativeWindowHandle<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedNativeWindowHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsOffscreen<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsOffscreen() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedOrientation<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFrameworkId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedFrameworkId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsRequiredForForm<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsRequiredForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemStatus<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedItemStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedBoundingRectangle<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLabeledBy<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedLabeledBy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAriaRole<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAriaRole() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAriaProperties<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAriaProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsDataValidForForm<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsDataValidForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedControllerFor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedControllerFor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDescribedBy<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedDescribedBy() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFlowsTo<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedFlowsTo() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedProviderDescription<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedProviderDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePoint<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClickablePoint(::core::mem::transmute_copy(&clickable), ::core::mem::transmute_copy(&gotclickable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, Impl, OFFSET>,
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindAll: FindAll::<Identity, Impl, OFFSET>,
            FindFirstBuildCache: FindFirstBuildCache::<Identity, Impl, OFFSET>,
            FindAllBuildCache: FindAllBuildCache::<Identity, Impl, OFFSET>,
            BuildUpdatedCache: BuildUpdatedCache::<Identity, Impl, OFFSET>,
            GetCurrentPropertyValue: GetCurrentPropertyValue::<Identity, Impl, OFFSET>,
            GetCurrentPropertyValueEx: GetCurrentPropertyValueEx::<Identity, Impl, OFFSET>,
            GetCachedPropertyValue: GetCachedPropertyValue::<Identity, Impl, OFFSET>,
            GetCachedPropertyValueEx: GetCachedPropertyValueEx::<Identity, Impl, OFFSET>,
            GetCurrentPatternAs: GetCurrentPatternAs::<Identity, Impl, OFFSET>,
            GetCachedPatternAs: GetCachedPatternAs::<Identity, Impl, OFFSET>,
            GetCurrentPattern: GetCurrentPattern::<Identity, Impl, OFFSET>,
            GetCachedPattern: GetCachedPattern::<Identity, Impl, OFFSET>,
            GetCachedParent: GetCachedParent::<Identity, Impl, OFFSET>,
            GetCachedChildren: GetCachedChildren::<Identity, Impl, OFFSET>,
            CurrentProcessId: CurrentProcessId::<Identity, Impl, OFFSET>,
            CurrentControlType: CurrentControlType::<Identity, Impl, OFFSET>,
            CurrentLocalizedControlType: CurrentLocalizedControlType::<Identity, Impl, OFFSET>,
            CurrentName: CurrentName::<Identity, Impl, OFFSET>,
            CurrentAcceleratorKey: CurrentAcceleratorKey::<Identity, Impl, OFFSET>,
            CurrentAccessKey: CurrentAccessKey::<Identity, Impl, OFFSET>,
            CurrentHasKeyboardFocus: CurrentHasKeyboardFocus::<Identity, Impl, OFFSET>,
            CurrentIsKeyboardFocusable: CurrentIsKeyboardFocusable::<Identity, Impl, OFFSET>,
            CurrentIsEnabled: CurrentIsEnabled::<Identity, Impl, OFFSET>,
            CurrentAutomationId: CurrentAutomationId::<Identity, Impl, OFFSET>,
            CurrentClassName: CurrentClassName::<Identity, Impl, OFFSET>,
            CurrentHelpText: CurrentHelpText::<Identity, Impl, OFFSET>,
            CurrentCulture: CurrentCulture::<Identity, Impl, OFFSET>,
            CurrentIsControlElement: CurrentIsControlElement::<Identity, Impl, OFFSET>,
            CurrentIsContentElement: CurrentIsContentElement::<Identity, Impl, OFFSET>,
            CurrentIsPassword: CurrentIsPassword::<Identity, Impl, OFFSET>,
            CurrentNativeWindowHandle: CurrentNativeWindowHandle::<Identity, Impl, OFFSET>,
            CurrentItemType: CurrentItemType::<Identity, Impl, OFFSET>,
            CurrentIsOffscreen: CurrentIsOffscreen::<Identity, Impl, OFFSET>,
            CurrentOrientation: CurrentOrientation::<Identity, Impl, OFFSET>,
            CurrentFrameworkId: CurrentFrameworkId::<Identity, Impl, OFFSET>,
            CurrentIsRequiredForForm: CurrentIsRequiredForForm::<Identity, Impl, OFFSET>,
            CurrentItemStatus: CurrentItemStatus::<Identity, Impl, OFFSET>,
            CurrentBoundingRectangle: CurrentBoundingRectangle::<Identity, Impl, OFFSET>,
            CurrentLabeledBy: CurrentLabeledBy::<Identity, Impl, OFFSET>,
            CurrentAriaRole: CurrentAriaRole::<Identity, Impl, OFFSET>,
            CurrentAriaProperties: CurrentAriaProperties::<Identity, Impl, OFFSET>,
            CurrentIsDataValidForForm: CurrentIsDataValidForForm::<Identity, Impl, OFFSET>,
            CurrentControllerFor: CurrentControllerFor::<Identity, Impl, OFFSET>,
            CurrentDescribedBy: CurrentDescribedBy::<Identity, Impl, OFFSET>,
            CurrentFlowsTo: CurrentFlowsTo::<Identity, Impl, OFFSET>,
            CurrentProviderDescription: CurrentProviderDescription::<Identity, Impl, OFFSET>,
            CachedProcessId: CachedProcessId::<Identity, Impl, OFFSET>,
            CachedControlType: CachedControlType::<Identity, Impl, OFFSET>,
            CachedLocalizedControlType: CachedLocalizedControlType::<Identity, Impl, OFFSET>,
            CachedName: CachedName::<Identity, Impl, OFFSET>,
            CachedAcceleratorKey: CachedAcceleratorKey::<Identity, Impl, OFFSET>,
            CachedAccessKey: CachedAccessKey::<Identity, Impl, OFFSET>,
            CachedHasKeyboardFocus: CachedHasKeyboardFocus::<Identity, Impl, OFFSET>,
            CachedIsKeyboardFocusable: CachedIsKeyboardFocusable::<Identity, Impl, OFFSET>,
            CachedIsEnabled: CachedIsEnabled::<Identity, Impl, OFFSET>,
            CachedAutomationId: CachedAutomationId::<Identity, Impl, OFFSET>,
            CachedClassName: CachedClassName::<Identity, Impl, OFFSET>,
            CachedHelpText: CachedHelpText::<Identity, Impl, OFFSET>,
            CachedCulture: CachedCulture::<Identity, Impl, OFFSET>,
            CachedIsControlElement: CachedIsControlElement::<Identity, Impl, OFFSET>,
            CachedIsContentElement: CachedIsContentElement::<Identity, Impl, OFFSET>,
            CachedIsPassword: CachedIsPassword::<Identity, Impl, OFFSET>,
            CachedNativeWindowHandle: CachedNativeWindowHandle::<Identity, Impl, OFFSET>,
            CachedItemType: CachedItemType::<Identity, Impl, OFFSET>,
            CachedIsOffscreen: CachedIsOffscreen::<Identity, Impl, OFFSET>,
            CachedOrientation: CachedOrientation::<Identity, Impl, OFFSET>,
            CachedFrameworkId: CachedFrameworkId::<Identity, Impl, OFFSET>,
            CachedIsRequiredForForm: CachedIsRequiredForForm::<Identity, Impl, OFFSET>,
            CachedItemStatus: CachedItemStatus::<Identity, Impl, OFFSET>,
            CachedBoundingRectangle: CachedBoundingRectangle::<Identity, Impl, OFFSET>,
            CachedLabeledBy: CachedLabeledBy::<Identity, Impl, OFFSET>,
            CachedAriaRole: CachedAriaRole::<Identity, Impl, OFFSET>,
            CachedAriaProperties: CachedAriaProperties::<Identity, Impl, OFFSET>,
            CachedIsDataValidForForm: CachedIsDataValidForForm::<Identity, Impl, OFFSET>,
            CachedControllerFor: CachedControllerFor::<Identity, Impl, OFFSET>,
            CachedDescribedBy: CachedDescribedBy::<Identity, Impl, OFFSET>,
            CachedFlowsTo: CachedFlowsTo::<Identity, Impl, OFFSET>,
            CachedProviderDescription: CachedProviderDescription::<Identity, Impl, OFFSET>,
            GetClickablePoint: GetClickablePoint::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>() -> IUIAutomationElement2_Vtbl {
        unsafe extern "system" fn CurrentOptimizeForVisualContent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentOptimizeForVisualContent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedOptimizeForVisualContent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedOptimizeForVisualContent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLiveSetting<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentLiveSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLiveSetting<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedLiveSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFlowsFrom<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFlowsFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFlowsFrom<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedFlowsFrom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentOptimizeForVisualContent: CurrentOptimizeForVisualContent::<Identity, Impl, OFFSET>,
            CachedOptimizeForVisualContent: CachedOptimizeForVisualContent::<Identity, Impl, OFFSET>,
            CurrentLiveSetting: CurrentLiveSetting::<Identity, Impl, OFFSET>,
            CachedLiveSetting: CachedLiveSetting::<Identity, Impl, OFFSET>,
            CurrentFlowsFrom: CurrentFlowsFrom::<Identity, Impl, OFFSET>,
            CachedFlowsFrom: CachedFlowsFrom::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement3_Impl, const OFFSET: isize>() -> IUIAutomationElement3_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        unsafe extern "system" fn CurrentIsPeripheral<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsPeripheral() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsPeripheral<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsPeripheral() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET>,
            CurrentIsPeripheral: CurrentIsPeripheral::<Identity, Impl, OFFSET>,
            CachedIsPeripheral: CachedIsPeripheral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement3 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>() -> IUIAutomationElement4_Vtbl {
        unsafe extern "system" fn CurrentPositionInSet<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentPositionInSet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSizeOfSet<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentSizeOfSet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLevel<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypes<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationObjects<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedPositionInSet<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedPositionInSet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSizeOfSet<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedSizeOfSet() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLevel<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypes<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationObjects<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement3_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentPositionInSet: CurrentPositionInSet::<Identity, Impl, OFFSET>,
            CurrentSizeOfSet: CurrentSizeOfSet::<Identity, Impl, OFFSET>,
            CurrentLevel: CurrentLevel::<Identity, Impl, OFFSET>,
            CurrentAnnotationTypes: CurrentAnnotationTypes::<Identity, Impl, OFFSET>,
            CurrentAnnotationObjects: CurrentAnnotationObjects::<Identity, Impl, OFFSET>,
            CachedPositionInSet: CachedPositionInSet::<Identity, Impl, OFFSET>,
            CachedSizeOfSet: CachedSizeOfSet::<Identity, Impl, OFFSET>,
            CachedLevel: CachedLevel::<Identity, Impl, OFFSET>,
            CachedAnnotationTypes: CachedAnnotationTypes::<Identity, Impl, OFFSET>,
            CachedAnnotationObjects: CachedAnnotationObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement4 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>() -> IUIAutomationElement5_Vtbl {
        unsafe extern "system" fn CurrentLandmarkType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLocalizedLandmarkType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentLocalizedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLandmarkType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLocalizedLandmarkType<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedLocalizedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement4_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentLandmarkType: CurrentLandmarkType::<Identity, Impl, OFFSET>,
            CurrentLocalizedLandmarkType: CurrentLocalizedLandmarkType::<Identity, Impl, OFFSET>,
            CachedLandmarkType: CachedLandmarkType::<Identity, Impl, OFFSET>,
            CachedLocalizedLandmarkType: CachedLocalizedLandmarkType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement5 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement6_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl + IUIAutomationElement4_Impl + IUIAutomationElement5_Impl {
    fn CurrentFullDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedFullDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement6_Impl, const OFFSET: isize>() -> IUIAutomationElement6_Vtbl {
        unsafe extern "system" fn CurrentFullDescription<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFullDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFullDescription<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedFullDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement5_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentFullDescription: CurrentFullDescription::<Identity, Impl, OFFSET>,
            CachedFullDescription: CachedFullDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement6 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement7_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl + IUIAutomationElement4_Impl + IUIAutomationElement5_Impl + IUIAutomationElement6_Impl {
    fn FindFirstWithOptions(&mut self, scope: TreeScope, condition: &::core::option::Option<IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn FindAllWithOptions(&mut self, scope: TreeScope, condition: &::core::option::Option<IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn FindFirstWithOptionsBuildCache(&mut self, scope: TreeScope, condition: &::core::option::Option<IUIAutomationCondition>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn FindAllWithOptionsBuildCache(&mut self, scope: TreeScope, condition: &::core::option::Option<IUIAutomationCondition>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetCurrentMetadataValue(&mut self, targetid: i32, metadataid: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>() -> IUIAutomationElement7_Vtbl {
        unsafe extern "system" fn FindFirstWithOptions<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindFirstWithOptions(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute_copy(&traversaloptions), ::core::mem::transmute(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithOptions<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindAllWithOptions(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute_copy(&traversaloptions), ::core::mem::transmute(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstWithOptionsBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindFirstWithOptionsBuildCache(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute(&cacherequest), ::core::mem::transmute_copy(&traversaloptions), ::core::mem::transmute(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithOptionsBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindAllWithOptionsBuildCache(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&condition), ::core::mem::transmute(&cacherequest), ::core::mem::transmute_copy(&traversaloptions), ::core::mem::transmute(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentMetadataValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: i32, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentMetadataValue(::core::mem::transmute_copy(&targetid), ::core::mem::transmute_copy(&metadataid)) {
                ::core::result::Result::Ok(ok__) => {
                    *returnval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement6_Vtbl::new::<Identity, Impl, OFFSET>(),
            FindFirstWithOptions: FindFirstWithOptions::<Identity, Impl, OFFSET>,
            FindAllWithOptions: FindAllWithOptions::<Identity, Impl, OFFSET>,
            FindFirstWithOptionsBuildCache: FindFirstWithOptionsBuildCache::<Identity, Impl, OFFSET>,
            FindAllWithOptionsBuildCache: FindAllWithOptionsBuildCache::<Identity, Impl, OFFSET>,
            GetCurrentMetadataValue: GetCurrentMetadataValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement7 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement5 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement8_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl + IUIAutomationElement4_Impl + IUIAutomationElement5_Impl + IUIAutomationElement6_Impl + IUIAutomationElement7_Impl {
    fn CurrentHeadingLevel(&mut self) -> ::windows::core::Result<i32>;
    fn CachedHeadingLevel(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement8_Impl, const OFFSET: isize>() -> IUIAutomationElement8_Vtbl {
        unsafe extern "system" fn CurrentHeadingLevel<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentHeadingLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHeadingLevel<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedHeadingLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement7_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentHeadingLevel: CurrentHeadingLevel::<Identity, Impl, OFFSET>,
            CachedHeadingLevel: CachedHeadingLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement8 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement5 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement6 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement9_Impl: Sized + IUIAutomationElement_Impl + IUIAutomationElement2_Impl + IUIAutomationElement3_Impl + IUIAutomationElement4_Impl + IUIAutomationElement5_Impl + IUIAutomationElement6_Impl + IUIAutomationElement7_Impl + IUIAutomationElement8_Impl {
    fn CurrentIsDialog(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedIsDialog(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement9_Impl, const OFFSET: isize>() -> IUIAutomationElement9_Vtbl {
        unsafe extern "system" fn CurrentIsDialog<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsDialog() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsDialog<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsDialog() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationElement8_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentIsDialog: CurrentIsDialog::<Identity, Impl, OFFSET>,
            CachedIsDialog: CachedIsDialog::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement9 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement3 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement4 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement5 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement6 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement7 as ::windows::core::Interface>::IID || iid == &<IUIAutomationElement8 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationElementArray_Impl: Sized {
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn GetElement(&mut self, index: i32) -> ::windows::core::Result<IUIAutomationElement>;
}
impl IUIAutomationElementArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElementArray_Impl, const OFFSET: isize>() -> IUIAutomationElementArray_Vtbl {
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElementArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElementArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetElement(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Length: Length::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElementArray as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationEventHandler_Impl: Sized {
    fn HandleAutomationEvent(&mut self, sender: &::core::option::Option<IUIAutomationElement>, eventid: i32) -> ::windows::core::Result<()>;
}
impl IUIAutomationEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationEventHandler_Vtbl {
        unsafe extern "system" fn HandleAutomationEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, eventid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandleAutomationEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&eventid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HandleAutomationEvent: HandleAutomationEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationEventHandlerGroup_Impl: Sized {
    fn AddActiveTextPositionChangedEventHandler(&mut self, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows::core::Result<()>;
    fn AddAutomationEventHandler(&mut self, eventid: i32, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationEventHandler>) -> ::windows::core::Result<()>;
    fn AddChangesEventHandler(&mut self, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationChangesEventHandler>) -> ::windows::core::Result<()>;
    fn AddNotificationEventHandler(&mut self, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationNotificationEventHandler>) -> ::windows::core::Result<()>;
    fn AddPropertyChangedEventHandler(&mut self, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const i32, propertycount: i32) -> ::windows::core::Result<()>;
    fn AddStructureChangedEventHandler(&mut self, scope: TreeScope, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationStructureChangedEventHandler>) -> ::windows::core::Result<()>;
    fn AddTextEditTextChangedEventHandler(&mut self, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>, handler: &::core::option::Option<IUIAutomationTextEditTextChangedEventHandler>) -> ::windows::core::Result<()>;
}
impl IUIAutomationEventHandlerGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>() -> IUIAutomationEventHandlerGroup_Vtbl {
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddActiveTextPositionChangedEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddAutomationEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAutomationEventHandler(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddChangesEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddChangesEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&changetypes), ::core::mem::transmute_copy(&changescount), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddNotificationEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddNotificationEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const i32, propertycount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyChangedEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler), ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStructureChangedEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTextEditTextChangedEventHandler(::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&texteditchangetype), ::core::mem::transmute(&cacherequest), ::core::mem::transmute(&handler)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Identity, Impl, OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Identity, Impl, OFFSET>,
            AddChangesEventHandler: AddChangesEventHandler::<Identity, Impl, OFFSET>,
            AddNotificationEventHandler: AddNotificationEventHandler::<Identity, Impl, OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Identity, Impl, OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Identity, Impl, OFFSET>,
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>() -> IUIAutomationExpandCollapsePattern_Vtbl {
        unsafe extern "system" fn Expand<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Expand().into()
        }
        unsafe extern "system" fn Collapse<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Collapse().into()
        }
        unsafe extern "system" fn CurrentExpandCollapseState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedExpandCollapseState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedExpandCollapseState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Expand: Expand::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            CurrentExpandCollapseState: CurrentExpandCollapseState::<Identity, Impl, OFFSET>,
            CachedExpandCollapseState: CachedExpandCollapseState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationExpandCollapsePattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationFocusChangedEventHandler_Impl: Sized {
    fn HandleFocusChangedEvent(&mut self, sender: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<()>;
}
impl IUIAutomationFocusChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationFocusChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationFocusChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleFocusChangedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationFocusChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandleFocusChangedEvent(::core::mem::transmute(&sender)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HandleFocusChangedEvent: HandleFocusChangedEvent::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationGridItemPattern_Vtbl {
        unsafe extern "system" fn CurrentContainingGrid<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRow<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentRow() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumn<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentColumn() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowSpan<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentRowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumnSpan<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedContainingGrid<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedContainingGrid() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRow<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedRow() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumn<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedColumn() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowSpan<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedRowSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumnSpan<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedColumnSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CurrentContainingGrid: CurrentContainingGrid::<Identity, Impl, OFFSET>,
            CurrentRow: CurrentRow::<Identity, Impl, OFFSET>,
            CurrentColumn: CurrentColumn::<Identity, Impl, OFFSET>,
            CurrentRowSpan: CurrentRowSpan::<Identity, Impl, OFFSET>,
            CurrentColumnSpan: CurrentColumnSpan::<Identity, Impl, OFFSET>,
            CachedContainingGrid: CachedContainingGrid::<Identity, Impl, OFFSET>,
            CachedRow: CachedRow::<Identity, Impl, OFFSET>,
            CachedColumn: CachedColumn::<Identity, Impl, OFFSET>,
            CachedRowSpan: CachedRowSpan::<Identity, Impl, OFFSET>,
            CachedColumnSpan: CachedColumnSpan::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>() -> IUIAutomationGridPattern_Vtbl {
        unsafe extern "system" fn GetItem<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItem(::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowCount<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentRowCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumnCount<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowCount<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedRowCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumnCount<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            CurrentRowCount: CurrentRowCount::<Identity, Impl, OFFSET>,
            CurrentColumnCount: CurrentColumnCount::<Identity, Impl, OFFSET>,
            CachedRowCount: CachedRowCount::<Identity, Impl, OFFSET>,
            CachedColumnCount: CachedColumnCount::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationInvokePattern_Impl, const OFFSET: isize>() -> IUIAutomationInvokePattern_Vtbl {
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationInvokePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationInvokePattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationItemContainerPattern_Impl: Sized {
    fn FindItemByProperty(&mut self, pstartafter: &::core::option::Option<IUIAutomationElement>, propertyid: i32, value: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationItemContainerPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationItemContainerPattern_Impl, const OFFSET: isize>() -> IUIAutomationItemContainerPattern_Vtbl {
        unsafe extern "system" fn FindItemByProperty<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationItemContainerPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: ::windows::core::RawPtr, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindItemByProperty(::core::mem::transmute(&pstartafter), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfound = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), FindItemByProperty: FindItemByProperty::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>() -> IUIAutomationLegacyIAccessiblePattern_Vtbl {
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select(::core::mem::transmute_copy(&flagsselect)).into()
        }
        unsafe extern "system" fn DoDefaultAction<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DoDefaultAction().into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&szvalue)).into()
        }
        unsafe extern "system" fn CurrentChildId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentChildId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentName() {
                ::core::result::Result::Ok(ok__) => {
                    *pszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentValue() {
                ::core::result::Result::Ok(ok__) => {
                    *pszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDescription<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRole<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentRole() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHelp<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentHelp() {
                ::core::result::Result::Ok(ok__) => {
                    *pszhelp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentKeyboardShortcut<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentKeyboardShortcut() {
                ::core::result::Result::Ok(ok__) => {
                    *pszkeyboardshortcut = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselectedchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDefaultAction<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentDefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdefaultaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedChildId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedChildId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedName() {
                ::core::result::Result::Ok(ok__) => {
                    *pszname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *pszvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDescription<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRole<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedRole() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHelp<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedHelp() {
                ::core::result::Result::Ok(ok__) => {
                    *pszhelp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedKeyboardShortcut<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedKeyboardShortcut() {
                ::core::result::Result::Ok(ok__) => {
                    *pszkeyboardshortcut = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarselectedchildren = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDefaultAction<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedDefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    *pszdefaultaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIAccessible<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIAccessible() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaccessible = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, Impl, OFFSET>,
            DoDefaultAction: DoDefaultAction::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            CurrentChildId: CurrentChildId::<Identity, Impl, OFFSET>,
            CurrentName: CurrentName::<Identity, Impl, OFFSET>,
            CurrentValue: CurrentValue::<Identity, Impl, OFFSET>,
            CurrentDescription: CurrentDescription::<Identity, Impl, OFFSET>,
            CurrentRole: CurrentRole::<Identity, Impl, OFFSET>,
            CurrentState: CurrentState::<Identity, Impl, OFFSET>,
            CurrentHelp: CurrentHelp::<Identity, Impl, OFFSET>,
            CurrentKeyboardShortcut: CurrentKeyboardShortcut::<Identity, Impl, OFFSET>,
            GetCurrentSelection: GetCurrentSelection::<Identity, Impl, OFFSET>,
            CurrentDefaultAction: CurrentDefaultAction::<Identity, Impl, OFFSET>,
            CachedChildId: CachedChildId::<Identity, Impl, OFFSET>,
            CachedName: CachedName::<Identity, Impl, OFFSET>,
            CachedValue: CachedValue::<Identity, Impl, OFFSET>,
            CachedDescription: CachedDescription::<Identity, Impl, OFFSET>,
            CachedRole: CachedRole::<Identity, Impl, OFFSET>,
            CachedState: CachedState::<Identity, Impl, OFFSET>,
            CachedHelp: CachedHelp::<Identity, Impl, OFFSET>,
            CachedKeyboardShortcut: CachedKeyboardShortcut::<Identity, Impl, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, Impl, OFFSET>,
            CachedDefaultAction: CachedDefaultAction::<Identity, Impl, OFFSET>,
            GetIAccessible: GetIAccessible::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>() -> IUIAutomationMultipleViewPattern_Vtbl {
        unsafe extern "system" fn GetViewName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetViewName(::core::mem::transmute_copy(&view)) {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCurrentView(::core::mem::transmute_copy(&view)).into()
        }
        unsafe extern "system" fn CurrentCurrentView<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSupportedViews<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCurrentView<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSupportedViews<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedSupportedViews() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetViewName: GetViewName::<Identity, Impl, OFFSET>,
            SetCurrentView: SetCurrentView::<Identity, Impl, OFFSET>,
            CurrentCurrentView: CurrentCurrentView::<Identity, Impl, OFFSET>,
            GetCurrentSupportedViews: GetCurrentSupportedViews::<Identity, Impl, OFFSET>,
            CachedCurrentView: CachedCurrentView::<Identity, Impl, OFFSET>,
            GetCachedSupportedViews: GetCachedSupportedViews::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotCondition_Impl, const OFFSET: isize>() -> IUIAutomationNotCondition_Vtbl {
        unsafe extern "system" fn GetChild<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChild() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(), GetChild: GetChild::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationNotCondition as ::windows::core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationNotificationEventHandler_Impl: Sized {
    fn HandleNotificationEvent(&mut self, sender: &::core::option::Option<IUIAutomationElement>, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: &super::super::Foundation::BSTR, activityid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationNotificationEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotificationEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationNotificationEventHandler_Vtbl {
        unsafe extern "system" fn HandleNotificationEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotificationEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, activityid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandleNotificationEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&notificationkind), ::core::mem::transmute_copy(&notificationprocessing), ::core::mem::transmute_copy(&displaystring), ::core::mem::transmute_copy(&activityid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HandleNotificationEvent: HandleNotificationEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationNotificationEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationObjectModelPattern_Impl: Sized {
    fn GetUnderlyingObjectModel(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl IUIAutomationObjectModelPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationObjectModelPattern_Impl, const OFFSET: isize>() -> IUIAutomationObjectModelPattern_Vtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationObjectModelPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUnderlyingObjectModel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>() -> IUIAutomationOrCondition_Vtbl {
        unsafe extern "system" fn ChildCount<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ChildCount() {
                ::core::result::Result::Ok(ok__) => {
                    *childcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::windows::core::RawPtr, childarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChildrenAsNativeArray(::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)).into()
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *childarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(),
            ChildCount: ChildCount::<Identity, Impl, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationOrCondition as ::windows::core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationPatternHandler_Impl: Sized {
    fn CreateClientWrapper(&mut self, ppatterninstance: &::core::option::Option<IUIAutomationPatternInstance>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Dispatch(&mut self, ptarget: &::core::option::Option<::windows::core::IUnknown>, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::Result<()>;
}
impl IUIAutomationPatternHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternHandler_Impl, const OFFSET: isize>() -> IUIAutomationPatternHandler_Vtbl {
        unsafe extern "system" fn CreateClientWrapper<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppatterninstance: ::windows::core::RawPtr, pclientwrapper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateClientWrapper(::core::mem::transmute(&ppatterninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *pclientwrapper = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatch<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Dispatch(::core::mem::transmute(&ptarget), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&cparams)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateClientWrapper: CreateClientWrapper::<Identity, Impl, OFFSET>,
            Dispatch: Dispatch::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternInstance_Impl, const OFFSET: isize>() -> IUIAutomationPatternInstance_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&cached), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pptr)).into()
        }
        unsafe extern "system" fn CallMethod<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CallMethod(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&cparams)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            CallMethod: CallMethod::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationPatternInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationPropertyChangedEventHandler_Impl: Sized {
    fn HandlePropertyChangedEvent(&mut self, sender: &::core::option::Option<IUIAutomationElement>, propertyid: i32, newvalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationPropertyChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationPropertyChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandlePropertyChangedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, propertyid: i32, newvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandlePropertyChangedEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&newvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HandlePropertyChangedEvent: HandlePropertyChangedEvent::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>() -> IUIAutomationPropertyCondition_Vtbl {
        unsafe extern "system" fn PropertyId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyId() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyValue() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyConditionFlags<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut PropertyConditionFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PropertyConditionFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationCondition_Vtbl::new::<Identity, Impl, OFFSET>(),
            PropertyId: PropertyId::<Identity, Impl, OFFSET>,
            PropertyValue: PropertyValue::<Identity, Impl, OFFSET>,
            PropertyConditionFlags: PropertyConditionFlags::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationPropertyCondition as ::windows::core::Interface>::IID || iid == &<IUIAutomationCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationProxyFactory_Impl: Sized {
    fn CreateProvider(&mut self, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32) -> ::windows::core::Result<IRawElementProviderSimple>;
    fn ProxyFactoryId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationProxyFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactory_Impl, const OFFSET: isize>() -> IUIAutomationProxyFactory_Vtbl {
        unsafe extern "system" fn CreateProvider<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, provider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateProvider(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *provider = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyFactoryId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoryid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyFactoryId() {
                ::core::result::Result::Ok(ok__) => {
                    *factoryid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateProvider: CreateProvider::<Identity, Impl, OFFSET>,
            ProxyFactoryId: ProxyFactoryId::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>() -> IUIAutomationProxyFactoryEntry_Vtbl {
        unsafe extern "system" fn ProxyFactory<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyFactory() {
                ::core::result::Result::Ok(ok__) => {
                    *factory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *classname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ImageName() {
                ::core::result::Result::Ok(ok__) => {
                    *imagename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSubstringMatch<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowSubstringMatch() {
                ::core::result::Result::Ok(ok__) => {
                    *allowsubstringmatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCheckBaseClass<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanCheckBaseClass() {
                ::core::result::Result::Ok(ok__) => {
                    *cancheckbaseclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeedsAdviseEvents<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NeedsAdviseEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *adviseevents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClassName(::core::mem::transmute_copy(&classname)).into()
        }
        unsafe extern "system" fn SetImageName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetImageName(::core::mem::transmute_copy(&imagename)).into()
        }
        unsafe extern "system" fn SetAllowSubstringMatch<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowSubstringMatch(::core::mem::transmute_copy(&allowsubstringmatch)).into()
        }
        unsafe extern "system" fn SetCanCheckBaseClass<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCanCheckBaseClass(::core::mem::transmute_copy(&cancheckbaseclass)).into()
        }
        unsafe extern "system" fn SetNeedsAdviseEvents<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNeedsAdviseEvents(::core::mem::transmute_copy(&adviseevents)).into()
        }
        unsafe extern "system" fn SetWinEventsForAutomationEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyid: i32, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWinEventsForAutomationEvent(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&winevents)).into()
        }
        unsafe extern "system" fn GetWinEventsForAutomationEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyid: i32, winevents: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWinEventsForAutomationEvent(::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *winevents = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProxyFactory: ProxyFactory::<Identity, Impl, OFFSET>,
            ClassName: ClassName::<Identity, Impl, OFFSET>,
            ImageName: ImageName::<Identity, Impl, OFFSET>,
            AllowSubstringMatch: AllowSubstringMatch::<Identity, Impl, OFFSET>,
            CanCheckBaseClass: CanCheckBaseClass::<Identity, Impl, OFFSET>,
            NeedsAdviseEvents: NeedsAdviseEvents::<Identity, Impl, OFFSET>,
            SetClassName: SetClassName::<Identity, Impl, OFFSET>,
            SetImageName: SetImageName::<Identity, Impl, OFFSET>,
            SetAllowSubstringMatch: SetAllowSubstringMatch::<Identity, Impl, OFFSET>,
            SetCanCheckBaseClass: SetCanCheckBaseClass::<Identity, Impl, OFFSET>,
            SetNeedsAdviseEvents: SetNeedsAdviseEvents::<Identity, Impl, OFFSET>,
            SetWinEventsForAutomationEvent: SetWinEventsForAutomationEvent::<Identity, Impl, OFFSET>,
            GetWinEventsForAutomationEvent: GetWinEventsForAutomationEvent::<Identity, Impl, OFFSET>,
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
    fn InsertEntry(&mut self, before: u32, factory: &::core::option::Option<IUIAutomationProxyFactoryEntry>) -> ::windows::core::Result<()>;
    fn RemoveEntry(&mut self, index: u32) -> ::windows::core::Result<()>;
    fn ClearTable(&mut self) -> ::windows::core::Result<()>;
    fn RestoreDefaultTable(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationProxyFactoryMapping_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>() -> IUIAutomationProxyFactoryMapping_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, table: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTable() {
                ::core::result::Result::Ok(ok__) => {
                    *table = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntry<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEntry(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *entry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTable(::core::mem::transmute_copy(&factorylist)).into()
        }
        unsafe extern "system" fn InsertEntries<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertEntries(::core::mem::transmute_copy(&before), ::core::mem::transmute_copy(&factorylist)).into()
        }
        unsafe extern "system" fn InsertEntry<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InsertEntry(::core::mem::transmute_copy(&before), ::core::mem::transmute(&factory)).into()
        }
        unsafe extern "system" fn RemoveEntry<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveEntry(::core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn ClearTable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearTable().into()
        }
        unsafe extern "system" fn RestoreDefaultTable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreDefaultTable().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetTable: GetTable::<Identity, Impl, OFFSET>,
            GetEntry: GetEntry::<Identity, Impl, OFFSET>,
            SetTable: SetTable::<Identity, Impl, OFFSET>,
            InsertEntries: InsertEntries::<Identity, Impl, OFFSET>,
            InsertEntry: InsertEntry::<Identity, Impl, OFFSET>,
            RemoveEntry: RemoveEntry::<Identity, Impl, OFFSET>,
            ClearTable: ClearTable::<Identity, Impl, OFFSET>,
            RestoreDefaultTable: RestoreDefaultTable::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>() -> IUIAutomationRangeValuePattern_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn CurrentValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMaximum<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMinimum<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLargeChange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentLargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSmallChange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentSmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedMaximum<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedMinimum<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLargeChange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedLargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSmallChange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedSmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            CurrentValue: CurrentValue::<Identity, Impl, OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Identity, Impl, OFFSET>,
            CurrentMaximum: CurrentMaximum::<Identity, Impl, OFFSET>,
            CurrentMinimum: CurrentMinimum::<Identity, Impl, OFFSET>,
            CurrentLargeChange: CurrentLargeChange::<Identity, Impl, OFFSET>,
            CurrentSmallChange: CurrentSmallChange::<Identity, Impl, OFFSET>,
            CachedValue: CachedValue::<Identity, Impl, OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Identity, Impl, OFFSET>,
            CachedMaximum: CachedMaximum::<Identity, Impl, OFFSET>,
            CachedMinimum: CachedMinimum::<Identity, Impl, OFFSET>,
            CachedLargeChange: CachedLargeChange::<Identity, Impl, OFFSET>,
            CachedSmallChange: CachedSmallChange::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>() -> IUIAutomationRegistrar_Vtbl {
        unsafe extern "system" fn RegisterProperty<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const UIAutomationPropertyInfo, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterProperty(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const UIAutomationEventInfo, eventid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterEvent(::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    *eventid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPattern<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterPattern(::core::mem::transmute_copy(&pattern), ::core::mem::transmute_copy(&ppatternid), ::core::mem::transmute_copy(&ppatternavailablepropertyid), ::core::mem::transmute_copy(&propertyidcount), ::core::mem::transmute_copy(&ppropertyids), ::core::mem::transmute_copy(&eventidcount), ::core::mem::transmute_copy(&peventids)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterProperty: RegisterProperty::<Identity, Impl, OFFSET>,
            RegisterEvent: RegisterEvent::<Identity, Impl, OFFSET>,
            RegisterPattern: RegisterPattern::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationScrollItemPattern_Vtbl {
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScrollIntoView().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>() -> IUIAutomationScrollPattern_Vtbl {
        unsafe extern "system" fn Scroll<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Scroll(::core::mem::transmute_copy(&horizontalamount), ::core::mem::transmute_copy(&verticalamount)).into()
        }
        unsafe extern "system" fn SetScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScrollPercent(::core::mem::transmute_copy(&horizontalpercent), ::core::mem::transmute_copy(&verticalpercent)).into()
        }
        unsafe extern "system" fn CurrentHorizontalScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentHorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticalScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentVerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHorizontalViewSize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentHorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticalViewSize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentVerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHorizontallyScrollable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentHorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticallyScrollable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentVerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontalScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedHorizontalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticalScrollPercent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedVerticalScrollPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontalViewSize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedHorizontalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticalViewSize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedVerticalViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontallyScrollable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedHorizontallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticallyScrollable<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedVerticallyScrollable() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            SetScrollPercent: SetScrollPercent::<Identity, Impl, OFFSET>,
            CurrentHorizontalScrollPercent: CurrentHorizontalScrollPercent::<Identity, Impl, OFFSET>,
            CurrentVerticalScrollPercent: CurrentVerticalScrollPercent::<Identity, Impl, OFFSET>,
            CurrentHorizontalViewSize: CurrentHorizontalViewSize::<Identity, Impl, OFFSET>,
            CurrentVerticalViewSize: CurrentVerticalViewSize::<Identity, Impl, OFFSET>,
            CurrentHorizontallyScrollable: CurrentHorizontallyScrollable::<Identity, Impl, OFFSET>,
            CurrentVerticallyScrollable: CurrentVerticallyScrollable::<Identity, Impl, OFFSET>,
            CachedHorizontalScrollPercent: CachedHorizontalScrollPercent::<Identity, Impl, OFFSET>,
            CachedVerticalScrollPercent: CachedVerticalScrollPercent::<Identity, Impl, OFFSET>,
            CachedHorizontalViewSize: CachedHorizontalViewSize::<Identity, Impl, OFFSET>,
            CachedVerticalViewSize: CachedVerticalViewSize::<Identity, Impl, OFFSET>,
            CachedHorizontallyScrollable: CachedHorizontallyScrollable::<Identity, Impl, OFFSET>,
            CachedVerticallyScrollable: CachedVerticallyScrollable::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationSelectionItemPattern_Vtbl {
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn CurrentIsSelected<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSelectionContainer<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentSelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsSelected<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSelectionContainer<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedSelectionContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            CurrentIsSelected: CurrentIsSelected::<Identity, Impl, OFFSET>,
            CurrentSelectionContainer: CurrentSelectionContainer::<Identity, Impl, OFFSET>,
            CachedIsSelected: CachedIsSelected::<Identity, Impl, OFFSET>,
            CachedSelectionContainer: CachedSelectionContainer::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>() -> IUIAutomationSelectionPattern_Vtbl {
        unsafe extern "system" fn GetCurrentSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanSelectMultiple<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsSelectionRequired<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanSelectMultiple<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCanSelectMultiple() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsSelectionRequired<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsSelectionRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCurrentSelection: GetCurrentSelection::<Identity, Impl, OFFSET>,
            CurrentCanSelectMultiple: CurrentCanSelectMultiple::<Identity, Impl, OFFSET>,
            CurrentIsSelectionRequired: CurrentIsSelectionRequired::<Identity, Impl, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, Impl, OFFSET>,
            CachedCanSelectMultiple: CachedCanSelectMultiple::<Identity, Impl, OFFSET>,
            CachedIsSelectionRequired: CachedIsSelectionRequired::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>() -> IUIAutomationSelectionPattern2_Vtbl {
        unsafe extern "system" fn CurrentFirstSelectedItem<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFirstSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLastSelectedItem<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentLastSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCurrentSelectedItem<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCurrentSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFirstSelectedItem<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedFirstSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLastSelectedItem<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedLastSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCurrentSelectedItem<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCurrentSelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemCount<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationSelectionPattern_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentFirstSelectedItem: CurrentFirstSelectedItem::<Identity, Impl, OFFSET>,
            CurrentLastSelectedItem: CurrentLastSelectedItem::<Identity, Impl, OFFSET>,
            CurrentCurrentSelectedItem: CurrentCurrentSelectedItem::<Identity, Impl, OFFSET>,
            CurrentItemCount: CurrentItemCount::<Identity, Impl, OFFSET>,
            CachedFirstSelectedItem: CachedFirstSelectedItem::<Identity, Impl, OFFSET>,
            CachedLastSelectedItem: CachedLastSelectedItem::<Identity, Impl, OFFSET>,
            CachedCurrentSelectedItem: CachedCurrentSelectedItem::<Identity, Impl, OFFSET>,
            CachedItemCount: CachedItemCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSelectionPattern2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationSelectionPattern as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationSpreadsheetItemPattern_Vtbl {
        unsafe extern "system" fn CurrentFormula<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFormula() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationObjects<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationTypes<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFormula<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedFormula() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedAnnotationObjects<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedAnnotationObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedAnnotationTypes<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedAnnotationTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CurrentFormula: CurrentFormula::<Identity, Impl, OFFSET>,
            GetCurrentAnnotationObjects: GetCurrentAnnotationObjects::<Identity, Impl, OFFSET>,
            GetCurrentAnnotationTypes: GetCurrentAnnotationTypes::<Identity, Impl, OFFSET>,
            CachedFormula: CachedFormula::<Identity, Impl, OFFSET>,
            GetCachedAnnotationObjects: GetCachedAnnotationObjects::<Identity, Impl, OFFSET>,
            GetCachedAnnotationTypes: GetCachedAnnotationTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetItemPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationSpreadsheetPattern_Impl: Sized {
    fn GetItemByName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<IUIAutomationElement>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationSpreadsheetPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetPattern_Impl, const OFFSET: isize>() -> IUIAutomationSpreadsheetPattern_Vtbl {
        unsafe extern "system" fn GetItemByName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetItemByName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetItemByName: GetItemByName::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationStructureChangedEventHandler_Impl: Sized {
    fn HandleStructureChangedEvent(&mut self, sender: &::core::option::Option<IUIAutomationElement>, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationStructureChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationStructureChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleStructureChangedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandleStructureChangedEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&runtimeid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            HandleStructureChangedEvent: HandleStructureChangedEvent::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>() -> IUIAutomationStylesPattern_Vtbl {
        unsafe extern "system" fn CurrentStyleId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentStyleId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStyleName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentStyleName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillColor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillPatternStyle<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentShape<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentShape() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillPatternColor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentFillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentExtendedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentExtendedPropertiesAsArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentExtendedPropertiesAsArray(::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        unsafe extern "system" fn CachedStyleId<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedStyleId() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedStyleName<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedStyleName() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillColor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedFillColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillPatternStyle<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedFillPatternStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedShape<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedShape() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillPatternColor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedFillPatternColor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedExtendedProperties<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedExtendedPropertiesAsArray<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCachedExtendedPropertiesAsArray(::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CurrentStyleId: CurrentStyleId::<Identity, Impl, OFFSET>,
            CurrentStyleName: CurrentStyleName::<Identity, Impl, OFFSET>,
            CurrentFillColor: CurrentFillColor::<Identity, Impl, OFFSET>,
            CurrentFillPatternStyle: CurrentFillPatternStyle::<Identity, Impl, OFFSET>,
            CurrentShape: CurrentShape::<Identity, Impl, OFFSET>,
            CurrentFillPatternColor: CurrentFillPatternColor::<Identity, Impl, OFFSET>,
            CurrentExtendedProperties: CurrentExtendedProperties::<Identity, Impl, OFFSET>,
            GetCurrentExtendedPropertiesAsArray: GetCurrentExtendedPropertiesAsArray::<Identity, Impl, OFFSET>,
            CachedStyleId: CachedStyleId::<Identity, Impl, OFFSET>,
            CachedStyleName: CachedStyleName::<Identity, Impl, OFFSET>,
            CachedFillColor: CachedFillColor::<Identity, Impl, OFFSET>,
            CachedFillPatternStyle: CachedFillPatternStyle::<Identity, Impl, OFFSET>,
            CachedShape: CachedShape::<Identity, Impl, OFFSET>,
            CachedFillPatternColor: CachedFillPatternColor::<Identity, Impl, OFFSET>,
            CachedExtendedProperties: CachedExtendedProperties::<Identity, Impl, OFFSET>,
            GetCachedExtendedPropertiesAsArray: GetCachedExtendedPropertiesAsArray::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>() -> IUIAutomationSynchronizedInputPattern_Vtbl {
        unsafe extern "system" fn StartListening<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartListening(::core::mem::transmute_copy(&inputtype)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StartListening: StartListening::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationTableItemPattern_Vtbl {
        unsafe extern "system" fn GetCurrentRowHeaderItems<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaderItems<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedRowHeaderItems<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedRowHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaderItems<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedColumnHeaderItems() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCurrentRowHeaderItems: GetCurrentRowHeaderItems::<Identity, Impl, OFFSET>,
            GetCurrentColumnHeaderItems: GetCurrentColumnHeaderItems::<Identity, Impl, OFFSET>,
            GetCachedRowHeaderItems: GetCachedRowHeaderItems::<Identity, Impl, OFFSET>,
            GetCachedColumnHeaderItems: GetCachedColumnHeaderItems::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>() -> IUIAutomationTablePattern_Vtbl {
        unsafe extern "system" fn GetCurrentRowHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowOrColumnMajor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentRowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedRowHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedRowHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCachedColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowOrColumnMajor<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedRowOrColumnMajor() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCurrentRowHeaders: GetCurrentRowHeaders::<Identity, Impl, OFFSET>,
            GetCurrentColumnHeaders: GetCurrentColumnHeaders::<Identity, Impl, OFFSET>,
            CurrentRowOrColumnMajor: CurrentRowOrColumnMajor::<Identity, Impl, OFFSET>,
            GetCachedRowHeaders: GetCachedRowHeaders::<Identity, Impl, OFFSET>,
            GetCachedColumnHeaders: GetCachedColumnHeaders::<Identity, Impl, OFFSET>,
            CachedRowOrColumnMajor: CachedRowOrColumnMajor::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>() -> IUIAutomationTextChildPattern_Vtbl {
        unsafe extern "system" fn TextContainer<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TextContainer() {
                ::core::result::Result::Ok(ok__) => {
                    *container = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TextRange() {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TextContainer: TextContainer::<Identity, Impl, OFFSET>,
            TextRange: TextRange::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>() -> IUIAutomationTextEditPattern_Vtbl {
        unsafe extern "system" fn GetActiveComposition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActiveComposition() {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConversionTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationTextPattern_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Identity, Impl, OFFSET>,
            GetConversionTarget: GetConversionTarget::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextEditPattern as ::windows::core::Interface>::IID || iid == &<IUIAutomationTextPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationTextEditTextChangedEventHandler_Impl: Sized {
    fn HandleTextEditTextChangedEvent(&mut self, sender: &::core::option::Option<IUIAutomationElement>, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationTextEditTextChangedEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: isize>() -> IUIAutomationTextEditTextChangedEventHandler_Vtbl {
        unsafe extern "system" fn HandleTextEditTextChangedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandleTextEditTextChangedEvent(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&texteditchangetype), ::core::mem::transmute_copy(&eventstrings)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            HandleTextEditTextChangedEvent: HandleTextEditTextChangedEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextEditTextChangedEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTextPattern_Impl: Sized {
    fn RangeFromPoint(&mut self, pt: &super::super::Foundation::POINT) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn RangeFromChild(&mut self, child: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn GetSelection(&mut self) -> ::windows::core::Result<IUIAutomationTextRangeArray>;
    fn GetVisibleRanges(&mut self) -> ::windows::core::Result<IUIAutomationTextRangeArray>;
    fn DocumentRange(&mut self) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn SupportedTextSelection(&mut self) -> ::windows::core::Result<SupportedTextSelection>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTextPattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>() -> IUIAutomationTextPattern_Vtbl {
        unsafe extern "system" fn RangeFromPoint<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromPoint(::core::mem::transmute_copy(&pt)) {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromChild<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, child: ::windows::core::RawPtr, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromChild(::core::mem::transmute(&child)) {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *ranges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVisibleRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *ranges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentRange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DocumentRange() {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedtextselection: *mut SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SupportedTextSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedtextselection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RangeFromPoint: RangeFromPoint::<Identity, Impl, OFFSET>,
            RangeFromChild: RangeFromChild::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Identity, Impl, OFFSET>,
            DocumentRange: DocumentRange::<Identity, Impl, OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTextPattern2_Impl: Sized + IUIAutomationTextPattern_Impl {
    fn RangeFromAnnotation(&mut self, annotation: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn GetCaretRange(&mut self, isactive: *mut super::super::Foundation::BOOL, range: *mut ::core::option::Option<IUIAutomationTextRange>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTextPattern2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern2_Impl, const OFFSET: isize>() -> IUIAutomationTextPattern2_Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotation: ::windows::core::RawPtr, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RangeFromAnnotation(::core::mem::transmute(&annotation)) {
                ::core::result::Result::Ok(ok__) => {
                    *range = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCaretRange(::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&range)).into()
        }
        Self {
            base: IUIAutomationTextPattern_Vtbl::new::<Identity, Impl, OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Identity, Impl, OFFSET>,
            GetCaretRange: GetCaretRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationTextPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationTextRange_Impl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn Compare(&mut self, range: &::core::option::Option<IUIAutomationTextRange>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CompareEndpoints(&mut self, srcendpoint: TextPatternRangeEndpoint, range: &::core::option::Option<IUIAutomationTextRange>, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<i32>;
    fn ExpandToEnclosingUnit(&mut self, textunit: TextUnit) -> ::windows::core::Result<()>;
    fn FindAttribute(&mut self, attr: i32, val: &super::super::System::Com::VARIANT, backward: super::super::Foundation::BOOL) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn FindText(&mut self, text: &super::super::Foundation::BSTR, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL) -> ::windows::core::Result<IUIAutomationTextRange>;
    fn GetAttributeValue(&mut self, attr: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetBoundingRectangles(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEnclosingElement(&mut self) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetText(&mut self, maxlength: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Move(&mut self, unit: TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByUnit(&mut self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows::core::Result<i32>;
    fn MoveEndpointByRange(&mut self, srcendpoint: TextPatternRangeEndpoint, range: &::core::option::Option<IUIAutomationTextRange>, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::Result<()>;
    fn Select(&mut self) -> ::windows::core::Result<()>;
    fn AddToSelection(&mut self) -> ::windows::core::Result<()>;
    fn RemoveFromSelection(&mut self) -> ::windows::core::Result<()>;
    fn ScrollIntoView(&mut self, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetChildren(&mut self) -> ::windows::core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationTextRange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>() -> IUIAutomationTextRange_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *clonedrange = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Compare(::core::mem::transmute(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *aresame = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint, compvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompareEndpoints(::core::mem::transmute_copy(&srcendpoint), ::core::mem::transmute(&range), ::core::mem::transmute_copy(&targetendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *compvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textunit: TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExpandToEnclosingUnit(::core::mem::transmute_copy(&textunit)).into()
        }
        unsafe extern "system" fn FindAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: i32, val: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, backward: super::super::Foundation::BOOL, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindAttribute(::core::mem::transmute_copy(&attr), ::core::mem::transmute_copy(&val), ::core::mem::transmute_copy(&backward)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindText(::core::mem::transmute_copy(&text), ::core::mem::transmute_copy(&backward), ::core::mem::transmute_copy(&ignorecase)) {
                ::core::result::Result::Ok(ok__) => {
                    *found = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: i32, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeValue(::core::mem::transmute_copy(&attr)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingrects: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBoundingRectangles() {
                ::core::result::Result::Ok(ok__) => {
                    *boundingrects = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosingelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnclosingElement() {
                ::core::result::Result::Ok(ok__) => {
                    *enclosingelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetText(::core::mem::transmute_copy(&maxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Move(::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *moved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MoveEndpointByUnit(::core::mem::transmute_copy(&endpoint), ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *moved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MoveEndpointByRange(::core::mem::transmute_copy(&srcendpoint), ::core::mem::transmute(&range), ::core::mem::transmute_copy(&targetendpoint)).into()
        }
        unsafe extern "system" fn Select<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Select().into()
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddToSelection().into()
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFromSelection().into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScrollIntoView(::core::mem::transmute_copy(&aligntotop)).into()
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Compare: Compare::<Identity, Impl, OFFSET>,
            CompareEndpoints: CompareEndpoints::<Identity, Impl, OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Identity, Impl, OFFSET>,
            FindAttribute: FindAttribute::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, Impl, OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Identity, Impl, OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Identity, Impl, OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange2_Impl, const OFFSET: isize>() -> IUIAutomationTextRange2_Vtbl {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        Self { base: IUIAutomationTextRange_Vtbl::new::<Identity, Impl, OFFSET>(), ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextRange2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationTextRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationTextRange3_Impl: Sized + IUIAutomationTextRange_Impl + IUIAutomationTextRange2_Impl {
    fn GetEnclosingElementBuildCache(&mut self, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetChildrenBuildCache(&mut self, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElementArray>;
    fn GetAttributeValues(&mut self, attributeids: *const i32, attributeidcount: i32) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationTextRange3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>() -> IUIAutomationTextRange3_Vtbl {
        unsafe extern "system" fn GetEnclosingElementBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, enclosingelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnclosingElementBuildCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *enclosingelement = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChildrenBuildCache(::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *children = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValues<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeids: *const i32, attributeidcount: i32, attributevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeValues(::core::mem::transmute_copy(&attributeids), ::core::mem::transmute_copy(&attributeidcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *attributevalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationTextRange2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetEnclosingElementBuildCache: GetEnclosingElementBuildCache::<Identity, Impl, OFFSET>,
            GetChildrenBuildCache: GetChildrenBuildCache::<Identity, Impl, OFFSET>,
            GetAttributeValues: GetAttributeValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextRange3 as ::windows::core::Interface>::IID || iid == &<IUIAutomationTextRange as ::windows::core::Interface>::IID || iid == &<IUIAutomationTextRange2 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationTextRangeArray_Impl: Sized {
    fn Length(&mut self) -> ::windows::core::Result<i32>;
    fn GetElement(&mut self, index: i32) -> ::windows::core::Result<IUIAutomationTextRange>;
}
impl IUIAutomationTextRangeArray_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>() -> IUIAutomationTextRangeArray_Vtbl {
        unsafe extern "system" fn Length<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *length = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetElement(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *element = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Length: Length::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>() -> IUIAutomationTogglePattern_Vtbl {
        unsafe extern "system" fn Toggle<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Toggle().into()
        }
        unsafe extern "system" fn CurrentToggleState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedToggleState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedToggleState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Toggle: Toggle::<Identity, Impl, OFFSET>,
            CurrentToggleState: CurrentToggleState::<Identity, Impl, OFFSET>,
            CachedToggleState: CachedToggleState::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>() -> IUIAutomationTransformPattern_Vtbl {
        unsafe extern "system" fn Move<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Move(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn Resize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn Rotate<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Rotate(::core::mem::transmute_copy(&degrees)).into()
        }
        unsafe extern "system" fn CurrentCanMove<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCanMove() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanResize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCanResize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanRotate<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMove<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCanMove() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanResize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCanResize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanRotate<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCanRotate() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Move: Move::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            CurrentCanMove: CurrentCanMove::<Identity, Impl, OFFSET>,
            CurrentCanResize: CurrentCanResize::<Identity, Impl, OFFSET>,
            CurrentCanRotate: CurrentCanRotate::<Identity, Impl, OFFSET>,
            CachedCanMove: CachedCanMove::<Identity, Impl, OFFSET>,
            CachedCanResize: CachedCanResize::<Identity, Impl, OFFSET>,
            CachedCanRotate: CachedCanRotate::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>() -> IUIAutomationTransformPattern2_Vtbl {
        unsafe extern "system" fn Zoom<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Zoom(::core::mem::transmute_copy(&zoomvalue)).into()
        }
        unsafe extern "system" fn ZoomByUnit<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ZoomByUnit(::core::mem::transmute_copy(&zoomunit)).into()
        }
        unsafe extern "system" fn CurrentCanZoom<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanZoom<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCanZoom() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomLevel<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomLevel<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedZoomLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomMinimum<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentZoomMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomMinimum<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedZoomMinimum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomMaximum<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentZoomMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomMaximum<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedZoomMaximum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IUIAutomationTransformPattern_Vtbl::new::<Identity, Impl, OFFSET>(),
            Zoom: Zoom::<Identity, Impl, OFFSET>,
            ZoomByUnit: ZoomByUnit::<Identity, Impl, OFFSET>,
            CurrentCanZoom: CurrentCanZoom::<Identity, Impl, OFFSET>,
            CachedCanZoom: CachedCanZoom::<Identity, Impl, OFFSET>,
            CurrentZoomLevel: CurrentZoomLevel::<Identity, Impl, OFFSET>,
            CachedZoomLevel: CachedZoomLevel::<Identity, Impl, OFFSET>,
            CurrentZoomMinimum: CurrentZoomMinimum::<Identity, Impl, OFFSET>,
            CachedZoomMinimum: CachedZoomMinimum::<Identity, Impl, OFFSET>,
            CurrentZoomMaximum: CurrentZoomMaximum::<Identity, Impl, OFFSET>,
            CachedZoomMaximum: CachedZoomMaximum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTransformPattern2 as ::windows::core::Interface>::IID || iid == &<IUIAutomationTransformPattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationTreeWalker_Impl: Sized {
    fn GetParentElement(&mut self, element: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetFirstChildElement(&mut self, element: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetLastChildElement(&mut self, element: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetNextSiblingElement(&mut self, element: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElement(&mut self, element: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn NormalizeElement(&mut self, element: &::core::option::Option<IUIAutomationElement>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetParentElementBuildCache(&mut self, element: &::core::option::Option<IUIAutomationElement>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetFirstChildElementBuildCache(&mut self, element: &::core::option::Option<IUIAutomationElement>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetLastChildElementBuildCache(&mut self, element: &::core::option::Option<IUIAutomationElement>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetNextSiblingElementBuildCache(&mut self, element: &::core::option::Option<IUIAutomationElement>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElementBuildCache(&mut self, element: &::core::option::Option<IUIAutomationElement>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn NormalizeElementBuildCache(&mut self, element: &::core::option::Option<IUIAutomationElement>, cacherequest: &::core::option::Option<IUIAutomationCacheRequest>) -> ::windows::core::Result<IUIAutomationElement>;
    fn Condition(&mut self) -> ::windows::core::Result<IUIAutomationCondition>;
}
impl IUIAutomationTreeWalker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>() -> IUIAutomationTreeWalker_Vtbl {
        unsafe extern "system" fn GetParentElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParentElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, first: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFirstChildElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *first = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastChildElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, last: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastChildElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *last = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNextSiblingElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *next = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, previous: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreviousSiblingElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *previous = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizeElement<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, normalized: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NormalizeElement(::core::mem::transmute(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *normalized = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentElementBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParentElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *parent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildElementBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, first: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFirstChildElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *first = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastChildElementBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, last: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastChildElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *last = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingElementBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNextSiblingElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *next = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElementBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, previous: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPreviousSiblingElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *previous = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizeElementBuildCache<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, normalized: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NormalizeElementBuildCache(::core::mem::transmute(&element), ::core::mem::transmute(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *normalized = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Condition<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Condition() {
                ::core::result::Result::Ok(ok__) => {
                    *condition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetParentElement: GetParentElement::<Identity, Impl, OFFSET>,
            GetFirstChildElement: GetFirstChildElement::<Identity, Impl, OFFSET>,
            GetLastChildElement: GetLastChildElement::<Identity, Impl, OFFSET>,
            GetNextSiblingElement: GetNextSiblingElement::<Identity, Impl, OFFSET>,
            GetPreviousSiblingElement: GetPreviousSiblingElement::<Identity, Impl, OFFSET>,
            NormalizeElement: NormalizeElement::<Identity, Impl, OFFSET>,
            GetParentElementBuildCache: GetParentElementBuildCache::<Identity, Impl, OFFSET>,
            GetFirstChildElementBuildCache: GetFirstChildElementBuildCache::<Identity, Impl, OFFSET>,
            GetLastChildElementBuildCache: GetLastChildElementBuildCache::<Identity, Impl, OFFSET>,
            GetNextSiblingElementBuildCache: GetNextSiblingElementBuildCache::<Identity, Impl, OFFSET>,
            GetPreviousSiblingElementBuildCache: GetPreviousSiblingElementBuildCache::<Identity, Impl, OFFSET>,
            NormalizeElementBuildCache: NormalizeElementBuildCache::<Identity, Impl, OFFSET>,
            Condition: Condition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTreeWalker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationValuePattern_Impl: Sized {
    fn SetValue(&mut self, val: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CurrentValue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentIsReadOnly(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CachedValue(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CachedIsReadOnly(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationValuePattern_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>() -> IUIAutomationValuePattern_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn CurrentValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            CurrentValue: CurrentValue::<Identity, Impl, OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Identity, Impl, OFFSET>,
            CachedValue: CachedValue::<Identity, Impl, OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationVirtualizedItemPattern_Impl, const OFFSET: isize>() -> IUIAutomationVirtualizedItemPattern_Vtbl {
        unsafe extern "system" fn Realize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationVirtualizedItemPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Realize().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Realize: Realize::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>() -> IUIAutomationWindowPattern_Vtbl {
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn WaitForInputIdle<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, success: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WaitForInputIdle(::core::mem::transmute_copy(&milliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *success = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowVisualState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWindowVisualState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CurrentCanMaximize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCanMaximize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanMinimize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCanMinimize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsModal<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsModal() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsTopmost<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentIsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWindowVisualState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentWindowVisualState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWindowInteractionState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentWindowInteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMaximize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCanMaximize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMinimize<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedCanMinimize() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsModal<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsModal() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsTopmost<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedIsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedWindowVisualState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedWindowVisualState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedWindowInteractionState<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CachedWindowInteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, Impl, OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Identity, Impl, OFFSET>,
            SetWindowVisualState: SetWindowVisualState::<Identity, Impl, OFFSET>,
            CurrentCanMaximize: CurrentCanMaximize::<Identity, Impl, OFFSET>,
            CurrentCanMinimize: CurrentCanMinimize::<Identity, Impl, OFFSET>,
            CurrentIsModal: CurrentIsModal::<Identity, Impl, OFFSET>,
            CurrentIsTopmost: CurrentIsTopmost::<Identity, Impl, OFFSET>,
            CurrentWindowVisualState: CurrentWindowVisualState::<Identity, Impl, OFFSET>,
            CurrentWindowInteractionState: CurrentWindowInteractionState::<Identity, Impl, OFFSET>,
            CachedCanMaximize: CachedCanMaximize::<Identity, Impl, OFFSET>,
            CachedCanMinimize: CachedCanMinimize::<Identity, Impl, OFFSET>,
            CachedIsModal: CachedIsModal::<Identity, Impl, OFFSET>,
            CachedIsTopmost: CachedIsTopmost::<Identity, Impl, OFFSET>,
            CachedWindowVisualState: CachedWindowVisualState::<Identity, Impl, OFFSET>,
            CachedWindowInteractionState: CachedWindowInteractionState::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueProvider_Impl, const OFFSET: isize>() -> IValueProvider_Vtbl {
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Identity: ::windows::core::IUnknownImpl, Impl: IValueProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizedItemProvider_Impl, const OFFSET: isize>() -> IVirtualizedItemProvider_Vtbl {
        unsafe extern "system" fn Realize<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizedItemProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Realize().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Realize: Realize::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>() -> IWindowProvider_Vtbl {
        unsafe extern "system" fn SetVisualState<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVisualState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn WaitForInputIdle<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WaitForInputIdle(::core::mem::transmute_copy(&milliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMaximize<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanMaximize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMinimize<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanMinimize() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsModal<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsModal() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowVisualState<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WindowVisualState() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowInteractionState<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WindowInteractionState() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTopmost<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetVisualState: SetVisualState::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Identity, Impl, OFFSET>,
            CanMaximize: CanMaximize::<Identity, Impl, OFFSET>,
            CanMinimize: CanMinimize::<Identity, Impl, OFFSET>,
            IsModal: IsModal::<Identity, Impl, OFFSET>,
            WindowVisualState: WindowVisualState::<Identity, Impl, OFFSET>,
            WindowInteractionState: WindowInteractionState::<Identity, Impl, OFFSET>,
            IsTopmost: IsTopmost::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowProvider as ::windows::core::Interface>::IID
    }
}
