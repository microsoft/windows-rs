pub trait IAccIdentityImpl: Sized {
    fn GetIdentityString();
}
impl ::windows::core::RuntimeName for IAccIdentity {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IAccIdentity";
}
impl IAccIdentityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccIdentityImpl, const OFFSET: isize>() -> IAccIdentityVtbl {
        unsafe extern "system" fn GetIdentityString<Impl: IAccIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentityString(dwidchild, ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccIdentity>, ::windows::core::GetTrustLevel, GetIdentityString::<Impl, OFFSET>)
    }
}
pub trait IAccPropServerImpl: Sized {
    fn GetPropValue();
}
impl ::windows::core::RuntimeName for IAccPropServer {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IAccPropServer";
}
impl IAccPropServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServerImpl, const OFFSET: isize>() -> IAccPropServerVtbl {
        unsafe extern "system" fn GetPropValue<Impl: IAccPropServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropValue(pidstring, dwidstringlen, &*(&idprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&pfhasprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccPropServer>, ::windows::core::GetTrustLevel, GetPropValue::<Impl, OFFSET>)
    }
}
pub trait IAccPropServicesImpl: Sized {
    fn SetPropValue();
    fn SetPropServer();
    fn ClearProps();
    fn SetHwndProp();
    fn SetHwndPropStr();
    fn SetHwndPropServer();
    fn ClearHwndProps();
    fn ComposeHwndIdentityString();
    fn DecomposeHwndIdentityString();
    fn SetHmenuProp();
    fn SetHmenuPropStr();
    fn SetHmenuPropServer();
    fn ClearHmenuProps();
    fn ComposeHmenuIdentityString();
    fn DecomposeHmenuIdentityString();
}
impl ::windows::core::RuntimeName for IAccPropServices {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IAccPropServices";
}
impl IAccPropServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServicesImpl, const OFFSET: isize>() -> IAccPropServicesVtbl {
        unsafe extern "system" fn SetPropValue<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropValue(pidstring, dwidstringlen, &*(&idprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&var as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropServer<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: &::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPropServer(pidstring, dwidstringlen, &*(&paprops as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cprops, &*(&pserver as *const <IAccPropServer as ::windows::core::Abi>::Abi as *const <IAccPropServer as ::windows::core::DefaultType>::DefaultType), annoscope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearProps<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: &::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearProps(pidstring, dwidstringlen, &*(&paprops as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cprops) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHwndProp<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHwndProp(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                idobject,
                idchild,
                &*(&idprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&var as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHwndPropStr<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHwndPropStr(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                idobject,
                idchild,
                &*(&idprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&str as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHwndPropServer<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: &::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHwndPropServer(
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                idobject,
                idchild,
                &*(&paprops as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cprops,
                &*(&pserver as *const <IAccPropServer as ::windows::core::Abi>::Abi as *const <IAccPropServer as ::windows::core::DefaultType>::DefaultType),
                annoscope,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearHwndProps<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: &::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearHwndProps(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), idobject, idchild, &*(&paprops as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cprops) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComposeHwndIdentityString<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComposeHwndIdentityString(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), idobject, idchild, ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecomposeHwndIdentityString<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecomposeHwndIdentityString(pidstring, dwidstringlen, ::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pidobject), ::core::mem::transmute_copy(&pidchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHmenuProp<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHmenuProp(
                &*(&hmenu as *const <super::WindowsAndMessaging::HMENU as ::windows::core::Abi>::Abi as *const <super::WindowsAndMessaging::HMENU as ::windows::core::DefaultType>::DefaultType),
                idchild,
                &*(&idprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&var as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHmenuPropStr<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHmenuPropStr(
                &*(&hmenu as *const <super::WindowsAndMessaging::HMENU as ::windows::core::Abi>::Abi as *const <super::WindowsAndMessaging::HMENU as ::windows::core::DefaultType>::DefaultType),
                idchild,
                &*(&idprop as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&str as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHmenuPropServer<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: &::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHmenuPropServer(
                &*(&hmenu as *const <super::WindowsAndMessaging::HMENU as ::windows::core::Abi>::Abi as *const <super::WindowsAndMessaging::HMENU as ::windows::core::DefaultType>::DefaultType),
                idchild,
                &*(&paprops as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                cprops,
                &*(&pserver as *const <IAccPropServer as ::windows::core::Abi>::Abi as *const <IAccPropServer as ::windows::core::DefaultType>::DefaultType),
                annoscope,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearHmenuProps<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: &::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearHmenuProps(&*(&hmenu as *const <super::WindowsAndMessaging::HMENU as ::windows::core::Abi>::Abi as *const <super::WindowsAndMessaging::HMENU as ::windows::core::DefaultType>::DefaultType), idchild, &*(&paprops as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cprops) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComposeHmenuIdentityString<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComposeHmenuIdentityString(&*(&hmenu as *const <super::WindowsAndMessaging::HMENU as ::windows::core::Abi>::Abi as *const <super::WindowsAndMessaging::HMENU as ::windows::core::DefaultType>::DefaultType), idchild, ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecomposeHmenuIdentityString<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DecomposeHmenuIdentityString(pidstring, dwidstringlen, ::core::mem::transmute_copy(&phmenu), ::core::mem::transmute_copy(&pidchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAccPropServices>,
            ::windows::core::GetTrustLevel,
            SetPropValue::<Impl, OFFSET>,
            SetPropServer::<Impl, OFFSET>,
            ClearProps::<Impl, OFFSET>,
            SetHwndProp::<Impl, OFFSET>,
            SetHwndPropStr::<Impl, OFFSET>,
            SetHwndPropServer::<Impl, OFFSET>,
            ClearHwndProps::<Impl, OFFSET>,
            ComposeHwndIdentityString::<Impl, OFFSET>,
            DecomposeHwndIdentityString::<Impl, OFFSET>,
            SetHmenuProp::<Impl, OFFSET>,
            SetHmenuPropStr::<Impl, OFFSET>,
            SetHmenuPropServer::<Impl, OFFSET>,
            ClearHmenuProps::<Impl, OFFSET>,
            ComposeHmenuIdentityString::<Impl, OFFSET>,
            DecomposeHmenuIdentityString::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleImpl: Sized + IDispatchImpl {
    fn accParent();
    fn accChildCount();
    fn accChild();
    fn accName();
    fn accValue();
    fn accDescription();
    fn accRole();
    fn accState();
    fn accHelp();
    fn accHelpTopic();
    fn accKeyboardShortcut();
    fn accFocus();
    fn accSelection();
    fn accDefaultAction();
    fn accSelect();
    fn accLocation();
    fn accNavigate();
    fn accHitTest();
    fn accDoDefaultAction();
    fn SetaccName();
    fn SetaccValue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAccessible {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IAccessible";
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleImpl, const OFFSET: isize>() -> IAccessibleVtbl {
        unsafe extern "system" fn accParent<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdispparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accParent(::core::mem::transmute_copy(&ppdispparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accChildCount<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcountchildren: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accChildCount(::core::mem::transmute_copy(&pcountchildren)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accChild<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppdispchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accChild(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdispchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accName<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accName(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accValue<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accValue(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDescription<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accDescription(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accRole<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarrole: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accRole(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accState<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarstate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accState(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHelp<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accHelp(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pszhelp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHelpTopic<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpfile: *mut super::super::Foundation::BSTR, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pidtopic: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accHelpTopic(::core::mem::transmute_copy(&pszhelpfile), &*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pidtopic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accKeyboardShortcut<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accKeyboardShortcut(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pszkeyboardshortcut)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accFocus<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accFocus(::core::mem::transmute_copy(&pvarchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accSelection<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchildren: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accSelection(::core::mem::transmute_copy(&pvarchildren)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDefaultAction<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accDefaultAction(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pszdefaultaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accSelect<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accSelect(flagsselect, &*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accLocation<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accLocation(::core::mem::transmute_copy(&pxleft), ::core::mem::transmute_copy(&pytop), ::core::mem::transmute_copy(&pcxwidth), ::core::mem::transmute_copy(&pcyheight), &*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accNavigate<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, navdir: i32, varstart: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarendupat: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accNavigate(navdir, &*(&varstart as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarendupat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accHitTest<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xleft: i32, ytop: i32, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accHitTest(xleft, ytop, ::core::mem::transmute_copy(&pvarchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn accDoDefaultAction<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).accDoDefaultAction(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetaccName<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, szname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetaccName(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&szname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetaccValue<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, szvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetaccValue(&*(&varchild as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&szvalue as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAccessible>,
            ::windows::core::GetTrustLevel,
            accParent::<Impl, OFFSET>,
            accChildCount::<Impl, OFFSET>,
            accChild::<Impl, OFFSET>,
            accName::<Impl, OFFSET>,
            accValue::<Impl, OFFSET>,
            accDescription::<Impl, OFFSET>,
            accRole::<Impl, OFFSET>,
            accState::<Impl, OFFSET>,
            accHelp::<Impl, OFFSET>,
            accHelpTopic::<Impl, OFFSET>,
            accKeyboardShortcut::<Impl, OFFSET>,
            accFocus::<Impl, OFFSET>,
            accSelection::<Impl, OFFSET>,
            accDefaultAction::<Impl, OFFSET>,
            accSelect::<Impl, OFFSET>,
            accLocation::<Impl, OFFSET>,
            accNavigate::<Impl, OFFSET>,
            accHitTest::<Impl, OFFSET>,
            accDoDefaultAction::<Impl, OFFSET>,
            SetaccName::<Impl, OFFSET>,
            SetaccValue::<Impl, OFFSET>,
        )
    }
}
pub trait IAccessibleExImpl: Sized {
    fn GetObjectForChild();
    fn GetIAccessiblePair();
    fn GetRuntimeId();
    fn ConvertReturnedElement();
}
impl ::windows::core::RuntimeName for IAccessibleEx {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IAccessibleEx";
}
impl IAccessibleExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleExImpl, const OFFSET: isize>() -> IAccessibleExVtbl {
        unsafe extern "system" fn GetObjectForChild<Impl: IAccessibleExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idchild: i32, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectForChild(idchild, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIAccessiblePair<Impl: IAccessibleExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacc: *mut ::windows::core::RawPtr, pidchild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIAccessiblePair(::core::mem::transmute_copy(&ppacc), ::core::mem::transmute_copy(&pidchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeId<Impl: IAccessibleExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRuntimeId(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertReturnedElement<Impl: IAccessibleExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, ppretvalout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertReturnedElement(&*(&pin as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppretvalout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccessibleEx>, ::windows::core::GetTrustLevel, GetObjectForChild::<Impl, OFFSET>, GetIAccessiblePair::<Impl, OFFSET>, GetRuntimeId::<Impl, OFFSET>, ConvertReturnedElement::<Impl, OFFSET>)
    }
}
pub trait IAccessibleHandlerImpl: Sized {
    fn AccessibleObjectFromID();
}
impl ::windows::core::RuntimeName for IAccessibleHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IAccessibleHandler";
}
impl IAccessibleHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHandlerImpl, const OFFSET: isize>() -> IAccessibleHandlerVtbl {
        unsafe extern "system" fn AccessibleObjectFromID<Impl: IAccessibleHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, lobjectid: i32, piaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessibleObjectFromID(hwnd, lobjectid, ::core::mem::transmute_copy(&piaccessible)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccessibleHandler>, ::windows::core::GetTrustLevel, AccessibleObjectFromID::<Impl, OFFSET>)
    }
}
pub trait IAccessibleHostingElementProvidersImpl: Sized {
    fn GetEmbeddedFragmentRoots();
    fn GetObjectIdForProvider();
}
impl ::windows::core::RuntimeName for IAccessibleHostingElementProviders {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IAccessibleHostingElementProviders";
}
impl IAccessibleHostingElementProvidersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHostingElementProvidersImpl, const OFFSET: isize>() -> IAccessibleHostingElementProvidersVtbl {
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Impl: IAccessibleHostingElementProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbeddedFragmentRoots(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectIdForProvider<Impl: IAccessibleHostingElementProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, pidobject: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectIdForProvider(&*(&pprovider as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pidobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccessibleHostingElementProviders>, ::windows::core::GetTrustLevel, GetEmbeddedFragmentRoots::<Impl, OFFSET>, GetObjectIdForProvider::<Impl, OFFSET>)
    }
}
pub trait IAccessibleWindowlessSiteImpl: Sized {
    fn AcquireObjectIdRange();
    fn ReleaseObjectIdRange();
    fn QueryObjectIdRanges();
    fn GetParentAccessible();
}
impl ::windows::core::RuntimeName for IAccessibleWindowlessSite {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IAccessibleWindowlessSite";
}
impl IAccessibleWindowlessSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWindowlessSiteImpl, const OFFSET: isize>() -> IAccessibleWindowlessSiteVtbl {
        unsafe extern "system" fn AcquireObjectIdRange<Impl: IAccessibleWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangesize: i32, prangeowner: ::windows::core::RawPtr, prangebase: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireObjectIdRange(rangesize, &*(&prangeowner as *const <IAccessibleHandler as ::windows::core::Abi>::Abi as *const <IAccessibleHandler as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prangebase)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseObjectIdRange<Impl: IAccessibleWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangebase: i32, prangeowner: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseObjectIdRange(rangebase, &*(&prangeowner as *const <IAccessibleHandler as ::windows::core::Abi>::Abi as *const <IAccessibleHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryObjectIdRanges<Impl: IAccessibleWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangesowner: ::windows::core::RawPtr, psaranges: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryObjectIdRanges(&*(&prangesowner as *const <IAccessibleHandler as ::windows::core::Abi>::Abi as *const <IAccessibleHandler as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&psaranges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentAccessible<Impl: IAccessibleWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentAccessible(::core::mem::transmute_copy(&ppparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAccessibleWindowlessSite>, ::windows::core::GetTrustLevel, AcquireObjectIdRange::<Impl, OFFSET>, ReleaseObjectIdRange::<Impl, OFFSET>, QueryObjectIdRanges::<Impl, OFFSET>, GetParentAccessible::<Impl, OFFSET>)
    }
}
pub trait IAnnotationProviderImpl: Sized {
    fn AnnotationTypeId();
    fn AnnotationTypeName();
    fn Author();
    fn DateTime();
    fn Target();
}
impl ::windows::core::RuntimeName for IAnnotationProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IAnnotationProvider";
}
impl IAnnotationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProviderImpl, const OFFSET: isize>() -> IAnnotationProviderVtbl {
        unsafe extern "system" fn AnnotationTypeId<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationTypeId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnnotationTypeName<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnnotationTypeName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTime<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateTime(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Target<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Target(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAnnotationProvider>, ::windows::core::GetTrustLevel, AnnotationTypeId::<Impl, OFFSET>, AnnotationTypeName::<Impl, OFFSET>, Author::<Impl, OFFSET>, DateTime::<Impl, OFFSET>, Target::<Impl, OFFSET>)
    }
}
pub trait ICustomNavigationProviderImpl: Sized {
    fn Navigate();
}
impl ::windows::core::RuntimeName for ICustomNavigationProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ICustomNavigationProvider";
}
impl ICustomNavigationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomNavigationProviderImpl, const OFFSET: isize>() -> ICustomNavigationProviderVtbl {
        unsafe extern "system" fn Navigate<Impl: ICustomNavigationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigate(direction, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICustomNavigationProvider>, ::windows::core::GetTrustLevel, Navigate::<Impl, OFFSET>)
    }
}
pub trait IDockProviderImpl: Sized {
    fn SetDockPosition();
    fn DockPosition();
}
impl ::windows::core::RuntimeName for IDockProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IDockProvider";
}
impl IDockProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockProviderImpl, const OFFSET: isize>() -> IDockProviderVtbl {
        unsafe extern "system" fn SetDockPosition<Impl: IDockProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockposition: DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDockPosition(dockposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DockPosition<Impl: IDockProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DockPosition(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDockProvider>, ::windows::core::GetTrustLevel, SetDockPosition::<Impl, OFFSET>, DockPosition::<Impl, OFFSET>)
    }
}
pub trait IDragProviderImpl: Sized {
    fn IsGrabbed();
    fn DropEffect();
    fn DropEffects();
    fn GetGrabbedItems();
}
impl ::windows::core::RuntimeName for IDragProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IDragProvider";
}
impl IDragProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragProviderImpl, const OFFSET: isize>() -> IDragProviderVtbl {
        unsafe extern "system" fn IsGrabbed<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGrabbed(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffect<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffect(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEffects<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEffects(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGrabbedItems<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGrabbedItems(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDragProvider>, ::windows::core::GetTrustLevel, IsGrabbed::<Impl, OFFSET>, DropEffect::<Impl, OFFSET>, DropEffects::<Impl, OFFSET>, GetGrabbedItems::<Impl, OFFSET>)
    }
}
pub trait IDropTargetProviderImpl: Sized {
    fn DropTargetEffect();
    fn DropTargetEffects();
}
impl ::windows::core::RuntimeName for IDropTargetProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IDropTargetProvider";
}
impl IDropTargetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProviderImpl, const OFFSET: isize>() -> IDropTargetProviderVtbl {
        unsafe extern "system" fn DropTargetEffect<Impl: IDropTargetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropTargetEffect(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropTargetEffects<Impl: IDropTargetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropTargetEffects(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDropTargetProvider>, ::windows::core::GetTrustLevel, DropTargetEffect::<Impl, OFFSET>, DropTargetEffects::<Impl, OFFSET>)
    }
}
pub trait IExpandCollapseProviderImpl: Sized {
    fn Expand();
    fn Collapse();
    fn ExpandCollapseState();
}
impl ::windows::core::RuntimeName for IExpandCollapseProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IExpandCollapseProvider";
}
impl IExpandCollapseProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProviderImpl, const OFFSET: isize>() -> IExpandCollapseProviderVtbl {
        unsafe extern "system" fn Expand<Impl: IExpandCollapseProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expand() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Impl: IExpandCollapseProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collapse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandCollapseState<Impl: IExpandCollapseProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandCollapseState(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExpandCollapseProvider>, ::windows::core::GetTrustLevel, Expand::<Impl, OFFSET>, Collapse::<Impl, OFFSET>, ExpandCollapseState::<Impl, OFFSET>)
    }
}
pub trait IGridItemProviderImpl: Sized {
    fn Row();
    fn Column();
    fn RowSpan();
    fn ColumnSpan();
    fn ContainingGrid();
}
impl ::windows::core::RuntimeName for IGridItemProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IGridItemProvider";
}
impl IGridItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProviderImpl, const OFFSET: isize>() -> IGridItemProviderVtbl {
        unsafe extern "system" fn Row<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Row(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Column<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Column(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowSpan<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowSpan(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnSpan<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnSpan(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainingGrid<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainingGrid(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridItemProvider>, ::windows::core::GetTrustLevel, Row::<Impl, OFFSET>, Column::<Impl, OFFSET>, RowSpan::<Impl, OFFSET>, ColumnSpan::<Impl, OFFSET>, ContainingGrid::<Impl, OFFSET>)
    }
}
pub trait IGridProviderImpl: Sized {
    fn GetItem();
    fn RowCount();
    fn ColumnCount();
}
impl ::windows::core::RuntimeName for IGridProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IGridProvider";
}
impl IGridProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridProviderImpl, const OFFSET: isize>() -> IGridProviderVtbl {
        unsafe extern "system" fn GetItem<Impl: IGridProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(row, column, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowCount<Impl: IGridProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowCount(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColumnCount<Impl: IGridProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColumnCount(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridProvider>, ::windows::core::GetTrustLevel, GetItem::<Impl, OFFSET>, RowCount::<Impl, OFFSET>, ColumnCount::<Impl, OFFSET>)
    }
}
pub trait IInvokeProviderImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for IInvokeProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IInvokeProvider";
}
impl IInvokeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInvokeProviderImpl, const OFFSET: isize>() -> IInvokeProviderVtbl {
        unsafe extern "system" fn Invoke<Impl: IInvokeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInvokeProvider>, ::windows::core::GetTrustLevel, Invoke::<Impl, OFFSET>)
    }
}
pub trait IItemContainerProviderImpl: Sized {
    fn FindItemByProperty();
}
impl ::windows::core::RuntimeName for IItemContainerProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IItemContainerProvider";
}
impl IItemContainerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerProviderImpl, const OFFSET: isize>() -> IItemContainerProviderVtbl {
        unsafe extern "system" fn FindItemByProperty<Impl: IItemContainerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: ::windows::core::RawPtr, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemByProperty(&*(&pstartafter as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType), propertyid, &*(&value as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfound)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemContainerProvider>, ::windows::core::GetTrustLevel, FindItemByProperty::<Impl, OFFSET>)
    }
}
pub trait ILegacyIAccessibleProviderImpl: Sized {
    fn Select();
    fn DoDefaultAction();
    fn SetValue();
    fn GetIAccessible();
    fn ChildId();
    fn Name();
    fn Value();
    fn Description();
    fn Role();
    fn State();
    fn Help();
    fn KeyboardShortcut();
    fn GetSelection();
    fn DefaultAction();
}
impl ::windows::core::RuntimeName for ILegacyIAccessibleProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ILegacyIAccessibleProvider";
}
impl ILegacyIAccessibleProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>() -> ILegacyIAccessibleProviderVtbl {
        unsafe extern "system" fn Select<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Select(flagsselect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoDefaultAction<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoDefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&szvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIAccessible<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIAccessible(::core::mem::transmute_copy(&ppaccessible)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildId<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildId(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&pszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Role<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Role(::core::mem::transmute_copy(&pdwrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pdwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Help<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Help(::core::mem::transmute_copy(&pszhelp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardShortcut<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardShortcut(::core::mem::transmute_copy(&pszkeyboardshortcut)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(::core::mem::transmute_copy(&pvarselectedchildren)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultAction<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultAction(::core::mem::transmute_copy(&pszdefaultaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILegacyIAccessibleProvider>,
            ::windows::core::GetTrustLevel,
            Select::<Impl, OFFSET>,
            DoDefaultAction::<Impl, OFFSET>,
            SetValue::<Impl, OFFSET>,
            GetIAccessible::<Impl, OFFSET>,
            ChildId::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            Value::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            Role::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            Help::<Impl, OFFSET>,
            KeyboardShortcut::<Impl, OFFSET>,
            GetSelection::<Impl, OFFSET>,
            DefaultAction::<Impl, OFFSET>,
        )
    }
}
pub trait IMultipleViewProviderImpl: Sized {
    fn GetViewName();
    fn SetCurrentView();
    fn CurrentView();
    fn GetSupportedViews();
}
impl ::windows::core::RuntimeName for IMultipleViewProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IMultipleViewProvider";
}
impl IMultipleViewProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProviderImpl, const OFFSET: isize>() -> IMultipleViewProviderVtbl {
        unsafe extern "system" fn GetViewName<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewName(viewid, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentView(viewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentView<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentView(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedViews<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedViews(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMultipleViewProvider>, ::windows::core::GetTrustLevel, GetViewName::<Impl, OFFSET>, SetCurrentView::<Impl, OFFSET>, CurrentView::<Impl, OFFSET>, GetSupportedViews::<Impl, OFFSET>)
    }
}
pub trait IObjectModelProviderImpl: Sized {
    fn GetUnderlyingObjectModel();
}
impl ::windows::core::RuntimeName for IObjectModelProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IObjectModelProvider";
}
impl IObjectModelProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectModelProviderImpl, const OFFSET: isize>() -> IObjectModelProviderVtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Impl: IObjectModelProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderlyingObjectModel(::core::mem::transmute_copy(&ppunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectModelProvider>, ::windows::core::GetTrustLevel, GetUnderlyingObjectModel::<Impl, OFFSET>)
    }
}
pub trait IProxyProviderWinEventHandlerImpl: Sized {
    fn RespondToWinEvent();
}
impl ::windows::core::RuntimeName for IProxyProviderWinEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IProxyProviderWinEventHandler";
}
impl IProxyProviderWinEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventHandlerImpl, const OFFSET: isize>() -> IProxyProviderWinEventHandlerVtbl {
        unsafe extern "system" fn RespondToWinEvent<Impl: IProxyProviderWinEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RespondToWinEvent(idwinevent, &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), idobject, idchild, &*(&psink as *const <IProxyProviderWinEventSink as ::windows::core::Abi>::Abi as *const <IProxyProviderWinEventSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProxyProviderWinEventHandler>, ::windows::core::GetTrustLevel, RespondToWinEvent::<Impl, OFFSET>)
    }
}
pub trait IProxyProviderWinEventSinkImpl: Sized {
    fn AddAutomationPropertyChangedEvent();
    fn AddAutomationEvent();
    fn AddStructureChangedEvent();
}
impl ::windows::core::RuntimeName for IProxyProviderWinEventSink {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IProxyProviderWinEventSink";
}
impl IProxyProviderWinEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventSinkImpl, const OFFSET: isize>() -> IProxyProviderWinEventSinkVtbl {
        unsafe extern "system" fn AddAutomationPropertyChangedEvent<Impl: IProxyProviderWinEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, id: i32, newvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAutomationPropertyChangedEvent(&*(&pprovider as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType), id, &*(&newvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAutomationEvent<Impl: IProxyProviderWinEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAutomationEvent(&*(&pprovider as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType), id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStructureChangedEvent<Impl: IProxyProviderWinEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStructureChangedEvent(&*(&pprovider as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType), structurechangetype, &*(&runtimeid as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProxyProviderWinEventSink>, ::windows::core::GetTrustLevel, AddAutomationPropertyChangedEvent::<Impl, OFFSET>, AddAutomationEvent::<Impl, OFFSET>, AddStructureChangedEvent::<Impl, OFFSET>)
    }
}
pub trait IRangeValueProviderImpl: Sized {
    fn SetValue();
    fn Value();
    fn IsReadOnly();
    fn Maximum();
    fn Minimum();
    fn LargeChange();
    fn SmallChange();
}
impl ::windows::core::RuntimeName for IRangeValueProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRangeValueProvider";
}
impl IRangeValueProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProviderImpl, const OFFSET: isize>() -> IRangeValueProviderVtbl {
        unsafe extern "system" fn SetValue<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(val) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maximum<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Maximum(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Minimum<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Minimum(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeChange<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LargeChange(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmallChange<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmallChange(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRangeValueProvider>, ::windows::core::GetTrustLevel, SetValue::<Impl, OFFSET>, Value::<Impl, OFFSET>, IsReadOnly::<Impl, OFFSET>, Maximum::<Impl, OFFSET>, Minimum::<Impl, OFFSET>, LargeChange::<Impl, OFFSET>, SmallChange::<Impl, OFFSET>)
    }
}
pub trait IRawElementProviderAdviseEventsImpl: Sized {
    fn AdviseEventAdded();
    fn AdviseEventRemoved();
}
impl ::windows::core::RuntimeName for IRawElementProviderAdviseEvents {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRawElementProviderAdviseEvents";
}
impl IRawElementProviderAdviseEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderAdviseEventsImpl, const OFFSET: isize>() -> IRawElementProviderAdviseEventsVtbl {
        unsafe extern "system" fn AdviseEventAdded<Impl: IRawElementProviderAdviseEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseEventAdded(eventid, &*(&propertyids as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdviseEventRemoved<Impl: IRawElementProviderAdviseEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseEventRemoved(eventid, &*(&propertyids as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawElementProviderAdviseEvents>, ::windows::core::GetTrustLevel, AdviseEventAdded::<Impl, OFFSET>, AdviseEventRemoved::<Impl, OFFSET>)
    }
}
pub trait IRawElementProviderFragmentImpl: Sized {
    fn Navigate();
    fn GetRuntimeId();
    fn BoundingRectangle();
    fn GetEmbeddedFragmentRoots();
    fn SetFocus();
    fn FragmentRoot();
}
impl ::windows::core::RuntimeName for IRawElementProviderFragment {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRawElementProviderFragment";
}
impl IRawElementProviderFragmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>() -> IRawElementProviderFragmentVtbl {
        unsafe extern "system" fn Navigate<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigate(direction, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeId<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRuntimeId(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingRectangle<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut UiaRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingRectangle(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbeddedFragmentRoots(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FragmentRoot<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FragmentRoot(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawElementProviderFragment>, ::windows::core::GetTrustLevel, Navigate::<Impl, OFFSET>, GetRuntimeId::<Impl, OFFSET>, BoundingRectangle::<Impl, OFFSET>, GetEmbeddedFragmentRoots::<Impl, OFFSET>, SetFocus::<Impl, OFFSET>, FragmentRoot::<Impl, OFFSET>)
    }
}
pub trait IRawElementProviderFragmentRootImpl: Sized {
    fn ElementProviderFromPoint();
    fn GetFocus();
}
impl ::windows::core::RuntimeName for IRawElementProviderFragmentRoot {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRawElementProviderFragmentRoot";
}
impl IRawElementProviderFragmentRootVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragmentRootImpl, const OFFSET: isize>() -> IRawElementProviderFragmentRootVtbl {
        unsafe extern "system" fn ElementProviderFromPoint<Impl: IRawElementProviderFragmentRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementProviderFromPoint(x, y, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocus<Impl: IRawElementProviderFragmentRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocus(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawElementProviderFragmentRoot>, ::windows::core::GetTrustLevel, ElementProviderFromPoint::<Impl, OFFSET>, GetFocus::<Impl, OFFSET>)
    }
}
pub trait IRawElementProviderHostingAccessiblesImpl: Sized {
    fn GetEmbeddedAccessibles();
}
impl ::windows::core::RuntimeName for IRawElementProviderHostingAccessibles {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRawElementProviderHostingAccessibles";
}
impl IRawElementProviderHostingAccessiblesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHostingAccessiblesImpl, const OFFSET: isize>() -> IRawElementProviderHostingAccessiblesVtbl {
        unsafe extern "system" fn GetEmbeddedAccessibles<Impl: IRawElementProviderHostingAccessiblesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEmbeddedAccessibles(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawElementProviderHostingAccessibles>, ::windows::core::GetTrustLevel, GetEmbeddedAccessibles::<Impl, OFFSET>)
    }
}
pub trait IRawElementProviderHwndOverrideImpl: Sized {
    fn GetOverrideProviderForHwnd();
}
impl ::windows::core::RuntimeName for IRawElementProviderHwndOverride {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRawElementProviderHwndOverride";
}
impl IRawElementProviderHwndOverrideVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHwndOverrideImpl, const OFFSET: isize>() -> IRawElementProviderHwndOverrideVtbl {
        unsafe extern "system" fn GetOverrideProviderForHwnd<Impl: IRawElementProviderHwndOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOverrideProviderForHwnd(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawElementProviderHwndOverride>, ::windows::core::GetTrustLevel, GetOverrideProviderForHwnd::<Impl, OFFSET>)
    }
}
pub trait IRawElementProviderSimpleImpl: Sized {
    fn ProviderOptions();
    fn GetPatternProvider();
    fn GetPropertyValue();
    fn HostRawElementProvider();
}
impl ::windows::core::RuntimeName for IRawElementProviderSimple {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRawElementProviderSimple";
}
impl IRawElementProviderSimpleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimpleImpl, const OFFSET: isize>() -> IRawElementProviderSimpleVtbl {
        unsafe extern "system" fn ProviderOptions<Impl: IRawElementProviderSimpleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ProviderOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderOptions(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatternProvider<Impl: IRawElementProviderSimpleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatternProvider(patternid, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyValue<Impl: IRawElementProviderSimpleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyValue(propertyid, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostRawElementProvider<Impl: IRawElementProviderSimpleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostRawElementProvider(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawElementProviderSimple>, ::windows::core::GetTrustLevel, ProviderOptions::<Impl, OFFSET>, GetPatternProvider::<Impl, OFFSET>, GetPropertyValue::<Impl, OFFSET>, HostRawElementProvider::<Impl, OFFSET>)
    }
}
pub trait IRawElementProviderSimple2Impl: Sized + IRawElementProviderSimpleImpl {
    fn ShowContextMenu();
}
impl ::windows::core::RuntimeName for IRawElementProviderSimple2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRawElementProviderSimple2";
}
impl IRawElementProviderSimple2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple2Impl, const OFFSET: isize>() -> IRawElementProviderSimple2Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: IRawElementProviderSimple2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowContextMenu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawElementProviderSimple2>, ::windows::core::GetTrustLevel, ShowContextMenu::<Impl, OFFSET>)
    }
}
pub trait IRawElementProviderSimple3Impl: Sized + IRawElementProviderSimple2Impl + IRawElementProviderSimpleImpl {
    fn GetMetadataValue();
}
impl ::windows::core::RuntimeName for IRawElementProviderSimple3 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRawElementProviderSimple3";
}
impl IRawElementProviderSimple3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple3Impl, const OFFSET: isize>() -> IRawElementProviderSimple3Vtbl {
        unsafe extern "system" fn GetMetadataValue<Impl: IRawElementProviderSimple3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: i32, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataValue(targetid, metadataid, ::core::mem::transmute_copy(&returnval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawElementProviderSimple3>, ::windows::core::GetTrustLevel, GetMetadataValue::<Impl, OFFSET>)
    }
}
pub trait IRawElementProviderWindowlessSiteImpl: Sized {
    fn GetAdjacentFragment();
    fn GetRuntimeIdPrefix();
}
impl ::windows::core::RuntimeName for IRawElementProviderWindowlessSite {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRawElementProviderWindowlessSite";
}
impl IRawElementProviderWindowlessSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderWindowlessSiteImpl, const OFFSET: isize>() -> IRawElementProviderWindowlessSiteVtbl {
        unsafe extern "system" fn GetAdjacentFragment<Impl: IRawElementProviderWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdjacentFragment(direction, ::core::mem::transmute_copy(&ppparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeIdPrefix<Impl: IRawElementProviderWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRuntimeIdPrefix(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRawElementProviderWindowlessSite>, ::windows::core::GetTrustLevel, GetAdjacentFragment::<Impl, OFFSET>, GetRuntimeIdPrefix::<Impl, OFFSET>)
    }
}
pub trait IRichEditUiaInformationImpl: Sized {
    fn GetBoundaryRectangle();
    fn IsVisible();
}
impl ::windows::core::RuntimeName for IRichEditUiaInformation {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRichEditUiaInformation";
}
impl IRichEditUiaInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditUiaInformationImpl, const OFFSET: isize>() -> IRichEditUiaInformationVtbl {
        unsafe extern "system" fn GetBoundaryRectangle<Impl: IRichEditUiaInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiarect: *mut UiaRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundaryRectangle(&*(&puiarect as *const <UiaRect as ::windows::core::Abi>::Abi as *const <UiaRect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisible<Impl: IRichEditUiaInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRichEditUiaInformation>, ::windows::core::GetTrustLevel, GetBoundaryRectangle::<Impl, OFFSET>, IsVisible::<Impl, OFFSET>)
    }
}
pub trait IRicheditWindowlessAccessibilityImpl: Sized {
    fn CreateProvider();
}
impl ::windows::core::RuntimeName for IRicheditWindowlessAccessibility {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IRicheditWindowlessAccessibility";
}
impl IRicheditWindowlessAccessibilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditWindowlessAccessibilityImpl, const OFFSET: isize>() -> IRicheditWindowlessAccessibilityVtbl {
        unsafe extern "system" fn CreateProvider<Impl: IRicheditWindowlessAccessibilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psite: ::windows::core::RawPtr, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProvider(&*(&psite as *const <IRawElementProviderWindowlessSite as ::windows::core::Abi>::Abi as *const <IRawElementProviderWindowlessSite as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRicheditWindowlessAccessibility>, ::windows::core::GetTrustLevel, CreateProvider::<Impl, OFFSET>)
    }
}
pub trait IScrollItemProviderImpl: Sized {
    fn ScrollIntoView();
}
impl ::windows::core::RuntimeName for IScrollItemProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IScrollItemProvider";
}
impl IScrollItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollItemProviderImpl, const OFFSET: isize>() -> IScrollItemProviderVtbl {
        unsafe extern "system" fn ScrollIntoView<Impl: IScrollItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollIntoView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IScrollItemProvider>, ::windows::core::GetTrustLevel, ScrollIntoView::<Impl, OFFSET>)
    }
}
pub trait IScrollProviderImpl: Sized {
    fn Scroll();
    fn SetScrollPercent();
    fn HorizontalScrollPercent();
    fn VerticalScrollPercent();
    fn HorizontalViewSize();
    fn VerticalViewSize();
    fn HorizontallyScrollable();
    fn VerticallyScrollable();
}
impl ::windows::core::RuntimeName for IScrollProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IScrollProvider";
}
impl IScrollProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProviderImpl, const OFFSET: isize>() -> IScrollProviderVtbl {
        unsafe extern "system" fn Scroll<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scroll(horizontalamount, verticalamount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollPercent<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScrollPercent(horizontalpercent, verticalpercent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalScrollPercent<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalScrollPercent(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalScrollPercent<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalScrollPercent(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalViewSize<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalViewSize(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalViewSize<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalViewSize(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontallyScrollable<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontallyScrollable(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticallyScrollable<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticallyScrollable(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IScrollProvider>,
            ::windows::core::GetTrustLevel,
            Scroll::<Impl, OFFSET>,
            SetScrollPercent::<Impl, OFFSET>,
            HorizontalScrollPercent::<Impl, OFFSET>,
            VerticalScrollPercent::<Impl, OFFSET>,
            HorizontalViewSize::<Impl, OFFSET>,
            VerticalViewSize::<Impl, OFFSET>,
            HorizontallyScrollable::<Impl, OFFSET>,
            VerticallyScrollable::<Impl, OFFSET>,
        )
    }
}
pub trait ISelectionItemProviderImpl: Sized {
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn IsSelected();
    fn SelectionContainer();
}
impl ::windows::core::RuntimeName for ISelectionItemProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ISelectionItemProvider";
}
impl ISelectionItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProviderImpl, const OFFSET: isize>() -> ISelectionItemProviderVtbl {
        unsafe extern "system" fn Select<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Select() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSelection<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddToSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveFromSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelected<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionContainer<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionContainer(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectionItemProvider>, ::windows::core::GetTrustLevel, Select::<Impl, OFFSET>, AddToSelection::<Impl, OFFSET>, RemoveFromSelection::<Impl, OFFSET>, IsSelected::<Impl, OFFSET>, SelectionContainer::<Impl, OFFSET>)
    }
}
pub trait ISelectionProviderImpl: Sized {
    fn GetSelection();
    fn CanSelectMultiple();
    fn IsSelectionRequired();
}
impl ::windows::core::RuntimeName for ISelectionProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ISelectionProvider";
}
impl ISelectionProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProviderImpl, const OFFSET: isize>() -> ISelectionProviderVtbl {
        unsafe extern "system" fn GetSelection<Impl: ISelectionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanSelectMultiple<Impl: ISelectionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSelectMultiple(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSelectionRequired<Impl: ISelectionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelectionRequired(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectionProvider>, ::windows::core::GetTrustLevel, GetSelection::<Impl, OFFSET>, CanSelectMultiple::<Impl, OFFSET>, IsSelectionRequired::<Impl, OFFSET>)
    }
}
pub trait ISelectionProvider2Impl: Sized + ISelectionProviderImpl {
    fn FirstSelectedItem();
    fn LastSelectedItem();
    fn CurrentSelectedItem();
    fn ItemCount();
}
impl ::windows::core::RuntimeName for ISelectionProvider2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ISelectionProvider2";
}
impl ISelectionProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider2Impl, const OFFSET: isize>() -> ISelectionProvider2Vtbl {
        unsafe extern "system" fn FirstSelectedItem<Impl: ISelectionProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstSelectedItem(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastSelectedItem<Impl: ISelectionProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastSelectedItem(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSelectedItem<Impl: ISelectionProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSelectedItem(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Impl: ISelectionProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCount(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectionProvider2>, ::windows::core::GetTrustLevel, FirstSelectedItem::<Impl, OFFSET>, LastSelectedItem::<Impl, OFFSET>, CurrentSelectedItem::<Impl, OFFSET>, ItemCount::<Impl, OFFSET>)
    }
}
pub trait ISpreadsheetItemProviderImpl: Sized {
    fn Formula();
    fn GetAnnotationObjects();
    fn GetAnnotationTypes();
}
impl ::windows::core::RuntimeName for ISpreadsheetItemProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ISpreadsheetItemProvider";
}
impl ISpreadsheetItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>() -> ISpreadsheetItemProviderVtbl {
        unsafe extern "system" fn Formula<Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Formula(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationObjects<Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationObjects(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationTypes<Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationTypes(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpreadsheetItemProvider>, ::windows::core::GetTrustLevel, Formula::<Impl, OFFSET>, GetAnnotationObjects::<Impl, OFFSET>, GetAnnotationTypes::<Impl, OFFSET>)
    }
}
pub trait ISpreadsheetProviderImpl: Sized {
    fn GetItemByName();
}
impl ::windows::core::RuntimeName for ISpreadsheetProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ISpreadsheetProvider";
}
impl ISpreadsheetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetProviderImpl, const OFFSET: isize>() -> ISpreadsheetProviderVtbl {
        unsafe extern "system" fn GetItemByName<Impl: ISpreadsheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemByName(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpreadsheetProvider>, ::windows::core::GetTrustLevel, GetItemByName::<Impl, OFFSET>)
    }
}
pub trait IStylesProviderImpl: Sized {
    fn StyleId();
    fn StyleName();
    fn FillColor();
    fn FillPatternStyle();
    fn Shape();
    fn FillPatternColor();
    fn ExtendedProperties();
}
impl ::windows::core::RuntimeName for IStylesProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IStylesProvider";
}
impl IStylesProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProviderImpl, const OFFSET: isize>() -> IStylesProviderVtbl {
        unsafe extern "system" fn StyleId<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleName<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillColor<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillColor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternStyle<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillPatternStyle(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shape<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shape(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FillPatternColor<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillPatternColor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedProperties(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStylesProvider>, ::windows::core::GetTrustLevel, StyleId::<Impl, OFFSET>, StyleName::<Impl, OFFSET>, FillColor::<Impl, OFFSET>, FillPatternStyle::<Impl, OFFSET>, Shape::<Impl, OFFSET>, FillPatternColor::<Impl, OFFSET>, ExtendedProperties::<Impl, OFFSET>)
    }
}
pub trait ISynchronizedInputProviderImpl: Sized {
    fn StartListening();
    fn Cancel();
}
impl ::windows::core::RuntimeName for ISynchronizedInputProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ISynchronizedInputProvider";
}
impl ISynchronizedInputProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>() -> ISynchronizedInputProviderVtbl {
        unsafe extern "system" fn StartListening<Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartListening(inputtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISynchronizedInputProvider>, ::windows::core::GetTrustLevel, StartListening::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait ITableItemProviderImpl: Sized {
    fn GetRowHeaderItems();
    fn GetColumnHeaderItems();
}
impl ::windows::core::RuntimeName for ITableItemProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITableItemProvider";
}
impl ITableItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProviderImpl, const OFFSET: isize>() -> ITableItemProviderVtbl {
        unsafe extern "system" fn GetRowHeaderItems<Impl: ITableItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRowHeaderItems(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaderItems<Impl: ITableItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnHeaderItems(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITableItemProvider>, ::windows::core::GetTrustLevel, GetRowHeaderItems::<Impl, OFFSET>, GetColumnHeaderItems::<Impl, OFFSET>)
    }
}
pub trait ITableProviderImpl: Sized {
    fn GetRowHeaders();
    fn GetColumnHeaders();
    fn RowOrColumnMajor();
}
impl ::windows::core::RuntimeName for ITableProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITableProvider";
}
impl ITableProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableProviderImpl, const OFFSET: isize>() -> ITableProviderVtbl {
        unsafe extern "system" fn GetRowHeaders<Impl: ITableProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRowHeaders(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Impl: ITableProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnHeaders(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RowOrColumnMajor<Impl: ITableProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RowOrColumnMajor(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITableProvider>, ::windows::core::GetTrustLevel, GetRowHeaders::<Impl, OFFSET>, GetColumnHeaders::<Impl, OFFSET>, RowOrColumnMajor::<Impl, OFFSET>)
    }
}
pub trait ITextChildProviderImpl: Sized {
    fn TextContainer();
    fn TextRange();
}
impl ::windows::core::RuntimeName for ITextChildProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITextChildProvider";
}
impl ITextChildProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProviderImpl, const OFFSET: isize>() -> ITextChildProviderVtbl {
        unsafe extern "system" fn TextContainer<Impl: ITextChildProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextContainer(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Impl: ITextChildProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextRange(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextChildProvider>, ::windows::core::GetTrustLevel, TextContainer::<Impl, OFFSET>, TextRange::<Impl, OFFSET>)
    }
}
pub trait ITextEditProviderImpl: Sized + ITextProviderImpl {
    fn GetActiveComposition();
    fn GetConversionTarget();
}
impl ::windows::core::RuntimeName for ITextEditProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITextEditProvider";
}
impl ITextEditProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProviderImpl, const OFFSET: isize>() -> ITextEditProviderVtbl {
        unsafe extern "system" fn GetActiveComposition<Impl: ITextEditProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveComposition(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Impl: ITextEditProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionTarget(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextEditProvider>, ::windows::core::GetTrustLevel, GetActiveComposition::<Impl, OFFSET>, GetConversionTarget::<Impl, OFFSET>)
    }
}
pub trait ITextProviderImpl: Sized {
    fn GetSelection();
    fn GetVisibleRanges();
    fn RangeFromChild();
    fn RangeFromPoint();
    fn DocumentRange();
    fn SupportedTextSelection();
}
impl ::windows::core::RuntimeName for ITextProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITextProvider";
}
impl ITextProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProviderImpl, const OFFSET: isize>() -> ITextProviderVtbl {
        unsafe extern "system" fn GetSelection<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisibleRanges(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromChild<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childelement: ::windows::core::RawPtr, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromChild(&*(&childelement as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromPoint<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: UiaPoint, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromPoint(&*(&point as *const <UiaPoint as ::windows::core::Abi>::Abi as *const <UiaPoint as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentRange<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentRange(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedTextSelection(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextProvider>, ::windows::core::GetTrustLevel, GetSelection::<Impl, OFFSET>, GetVisibleRanges::<Impl, OFFSET>, RangeFromChild::<Impl, OFFSET>, RangeFromPoint::<Impl, OFFSET>, DocumentRange::<Impl, OFFSET>, SupportedTextSelection::<Impl, OFFSET>)
    }
}
pub trait ITextProvider2Impl: Sized + ITextProviderImpl {
    fn RangeFromAnnotation();
    fn GetCaretRange();
}
impl ::windows::core::RuntimeName for ITextProvider2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITextProvider2";
}
impl ITextProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2Impl, const OFFSET: isize>() -> ITextProvider2Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Impl: ITextProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationelement: ::windows::core::RawPtr, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromAnnotation(&*(&annotationelement as *const <IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Impl: ITextProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaretRange(::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextProvider2>, ::windows::core::GetTrustLevel, RangeFromAnnotation::<Impl, OFFSET>, GetCaretRange::<Impl, OFFSET>)
    }
}
pub trait ITextRangeProviderImpl: Sized {
    fn Clone();
    fn Compare();
    fn CompareEndpoints();
    fn ExpandToEnclosingUnit();
    fn FindAttribute();
    fn FindText();
    fn GetAttributeValue();
    fn GetBoundingRectangles();
    fn GetEnclosingElement();
    fn GetText();
    fn Move();
    fn MoveEndpointByUnit();
    fn MoveEndpointByRange();
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn ScrollIntoView();
    fn GetChildren();
}
impl ::windows::core::RuntimeName for ITextRangeProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITextRangeProvider";
}
impl ITextRangeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProviderImpl, const OFFSET: isize>() -> ITextRangeProviderVtbl {
        unsafe extern "system" fn Clone<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&range as *const <ITextRangeProvider as ::windows::core::Abi>::Abi as *const <ITextRangeProvider as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEndpoints(endpoint, &*(&targetrange as *const <ITextRangeProvider as ::windows::core::Abi>::Abi as *const <ITextRangeProvider as ::windows::core::DefaultType>::DefaultType), targetendpoint, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandToEnclosingUnit(unit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAttribute<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, val: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, backward: super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAttribute(attributeid, &*(&val as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&backward as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindText(
                &*(&text as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&backward as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&ignorecase as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pretval),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeValue(attributeid, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingRectangles(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnclosingElement(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(maxlength, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(unit, count, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndpointByUnit(endpoint, unit, count, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndpointByRange(endpoint, &*(&targetrange as *const <ITextRangeProvider as ::windows::core::Abi>::Abi as *const <ITextRangeProvider as ::windows::core::DefaultType>::DefaultType), targetendpoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Select<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Select() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSelection<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddToSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveFromSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollIntoView<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollIntoView(&*(&aligntotop as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITextRangeProvider>,
            ::windows::core::GetTrustLevel,
            Clone::<Impl, OFFSET>,
            Compare::<Impl, OFFSET>,
            CompareEndpoints::<Impl, OFFSET>,
            ExpandToEnclosingUnit::<Impl, OFFSET>,
            FindAttribute::<Impl, OFFSET>,
            FindText::<Impl, OFFSET>,
            GetAttributeValue::<Impl, OFFSET>,
            GetBoundingRectangles::<Impl, OFFSET>,
            GetEnclosingElement::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            MoveEndpointByUnit::<Impl, OFFSET>,
            MoveEndpointByRange::<Impl, OFFSET>,
            Select::<Impl, OFFSET>,
            AddToSelection::<Impl, OFFSET>,
            RemoveFromSelection::<Impl, OFFSET>,
            ScrollIntoView::<Impl, OFFSET>,
            GetChildren::<Impl, OFFSET>,
        )
    }
}
pub trait ITextRangeProvider2Impl: Sized + ITextRangeProviderImpl {
    fn ShowContextMenu();
}
impl ::windows::core::RuntimeName for ITextRangeProvider2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITextRangeProvider2";
}
impl ITextRangeProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider2Impl, const OFFSET: isize>() -> ITextRangeProvider2Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: ITextRangeProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowContextMenu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextRangeProvider2>, ::windows::core::GetTrustLevel, ShowContextMenu::<Impl, OFFSET>)
    }
}
pub trait IToggleProviderImpl: Sized {
    fn Toggle();
    fn ToggleState();
}
impl ::windows::core::RuntimeName for IToggleProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IToggleProvider";
}
impl IToggleProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProviderImpl, const OFFSET: isize>() -> IToggleProviderVtbl {
        unsafe extern "system" fn Toggle<Impl: IToggleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Toggle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ToggleState<Impl: IToggleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToggleState(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToggleProvider>, ::windows::core::GetTrustLevel, Toggle::<Impl, OFFSET>, ToggleState::<Impl, OFFSET>)
    }
}
pub trait ITransformProviderImpl: Sized {
    fn Move();
    fn Resize();
    fn Rotate();
    fn CanMove();
    fn CanResize();
    fn CanRotate();
}
impl ::windows::core::RuntimeName for ITransformProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITransformProvider";
}
impl ITransformProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProviderImpl, const OFFSET: isize>() -> ITransformProviderVtbl {
        unsafe extern "system" fn Move<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resize<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resize(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rotate<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotate(degrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMove<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMove(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanResize<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanResize(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRotate<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanRotate(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransformProvider>, ::windows::core::GetTrustLevel, Move::<Impl, OFFSET>, Resize::<Impl, OFFSET>, Rotate::<Impl, OFFSET>, CanMove::<Impl, OFFSET>, CanResize::<Impl, OFFSET>, CanRotate::<Impl, OFFSET>)
    }
}
pub trait ITransformProvider2Impl: Sized + ITransformProviderImpl {
    fn Zoom();
    fn CanZoom();
    fn ZoomLevel();
    fn ZoomMinimum();
    fn ZoomMaximum();
    fn ZoomByUnit();
}
impl ::windows::core::RuntimeName for ITransformProvider2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.ITransformProvider2";
}
impl ITransformProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2Impl, const OFFSET: isize>() -> ITransformProvider2Vtbl {
        unsafe extern "system" fn Zoom<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Zoom(zoom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanZoom<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanZoom(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomLevel<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomLevel(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomMinimum<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomMinimum(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomMaximum<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomMaximum(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomByUnit<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomByUnit(zoomunit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransformProvider2>, ::windows::core::GetTrustLevel, Zoom::<Impl, OFFSET>, CanZoom::<Impl, OFFSET>, ZoomLevel::<Impl, OFFSET>, ZoomMinimum::<Impl, OFFSET>, ZoomMaximum::<Impl, OFFSET>, ZoomByUnit::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationImpl: Sized {
    fn CompareElements();
    fn CompareRuntimeIds();
    fn GetRootElement();
    fn ElementFromHandle();
    fn ElementFromPoint();
    fn GetFocusedElement();
    fn GetRootElementBuildCache();
    fn ElementFromHandleBuildCache();
    fn ElementFromPointBuildCache();
    fn GetFocusedElementBuildCache();
    fn CreateTreeWalker();
    fn ControlViewWalker();
    fn ContentViewWalker();
    fn RawViewWalker();
    fn RawViewCondition();
    fn ControlViewCondition();
    fn ContentViewCondition();
    fn CreateCacheRequest();
    fn CreateTrueCondition();
    fn CreateFalseCondition();
    fn CreatePropertyCondition();
    fn CreatePropertyConditionEx();
    fn CreateAndCondition();
    fn CreateAndConditionFromArray();
    fn CreateAndConditionFromNativeArray();
    fn CreateOrCondition();
    fn CreateOrConditionFromArray();
    fn CreateOrConditionFromNativeArray();
    fn CreateNotCondition();
    fn AddAutomationEventHandler();
    fn RemoveAutomationEventHandler();
    fn AddPropertyChangedEventHandlerNativeArray();
    fn AddPropertyChangedEventHandler();
    fn RemovePropertyChangedEventHandler();
    fn AddStructureChangedEventHandler();
    fn RemoveStructureChangedEventHandler();
    fn AddFocusChangedEventHandler();
    fn RemoveFocusChangedEventHandler();
    fn RemoveAllEventHandlers();
    fn IntNativeArrayToSafeArray();
    fn IntSafeArrayToNativeArray();
    fn RectToVariant();
    fn VariantToRect();
    fn SafeArrayToRectNativeArray();
    fn CreateProxyFactoryEntry();
    fn ProxyFactoryMapping();
    fn GetPropertyProgrammaticName();
    fn GetPatternProgrammaticName();
    fn PollForPotentialSupportedPatterns();
    fn PollForPotentialSupportedProperties();
    fn CheckNotSupported();
    fn ReservedNotSupportedValue();
    fn ReservedMixedAttributeValue();
    fn ElementFromIAccessible();
    fn ElementFromIAccessibleBuildCache();
}
impl ::windows::core::RuntimeName for IUIAutomation {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomation";
}
impl IUIAutomationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationImpl, const OFFSET: isize>() -> IUIAutomationVtbl {
        unsafe extern "system" fn CompareElements<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, el1: ::windows::core::RawPtr, el2: ::windows::core::RawPtr, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareElements(&*(&el1 as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&el2 as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&aresame)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareRuntimeIds<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareRuntimeIds(&*(&runtimeid1 as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), &*(&runtimeid2 as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&aresame)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootElement<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootElement(::core::mem::transmute_copy(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromHandle<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromHandle(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromPoint<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromPoint(&*(&pt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElement<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocusedElement(::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRootElementBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootElementBuildCache(&*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromHandleBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromHandleBuildCache(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromPointBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromPointBuildCache(&*(&pt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElementBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocusedElementBuildCache(&*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTreeWalker<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcondition: ::windows::core::RawPtr, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTreeWalker(&*(&pcondition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&walker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlViewWalker<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlViewWalker(::core::mem::transmute_copy(&walker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentViewWalker<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentViewWalker(::core::mem::transmute_copy(&walker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawViewWalker<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawViewWalker(::core::mem::transmute_copy(&walker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawViewCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawViewCondition(::core::mem::transmute_copy(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControlViewCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlViewCondition(::core::mem::transmute_copy(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentViewCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentViewCondition(::core::mem::transmute_copy(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCacheRequest<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCacheRequest(::core::mem::transmute_copy(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrueCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTrueCondition(::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFalseCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFalseCondition(::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyCondition(propertyid, &*(&value as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePropertyConditionEx<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, flags: PropertyConditionFlags, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyConditionEx(propertyid, &*(&value as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: ::windows::core::RawPtr, condition2: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAndCondition(&*(&condition1 as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), &*(&condition2 as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndConditionFromArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAndConditionFromArray(&*(&conditions as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAndConditionFromNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const ::windows::core::RawPtr, conditioncount: i32, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAndConditionFromNativeArray(&*(&conditions as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), conditioncount, ::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: ::windows::core::RawPtr, condition2: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOrCondition(&*(&condition1 as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), &*(&condition2 as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrConditionFromArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOrConditionFromArray(&*(&conditions as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOrConditionFromNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const ::windows::core::RawPtr, conditioncount: i32, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOrConditionFromNativeArray(&*(&conditions as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), conditioncount, ::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNotCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNotCondition(&*(&condition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAutomationEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAutomationEventHandler(
                eventid,
                &*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                scope,
                &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <IUIAutomationEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationEventHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutomationEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAutomationEventHandler(eventid, &*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandlerNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const i32, propertycount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyChangedEventHandlerNativeArray(
                &*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                scope,
                &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <IUIAutomationPropertyChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationPropertyChangedEventHandler as ::windows::core::DefaultType>::DefaultType),
                propertyarray,
                propertycount,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyChangedEventHandler(
                &*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                scope,
                &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <IUIAutomationPropertyChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationPropertyChangedEventHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&propertyarray as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePropertyChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePropertyChangedEventHandler(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationPropertyChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationPropertyChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStructureChangedEventHandler(
                &*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                scope,
                &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <IUIAutomationStructureChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationStructureChangedEventHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStructureChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveStructureChangedEventHandler(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationStructureChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationStructureChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFocusChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFocusChangedEventHandler(&*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationFocusChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationFocusChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFocusChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveFocusChangedEventHandler(&*(&handler as *const <IUIAutomationFocusChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationFocusChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllEventHandlers<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAllEventHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntNativeArrayToSafeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, array: *const i32, arraycount: i32, safearray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntNativeArrayToSafeArray(array, arraycount, ::core::mem::transmute_copy(&safearray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntSafeArrayToNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntSafeArrayToNativeArray(&*(&intarray as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&array), ::core::mem::transmute_copy(&arraycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RectToVariant<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, var: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RectToVariant(&*(&rc as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&var)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VariantToRect<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, rc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VariantToRect(&*(&var as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SafeArrayToRectNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SafeArrayToRectNativeArray(&*(&rects as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&rectarray), ::core::mem::transmute_copy(&rectarraycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProxyFactoryEntry<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, factoryentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProxyFactoryEntry(&*(&factory as *const <IUIAutomationProxyFactory as ::windows::core::Abi>::Abi as *const <IUIAutomationProxyFactory as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&factoryentry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyFactoryMapping<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorymapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyFactoryMapping(::core::mem::transmute_copy(&factorymapping)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyProgrammaticName<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyProgrammaticName(property, ::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPatternProgrammaticName<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatternProgrammaticName(pattern, ::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedPatterns<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PollForPotentialSupportedPatterns(&*(&pelement as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&patternids), ::core::mem::transmute_copy(&patternnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedProperties<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PollForPotentialSupportedProperties(&*(&pelement as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&propertyids), ::core::mem::transmute_copy(&propertynames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckNotSupported<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, isnotsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckNotSupported(&*(&value as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&isnotsupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReservedNotSupportedValue<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notsupportedvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReservedNotSupportedValue(::core::mem::transmute_copy(&notsupportedvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReservedMixedAttributeValue<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mixedattributevalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReservedMixedAttributeValue(::core::mem::transmute_copy(&mixedattributevalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromIAccessible<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: ::windows::core::RawPtr, childid: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromIAccessible(&*(&accessible as *const <IAccessible as ::windows::core::Abi>::Abi as *const <IAccessible as ::windows::core::DefaultType>::DefaultType), childid, ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementFromIAccessibleBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: ::windows::core::RawPtr, childid: i32, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementFromIAccessibleBuildCache(&*(&accessible as *const <IAccessible as ::windows::core::Abi>::Abi as *const <IAccessible as ::windows::core::DefaultType>::DefaultType), childid, &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomation>,
            ::windows::core::GetTrustLevel,
            CompareElements::<Impl, OFFSET>,
            CompareRuntimeIds::<Impl, OFFSET>,
            GetRootElement::<Impl, OFFSET>,
            ElementFromHandle::<Impl, OFFSET>,
            ElementFromPoint::<Impl, OFFSET>,
            GetFocusedElement::<Impl, OFFSET>,
            GetRootElementBuildCache::<Impl, OFFSET>,
            ElementFromHandleBuildCache::<Impl, OFFSET>,
            ElementFromPointBuildCache::<Impl, OFFSET>,
            GetFocusedElementBuildCache::<Impl, OFFSET>,
            CreateTreeWalker::<Impl, OFFSET>,
            ControlViewWalker::<Impl, OFFSET>,
            ContentViewWalker::<Impl, OFFSET>,
            RawViewWalker::<Impl, OFFSET>,
            RawViewCondition::<Impl, OFFSET>,
            ControlViewCondition::<Impl, OFFSET>,
            ContentViewCondition::<Impl, OFFSET>,
            CreateCacheRequest::<Impl, OFFSET>,
            CreateTrueCondition::<Impl, OFFSET>,
            CreateFalseCondition::<Impl, OFFSET>,
            CreatePropertyCondition::<Impl, OFFSET>,
            CreatePropertyConditionEx::<Impl, OFFSET>,
            CreateAndCondition::<Impl, OFFSET>,
            CreateAndConditionFromArray::<Impl, OFFSET>,
            CreateAndConditionFromNativeArray::<Impl, OFFSET>,
            CreateOrCondition::<Impl, OFFSET>,
            CreateOrConditionFromArray::<Impl, OFFSET>,
            CreateOrConditionFromNativeArray::<Impl, OFFSET>,
            CreateNotCondition::<Impl, OFFSET>,
            AddAutomationEventHandler::<Impl, OFFSET>,
            RemoveAutomationEventHandler::<Impl, OFFSET>,
            AddPropertyChangedEventHandlerNativeArray::<Impl, OFFSET>,
            AddPropertyChangedEventHandler::<Impl, OFFSET>,
            RemovePropertyChangedEventHandler::<Impl, OFFSET>,
            AddStructureChangedEventHandler::<Impl, OFFSET>,
            RemoveStructureChangedEventHandler::<Impl, OFFSET>,
            AddFocusChangedEventHandler::<Impl, OFFSET>,
            RemoveFocusChangedEventHandler::<Impl, OFFSET>,
            RemoveAllEventHandlers::<Impl, OFFSET>,
            IntNativeArrayToSafeArray::<Impl, OFFSET>,
            IntSafeArrayToNativeArray::<Impl, OFFSET>,
            RectToVariant::<Impl, OFFSET>,
            VariantToRect::<Impl, OFFSET>,
            SafeArrayToRectNativeArray::<Impl, OFFSET>,
            CreateProxyFactoryEntry::<Impl, OFFSET>,
            ProxyFactoryMapping::<Impl, OFFSET>,
            GetPropertyProgrammaticName::<Impl, OFFSET>,
            GetPatternProgrammaticName::<Impl, OFFSET>,
            PollForPotentialSupportedPatterns::<Impl, OFFSET>,
            PollForPotentialSupportedProperties::<Impl, OFFSET>,
            CheckNotSupported::<Impl, OFFSET>,
            ReservedNotSupportedValue::<Impl, OFFSET>,
            ReservedMixedAttributeValue::<Impl, OFFSET>,
            ElementFromIAccessible::<Impl, OFFSET>,
            ElementFromIAccessibleBuildCache::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomation2Impl: Sized + IUIAutomationImpl {
    fn AutoSetFocus();
    fn SetAutoSetFocus();
    fn ConnectionTimeout();
    fn SetConnectionTimeout();
    fn TransactionTimeout();
    fn SetTransactionTimeout();
}
impl ::windows::core::RuntimeName for IUIAutomation2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomation2";
}
impl IUIAutomation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2Impl, const OFFSET: isize>() -> IUIAutomation2Vtbl {
        unsafe extern "system" fn AutoSetFocus<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoSetFocus(::core::mem::transmute_copy(&autosetfocus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoSetFocus<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoSetFocus(&*(&autosetfocus as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionTimeout<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionTimeout(::core::mem::transmute_copy(&timeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionTimeout<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnectionTimeout(timeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionTimeout<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionTimeout(::core::mem::transmute_copy(&timeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransactionTimeout<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransactionTimeout(timeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomation2>, ::windows::core::GetTrustLevel, AutoSetFocus::<Impl, OFFSET>, SetAutoSetFocus::<Impl, OFFSET>, ConnectionTimeout::<Impl, OFFSET>, SetConnectionTimeout::<Impl, OFFSET>, TransactionTimeout::<Impl, OFFSET>, SetTransactionTimeout::<Impl, OFFSET>)
    }
}
pub trait IUIAutomation3Impl: Sized + IUIAutomation2Impl + IUIAutomationImpl {
    fn AddTextEditTextChangedEventHandler();
    fn RemoveTextEditTextChangedEventHandler();
}
impl ::windows::core::RuntimeName for IUIAutomation3 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomation3";
}
impl IUIAutomation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation3Impl, const OFFSET: isize>() -> IUIAutomation3Vtbl {
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Impl: IUIAutomation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTextEditTextChangedEventHandler(
                &*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                scope,
                texteditchangetype,
                &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <IUIAutomationTextEditTextChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationTextEditTextChangedEventHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTextEditTextChangedEventHandler<Impl: IUIAutomation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveTextEditTextChangedEventHandler(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationTextEditTextChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationTextEditTextChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomation3>, ::windows::core::GetTrustLevel, AddTextEditTextChangedEventHandler::<Impl, OFFSET>, RemoveTextEditTextChangedEventHandler::<Impl, OFFSET>)
    }
}
pub trait IUIAutomation4Impl: Sized + IUIAutomation3Impl + IUIAutomation2Impl + IUIAutomationImpl {
    fn AddChangesEventHandler();
    fn RemoveChangesEventHandler();
}
impl ::windows::core::RuntimeName for IUIAutomation4 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomation4";
}
impl IUIAutomation4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation4Impl, const OFFSET: isize>() -> IUIAutomation4Vtbl {
        unsafe extern "system" fn AddChangesEventHandler<Impl: IUIAutomation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddChangesEventHandler(
                &*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                scope,
                changetypes,
                changescount,
                &*(&pcacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <IUIAutomationChangesEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationChangesEventHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChangesEventHandler<Impl: IUIAutomation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveChangesEventHandler(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationChangesEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationChangesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomation4>, ::windows::core::GetTrustLevel, AddChangesEventHandler::<Impl, OFFSET>, RemoveChangesEventHandler::<Impl, OFFSET>)
    }
}
pub trait IUIAutomation5Impl: Sized + IUIAutomation4Impl + IUIAutomation3Impl + IUIAutomation2Impl + IUIAutomationImpl {
    fn AddNotificationEventHandler();
    fn RemoveNotificationEventHandler();
}
impl ::windows::core::RuntimeName for IUIAutomation5 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomation5";
}
impl IUIAutomation5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation5Impl, const OFFSET: isize>() -> IUIAutomation5Vtbl {
        unsafe extern "system" fn AddNotificationEventHandler<Impl: IUIAutomation5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNotificationEventHandler(
                &*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                scope,
                &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <IUIAutomationNotificationEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationNotificationEventHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNotificationEventHandler<Impl: IUIAutomation5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveNotificationEventHandler(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationNotificationEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationNotificationEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomation5>, ::windows::core::GetTrustLevel, AddNotificationEventHandler::<Impl, OFFSET>, RemoveNotificationEventHandler::<Impl, OFFSET>)
    }
}
pub trait IUIAutomation6Impl: Sized + IUIAutomation5Impl + IUIAutomation4Impl + IUIAutomation3Impl + IUIAutomation2Impl + IUIAutomationImpl {
    fn CreateEventHandlerGroup();
    fn AddEventHandlerGroup();
    fn RemoveEventHandlerGroup();
    fn ConnectionRecoveryBehavior();
    fn SetConnectionRecoveryBehavior();
    fn CoalesceEvents();
    fn SetCoalesceEvents();
    fn AddActiveTextPositionChangedEventHandler();
    fn RemoveActiveTextPositionChangedEventHandler();
}
impl ::windows::core::RuntimeName for IUIAutomation6 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomation6";
}
impl IUIAutomation6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6Impl, const OFFSET: isize>() -> IUIAutomation6Vtbl {
        unsafe extern "system" fn CreateEventHandlerGroup<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handlergroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEventHandlerGroup(::core::mem::transmute_copy(&handlergroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEventHandlerGroup<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handlergroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddEventHandlerGroup(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&handlergroup as *const <IUIAutomationEventHandlerGroup as ::windows::core::Abi>::Abi as *const <IUIAutomationEventHandlerGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventHandlerGroup<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handlergroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveEventHandlerGroup(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&handlergroup as *const <IUIAutomationEventHandlerGroup as ::windows::core::Abi>::Abi as *const <IUIAutomationEventHandlerGroup as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionRecoveryBehavior<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: *mut ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionRecoveryBehavior(::core::mem::transmute_copy(&connectionrecoverybehavioroptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionRecoveryBehavior<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConnectionRecoveryBehavior(connectionrecoverybehavioroptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoalesceEvents<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: *mut CoalesceEventsOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoalesceEvents(::core::mem::transmute_copy(&coalesceeventsoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCoalesceEvents<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCoalesceEvents(coalesceeventsoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddActiveTextPositionChangedEventHandler(
                &*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                scope,
                &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <IUIAutomationActiveTextPositionChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationActiveTextPositionChangedEventHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActiveTextPositionChangedEventHandler<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveActiveTextPositionChangedEventHandler(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationActiveTextPositionChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationActiveTextPositionChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomation6>,
            ::windows::core::GetTrustLevel,
            CreateEventHandlerGroup::<Impl, OFFSET>,
            AddEventHandlerGroup::<Impl, OFFSET>,
            RemoveEventHandlerGroup::<Impl, OFFSET>,
            ConnectionRecoveryBehavior::<Impl, OFFSET>,
            SetConnectionRecoveryBehavior::<Impl, OFFSET>,
            CoalesceEvents::<Impl, OFFSET>,
            SetCoalesceEvents::<Impl, OFFSET>,
            AddActiveTextPositionChangedEventHandler::<Impl, OFFSET>,
            RemoveActiveTextPositionChangedEventHandler::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationActiveTextPositionChangedEventHandlerImpl: Sized {
    fn HandleActiveTextPositionChangedEvent();
}
impl ::windows::core::RuntimeName for IUIAutomationActiveTextPositionChangedEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationActiveTextPositionChangedEventHandler";
}
impl IUIAutomationActiveTextPositionChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationActiveTextPositionChangedEventHandlerImpl, const OFFSET: isize>() -> IUIAutomationActiveTextPositionChangedEventHandlerVtbl {
        unsafe extern "system" fn HandleActiveTextPositionChangedEvent<Impl: IUIAutomationActiveTextPositionChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, range: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandleActiveTextPositionChangedEvent(&*(&sender as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&range as *const <IUIAutomationTextRange as ::windows::core::Abi>::Abi as *const <IUIAutomationTextRange as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationActiveTextPositionChangedEventHandler>, ::windows::core::GetTrustLevel, HandleActiveTextPositionChangedEvent::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationAndConditionImpl: Sized + IUIAutomationConditionImpl {
    fn ChildCount();
    fn GetChildrenAsNativeArray();
    fn GetChildren();
}
impl ::windows::core::RuntimeName for IUIAutomationAndCondition {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationAndCondition";
}
impl IUIAutomationAndConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAndConditionImpl, const OFFSET: isize>() -> IUIAutomationAndConditionVtbl {
        unsafe extern "system" fn ChildCount<Impl: IUIAutomationAndConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildCount(::core::mem::transmute_copy(&childcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Impl: IUIAutomationAndConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::windows::core::RawPtr, childarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildrenAsNativeArray(::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Impl: IUIAutomationAndConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren(::core::mem::transmute_copy(&childarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationAndCondition>, ::windows::core::GetTrustLevel, ChildCount::<Impl, OFFSET>, GetChildrenAsNativeArray::<Impl, OFFSET>, GetChildren::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationAnnotationPatternImpl: Sized {
    fn CurrentAnnotationTypeId();
    fn CurrentAnnotationTypeName();
    fn CurrentAuthor();
    fn CurrentDateTime();
    fn CurrentTarget();
    fn CachedAnnotationTypeId();
    fn CachedAnnotationTypeName();
    fn CachedAuthor();
    fn CachedDateTime();
    fn CachedTarget();
}
impl ::windows::core::RuntimeName for IUIAutomationAnnotationPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationAnnotationPattern";
}
impl IUIAutomationAnnotationPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>() -> IUIAutomationAnnotationPatternVtbl {
        unsafe extern "system" fn CurrentAnnotationTypeId<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnnotationTypeId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypeName<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnnotationTypeName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAuthor<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAuthor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDateTime<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDateTime(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentTarget<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentTarget(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeId<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAnnotationTypeId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeName<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAnnotationTypeName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAuthor<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAuthor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDateTime<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDateTime(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedTarget<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedTarget(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationAnnotationPattern>,
            ::windows::core::GetTrustLevel,
            CurrentAnnotationTypeId::<Impl, OFFSET>,
            CurrentAnnotationTypeName::<Impl, OFFSET>,
            CurrentAuthor::<Impl, OFFSET>,
            CurrentDateTime::<Impl, OFFSET>,
            CurrentTarget::<Impl, OFFSET>,
            CachedAnnotationTypeId::<Impl, OFFSET>,
            CachedAnnotationTypeName::<Impl, OFFSET>,
            CachedAuthor::<Impl, OFFSET>,
            CachedDateTime::<Impl, OFFSET>,
            CachedTarget::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationBoolConditionImpl: Sized + IUIAutomationConditionImpl {
    fn BooleanValue();
}
impl ::windows::core::RuntimeName for IUIAutomationBoolCondition {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationBoolCondition";
}
impl IUIAutomationBoolConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationBoolConditionImpl, const OFFSET: isize>() -> IUIAutomationBoolConditionVtbl {
        unsafe extern "system" fn BooleanValue<Impl: IUIAutomationBoolConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boolval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BooleanValue(::core::mem::transmute_copy(&boolval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationBoolCondition>, ::windows::core::GetTrustLevel, BooleanValue::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationCacheRequestImpl: Sized {
    fn AddProperty();
    fn AddPattern();
    fn Clone();
    fn TreeScope();
    fn SetTreeScope();
    fn TreeFilter();
    fn SetTreeFilter();
    fn AutomationElementMode();
    fn SetAutomationElementMode();
}
impl ::windows::core::RuntimeName for IUIAutomationCacheRequest {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationCacheRequest";
}
impl IUIAutomationCacheRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>() -> IUIAutomationCacheRequestVtbl {
        unsafe extern "system" fn AddProperty<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddProperty(propertyid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPattern<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPattern(patternid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&clonedrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TreeScope<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut TreeScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TreeScope(::core::mem::transmute_copy(&scope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreeScope<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTreeScope(scope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TreeFilter<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TreeFilter(::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreeFilter<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTreeFilter(&*(&filter as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutomationElementMode<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut AutomationElementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationElementMode(::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomationElementMode<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: AutomationElementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutomationElementMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationCacheRequest>,
            ::windows::core::GetTrustLevel,
            AddProperty::<Impl, OFFSET>,
            AddPattern::<Impl, OFFSET>,
            Clone::<Impl, OFFSET>,
            TreeScope::<Impl, OFFSET>,
            SetTreeScope::<Impl, OFFSET>,
            TreeFilter::<Impl, OFFSET>,
            SetTreeFilter::<Impl, OFFSET>,
            AutomationElementMode::<Impl, OFFSET>,
            SetAutomationElementMode::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationChangesEventHandlerImpl: Sized {
    fn HandleChangesEvent();
}
impl ::windows::core::RuntimeName for IUIAutomationChangesEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationChangesEventHandler";
}
impl IUIAutomationChangesEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationChangesEventHandlerImpl, const OFFSET: isize>() -> IUIAutomationChangesEventHandlerVtbl {
        unsafe extern "system" fn HandleChangesEvent<Impl: IUIAutomationChangesEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandleChangesEvent(&*(&sender as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&uiachanges as *const <UiaChangeInfo as ::windows::core::Abi>::Abi as *const <UiaChangeInfo as ::windows::core::DefaultType>::DefaultType), changescount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationChangesEventHandler>, ::windows::core::GetTrustLevel, HandleChangesEvent::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationConditionImpl: Sized {}
impl ::windows::core::RuntimeName for IUIAutomationCondition {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationCondition";
}
impl IUIAutomationConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationConditionImpl, const OFFSET: isize>() -> IUIAutomationConditionVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationCondition>, ::windows::core::GetTrustLevel)
    }
}
pub trait IUIAutomationCustomNavigationPatternImpl: Sized {
    fn Navigate();
}
impl ::windows::core::RuntimeName for IUIAutomationCustomNavigationPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationCustomNavigationPattern";
}
impl IUIAutomationCustomNavigationPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCustomNavigationPatternImpl, const OFFSET: isize>() -> IUIAutomationCustomNavigationPatternVtbl {
        unsafe extern "system" fn Navigate<Impl: IUIAutomationCustomNavigationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigate(direction, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationCustomNavigationPattern>, ::windows::core::GetTrustLevel, Navigate::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationDockPatternImpl: Sized {
    fn SetDockPosition();
    fn CurrentDockPosition();
    fn CachedDockPosition();
}
impl ::windows::core::RuntimeName for IUIAutomationDockPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationDockPattern";
}
impl IUIAutomationDockPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDockPatternImpl, const OFFSET: isize>() -> IUIAutomationDockPatternVtbl {
        unsafe extern "system" fn SetDockPosition<Impl: IUIAutomationDockPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockpos: DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDockPosition(dockpos) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDockPosition<Impl: IUIAutomationDockPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDockPosition(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDockPosition<Impl: IUIAutomationDockPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDockPosition(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationDockPattern>, ::windows::core::GetTrustLevel, SetDockPosition::<Impl, OFFSET>, CurrentDockPosition::<Impl, OFFSET>, CachedDockPosition::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationDragPatternImpl: Sized {
    fn CurrentIsGrabbed();
    fn CachedIsGrabbed();
    fn CurrentDropEffect();
    fn CachedDropEffect();
    fn CurrentDropEffects();
    fn CachedDropEffects();
    fn GetCurrentGrabbedItems();
    fn GetCachedGrabbedItems();
}
impl ::windows::core::RuntimeName for IUIAutomationDragPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationDragPattern";
}
impl IUIAutomationDragPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>() -> IUIAutomationDragPatternVtbl {
        unsafe extern "system" fn CurrentIsGrabbed<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsGrabbed(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsGrabbed<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsGrabbed(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropEffect<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDropEffect(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropEffect<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDropEffect(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropEffects<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDropEffects(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropEffects<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDropEffects(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentGrabbedItems<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentGrabbedItems(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedGrabbedItems<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedGrabbedItems(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationDragPattern>,
            ::windows::core::GetTrustLevel,
            CurrentIsGrabbed::<Impl, OFFSET>,
            CachedIsGrabbed::<Impl, OFFSET>,
            CurrentDropEffect::<Impl, OFFSET>,
            CachedDropEffect::<Impl, OFFSET>,
            CurrentDropEffects::<Impl, OFFSET>,
            CachedDropEffects::<Impl, OFFSET>,
            GetCurrentGrabbedItems::<Impl, OFFSET>,
            GetCachedGrabbedItems::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationDropTargetPatternImpl: Sized {
    fn CurrentDropTargetEffect();
    fn CachedDropTargetEffect();
    fn CurrentDropTargetEffects();
    fn CachedDropTargetEffects();
}
impl ::windows::core::RuntimeName for IUIAutomationDropTargetPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationDropTargetPattern";
}
impl IUIAutomationDropTargetPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDropTargetPatternImpl, const OFFSET: isize>() -> IUIAutomationDropTargetPatternVtbl {
        unsafe extern "system" fn CurrentDropTargetEffect<Impl: IUIAutomationDropTargetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDropTargetEffect(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropTargetEffect<Impl: IUIAutomationDropTargetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDropTargetEffect(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDropTargetEffects<Impl: IUIAutomationDropTargetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDropTargetEffects(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDropTargetEffects<Impl: IUIAutomationDropTargetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDropTargetEffects(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationDropTargetPattern>, ::windows::core::GetTrustLevel, CurrentDropTargetEffect::<Impl, OFFSET>, CachedDropTargetEffect::<Impl, OFFSET>, CurrentDropTargetEffects::<Impl, OFFSET>, CachedDropTargetEffects::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationElementImpl: Sized {
    fn SetFocus();
    fn GetRuntimeId();
    fn FindFirst();
    fn FindAll();
    fn FindFirstBuildCache();
    fn FindAllBuildCache();
    fn BuildUpdatedCache();
    fn GetCurrentPropertyValue();
    fn GetCurrentPropertyValueEx();
    fn GetCachedPropertyValue();
    fn GetCachedPropertyValueEx();
    fn GetCurrentPatternAs();
    fn GetCachedPatternAs();
    fn GetCurrentPattern();
    fn GetCachedPattern();
    fn GetCachedParent();
    fn GetCachedChildren();
    fn CurrentProcessId();
    fn CurrentControlType();
    fn CurrentLocalizedControlType();
    fn CurrentName();
    fn CurrentAcceleratorKey();
    fn CurrentAccessKey();
    fn CurrentHasKeyboardFocus();
    fn CurrentIsKeyboardFocusable();
    fn CurrentIsEnabled();
    fn CurrentAutomationId();
    fn CurrentClassName();
    fn CurrentHelpText();
    fn CurrentCulture();
    fn CurrentIsControlElement();
    fn CurrentIsContentElement();
    fn CurrentIsPassword();
    fn CurrentNativeWindowHandle();
    fn CurrentItemType();
    fn CurrentIsOffscreen();
    fn CurrentOrientation();
    fn CurrentFrameworkId();
    fn CurrentIsRequiredForForm();
    fn CurrentItemStatus();
    fn CurrentBoundingRectangle();
    fn CurrentLabeledBy();
    fn CurrentAriaRole();
    fn CurrentAriaProperties();
    fn CurrentIsDataValidForForm();
    fn CurrentControllerFor();
    fn CurrentDescribedBy();
    fn CurrentFlowsTo();
    fn CurrentProviderDescription();
    fn CachedProcessId();
    fn CachedControlType();
    fn CachedLocalizedControlType();
    fn CachedName();
    fn CachedAcceleratorKey();
    fn CachedAccessKey();
    fn CachedHasKeyboardFocus();
    fn CachedIsKeyboardFocusable();
    fn CachedIsEnabled();
    fn CachedAutomationId();
    fn CachedClassName();
    fn CachedHelpText();
    fn CachedCulture();
    fn CachedIsControlElement();
    fn CachedIsContentElement();
    fn CachedIsPassword();
    fn CachedNativeWindowHandle();
    fn CachedItemType();
    fn CachedIsOffscreen();
    fn CachedOrientation();
    fn CachedFrameworkId();
    fn CachedIsRequiredForForm();
    fn CachedItemStatus();
    fn CachedBoundingRectangle();
    fn CachedLabeledBy();
    fn CachedAriaRole();
    fn CachedAriaProperties();
    fn CachedIsDataValidForForm();
    fn CachedControllerFor();
    fn CachedDescribedBy();
    fn CachedFlowsTo();
    fn CachedProviderDescription();
    fn GetClickablePoint();
}
impl ::windows::core::RuntimeName for IUIAutomationElement {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElement";
}
impl IUIAutomationElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElementImpl, const OFFSET: isize>() -> IUIAutomationElementVtbl {
        unsafe extern "system" fn SetFocus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRuntimeId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRuntimeId(::core::mem::transmute_copy(&runtimeid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirst<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirst(scope, &*(&condition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&found)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAll<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAll(scope, &*(&condition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&found)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstBuildCache<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstBuildCache(scope, &*(&condition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&found)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllBuildCache<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllBuildCache(scope, &*(&condition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&found)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildUpdatedCache<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, updatedelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildUpdatedCache(&*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&updatedelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValue<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPropertyValue(propertyid, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValueEx<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPropertyValueEx(propertyid, &*(&ignoredefaultvalue as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPropertyValue<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedPropertyValue(propertyid, ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPropertyValueEx<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedPropertyValueEx(propertyid, &*(&ignoredefaultvalue as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPatternAs<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, riid: &::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPatternAs(patternid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&patternobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPatternAs<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, riid: &::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedPatternAs(patternid, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&patternobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPattern<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPattern(patternid, ::core::mem::transmute_copy(&patternobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedPattern<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedPattern(patternid, ::core::mem::transmute_copy(&patternobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedParent<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedParent(::core::mem::transmute_copy(&parent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedChildren<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedChildren(::core::mem::transmute_copy(&children)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProcessId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProcessId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentControlType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentControlType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLocalizedControlType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLocalizedControlType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentName<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAcceleratorKey<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAcceleratorKey(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAccessKey<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAccessKey(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHasKeyboardFocus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHasKeyboardFocus(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsKeyboardFocusable<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsKeyboardFocusable(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsEnabled<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsEnabled(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAutomationId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAutomationId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentClassName<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentClassName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHelpText<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHelpText(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCulture<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCulture(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsControlElement<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsControlElement(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsContentElement<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsContentElement(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsPassword<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsPassword(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentNativeWindowHandle<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentNativeWindowHandle(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItemType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsOffscreen<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsOffscreen(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentOrientation<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentOrientation(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFrameworkId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFrameworkId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsRequiredForForm<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsRequiredForForm(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemStatus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItemStatus(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentBoundingRectangle<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentBoundingRectangle(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLabeledBy<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLabeledBy(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAriaRole<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAriaRole(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAriaProperties<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAriaProperties(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsDataValidForForm<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsDataValidForForm(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentControllerFor<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentControllerFor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDescribedBy<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDescribedBy(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFlowsTo<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFlowsTo(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProviderDescription<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProviderDescription(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedProcessId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedProcessId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedControlType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedControlType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLocalizedControlType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLocalizedControlType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedName<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAcceleratorKey<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAcceleratorKey(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAccessKey<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAccessKey(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHasKeyboardFocus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHasKeyboardFocus(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsKeyboardFocusable<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsKeyboardFocusable(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsEnabled<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsEnabled(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAutomationId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAutomationId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedClassName<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedClassName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHelpText<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHelpText(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCulture<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCulture(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsControlElement<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsControlElement(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsContentElement<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsContentElement(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsPassword<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsPassword(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedNativeWindowHandle<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedNativeWindowHandle(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedItemType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsOffscreen<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsOffscreen(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedOrientation<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedOrientation(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFrameworkId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFrameworkId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsRequiredForForm<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsRequiredForForm(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemStatus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedItemStatus(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedBoundingRectangle<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedBoundingRectangle(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLabeledBy<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLabeledBy(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAriaRole<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAriaRole(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAriaProperties<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAriaProperties(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsDataValidForForm<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsDataValidForForm(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedControllerFor<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedControllerFor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDescribedBy<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDescribedBy(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFlowsTo<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFlowsTo(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedProviderDescription<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedProviderDescription(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePoint<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClickablePoint(::core::mem::transmute_copy(&clickable), ::core::mem::transmute_copy(&gotclickable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationElement>,
            ::windows::core::GetTrustLevel,
            SetFocus::<Impl, OFFSET>,
            GetRuntimeId::<Impl, OFFSET>,
            FindFirst::<Impl, OFFSET>,
            FindAll::<Impl, OFFSET>,
            FindFirstBuildCache::<Impl, OFFSET>,
            FindAllBuildCache::<Impl, OFFSET>,
            BuildUpdatedCache::<Impl, OFFSET>,
            GetCurrentPropertyValue::<Impl, OFFSET>,
            GetCurrentPropertyValueEx::<Impl, OFFSET>,
            GetCachedPropertyValue::<Impl, OFFSET>,
            GetCachedPropertyValueEx::<Impl, OFFSET>,
            GetCurrentPatternAs::<Impl, OFFSET>,
            GetCachedPatternAs::<Impl, OFFSET>,
            GetCurrentPattern::<Impl, OFFSET>,
            GetCachedPattern::<Impl, OFFSET>,
            GetCachedParent::<Impl, OFFSET>,
            GetCachedChildren::<Impl, OFFSET>,
            CurrentProcessId::<Impl, OFFSET>,
            CurrentControlType::<Impl, OFFSET>,
            CurrentLocalizedControlType::<Impl, OFFSET>,
            CurrentName::<Impl, OFFSET>,
            CurrentAcceleratorKey::<Impl, OFFSET>,
            CurrentAccessKey::<Impl, OFFSET>,
            CurrentHasKeyboardFocus::<Impl, OFFSET>,
            CurrentIsKeyboardFocusable::<Impl, OFFSET>,
            CurrentIsEnabled::<Impl, OFFSET>,
            CurrentAutomationId::<Impl, OFFSET>,
            CurrentClassName::<Impl, OFFSET>,
            CurrentHelpText::<Impl, OFFSET>,
            CurrentCulture::<Impl, OFFSET>,
            CurrentIsControlElement::<Impl, OFFSET>,
            CurrentIsContentElement::<Impl, OFFSET>,
            CurrentIsPassword::<Impl, OFFSET>,
            CurrentNativeWindowHandle::<Impl, OFFSET>,
            CurrentItemType::<Impl, OFFSET>,
            CurrentIsOffscreen::<Impl, OFFSET>,
            CurrentOrientation::<Impl, OFFSET>,
            CurrentFrameworkId::<Impl, OFFSET>,
            CurrentIsRequiredForForm::<Impl, OFFSET>,
            CurrentItemStatus::<Impl, OFFSET>,
            CurrentBoundingRectangle::<Impl, OFFSET>,
            CurrentLabeledBy::<Impl, OFFSET>,
            CurrentAriaRole::<Impl, OFFSET>,
            CurrentAriaProperties::<Impl, OFFSET>,
            CurrentIsDataValidForForm::<Impl, OFFSET>,
            CurrentControllerFor::<Impl, OFFSET>,
            CurrentDescribedBy::<Impl, OFFSET>,
            CurrentFlowsTo::<Impl, OFFSET>,
            CurrentProviderDescription::<Impl, OFFSET>,
            CachedProcessId::<Impl, OFFSET>,
            CachedControlType::<Impl, OFFSET>,
            CachedLocalizedControlType::<Impl, OFFSET>,
            CachedName::<Impl, OFFSET>,
            CachedAcceleratorKey::<Impl, OFFSET>,
            CachedAccessKey::<Impl, OFFSET>,
            CachedHasKeyboardFocus::<Impl, OFFSET>,
            CachedIsKeyboardFocusable::<Impl, OFFSET>,
            CachedIsEnabled::<Impl, OFFSET>,
            CachedAutomationId::<Impl, OFFSET>,
            CachedClassName::<Impl, OFFSET>,
            CachedHelpText::<Impl, OFFSET>,
            CachedCulture::<Impl, OFFSET>,
            CachedIsControlElement::<Impl, OFFSET>,
            CachedIsContentElement::<Impl, OFFSET>,
            CachedIsPassword::<Impl, OFFSET>,
            CachedNativeWindowHandle::<Impl, OFFSET>,
            CachedItemType::<Impl, OFFSET>,
            CachedIsOffscreen::<Impl, OFFSET>,
            CachedOrientation::<Impl, OFFSET>,
            CachedFrameworkId::<Impl, OFFSET>,
            CachedIsRequiredForForm::<Impl, OFFSET>,
            CachedItemStatus::<Impl, OFFSET>,
            CachedBoundingRectangle::<Impl, OFFSET>,
            CachedLabeledBy::<Impl, OFFSET>,
            CachedAriaRole::<Impl, OFFSET>,
            CachedAriaProperties::<Impl, OFFSET>,
            CachedIsDataValidForForm::<Impl, OFFSET>,
            CachedControllerFor::<Impl, OFFSET>,
            CachedDescribedBy::<Impl, OFFSET>,
            CachedFlowsTo::<Impl, OFFSET>,
            CachedProviderDescription::<Impl, OFFSET>,
            GetClickablePoint::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationElement2Impl: Sized + IUIAutomationElementImpl {
    fn CurrentOptimizeForVisualContent();
    fn CachedOptimizeForVisualContent();
    fn CurrentLiveSetting();
    fn CachedLiveSetting();
    fn CurrentFlowsFrom();
    fn CachedFlowsFrom();
}
impl ::windows::core::RuntimeName for IUIAutomationElement2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElement2";
}
impl IUIAutomationElement2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2Impl, const OFFSET: isize>() -> IUIAutomationElement2Vtbl {
        unsafe extern "system" fn CurrentOptimizeForVisualContent<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentOptimizeForVisualContent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedOptimizeForVisualContent<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedOptimizeForVisualContent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLiveSetting<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLiveSetting(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLiveSetting<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLiveSetting(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFlowsFrom<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFlowsFrom(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFlowsFrom<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFlowsFrom(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationElement2>,
            ::windows::core::GetTrustLevel,
            CurrentOptimizeForVisualContent::<Impl, OFFSET>,
            CachedOptimizeForVisualContent::<Impl, OFFSET>,
            CurrentLiveSetting::<Impl, OFFSET>,
            CachedLiveSetting::<Impl, OFFSET>,
            CurrentFlowsFrom::<Impl, OFFSET>,
            CachedFlowsFrom::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationElement3Impl: Sized + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn ShowContextMenu();
    fn CurrentIsPeripheral();
    fn CachedIsPeripheral();
}
impl ::windows::core::RuntimeName for IUIAutomationElement3 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElement3";
}
impl IUIAutomationElement3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement3Impl, const OFFSET: isize>() -> IUIAutomationElement3Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: IUIAutomationElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowContextMenu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsPeripheral<Impl: IUIAutomationElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsPeripheral(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsPeripheral<Impl: IUIAutomationElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsPeripheral(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationElement3>, ::windows::core::GetTrustLevel, ShowContextMenu::<Impl, OFFSET>, CurrentIsPeripheral::<Impl, OFFSET>, CachedIsPeripheral::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationElement4Impl: Sized + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentPositionInSet();
    fn CurrentSizeOfSet();
    fn CurrentLevel();
    fn CurrentAnnotationTypes();
    fn CurrentAnnotationObjects();
    fn CachedPositionInSet();
    fn CachedSizeOfSet();
    fn CachedLevel();
    fn CachedAnnotationTypes();
    fn CachedAnnotationObjects();
}
impl ::windows::core::RuntimeName for IUIAutomationElement4 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElement4";
}
impl IUIAutomationElement4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4Impl, const OFFSET: isize>() -> IUIAutomationElement4Vtbl {
        unsafe extern "system" fn CurrentPositionInSet<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPositionInSet(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSizeOfSet<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSizeOfSet(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLevel<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLevel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypes<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnnotationTypes(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAnnotationObjects<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAnnotationObjects(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedPositionInSet<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedPositionInSet(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSizeOfSet<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedSizeOfSet(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLevel<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLevel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationTypes<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAnnotationTypes(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedAnnotationObjects<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedAnnotationObjects(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationElement4>,
            ::windows::core::GetTrustLevel,
            CurrentPositionInSet::<Impl, OFFSET>,
            CurrentSizeOfSet::<Impl, OFFSET>,
            CurrentLevel::<Impl, OFFSET>,
            CurrentAnnotationTypes::<Impl, OFFSET>,
            CurrentAnnotationObjects::<Impl, OFFSET>,
            CachedPositionInSet::<Impl, OFFSET>,
            CachedSizeOfSet::<Impl, OFFSET>,
            CachedLevel::<Impl, OFFSET>,
            CachedAnnotationTypes::<Impl, OFFSET>,
            CachedAnnotationObjects::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationElement5Impl: Sized + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentLandmarkType();
    fn CurrentLocalizedLandmarkType();
    fn CachedLandmarkType();
    fn CachedLocalizedLandmarkType();
}
impl ::windows::core::RuntimeName for IUIAutomationElement5 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElement5";
}
impl IUIAutomationElement5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement5Impl, const OFFSET: isize>() -> IUIAutomationElement5Vtbl {
        unsafe extern "system" fn CurrentLandmarkType<Impl: IUIAutomationElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLandmarkType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLocalizedLandmarkType<Impl: IUIAutomationElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLocalizedLandmarkType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLandmarkType<Impl: IUIAutomationElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLandmarkType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLocalizedLandmarkType<Impl: IUIAutomationElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLocalizedLandmarkType(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationElement5>, ::windows::core::GetTrustLevel, CurrentLandmarkType::<Impl, OFFSET>, CurrentLocalizedLandmarkType::<Impl, OFFSET>, CachedLandmarkType::<Impl, OFFSET>, CachedLocalizedLandmarkType::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationElement6Impl: Sized + IUIAutomationElement5Impl + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentFullDescription();
    fn CachedFullDescription();
}
impl ::windows::core::RuntimeName for IUIAutomationElement6 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElement6";
}
impl IUIAutomationElement6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement6Impl, const OFFSET: isize>() -> IUIAutomationElement6Vtbl {
        unsafe extern "system" fn CurrentFullDescription<Impl: IUIAutomationElement6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFullDescription(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFullDescription<Impl: IUIAutomationElement6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFullDescription(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationElement6>, ::windows::core::GetTrustLevel, CurrentFullDescription::<Impl, OFFSET>, CachedFullDescription::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationElement7Impl: Sized + IUIAutomationElement6Impl + IUIAutomationElement5Impl + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn FindFirstWithOptions();
    fn FindAllWithOptions();
    fn FindFirstWithOptionsBuildCache();
    fn FindAllWithOptionsBuildCache();
    fn GetCurrentMetadataValue();
}
impl ::windows::core::RuntimeName for IUIAutomationElement7 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElement7";
}
impl IUIAutomationElement7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement7Impl, const OFFSET: isize>() -> IUIAutomationElement7Vtbl {
        unsafe extern "system" fn FindFirstWithOptions<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstWithOptions(scope, &*(&condition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), traversaloptions, &*(&root as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&found)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithOptions<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllWithOptions(scope, &*(&condition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType), traversaloptions, &*(&root as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&found)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstWithOptionsBuildCache<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstWithOptionsBuildCache(
                scope,
                &*(&condition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType),
                &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                traversaloptions,
                &*(&root as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&found),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithOptionsBuildCache<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllWithOptionsBuildCache(
                scope,
                &*(&condition as *const <IUIAutomationCondition as ::windows::core::Abi>::Abi as *const <IUIAutomationCondition as ::windows::core::DefaultType>::DefaultType),
                &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType),
                traversaloptions,
                &*(&root as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&found),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentMetadataValue<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: i32, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentMetadataValue(targetid, metadataid, ::core::mem::transmute_copy(&returnval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationElement7>, ::windows::core::GetTrustLevel, FindFirstWithOptions::<Impl, OFFSET>, FindAllWithOptions::<Impl, OFFSET>, FindFirstWithOptionsBuildCache::<Impl, OFFSET>, FindAllWithOptionsBuildCache::<Impl, OFFSET>, GetCurrentMetadataValue::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationElement8Impl: Sized + IUIAutomationElement7Impl + IUIAutomationElement6Impl + IUIAutomationElement5Impl + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentHeadingLevel();
    fn CachedHeadingLevel();
}
impl ::windows::core::RuntimeName for IUIAutomationElement8 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElement8";
}
impl IUIAutomationElement8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement8Impl, const OFFSET: isize>() -> IUIAutomationElement8Vtbl {
        unsafe extern "system" fn CurrentHeadingLevel<Impl: IUIAutomationElement8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHeadingLevel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHeadingLevel<Impl: IUIAutomationElement8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHeadingLevel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationElement8>, ::windows::core::GetTrustLevel, CurrentHeadingLevel::<Impl, OFFSET>, CachedHeadingLevel::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationElement9Impl: Sized + IUIAutomationElement8Impl + IUIAutomationElement7Impl + IUIAutomationElement6Impl + IUIAutomationElement5Impl + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentIsDialog();
    fn CachedIsDialog();
}
impl ::windows::core::RuntimeName for IUIAutomationElement9 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElement9";
}
impl IUIAutomationElement9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement9Impl, const OFFSET: isize>() -> IUIAutomationElement9Vtbl {
        unsafe extern "system" fn CurrentIsDialog<Impl: IUIAutomationElement9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsDialog(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsDialog<Impl: IUIAutomationElement9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsDialog(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationElement9>, ::windows::core::GetTrustLevel, CurrentIsDialog::<Impl, OFFSET>, CachedIsDialog::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationElementArrayImpl: Sized {
    fn Length();
    fn GetElement();
}
impl ::windows::core::RuntimeName for IUIAutomationElementArray {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationElementArray";
}
impl IUIAutomationElementArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElementArrayImpl, const OFFSET: isize>() -> IUIAutomationElementArrayVtbl {
        unsafe extern "system" fn Length<Impl: IUIAutomationElementArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Impl: IUIAutomationElementArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElement(index, ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationElementArray>, ::windows::core::GetTrustLevel, Length::<Impl, OFFSET>, GetElement::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationEventHandlerImpl: Sized {
    fn HandleAutomationEvent();
}
impl ::windows::core::RuntimeName for IUIAutomationEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationEventHandler";
}
impl IUIAutomationEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerImpl, const OFFSET: isize>() -> IUIAutomationEventHandlerVtbl {
        unsafe extern "system" fn HandleAutomationEvent<Impl: IUIAutomationEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, eventid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandleAutomationEvent(&*(&sender as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), eventid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationEventHandler>, ::windows::core::GetTrustLevel, HandleAutomationEvent::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationEventHandlerGroupImpl: Sized {
    fn AddActiveTextPositionChangedEventHandler();
    fn AddAutomationEventHandler();
    fn AddChangesEventHandler();
    fn AddNotificationEventHandler();
    fn AddPropertyChangedEventHandler();
    fn AddStructureChangedEventHandler();
    fn AddTextEditTextChangedEventHandler();
}
impl ::windows::core::RuntimeName for IUIAutomationEventHandlerGroup {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationEventHandlerGroup";
}
impl IUIAutomationEventHandlerGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>() -> IUIAutomationEventHandlerGroupVtbl {
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddActiveTextPositionChangedEventHandler(scope, &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationActiveTextPositionChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationActiveTextPositionChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAutomationEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAutomationEventHandler(eventid, scope, &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddChangesEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddChangesEventHandler(scope, changetypes, changescount, &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationChangesEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationChangesEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddNotificationEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNotificationEventHandler(scope, &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationNotificationEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationNotificationEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const i32, propertycount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyChangedEventHandler(scope, &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationPropertyChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationPropertyChangedEventHandler as ::windows::core::DefaultType>::DefaultType), propertyarray, propertycount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStructureChangedEventHandler(scope, &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationStructureChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationStructureChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTextEditTextChangedEventHandler(scope, texteditchangetype, &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <IUIAutomationTextEditTextChangedEventHandler as ::windows::core::Abi>::Abi as *const <IUIAutomationTextEditTextChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationEventHandlerGroup>,
            ::windows::core::GetTrustLevel,
            AddActiveTextPositionChangedEventHandler::<Impl, OFFSET>,
            AddAutomationEventHandler::<Impl, OFFSET>,
            AddChangesEventHandler::<Impl, OFFSET>,
            AddNotificationEventHandler::<Impl, OFFSET>,
            AddPropertyChangedEventHandler::<Impl, OFFSET>,
            AddStructureChangedEventHandler::<Impl, OFFSET>,
            AddTextEditTextChangedEventHandler::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationExpandCollapsePatternImpl: Sized {
    fn Expand();
    fn Collapse();
    fn CurrentExpandCollapseState();
    fn CachedExpandCollapseState();
}
impl ::windows::core::RuntimeName for IUIAutomationExpandCollapsePattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationExpandCollapsePattern";
}
impl IUIAutomationExpandCollapsePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationExpandCollapsePatternImpl, const OFFSET: isize>() -> IUIAutomationExpandCollapsePatternVtbl {
        unsafe extern "system" fn Expand<Impl: IUIAutomationExpandCollapsePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Expand() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Collapse<Impl: IUIAutomationExpandCollapsePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Collapse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentExpandCollapseState<Impl: IUIAutomationExpandCollapsePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentExpandCollapseState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedExpandCollapseState<Impl: IUIAutomationExpandCollapsePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedExpandCollapseState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationExpandCollapsePattern>, ::windows::core::GetTrustLevel, Expand::<Impl, OFFSET>, Collapse::<Impl, OFFSET>, CurrentExpandCollapseState::<Impl, OFFSET>, CachedExpandCollapseState::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationFocusChangedEventHandlerImpl: Sized {
    fn HandleFocusChangedEvent();
}
impl ::windows::core::RuntimeName for IUIAutomationFocusChangedEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationFocusChangedEventHandler";
}
impl IUIAutomationFocusChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationFocusChangedEventHandlerImpl, const OFFSET: isize>() -> IUIAutomationFocusChangedEventHandlerVtbl {
        unsafe extern "system" fn HandleFocusChangedEvent<Impl: IUIAutomationFocusChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandleFocusChangedEvent(&*(&sender as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationFocusChangedEventHandler>, ::windows::core::GetTrustLevel, HandleFocusChangedEvent::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationGridItemPatternImpl: Sized {
    fn CurrentContainingGrid();
    fn CurrentRow();
    fn CurrentColumn();
    fn CurrentRowSpan();
    fn CurrentColumnSpan();
    fn CachedContainingGrid();
    fn CachedRow();
    fn CachedColumn();
    fn CachedRowSpan();
    fn CachedColumnSpan();
}
impl ::windows::core::RuntimeName for IUIAutomationGridItemPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationGridItemPattern";
}
impl IUIAutomationGridItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>() -> IUIAutomationGridItemPatternVtbl {
        unsafe extern "system" fn CurrentContainingGrid<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentContainingGrid(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRow<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRow(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumn<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentColumn(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowSpan<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRowSpan(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumnSpan<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentColumnSpan(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedContainingGrid<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedContainingGrid(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRow<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRow(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumn<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedColumn(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowSpan<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRowSpan(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumnSpan<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedColumnSpan(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationGridItemPattern>,
            ::windows::core::GetTrustLevel,
            CurrentContainingGrid::<Impl, OFFSET>,
            CurrentRow::<Impl, OFFSET>,
            CurrentColumn::<Impl, OFFSET>,
            CurrentRowSpan::<Impl, OFFSET>,
            CurrentColumnSpan::<Impl, OFFSET>,
            CachedContainingGrid::<Impl, OFFSET>,
            CachedRow::<Impl, OFFSET>,
            CachedColumn::<Impl, OFFSET>,
            CachedRowSpan::<Impl, OFFSET>,
            CachedColumnSpan::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationGridPatternImpl: Sized {
    fn GetItem();
    fn CurrentRowCount();
    fn CurrentColumnCount();
    fn CachedRowCount();
    fn CachedColumnCount();
}
impl ::windows::core::RuntimeName for IUIAutomationGridPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationGridPattern";
}
impl IUIAutomationGridPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>() -> IUIAutomationGridPatternVtbl {
        unsafe extern "system" fn GetItem<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItem(row, column, ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowCount<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRowCount(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentColumnCount<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentColumnCount(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowCount<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRowCount(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedColumnCount<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedColumnCount(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationGridPattern>, ::windows::core::GetTrustLevel, GetItem::<Impl, OFFSET>, CurrentRowCount::<Impl, OFFSET>, CurrentColumnCount::<Impl, OFFSET>, CachedRowCount::<Impl, OFFSET>, CachedColumnCount::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationInvokePatternImpl: Sized {
    fn Invoke();
}
impl ::windows::core::RuntimeName for IUIAutomationInvokePattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationInvokePattern";
}
impl IUIAutomationInvokePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationInvokePatternImpl, const OFFSET: isize>() -> IUIAutomationInvokePatternVtbl {
        unsafe extern "system" fn Invoke<Impl: IUIAutomationInvokePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationInvokePattern>, ::windows::core::GetTrustLevel, Invoke::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationItemContainerPatternImpl: Sized {
    fn FindItemByProperty();
}
impl ::windows::core::RuntimeName for IUIAutomationItemContainerPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationItemContainerPattern";
}
impl IUIAutomationItemContainerPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationItemContainerPatternImpl, const OFFSET: isize>() -> IUIAutomationItemContainerPatternVtbl {
        unsafe extern "system" fn FindItemByProperty<Impl: IUIAutomationItemContainerPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: ::windows::core::RawPtr, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindItemByProperty(&*(&pstartafter as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), propertyid, &*(&value as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfound)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationItemContainerPattern>, ::windows::core::GetTrustLevel, FindItemByProperty::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationLegacyIAccessiblePatternImpl: Sized {
    fn Select();
    fn DoDefaultAction();
    fn SetValue();
    fn CurrentChildId();
    fn CurrentName();
    fn CurrentValue();
    fn CurrentDescription();
    fn CurrentRole();
    fn CurrentState();
    fn CurrentHelp();
    fn CurrentKeyboardShortcut();
    fn GetCurrentSelection();
    fn CurrentDefaultAction();
    fn CachedChildId();
    fn CachedName();
    fn CachedValue();
    fn CachedDescription();
    fn CachedRole();
    fn CachedState();
    fn CachedHelp();
    fn CachedKeyboardShortcut();
    fn GetCachedSelection();
    fn CachedDefaultAction();
    fn GetIAccessible();
}
impl ::windows::core::RuntimeName for IUIAutomationLegacyIAccessiblePattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationLegacyIAccessiblePattern";
}
impl IUIAutomationLegacyIAccessiblePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>() -> IUIAutomationLegacyIAccessiblePatternVtbl {
        unsafe extern "system" fn Select<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Select(flagsselect) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoDefaultAction<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoDefaultAction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&szvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentChildId<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentChildId(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentName<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentName(::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentValue<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentValue(::core::mem::transmute_copy(&pszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDescription<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDescription(::core::mem::transmute_copy(&pszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRole<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRole(::core::mem::transmute_copy(&pdwrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentState<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentState(::core::mem::transmute_copy(&pdwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHelp<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHelp(::core::mem::transmute_copy(&pszhelp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentKeyboardShortcut<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentKeyboardShortcut(::core::mem::transmute_copy(&pszkeyboardshortcut)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSelection<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSelection(::core::mem::transmute_copy(&pvarselectedchildren)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDefaultAction<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDefaultAction(::core::mem::transmute_copy(&pszdefaultaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedChildId<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedChildId(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedName<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedName(::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedValue(::core::mem::transmute_copy(&pszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDescription<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDescription(::core::mem::transmute_copy(&pszdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRole<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRole(::core::mem::transmute_copy(&pdwrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedState<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedState(::core::mem::transmute_copy(&pdwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHelp<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHelp(::core::mem::transmute_copy(&pszhelp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedKeyboardShortcut<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedKeyboardShortcut(::core::mem::transmute_copy(&pszkeyboardshortcut)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSelection<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedSelection(::core::mem::transmute_copy(&pvarselectedchildren)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedDefaultAction<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedDefaultAction(::core::mem::transmute_copy(&pszdefaultaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIAccessible<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIAccessible(::core::mem::transmute_copy(&ppaccessible)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationLegacyIAccessiblePattern>,
            ::windows::core::GetTrustLevel,
            Select::<Impl, OFFSET>,
            DoDefaultAction::<Impl, OFFSET>,
            SetValue::<Impl, OFFSET>,
            CurrentChildId::<Impl, OFFSET>,
            CurrentName::<Impl, OFFSET>,
            CurrentValue::<Impl, OFFSET>,
            CurrentDescription::<Impl, OFFSET>,
            CurrentRole::<Impl, OFFSET>,
            CurrentState::<Impl, OFFSET>,
            CurrentHelp::<Impl, OFFSET>,
            CurrentKeyboardShortcut::<Impl, OFFSET>,
            GetCurrentSelection::<Impl, OFFSET>,
            CurrentDefaultAction::<Impl, OFFSET>,
            CachedChildId::<Impl, OFFSET>,
            CachedName::<Impl, OFFSET>,
            CachedValue::<Impl, OFFSET>,
            CachedDescription::<Impl, OFFSET>,
            CachedRole::<Impl, OFFSET>,
            CachedState::<Impl, OFFSET>,
            CachedHelp::<Impl, OFFSET>,
            CachedKeyboardShortcut::<Impl, OFFSET>,
            GetCachedSelection::<Impl, OFFSET>,
            CachedDefaultAction::<Impl, OFFSET>,
            GetIAccessible::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationMultipleViewPatternImpl: Sized {
    fn GetViewName();
    fn SetCurrentView();
    fn CurrentCurrentView();
    fn GetCurrentSupportedViews();
    fn CachedCurrentView();
    fn GetCachedSupportedViews();
}
impl ::windows::core::RuntimeName for IUIAutomationMultipleViewPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationMultipleViewPattern";
}
impl IUIAutomationMultipleViewPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>() -> IUIAutomationMultipleViewPatternVtbl {
        unsafe extern "system" fn GetViewName<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewName(view, ::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentView<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentView(view) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCurrentView<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCurrentView(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSupportedViews<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSupportedViews(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCurrentView<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCurrentView(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSupportedViews<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedSupportedViews(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationMultipleViewPattern>,
            ::windows::core::GetTrustLevel,
            GetViewName::<Impl, OFFSET>,
            SetCurrentView::<Impl, OFFSET>,
            CurrentCurrentView::<Impl, OFFSET>,
            GetCurrentSupportedViews::<Impl, OFFSET>,
            CachedCurrentView::<Impl, OFFSET>,
            GetCachedSupportedViews::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationNotConditionImpl: Sized + IUIAutomationConditionImpl {
    fn GetChild();
}
impl ::windows::core::RuntimeName for IUIAutomationNotCondition {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationNotCondition";
}
impl IUIAutomationNotConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotConditionImpl, const OFFSET: isize>() -> IUIAutomationNotConditionVtbl {
        unsafe extern "system" fn GetChild<Impl: IUIAutomationNotConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChild(::core::mem::transmute_copy(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationNotCondition>, ::windows::core::GetTrustLevel, GetChild::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationNotificationEventHandlerImpl: Sized {
    fn HandleNotificationEvent();
}
impl ::windows::core::RuntimeName for IUIAutomationNotificationEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationNotificationEventHandler";
}
impl IUIAutomationNotificationEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotificationEventHandlerImpl, const OFFSET: isize>() -> IUIAutomationNotificationEventHandlerVtbl {
        unsafe extern "system" fn HandleNotificationEvent<Impl: IUIAutomationNotificationEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, activityid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandleNotificationEvent(
                &*(&sender as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType),
                notificationkind,
                notificationprocessing,
                &*(&displaystring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&activityid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationNotificationEventHandler>, ::windows::core::GetTrustLevel, HandleNotificationEvent::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationObjectModelPatternImpl: Sized {
    fn GetUnderlyingObjectModel();
}
impl ::windows::core::RuntimeName for IUIAutomationObjectModelPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationObjectModelPattern";
}
impl IUIAutomationObjectModelPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationObjectModelPatternImpl, const OFFSET: isize>() -> IUIAutomationObjectModelPatternVtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Impl: IUIAutomationObjectModelPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnderlyingObjectModel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationObjectModelPattern>, ::windows::core::GetTrustLevel, GetUnderlyingObjectModel::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationOrConditionImpl: Sized + IUIAutomationConditionImpl {
    fn ChildCount();
    fn GetChildrenAsNativeArray();
    fn GetChildren();
}
impl ::windows::core::RuntimeName for IUIAutomationOrCondition {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationOrCondition";
}
impl IUIAutomationOrConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationOrConditionImpl, const OFFSET: isize>() -> IUIAutomationOrConditionVtbl {
        unsafe extern "system" fn ChildCount<Impl: IUIAutomationOrConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildCount(::core::mem::transmute_copy(&childcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Impl: IUIAutomationOrConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::windows::core::RawPtr, childarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildrenAsNativeArray(::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Impl: IUIAutomationOrConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren(::core::mem::transmute_copy(&childarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationOrCondition>, ::windows::core::GetTrustLevel, ChildCount::<Impl, OFFSET>, GetChildrenAsNativeArray::<Impl, OFFSET>, GetChildren::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationPatternHandlerImpl: Sized {
    fn CreateClientWrapper();
    fn Dispatch();
}
impl ::windows::core::RuntimeName for IUIAutomationPatternHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationPatternHandler";
}
impl IUIAutomationPatternHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternHandlerImpl, const OFFSET: isize>() -> IUIAutomationPatternHandlerVtbl {
        unsafe extern "system" fn CreateClientWrapper<Impl: IUIAutomationPatternHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppatterninstance: ::windows::core::RawPtr, pclientwrapper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateClientWrapper(&*(&ppatterninstance as *const <IUIAutomationPatternInstance as ::windows::core::Abi>::Abi as *const <IUIAutomationPatternInstance as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pclientwrapper)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatch<Impl: IUIAutomationPatternHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dispatch(&*(&ptarget as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), index, &*(&pparams as *const <UIAutomationParameter as ::windows::core::Abi>::Abi as *const <UIAutomationParameter as ::windows::core::DefaultType>::DefaultType), cparams) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationPatternHandler>, ::windows::core::GetTrustLevel, CreateClientWrapper::<Impl, OFFSET>, Dispatch::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationPatternInstanceImpl: Sized {
    fn GetProperty();
    fn CallMethod();
}
impl ::windows::core::RuntimeName for IUIAutomationPatternInstance {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationPatternInstance";
}
impl IUIAutomationPatternInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternInstanceImpl, const OFFSET: isize>() -> IUIAutomationPatternInstanceVtbl {
        unsafe extern "system" fn GetProperty<Impl: IUIAutomationPatternInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(index, &*(&cached as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), r#type, ::core::mem::transmute_copy(&pptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallMethod<Impl: IUIAutomationPatternInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallMethod(index, &*(&pparams as *const <UIAutomationParameter as ::windows::core::Abi>::Abi as *const <UIAutomationParameter as ::windows::core::DefaultType>::DefaultType), cparams) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationPatternInstance>, ::windows::core::GetTrustLevel, GetProperty::<Impl, OFFSET>, CallMethod::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationPropertyChangedEventHandlerImpl: Sized {
    fn HandlePropertyChangedEvent();
}
impl ::windows::core::RuntimeName for IUIAutomationPropertyChangedEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationPropertyChangedEventHandler";
}
impl IUIAutomationPropertyChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyChangedEventHandlerImpl, const OFFSET: isize>() -> IUIAutomationPropertyChangedEventHandlerVtbl {
        unsafe extern "system" fn HandlePropertyChangedEvent<Impl: IUIAutomationPropertyChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, propertyid: i32, newvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandlePropertyChangedEvent(&*(&sender as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), propertyid, &*(&newvalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationPropertyChangedEventHandler>, ::windows::core::GetTrustLevel, HandlePropertyChangedEvent::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationPropertyConditionImpl: Sized + IUIAutomationConditionImpl {
    fn PropertyId();
    fn PropertyValue();
    fn PropertyConditionFlags();
}
impl ::windows::core::RuntimeName for IUIAutomationPropertyCondition {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationPropertyCondition";
}
impl IUIAutomationPropertyConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyConditionImpl, const OFFSET: isize>() -> IUIAutomationPropertyConditionVtbl {
        unsafe extern "system" fn PropertyId<Impl: IUIAutomationPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyId(::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Impl: IUIAutomationPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyValue(::core::mem::transmute_copy(&propertyvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyConditionFlags<Impl: IUIAutomationPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut PropertyConditionFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyConditionFlags(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationPropertyCondition>, ::windows::core::GetTrustLevel, PropertyId::<Impl, OFFSET>, PropertyValue::<Impl, OFFSET>, PropertyConditionFlags::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationProxyFactoryImpl: Sized {
    fn CreateProvider();
    fn ProxyFactoryId();
}
impl ::windows::core::RuntimeName for IUIAutomationProxyFactory {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationProxyFactory";
}
impl IUIAutomationProxyFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryImpl, const OFFSET: isize>() -> IUIAutomationProxyFactoryVtbl {
        unsafe extern "system" fn CreateProvider<Impl: IUIAutomationProxyFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, provider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProvider(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), idobject, idchild, ::core::mem::transmute_copy(&provider)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyFactoryId<Impl: IUIAutomationProxyFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoryid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyFactoryId(::core::mem::transmute_copy(&factoryid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationProxyFactory>, ::windows::core::GetTrustLevel, CreateProvider::<Impl, OFFSET>, ProxyFactoryId::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationProxyFactoryEntryImpl: Sized {
    fn ProxyFactory();
    fn ClassName();
    fn ImageName();
    fn AllowSubstringMatch();
    fn CanCheckBaseClass();
    fn NeedsAdviseEvents();
    fn SetClassName();
    fn SetImageName();
    fn SetAllowSubstringMatch();
    fn SetCanCheckBaseClass();
    fn SetNeedsAdviseEvents();
    fn SetWinEventsForAutomationEvent();
    fn GetWinEventsForAutomationEvent();
}
impl ::windows::core::RuntimeName for IUIAutomationProxyFactoryEntry {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationProxyFactoryEntry";
}
impl IUIAutomationProxyFactoryEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>() -> IUIAutomationProxyFactoryEntryVtbl {
        unsafe extern "system" fn ProxyFactory<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyFactory(::core::mem::transmute_copy(&factory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassName<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassName(::core::mem::transmute_copy(&classname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageName<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageName(::core::mem::transmute_copy(&imagename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowSubstringMatch<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowSubstringMatch(::core::mem::transmute_copy(&allowsubstringmatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCheckBaseClass<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCheckBaseClass(::core::mem::transmute_copy(&cancheckbaseclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeedsAdviseEvents<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeedsAdviseEvents(::core::mem::transmute_copy(&adviseevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassName<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClassName(&*(&classname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageName<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetImageName(&*(&imagename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowSubstringMatch<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllowSubstringMatch(&*(&allowsubstringmatch as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanCheckBaseClass<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCanCheckBaseClass(&*(&cancheckbaseclass as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNeedsAdviseEvents<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNeedsAdviseEvents(&*(&adviseevents as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWinEventsForAutomationEvent<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyid: i32, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWinEventsForAutomationEvent(eventid, propertyid, &*(&winevents as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWinEventsForAutomationEvent<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyid: i32, winevents: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWinEventsForAutomationEvent(eventid, propertyid, ::core::mem::transmute_copy(&winevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationProxyFactoryEntry>,
            ::windows::core::GetTrustLevel,
            ProxyFactory::<Impl, OFFSET>,
            ClassName::<Impl, OFFSET>,
            ImageName::<Impl, OFFSET>,
            AllowSubstringMatch::<Impl, OFFSET>,
            CanCheckBaseClass::<Impl, OFFSET>,
            NeedsAdviseEvents::<Impl, OFFSET>,
            SetClassName::<Impl, OFFSET>,
            SetImageName::<Impl, OFFSET>,
            SetAllowSubstringMatch::<Impl, OFFSET>,
            SetCanCheckBaseClass::<Impl, OFFSET>,
            SetNeedsAdviseEvents::<Impl, OFFSET>,
            SetWinEventsForAutomationEvent::<Impl, OFFSET>,
            GetWinEventsForAutomationEvent::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationProxyFactoryMappingImpl: Sized {
    fn Count();
    fn GetTable();
    fn GetEntry();
    fn SetTable();
    fn InsertEntries();
    fn InsertEntry();
    fn RemoveEntry();
    fn ClearTable();
    fn RestoreDefaultTable();
}
impl ::windows::core::RuntimeName for IUIAutomationProxyFactoryMapping {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationProxyFactoryMapping";
}
impl IUIAutomationProxyFactoryMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>() -> IUIAutomationProxyFactoryMappingVtbl {
        unsafe extern "system" fn Count<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTable<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, table: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTable(::core::mem::transmute_copy(&table)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntry<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEntry(index, ::core::mem::transmute_copy(&entry)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTable<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTable(&*(&factorylist as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEntries<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEntries(before, &*(&factorylist as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertEntry<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsertEntry(before, &*(&factory as *const <IUIAutomationProxyFactoryEntry as ::windows::core::Abi>::Abi as *const <IUIAutomationProxyFactoryEntry as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEntry<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveEntry(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearTable<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearTable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreDefaultTable<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestoreDefaultTable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationProxyFactoryMapping>,
            ::windows::core::GetTrustLevel,
            Count::<Impl, OFFSET>,
            GetTable::<Impl, OFFSET>,
            GetEntry::<Impl, OFFSET>,
            SetTable::<Impl, OFFSET>,
            InsertEntries::<Impl, OFFSET>,
            InsertEntry::<Impl, OFFSET>,
            RemoveEntry::<Impl, OFFSET>,
            ClearTable::<Impl, OFFSET>,
            RestoreDefaultTable::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationRangeValuePatternImpl: Sized {
    fn SetValue();
    fn CurrentValue();
    fn CurrentIsReadOnly();
    fn CurrentMaximum();
    fn CurrentMinimum();
    fn CurrentLargeChange();
    fn CurrentSmallChange();
    fn CachedValue();
    fn CachedIsReadOnly();
    fn CachedMaximum();
    fn CachedMinimum();
    fn CachedLargeChange();
    fn CachedSmallChange();
}
impl ::windows::core::RuntimeName for IUIAutomationRangeValuePattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationRangeValuePattern";
}
impl IUIAutomationRangeValuePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>() -> IUIAutomationRangeValuePatternVtbl {
        unsafe extern "system" fn SetValue<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(val) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentValue<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentValue(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsReadOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMaximum<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMaximum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMinimum<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMinimum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLargeChange<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLargeChange(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSmallChange<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSmallChange(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedValue(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsReadOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedMaximum<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedMaximum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedMinimum<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedMinimum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLargeChange<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLargeChange(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSmallChange<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedSmallChange(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationRangeValuePattern>,
            ::windows::core::GetTrustLevel,
            SetValue::<Impl, OFFSET>,
            CurrentValue::<Impl, OFFSET>,
            CurrentIsReadOnly::<Impl, OFFSET>,
            CurrentMaximum::<Impl, OFFSET>,
            CurrentMinimum::<Impl, OFFSET>,
            CurrentLargeChange::<Impl, OFFSET>,
            CurrentSmallChange::<Impl, OFFSET>,
            CachedValue::<Impl, OFFSET>,
            CachedIsReadOnly::<Impl, OFFSET>,
            CachedMaximum::<Impl, OFFSET>,
            CachedMinimum::<Impl, OFFSET>,
            CachedLargeChange::<Impl, OFFSET>,
            CachedSmallChange::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationRegistrarImpl: Sized {
    fn RegisterProperty();
    fn RegisterEvent();
    fn RegisterPattern();
}
impl ::windows::core::RuntimeName for IUIAutomationRegistrar {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationRegistrar";
}
impl IUIAutomationRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRegistrarImpl, const OFFSET: isize>() -> IUIAutomationRegistrarVtbl {
        unsafe extern "system" fn RegisterProperty<Impl: IUIAutomationRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const UIAutomationPropertyInfo, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterProperty(&*(&property as *const <UIAutomationPropertyInfo as ::windows::core::Abi>::Abi as *const <UIAutomationPropertyInfo as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEvent<Impl: IUIAutomationRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const UIAutomationEventInfo, eventid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterEvent(&*(&event as *const <UIAutomationEventInfo as ::windows::core::Abi>::Abi as *const <UIAutomationEventInfo as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&eventid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPattern<Impl: IUIAutomationRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPattern(&*(&pattern as *const <UIAutomationPatternInfo as ::windows::core::Abi>::Abi as *const <UIAutomationPatternInfo as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppatternid), ::core::mem::transmute_copy(&ppatternavailablepropertyid), propertyidcount, ::core::mem::transmute_copy(&ppropertyids), eventidcount, ::core::mem::transmute_copy(&peventids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationRegistrar>, ::windows::core::GetTrustLevel, RegisterProperty::<Impl, OFFSET>, RegisterEvent::<Impl, OFFSET>, RegisterPattern::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationScrollItemPatternImpl: Sized {
    fn ScrollIntoView();
}
impl ::windows::core::RuntimeName for IUIAutomationScrollItemPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationScrollItemPattern";
}
impl IUIAutomationScrollItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollItemPatternImpl, const OFFSET: isize>() -> IUIAutomationScrollItemPatternVtbl {
        unsafe extern "system" fn ScrollIntoView<Impl: IUIAutomationScrollItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollIntoView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationScrollItemPattern>, ::windows::core::GetTrustLevel, ScrollIntoView::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationScrollPatternImpl: Sized {
    fn Scroll();
    fn SetScrollPercent();
    fn CurrentHorizontalScrollPercent();
    fn CurrentVerticalScrollPercent();
    fn CurrentHorizontalViewSize();
    fn CurrentVerticalViewSize();
    fn CurrentHorizontallyScrollable();
    fn CurrentVerticallyScrollable();
    fn CachedHorizontalScrollPercent();
    fn CachedVerticalScrollPercent();
    fn CachedHorizontalViewSize();
    fn CachedVerticalViewSize();
    fn CachedHorizontallyScrollable();
    fn CachedVerticallyScrollable();
}
impl ::windows::core::RuntimeName for IUIAutomationScrollPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationScrollPattern";
}
impl IUIAutomationScrollPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>() -> IUIAutomationScrollPatternVtbl {
        unsafe extern "system" fn Scroll<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scroll(horizontalamount, verticalamount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScrollPercent(horizontalpercent, verticalpercent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHorizontalScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHorizontalScrollPercent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticalScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentVerticalScrollPercent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHorizontalViewSize<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHorizontalViewSize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticalViewSize<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentVerticalViewSize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentHorizontallyScrollable<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentHorizontallyScrollable(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentVerticallyScrollable<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentVerticallyScrollable(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontalScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHorizontalScrollPercent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticalScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedVerticalScrollPercent(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontalViewSize<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHorizontalViewSize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticalViewSize<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedVerticalViewSize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedHorizontallyScrollable<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedHorizontallyScrollable(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedVerticallyScrollable<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedVerticallyScrollable(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationScrollPattern>,
            ::windows::core::GetTrustLevel,
            Scroll::<Impl, OFFSET>,
            SetScrollPercent::<Impl, OFFSET>,
            CurrentHorizontalScrollPercent::<Impl, OFFSET>,
            CurrentVerticalScrollPercent::<Impl, OFFSET>,
            CurrentHorizontalViewSize::<Impl, OFFSET>,
            CurrentVerticalViewSize::<Impl, OFFSET>,
            CurrentHorizontallyScrollable::<Impl, OFFSET>,
            CurrentVerticallyScrollable::<Impl, OFFSET>,
            CachedHorizontalScrollPercent::<Impl, OFFSET>,
            CachedVerticalScrollPercent::<Impl, OFFSET>,
            CachedHorizontalViewSize::<Impl, OFFSET>,
            CachedVerticalViewSize::<Impl, OFFSET>,
            CachedHorizontallyScrollable::<Impl, OFFSET>,
            CachedVerticallyScrollable::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationSelectionItemPatternImpl: Sized {
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn CurrentIsSelected();
    fn CurrentSelectionContainer();
    fn CachedIsSelected();
    fn CachedSelectionContainer();
}
impl ::windows::core::RuntimeName for IUIAutomationSelectionItemPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationSelectionItemPattern";
}
impl IUIAutomationSelectionItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>() -> IUIAutomationSelectionItemPatternVtbl {
        unsafe extern "system" fn Select<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Select() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSelection<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddToSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveFromSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsSelected<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsSelected(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSelectionContainer<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSelectionContainer(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsSelected<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsSelected(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedSelectionContainer<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedSelectionContainer(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationSelectionItemPattern>,
            ::windows::core::GetTrustLevel,
            Select::<Impl, OFFSET>,
            AddToSelection::<Impl, OFFSET>,
            RemoveFromSelection::<Impl, OFFSET>,
            CurrentIsSelected::<Impl, OFFSET>,
            CurrentSelectionContainer::<Impl, OFFSET>,
            CachedIsSelected::<Impl, OFFSET>,
            CachedSelectionContainer::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationSelectionPatternImpl: Sized {
    fn GetCurrentSelection();
    fn CurrentCanSelectMultiple();
    fn CurrentIsSelectionRequired();
    fn GetCachedSelection();
    fn CachedCanSelectMultiple();
    fn CachedIsSelectionRequired();
}
impl ::windows::core::RuntimeName for IUIAutomationSelectionPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationSelectionPattern";
}
impl IUIAutomationSelectionPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>() -> IUIAutomationSelectionPatternVtbl {
        unsafe extern "system" fn GetCurrentSelection<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSelection(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanSelectMultiple<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanSelectMultiple(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsSelectionRequired<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsSelectionRequired(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedSelection<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedSelection(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanSelectMultiple<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanSelectMultiple(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsSelectionRequired<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsSelectionRequired(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationSelectionPattern>,
            ::windows::core::GetTrustLevel,
            GetCurrentSelection::<Impl, OFFSET>,
            CurrentCanSelectMultiple::<Impl, OFFSET>,
            CurrentIsSelectionRequired::<Impl, OFFSET>,
            GetCachedSelection::<Impl, OFFSET>,
            CachedCanSelectMultiple::<Impl, OFFSET>,
            CachedIsSelectionRequired::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationSelectionPattern2Impl: Sized + IUIAutomationSelectionPatternImpl {
    fn CurrentFirstSelectedItem();
    fn CurrentLastSelectedItem();
    fn CurrentCurrentSelectedItem();
    fn CurrentItemCount();
    fn CachedFirstSelectedItem();
    fn CachedLastSelectedItem();
    fn CachedCurrentSelectedItem();
    fn CachedItemCount();
}
impl ::windows::core::RuntimeName for IUIAutomationSelectionPattern2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationSelectionPattern2";
}
impl IUIAutomationSelectionPattern2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>() -> IUIAutomationSelectionPattern2Vtbl {
        unsafe extern "system" fn CurrentFirstSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFirstSelectedItem(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLastSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLastSelectedItem(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCurrentSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCurrentSelectedItem(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentItemCount<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentItemCount(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFirstSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFirstSelectedItem(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedLastSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedLastSelectedItem(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCurrentSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCurrentSelectedItem(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedItemCount<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedItemCount(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationSelectionPattern2>,
            ::windows::core::GetTrustLevel,
            CurrentFirstSelectedItem::<Impl, OFFSET>,
            CurrentLastSelectedItem::<Impl, OFFSET>,
            CurrentCurrentSelectedItem::<Impl, OFFSET>,
            CurrentItemCount::<Impl, OFFSET>,
            CachedFirstSelectedItem::<Impl, OFFSET>,
            CachedLastSelectedItem::<Impl, OFFSET>,
            CachedCurrentSelectedItem::<Impl, OFFSET>,
            CachedItemCount::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationSpreadsheetItemPatternImpl: Sized {
    fn CurrentFormula();
    fn GetCurrentAnnotationObjects();
    fn GetCurrentAnnotationTypes();
    fn CachedFormula();
    fn GetCachedAnnotationObjects();
    fn GetCachedAnnotationTypes();
}
impl ::windows::core::RuntimeName for IUIAutomationSpreadsheetItemPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationSpreadsheetItemPattern";
}
impl IUIAutomationSpreadsheetItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>() -> IUIAutomationSpreadsheetItemPatternVtbl {
        unsafe extern "system" fn CurrentFormula<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFormula(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationObjects<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentAnnotationObjects(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationTypes<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentAnnotationTypes(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFormula<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFormula(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedAnnotationObjects<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedAnnotationObjects(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedAnnotationTypes<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedAnnotationTypes(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationSpreadsheetItemPattern>,
            ::windows::core::GetTrustLevel,
            CurrentFormula::<Impl, OFFSET>,
            GetCurrentAnnotationObjects::<Impl, OFFSET>,
            GetCurrentAnnotationTypes::<Impl, OFFSET>,
            CachedFormula::<Impl, OFFSET>,
            GetCachedAnnotationObjects::<Impl, OFFSET>,
            GetCachedAnnotationTypes::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationSpreadsheetPatternImpl: Sized {
    fn GetItemByName();
}
impl ::windows::core::RuntimeName for IUIAutomationSpreadsheetPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationSpreadsheetPattern";
}
impl IUIAutomationSpreadsheetPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetPatternImpl, const OFFSET: isize>() -> IUIAutomationSpreadsheetPatternVtbl {
        unsafe extern "system" fn GetItemByName<Impl: IUIAutomationSpreadsheetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemByName(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationSpreadsheetPattern>, ::windows::core::GetTrustLevel, GetItemByName::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationStructureChangedEventHandlerImpl: Sized {
    fn HandleStructureChangedEvent();
}
impl ::windows::core::RuntimeName for IUIAutomationStructureChangedEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationStructureChangedEventHandler";
}
impl IUIAutomationStructureChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStructureChangedEventHandlerImpl, const OFFSET: isize>() -> IUIAutomationStructureChangedEventHandlerVtbl {
        unsafe extern "system" fn HandleStructureChangedEvent<Impl: IUIAutomationStructureChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandleStructureChangedEvent(&*(&sender as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), changetype, &*(&runtimeid as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationStructureChangedEventHandler>, ::windows::core::GetTrustLevel, HandleStructureChangedEvent::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationStylesPatternImpl: Sized {
    fn CurrentStyleId();
    fn CurrentStyleName();
    fn CurrentFillColor();
    fn CurrentFillPatternStyle();
    fn CurrentShape();
    fn CurrentFillPatternColor();
    fn CurrentExtendedProperties();
    fn GetCurrentExtendedPropertiesAsArray();
    fn CachedStyleId();
    fn CachedStyleName();
    fn CachedFillColor();
    fn CachedFillPatternStyle();
    fn CachedShape();
    fn CachedFillPatternColor();
    fn CachedExtendedProperties();
    fn GetCachedExtendedPropertiesAsArray();
}
impl ::windows::core::RuntimeName for IUIAutomationStylesPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationStylesPattern";
}
impl IUIAutomationStylesPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>() -> IUIAutomationStylesPatternVtbl {
        unsafe extern "system" fn CurrentStyleId<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStyleId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStyleName<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStyleName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillColor<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFillColor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillPatternStyle<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFillPatternStyle(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentShape<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentShape(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFillPatternColor<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFillPatternColor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentExtendedProperties<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentExtendedProperties(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentExtendedPropertiesAsArray<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentExtendedPropertiesAsArray(::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedStyleId<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedStyleId(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedStyleName<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedStyleName(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillColor<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFillColor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillPatternStyle<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFillPatternStyle(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedShape<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedShape(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedFillPatternColor<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedFillPatternColor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedExtendedProperties<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedExtendedProperties(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedExtendedPropertiesAsArray<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedExtendedPropertiesAsArray(::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationStylesPattern>,
            ::windows::core::GetTrustLevel,
            CurrentStyleId::<Impl, OFFSET>,
            CurrentStyleName::<Impl, OFFSET>,
            CurrentFillColor::<Impl, OFFSET>,
            CurrentFillPatternStyle::<Impl, OFFSET>,
            CurrentShape::<Impl, OFFSET>,
            CurrentFillPatternColor::<Impl, OFFSET>,
            CurrentExtendedProperties::<Impl, OFFSET>,
            GetCurrentExtendedPropertiesAsArray::<Impl, OFFSET>,
            CachedStyleId::<Impl, OFFSET>,
            CachedStyleName::<Impl, OFFSET>,
            CachedFillColor::<Impl, OFFSET>,
            CachedFillPatternStyle::<Impl, OFFSET>,
            CachedShape::<Impl, OFFSET>,
            CachedFillPatternColor::<Impl, OFFSET>,
            CachedExtendedProperties::<Impl, OFFSET>,
            GetCachedExtendedPropertiesAsArray::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationSynchronizedInputPatternImpl: Sized {
    fn StartListening();
    fn Cancel();
}
impl ::windows::core::RuntimeName for IUIAutomationSynchronizedInputPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationSynchronizedInputPattern";
}
impl IUIAutomationSynchronizedInputPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSynchronizedInputPatternImpl, const OFFSET: isize>() -> IUIAutomationSynchronizedInputPatternVtbl {
        unsafe extern "system" fn StartListening<Impl: IUIAutomationSynchronizedInputPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartListening(inputtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IUIAutomationSynchronizedInputPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationSynchronizedInputPattern>, ::windows::core::GetTrustLevel, StartListening::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTableItemPatternImpl: Sized {
    fn GetCurrentRowHeaderItems();
    fn GetCurrentColumnHeaderItems();
    fn GetCachedRowHeaderItems();
    fn GetCachedColumnHeaderItems();
}
impl ::windows::core::RuntimeName for IUIAutomationTableItemPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTableItemPattern";
}
impl IUIAutomationTableItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTableItemPatternImpl, const OFFSET: isize>() -> IUIAutomationTableItemPatternVtbl {
        unsafe extern "system" fn GetCurrentRowHeaderItems<Impl: IUIAutomationTableItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentRowHeaderItems(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaderItems<Impl: IUIAutomationTableItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentColumnHeaderItems(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedRowHeaderItems<Impl: IUIAutomationTableItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedRowHeaderItems(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaderItems<Impl: IUIAutomationTableItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedColumnHeaderItems(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTableItemPattern>, ::windows::core::GetTrustLevel, GetCurrentRowHeaderItems::<Impl, OFFSET>, GetCurrentColumnHeaderItems::<Impl, OFFSET>, GetCachedRowHeaderItems::<Impl, OFFSET>, GetCachedColumnHeaderItems::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTablePatternImpl: Sized {
    fn GetCurrentRowHeaders();
    fn GetCurrentColumnHeaders();
    fn CurrentRowOrColumnMajor();
    fn GetCachedRowHeaders();
    fn GetCachedColumnHeaders();
    fn CachedRowOrColumnMajor();
}
impl ::windows::core::RuntimeName for IUIAutomationTablePattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTablePattern";
}
impl IUIAutomationTablePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>() -> IUIAutomationTablePatternVtbl {
        unsafe extern "system" fn GetCurrentRowHeaders<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentRowHeaders(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaders<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentColumnHeaders(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRowOrColumnMajor<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRowOrColumnMajor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedRowHeaders<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedRowHeaders(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaders<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCachedColumnHeaders(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedRowOrColumnMajor<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedRowOrColumnMajor(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationTablePattern>,
            ::windows::core::GetTrustLevel,
            GetCurrentRowHeaders::<Impl, OFFSET>,
            GetCurrentColumnHeaders::<Impl, OFFSET>,
            CurrentRowOrColumnMajor::<Impl, OFFSET>,
            GetCachedRowHeaders::<Impl, OFFSET>,
            GetCachedColumnHeaders::<Impl, OFFSET>,
            CachedRowOrColumnMajor::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationTextChildPatternImpl: Sized {
    fn TextContainer();
    fn TextRange();
}
impl ::windows::core::RuntimeName for IUIAutomationTextChildPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTextChildPattern";
}
impl IUIAutomationTextChildPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextChildPatternImpl, const OFFSET: isize>() -> IUIAutomationTextChildPatternVtbl {
        unsafe extern "system" fn TextContainer<Impl: IUIAutomationTextChildPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextContainer(::core::mem::transmute_copy(&container)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextRange<Impl: IUIAutomationTextChildPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextRange(::core::mem::transmute_copy(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTextChildPattern>, ::windows::core::GetTrustLevel, TextContainer::<Impl, OFFSET>, TextRange::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTextEditPatternImpl: Sized + IUIAutomationTextPatternImpl {
    fn GetActiveComposition();
    fn GetConversionTarget();
}
impl ::windows::core::RuntimeName for IUIAutomationTextEditPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTextEditPattern";
}
impl IUIAutomationTextEditPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditPatternImpl, const OFFSET: isize>() -> IUIAutomationTextEditPatternVtbl {
        unsafe extern "system" fn GetActiveComposition<Impl: IUIAutomationTextEditPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveComposition(::core::mem::transmute_copy(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConversionTarget<Impl: IUIAutomationTextEditPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConversionTarget(::core::mem::transmute_copy(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTextEditPattern>, ::windows::core::GetTrustLevel, GetActiveComposition::<Impl, OFFSET>, GetConversionTarget::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTextEditTextChangedEventHandlerImpl: Sized {
    fn HandleTextEditTextChangedEvent();
}
impl ::windows::core::RuntimeName for IUIAutomationTextEditTextChangedEventHandler {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTextEditTextChangedEventHandler";
}
impl IUIAutomationTextEditTextChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditTextChangedEventHandlerImpl, const OFFSET: isize>() -> IUIAutomationTextEditTextChangedEventHandlerVtbl {
        unsafe extern "system" fn HandleTextEditTextChangedEvent<Impl: IUIAutomationTextEditTextChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandleTextEditTextChangedEvent(&*(&sender as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), texteditchangetype, &*(&eventstrings as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTextEditTextChangedEventHandler>, ::windows::core::GetTrustLevel, HandleTextEditTextChangedEvent::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTextPatternImpl: Sized {
    fn RangeFromPoint();
    fn RangeFromChild();
    fn GetSelection();
    fn GetVisibleRanges();
    fn DocumentRange();
    fn SupportedTextSelection();
}
impl ::windows::core::RuntimeName for IUIAutomationTextPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTextPattern";
}
impl IUIAutomationTextPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>() -> IUIAutomationTextPatternVtbl {
        unsafe extern "system" fn RangeFromPoint<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromPoint(&*(&pt as *const <super::super::Foundation::POINT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::POINT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RangeFromChild<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, child: ::windows::core::RawPtr, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromChild(&*(&child as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelection<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(::core::mem::transmute_copy(&ranges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisibleRanges(::core::mem::transmute_copy(&ranges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentRange<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentRange(::core::mem::transmute_copy(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedtextselection: *mut SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedTextSelection(::core::mem::transmute_copy(&supportedtextselection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTextPattern>, ::windows::core::GetTrustLevel, RangeFromPoint::<Impl, OFFSET>, RangeFromChild::<Impl, OFFSET>, GetSelection::<Impl, OFFSET>, GetVisibleRanges::<Impl, OFFSET>, DocumentRange::<Impl, OFFSET>, SupportedTextSelection::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTextPattern2Impl: Sized + IUIAutomationTextPatternImpl {
    fn RangeFromAnnotation();
    fn GetCaretRange();
}
impl ::windows::core::RuntimeName for IUIAutomationTextPattern2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTextPattern2";
}
impl IUIAutomationTextPattern2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern2Impl, const OFFSET: isize>() -> IUIAutomationTextPattern2Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Impl: IUIAutomationTextPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotation: ::windows::core::RawPtr, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RangeFromAnnotation(&*(&annotation as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaretRange<Impl: IUIAutomationTextPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaretRange(::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTextPattern2>, ::windows::core::GetTrustLevel, RangeFromAnnotation::<Impl, OFFSET>, GetCaretRange::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTextRangeImpl: Sized {
    fn Clone();
    fn Compare();
    fn CompareEndpoints();
    fn ExpandToEnclosingUnit();
    fn FindAttribute();
    fn FindText();
    fn GetAttributeValue();
    fn GetBoundingRectangles();
    fn GetEnclosingElement();
    fn GetText();
    fn Move();
    fn MoveEndpointByUnit();
    fn MoveEndpointByRange();
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn ScrollIntoView();
    fn GetChildren();
}
impl ::windows::core::RuntimeName for IUIAutomationTextRange {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTextRange";
}
impl IUIAutomationTextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>() -> IUIAutomationTextRangeVtbl {
        unsafe extern "system" fn Clone<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&clonedrange)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&range as *const <IUIAutomationTextRange as ::windows::core::Abi>::Abi as *const <IUIAutomationTextRange as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&aresame)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareEndpoints<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint, compvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareEndpoints(srcendpoint, &*(&range as *const <IUIAutomationTextRange as ::windows::core::Abi>::Abi as *const <IUIAutomationTextRange as ::windows::core::DefaultType>::DefaultType), targetendpoint, ::core::mem::transmute_copy(&compvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textunit: TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandToEnclosingUnit(textunit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAttribute<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: i32, val: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, backward: super::super::Foundation::BOOL, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAttribute(attr, &*(&val as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&backward as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&found)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindText(
                &*(&text as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&backward as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&ignorecase as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&found),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValue<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: i32, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeValue(attr, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingrects: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingRectangles(::core::mem::transmute_copy(&boundingrects)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosingelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnclosingElement(::core::mem::transmute_copy(&enclosingelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(maxlength, ::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(unit, count, ::core::mem::transmute_copy(&moved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndpointByUnit(endpoint, unit, count, ::core::mem::transmute_copy(&moved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveEndpointByRange(srcendpoint, &*(&range as *const <IUIAutomationTextRange as ::windows::core::Abi>::Abi as *const <IUIAutomationTextRange as ::windows::core::DefaultType>::DefaultType), targetendpoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Select<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Select() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToSelection<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddToSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveFromSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollIntoView<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollIntoView(&*(&aligntotop as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren(::core::mem::transmute_copy(&children)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationTextRange>,
            ::windows::core::GetTrustLevel,
            Clone::<Impl, OFFSET>,
            Compare::<Impl, OFFSET>,
            CompareEndpoints::<Impl, OFFSET>,
            ExpandToEnclosingUnit::<Impl, OFFSET>,
            FindAttribute::<Impl, OFFSET>,
            FindText::<Impl, OFFSET>,
            GetAttributeValue::<Impl, OFFSET>,
            GetBoundingRectangles::<Impl, OFFSET>,
            GetEnclosingElement::<Impl, OFFSET>,
            GetText::<Impl, OFFSET>,
            Move::<Impl, OFFSET>,
            MoveEndpointByUnit::<Impl, OFFSET>,
            MoveEndpointByRange::<Impl, OFFSET>,
            Select::<Impl, OFFSET>,
            AddToSelection::<Impl, OFFSET>,
            RemoveFromSelection::<Impl, OFFSET>,
            ScrollIntoView::<Impl, OFFSET>,
            GetChildren::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationTextRange2Impl: Sized + IUIAutomationTextRangeImpl {
    fn ShowContextMenu();
}
impl ::windows::core::RuntimeName for IUIAutomationTextRange2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTextRange2";
}
impl IUIAutomationTextRange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange2Impl, const OFFSET: isize>() -> IUIAutomationTextRange2Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: IUIAutomationTextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowContextMenu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTextRange2>, ::windows::core::GetTrustLevel, ShowContextMenu::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTextRange3Impl: Sized + IUIAutomationTextRange2Impl + IUIAutomationTextRangeImpl {
    fn GetEnclosingElementBuildCache();
    fn GetChildrenBuildCache();
    fn GetAttributeValues();
}
impl ::windows::core::RuntimeName for IUIAutomationTextRange3 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTextRange3";
}
impl IUIAutomationTextRange3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange3Impl, const OFFSET: isize>() -> IUIAutomationTextRange3Vtbl {
        unsafe extern "system" fn GetEnclosingElementBuildCache<Impl: IUIAutomationTextRange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, enclosingelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnclosingElementBuildCache(&*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&enclosingelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenBuildCache<Impl: IUIAutomationTextRange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildrenBuildCache(&*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&children)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeValues<Impl: IUIAutomationTextRange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeids: *const i32, attributeidcount: i32, attributevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeValues(attributeids, attributeidcount, ::core::mem::transmute_copy(&attributevalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTextRange3>, ::windows::core::GetTrustLevel, GetEnclosingElementBuildCache::<Impl, OFFSET>, GetChildrenBuildCache::<Impl, OFFSET>, GetAttributeValues::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTextRangeArrayImpl: Sized {
    fn Length();
    fn GetElement();
}
impl ::windows::core::RuntimeName for IUIAutomationTextRangeArray {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTextRangeArray";
}
impl IUIAutomationTextRangeArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRangeArrayImpl, const OFFSET: isize>() -> IUIAutomationTextRangeArrayVtbl {
        unsafe extern "system" fn Length<Impl: IUIAutomationTextRangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<Impl: IUIAutomationTextRangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElement(index, ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTextRangeArray>, ::windows::core::GetTrustLevel, Length::<Impl, OFFSET>, GetElement::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTogglePatternImpl: Sized {
    fn Toggle();
    fn CurrentToggleState();
    fn CachedToggleState();
}
impl ::windows::core::RuntimeName for IUIAutomationTogglePattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTogglePattern";
}
impl IUIAutomationTogglePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTogglePatternImpl, const OFFSET: isize>() -> IUIAutomationTogglePatternVtbl {
        unsafe extern "system" fn Toggle<Impl: IUIAutomationTogglePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Toggle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentToggleState<Impl: IUIAutomationTogglePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentToggleState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedToggleState<Impl: IUIAutomationTogglePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedToggleState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationTogglePattern>, ::windows::core::GetTrustLevel, Toggle::<Impl, OFFSET>, CurrentToggleState::<Impl, OFFSET>, CachedToggleState::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationTransformPatternImpl: Sized {
    fn Move();
    fn Resize();
    fn Rotate();
    fn CurrentCanMove();
    fn CurrentCanResize();
    fn CurrentCanRotate();
    fn CachedCanMove();
    fn CachedCanResize();
    fn CachedCanRotate();
}
impl ::windows::core::RuntimeName for IUIAutomationTransformPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTransformPattern";
}
impl IUIAutomationTransformPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>() -> IUIAutomationTransformPatternVtbl {
        unsafe extern "system" fn Move<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Move(x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resize<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resize(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rotate<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotate(degrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanMove<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanMove(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanResize<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanResize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanRotate<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanRotate(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMove<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanMove(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanResize<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanResize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanRotate<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanRotate(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationTransformPattern>,
            ::windows::core::GetTrustLevel,
            Move::<Impl, OFFSET>,
            Resize::<Impl, OFFSET>,
            Rotate::<Impl, OFFSET>,
            CurrentCanMove::<Impl, OFFSET>,
            CurrentCanResize::<Impl, OFFSET>,
            CurrentCanRotate::<Impl, OFFSET>,
            CachedCanMove::<Impl, OFFSET>,
            CachedCanResize::<Impl, OFFSET>,
            CachedCanRotate::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationTransformPattern2Impl: Sized + IUIAutomationTransformPatternImpl {
    fn Zoom();
    fn ZoomByUnit();
    fn CurrentCanZoom();
    fn CachedCanZoom();
    fn CurrentZoomLevel();
    fn CachedZoomLevel();
    fn CurrentZoomMinimum();
    fn CachedZoomMinimum();
    fn CurrentZoomMaximum();
    fn CachedZoomMaximum();
}
impl ::windows::core::RuntimeName for IUIAutomationTransformPattern2 {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTransformPattern2";
}
impl IUIAutomationTransformPattern2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>() -> IUIAutomationTransformPattern2Vtbl {
        unsafe extern "system" fn Zoom<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Zoom(zoomvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomByUnit<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ZoomByUnit(zoomunit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanZoom<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanZoom(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanZoom<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanZoom(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomLevel<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentZoomLevel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomLevel<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedZoomLevel(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomMinimum<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentZoomMinimum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomMinimum<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedZoomMinimum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentZoomMaximum<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentZoomMaximum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedZoomMaximum<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedZoomMaximum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationTransformPattern2>,
            ::windows::core::GetTrustLevel,
            Zoom::<Impl, OFFSET>,
            ZoomByUnit::<Impl, OFFSET>,
            CurrentCanZoom::<Impl, OFFSET>,
            CachedCanZoom::<Impl, OFFSET>,
            CurrentZoomLevel::<Impl, OFFSET>,
            CachedZoomLevel::<Impl, OFFSET>,
            CurrentZoomMinimum::<Impl, OFFSET>,
            CachedZoomMinimum::<Impl, OFFSET>,
            CurrentZoomMaximum::<Impl, OFFSET>,
            CachedZoomMaximum::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationTreeWalkerImpl: Sized {
    fn GetParentElement();
    fn GetFirstChildElement();
    fn GetLastChildElement();
    fn GetNextSiblingElement();
    fn GetPreviousSiblingElement();
    fn NormalizeElement();
    fn GetParentElementBuildCache();
    fn GetFirstChildElementBuildCache();
    fn GetLastChildElementBuildCache();
    fn GetNextSiblingElementBuildCache();
    fn GetPreviousSiblingElementBuildCache();
    fn NormalizeElementBuildCache();
    fn Condition();
}
impl ::windows::core::RuntimeName for IUIAutomationTreeWalker {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationTreeWalker";
}
impl IUIAutomationTreeWalkerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>() -> IUIAutomationTreeWalkerVtbl {
        unsafe extern "system" fn GetParentElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentElement(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&parent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, first: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstChildElement(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&first)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastChildElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, last: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastChildElement(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&last)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextSiblingElement(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&next)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, previous: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousSiblingElement(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&previous)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizeElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, normalized: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizeElement(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&normalized)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParentElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParentElementBuildCache(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&parent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstChildElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, first: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstChildElementBuildCache(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&first)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastChildElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, last: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastChildElementBuildCache(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&last)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextSiblingElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextSiblingElementBuildCache(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&next)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, previous: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousSiblingElementBuildCache(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&previous)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NormalizeElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, normalized: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalizeElementBuildCache(&*(&element as *const <IUIAutomationElement as ::windows::core::Abi>::Abi as *const <IUIAutomationElement as ::windows::core::DefaultType>::DefaultType), &*(&cacherequest as *const <IUIAutomationCacheRequest as ::windows::core::Abi>::Abi as *const <IUIAutomationCacheRequest as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&normalized)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Condition<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Condition(::core::mem::transmute_copy(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationTreeWalker>,
            ::windows::core::GetTrustLevel,
            GetParentElement::<Impl, OFFSET>,
            GetFirstChildElement::<Impl, OFFSET>,
            GetLastChildElement::<Impl, OFFSET>,
            GetNextSiblingElement::<Impl, OFFSET>,
            GetPreviousSiblingElement::<Impl, OFFSET>,
            NormalizeElement::<Impl, OFFSET>,
            GetParentElementBuildCache::<Impl, OFFSET>,
            GetFirstChildElementBuildCache::<Impl, OFFSET>,
            GetLastChildElementBuildCache::<Impl, OFFSET>,
            GetNextSiblingElementBuildCache::<Impl, OFFSET>,
            GetPreviousSiblingElementBuildCache::<Impl, OFFSET>,
            NormalizeElementBuildCache::<Impl, OFFSET>,
            Condition::<Impl, OFFSET>,
        )
    }
}
pub trait IUIAutomationValuePatternImpl: Sized {
    fn SetValue();
    fn CurrentValue();
    fn CurrentIsReadOnly();
    fn CachedValue();
    fn CachedIsReadOnly();
}
impl ::windows::core::RuntimeName for IUIAutomationValuePattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationValuePattern";
}
impl IUIAutomationValuePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>() -> IUIAutomationValuePatternVtbl {
        unsafe extern "system" fn SetValue<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&val as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentValue<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentValue(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsReadOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedValue<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedValue(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsReadOnly(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationValuePattern>, ::windows::core::GetTrustLevel, SetValue::<Impl, OFFSET>, CurrentValue::<Impl, OFFSET>, CurrentIsReadOnly::<Impl, OFFSET>, CachedValue::<Impl, OFFSET>, CachedIsReadOnly::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationVirtualizedItemPatternImpl: Sized {
    fn Realize();
}
impl ::windows::core::RuntimeName for IUIAutomationVirtualizedItemPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationVirtualizedItemPattern";
}
impl IUIAutomationVirtualizedItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationVirtualizedItemPatternImpl, const OFFSET: isize>() -> IUIAutomationVirtualizedItemPatternVtbl {
        unsafe extern "system" fn Realize<Impl: IUIAutomationVirtualizedItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Realize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUIAutomationVirtualizedItemPattern>, ::windows::core::GetTrustLevel, Realize::<Impl, OFFSET>)
    }
}
pub trait IUIAutomationWindowPatternImpl: Sized {
    fn Close();
    fn WaitForInputIdle();
    fn SetWindowVisualState();
    fn CurrentCanMaximize();
    fn CurrentCanMinimize();
    fn CurrentIsModal();
    fn CurrentIsTopmost();
    fn CurrentWindowVisualState();
    fn CurrentWindowInteractionState();
    fn CachedCanMaximize();
    fn CachedCanMinimize();
    fn CachedIsModal();
    fn CachedIsTopmost();
    fn CachedWindowVisualState();
    fn CachedWindowInteractionState();
}
impl ::windows::core::RuntimeName for IUIAutomationWindowPattern {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IUIAutomationWindowPattern";
}
impl IUIAutomationWindowPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>() -> IUIAutomationWindowPatternVtbl {
        unsafe extern "system" fn Close<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForInputIdle<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, success: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForInputIdle(milliseconds, ::core::mem::transmute_copy(&success)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowVisualState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWindowVisualState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanMaximize<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanMaximize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCanMinimize<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCanMinimize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsModal<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsModal(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentIsTopmost<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentIsTopmost(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWindowVisualState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWindowVisualState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWindowInteractionState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWindowInteractionState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMaximize<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanMaximize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedCanMinimize<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedCanMinimize(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsModal<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsModal(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedIsTopmost<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedIsTopmost(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedWindowVisualState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedWindowVisualState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CachedWindowInteractionState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CachedWindowInteractionState(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUIAutomationWindowPattern>,
            ::windows::core::GetTrustLevel,
            Close::<Impl, OFFSET>,
            WaitForInputIdle::<Impl, OFFSET>,
            SetWindowVisualState::<Impl, OFFSET>,
            CurrentCanMaximize::<Impl, OFFSET>,
            CurrentCanMinimize::<Impl, OFFSET>,
            CurrentIsModal::<Impl, OFFSET>,
            CurrentIsTopmost::<Impl, OFFSET>,
            CurrentWindowVisualState::<Impl, OFFSET>,
            CurrentWindowInteractionState::<Impl, OFFSET>,
            CachedCanMaximize::<Impl, OFFSET>,
            CachedCanMinimize::<Impl, OFFSET>,
            CachedIsModal::<Impl, OFFSET>,
            CachedIsTopmost::<Impl, OFFSET>,
            CachedWindowVisualState::<Impl, OFFSET>,
            CachedWindowInteractionState::<Impl, OFFSET>,
        )
    }
}
pub trait IValueProviderImpl: Sized {
    fn SetValue();
    fn Value();
    fn IsReadOnly();
}
impl ::windows::core::RuntimeName for IValueProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IValueProvider";
}
impl IValueProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueProviderImpl, const OFFSET: isize>() -> IValueProviderVtbl {
        unsafe extern "system" fn SetValue<Impl: IValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&val as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: IValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IValueProvider>, ::windows::core::GetTrustLevel, SetValue::<Impl, OFFSET>, Value::<Impl, OFFSET>, IsReadOnly::<Impl, OFFSET>)
    }
}
pub trait IVirtualizedItemProviderImpl: Sized {
    fn Realize();
}
impl ::windows::core::RuntimeName for IVirtualizedItemProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IVirtualizedItemProvider";
}
impl IVirtualizedItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizedItemProviderImpl, const OFFSET: isize>() -> IVirtualizedItemProviderVtbl {
        unsafe extern "system" fn Realize<Impl: IVirtualizedItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Realize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVirtualizedItemProvider>, ::windows::core::GetTrustLevel, Realize::<Impl, OFFSET>)
    }
}
pub trait IWindowProviderImpl: Sized {
    fn SetVisualState();
    fn Close();
    fn WaitForInputIdle();
    fn CanMaximize();
    fn CanMinimize();
    fn IsModal();
    fn WindowVisualState();
    fn WindowInteractionState();
    fn IsTopmost();
}
impl ::windows::core::RuntimeName for IWindowProvider {
    const NAME: &'static str = "Windows.Win32.UI.Accessibility.IWindowProvider";
}
impl IWindowProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProviderImpl, const OFFSET: isize>() -> IWindowProviderVtbl {
        unsafe extern "system" fn SetVisualState<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVisualState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForInputIdle<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForInputIdle(milliseconds, ::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMaximize<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMaximize(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanMinimize<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMinimize(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsModal<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsModal(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowVisualState<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowVisualState(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowInteractionState<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowInteractionState(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTopmost<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTopmost(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWindowProvider>,
            ::windows::core::GetTrustLevel,
            SetVisualState::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            WaitForInputIdle::<Impl, OFFSET>,
            CanMaximize::<Impl, OFFSET>,
            CanMinimize::<Impl, OFFSET>,
            IsModal::<Impl, OFFSET>,
            WindowVisualState::<Impl, OFFSET>,
            WindowInteractionState::<Impl, OFFSET>,
            IsTopmost::<Impl, OFFSET>,
        )
    }
}
