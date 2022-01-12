pub trait IAccIdentityImpl: Sized {
    fn GetIdentityString();
}
impl IAccIdentityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccIdentityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccIdentityVtbl {
        unsafe extern "system" fn GetIdentityString<Impl: IAccIdentityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIdentityString: GetIdentityString::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccIdentity as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAccPropServerImpl: Sized {
    fn GetPropValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAccPropServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccPropServerVtbl {
        unsafe extern "system" fn GetPropValue<Impl: IAccPropServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetPropValue: GetPropValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccPropServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_WindowsAndMessaging"))]
impl IAccPropServicesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccPropServicesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccPropServicesVtbl {
        unsafe extern "system" fn SetPropValue<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPropServer<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearProps<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHwndProp<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHwndPropStr<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHwndPropServer<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearHwndProps<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComposeHwndIdentityString<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecomposeHwndIdentityString<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHmenuProp<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHmenuPropStr<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows::core::GUID, str: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHmenuPropServer<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32, pserver: ::windows::core::RawPtr, annoscope: AnnoScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearHmenuProps<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows::core::GUID, cprops: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComposeHmenuIdentityString<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DecomposeHmenuIdentityString<Impl: IAccPropServicesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAccessibleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleVtbl {
        unsafe extern "system" fn accParent<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdispparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accChildCount<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcountchildren: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accChild<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppdispchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accName<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accValue<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accDescription<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accRole<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarrole: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accState<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarstate: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accHelp<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accHelpTopic<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelpfile: *mut super::super::Foundation::BSTR, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pidtopic: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accKeyboardShortcut<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accFocus<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accSelection<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarchildren: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accDefaultAction<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accSelect<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accLocation<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accNavigate<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, navdir: i32, varstart: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarendupat: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accHitTest<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xleft: i32, ytop: i32, pvarchild: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn accDoDefaultAction<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetaccName<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, szname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetaccValue<Impl: IAccessibleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varchild: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, szvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IAccessibleExImpl: Sized {
    fn GetObjectForChild();
    fn GetIAccessiblePair();
    fn GetRuntimeId();
    fn ConvertReturnedElement();
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleExVtbl {
        unsafe extern "system" fn GetObjectForChild<Impl: IAccessibleExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idchild: i32, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIAccessiblePair<Impl: IAccessibleExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacc: *mut ::windows::core::RawPtr, pidchild: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRuntimeId<Impl: IAccessibleExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConvertReturnedElement<Impl: IAccessibleExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::windows::core::RawPtr, ppretvalout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAccessibleHandlerImpl: Sized {
    fn AccessibleObjectFromID();
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleHandlerVtbl {
        unsafe extern "system" fn AccessibleObjectFromID<Impl: IAccessibleHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, lobjectid: i32, piaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AccessibleObjectFromID: AccessibleObjectFromID::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibleHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleHostingElementProvidersImpl: Sized {
    fn GetEmbeddedFragmentRoots();
    fn GetObjectIdForProvider();
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleHostingElementProvidersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleHostingElementProvidersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleHostingElementProvidersVtbl {
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Impl: IAccessibleHostingElementProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectIdForProvider<Impl: IAccessibleHostingElementProvidersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, pidobject: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAccessibleWindowlessSiteImpl: Sized {
    fn AcquireObjectIdRange();
    fn ReleaseObjectIdRange();
    fn QueryObjectIdRanges();
    fn GetParentAccessible();
}
#[cfg(feature = "Win32_System_Com")]
impl IAccessibleWindowlessSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibleWindowlessSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibleWindowlessSiteVtbl {
        unsafe extern "system" fn AcquireObjectIdRange<Impl: IAccessibleWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangesize: i32, prangeowner: ::windows::core::RawPtr, prangebase: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseObjectIdRange<Impl: IAccessibleWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rangebase: i32, prangeowner: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryObjectIdRanges<Impl: IAccessibleWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangesowner: ::windows::core::RawPtr, psaranges: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentAccessible<Impl: IAccessibleWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IAnnotationProviderImpl: Sized {
    fn AnnotationTypeId();
    fn AnnotationTypeName();
    fn Author();
    fn DateTime();
    fn Target();
}
#[cfg(feature = "Win32_Foundation")]
impl IAnnotationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAnnotationProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAnnotationProviderVtbl {
        unsafe extern "system" fn AnnotationTypeId<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AnnotationTypeName<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Author<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DateTime<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Target<Impl: IAnnotationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ICustomNavigationProviderImpl: Sized {
    fn Navigate();
}
impl ICustomNavigationProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICustomNavigationProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICustomNavigationProviderVtbl {
        unsafe extern "system" fn Navigate<Impl: ICustomNavigationProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Navigate: Navigate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICustomNavigationProvider as ::windows::core::Interface>::IID
    }
}
pub trait IDockProviderImpl: Sized {
    fn SetDockPosition();
    fn DockPosition();
}
impl IDockProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDockProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDockProviderVtbl {
        unsafe extern "system" fn SetDockPosition<Impl: IDockProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockposition: DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DockPosition<Impl: IDockProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDragProviderImpl: Sized {
    fn IsGrabbed();
    fn DropEffect();
    fn DropEffects();
    fn GetGrabbedItems();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDragProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragProviderVtbl {
        unsafe extern "system" fn IsGrabbed<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DropEffect<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DropEffects<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGrabbedItems<Impl: IDragProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDropTargetProviderImpl: Sized {
    fn DropTargetEffect();
    fn DropTargetEffects();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IDropTargetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropTargetProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropTargetProviderVtbl {
        unsafe extern "system" fn DropTargetEffect<Impl: IDropTargetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DropTargetEffects<Impl: IDropTargetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IExpandCollapseProviderImpl: Sized {
    fn Expand();
    fn Collapse();
    fn ExpandCollapseState();
}
impl IExpandCollapseProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExpandCollapseProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExpandCollapseProviderVtbl {
        unsafe extern "system" fn Expand<Impl: IExpandCollapseProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Collapse<Impl: IExpandCollapseProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpandCollapseState<Impl: IExpandCollapseProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IGridItemProviderImpl: Sized {
    fn Row();
    fn Column();
    fn RowSpan();
    fn ColumnSpan();
    fn ContainingGrid();
}
impl IGridItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridItemProviderVtbl {
        unsafe extern "system" fn Row<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Column<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RowSpan<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ColumnSpan<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContainingGrid<Impl: IGridItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IGridProviderImpl: Sized {
    fn GetItem();
    fn RowCount();
    fn ColumnCount();
}
impl IGridProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridProviderVtbl {
        unsafe extern "system" fn GetItem<Impl: IGridProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RowCount<Impl: IGridProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ColumnCount<Impl: IGridProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IInvokeProviderImpl: Sized {
    fn Invoke();
}
impl IInvokeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInvokeProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInvokeProviderVtbl {
        unsafe extern "system" fn Invoke<Impl: IInvokeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInvokeProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IItemContainerProviderImpl: Sized {
    fn FindItemByProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IItemContainerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemContainerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemContainerProviderVtbl {
        unsafe extern "system" fn FindItemByProperty<Impl: IItemContainerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: ::windows::core::RawPtr, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FindItemByProperty: FindItemByProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemContainerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ILegacyIAccessibleProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILegacyIAccessibleProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILegacyIAccessibleProviderVtbl {
        unsafe extern "system" fn Select<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoDefaultAction<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIAccessible<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChildId<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Role<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn State<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Help<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KeyboardShortcut<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelection<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DefaultAction<Impl: ILegacyIAccessibleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IMultipleViewProviderImpl: Sized {
    fn GetViewName();
    fn SetCurrentView();
    fn CurrentView();
    fn GetSupportedViews();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IMultipleViewProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultipleViewProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultipleViewProviderVtbl {
        unsafe extern "system" fn GetViewName<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentView<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentView<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedViews<Impl: IMultipleViewProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IObjectModelProviderImpl: Sized {
    fn GetUnderlyingObjectModel();
}
impl IObjectModelProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectModelProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectModelProviderVtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Impl: IObjectModelProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectModelProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProxyProviderWinEventHandlerImpl: Sized {
    fn RespondToWinEvent();
}
#[cfg(feature = "Win32_Foundation")]
impl IProxyProviderWinEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProxyProviderWinEventHandlerVtbl {
        unsafe extern "system" fn RespondToWinEvent<Impl: IProxyProviderWinEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), RespondToWinEvent: RespondToWinEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProxyProviderWinEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProxyProviderWinEventSinkImpl: Sized {
    fn AddAutomationPropertyChangedEvent();
    fn AddAutomationEvent();
    fn AddStructureChangedEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProxyProviderWinEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProxyProviderWinEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProxyProviderWinEventSinkVtbl {
        unsafe extern "system" fn AddAutomationPropertyChangedEvent<Impl: IProxyProviderWinEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, id: i32, newvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAutomationEvent<Impl: IProxyProviderWinEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStructureChangedEvent<Impl: IProxyProviderWinEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprovider: ::windows::core::RawPtr, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRangeValueProviderImpl: Sized {
    fn SetValue();
    fn Value();
    fn IsReadOnly();
    fn Maximum();
    fn Minimum();
    fn LargeChange();
    fn SmallChange();
}
#[cfg(feature = "Win32_Foundation")]
impl IRangeValueProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeValueProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeValueProviderVtbl {
        unsafe extern "system" fn SetValue<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsReadOnly<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Maximum<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Minimum<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LargeChange<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SmallChange<Impl: IRangeValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRawElementProviderAdviseEventsImpl: Sized {
    fn AdviseEventAdded();
    fn AdviseEventRemoved();
}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderAdviseEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderAdviseEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderAdviseEventsVtbl {
        unsafe extern "system" fn AdviseEventAdded<Impl: IRawElementProviderAdviseEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdviseEventRemoved<Impl: IRawElementProviderAdviseEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRawElementProviderFragmentImpl: Sized {
    fn Navigate();
    fn GetRuntimeId();
    fn BoundingRectangle();
    fn GetEmbeddedFragmentRoots();
    fn SetFocus();
    fn FragmentRoot();
}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderFragmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderFragmentVtbl {
        unsafe extern "system" fn Navigate<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRuntimeId<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BoundingRectangle<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut UiaRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFocus<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FragmentRoot<Impl: IRawElementProviderFragmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRawElementProviderFragmentRootImpl: Sized {
    fn ElementProviderFromPoint();
    fn GetFocus();
}
impl IRawElementProviderFragmentRootVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderFragmentRootImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderFragmentRootVtbl {
        unsafe extern "system" fn ElementProviderFromPoint<Impl: IRawElementProviderFragmentRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFocus<Impl: IRawElementProviderFragmentRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRawElementProviderHostingAccessiblesImpl: Sized {
    fn GetEmbeddedAccessibles();
}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderHostingAccessiblesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHostingAccessiblesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderHostingAccessiblesVtbl {
        unsafe extern "system" fn GetEmbeddedAccessibles<Impl: IRawElementProviderHostingAccessiblesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetEmbeddedAccessibles: GetEmbeddedAccessibles::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderHostingAccessibles as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRawElementProviderHwndOverrideImpl: Sized {
    fn GetOverrideProviderForHwnd();
}
#[cfg(feature = "Win32_Foundation")]
impl IRawElementProviderHwndOverrideVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderHwndOverrideImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderHwndOverrideVtbl {
        unsafe extern "system" fn GetOverrideProviderForHwnd<Impl: IRawElementProviderHwndOverrideImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOverrideProviderForHwnd: GetOverrideProviderForHwnd::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderHwndOverride as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawElementProviderSimpleImpl: Sized {
    fn ProviderOptions();
    fn GetPatternProvider();
    fn GetPropertyValue();
    fn HostRawElementProvider();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawElementProviderSimpleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimpleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderSimpleVtbl {
        unsafe extern "system" fn ProviderOptions<Impl: IRawElementProviderSimpleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ProviderOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPatternProvider<Impl: IRawElementProviderSimpleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyValue<Impl: IRawElementProviderSimpleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HostRawElementProvider<Impl: IRawElementProviderSimpleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRawElementProviderSimple2Impl: Sized + IRawElementProviderSimpleImpl {
    fn ShowContextMenu();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawElementProviderSimple2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderSimple2Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: IRawElementProviderSimple2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IRawElementProviderSimpleVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ShowContextMenu: ShowContextMenu::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderSimple2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawElementProviderSimple3Impl: Sized + IRawElementProviderSimpleImpl + IRawElementProviderSimple2Impl {
    fn GetMetadataValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawElementProviderSimple3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderSimple3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderSimple3Vtbl {
        unsafe extern "system" fn GetMetadataValue<Impl: IRawElementProviderSimple3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: i32, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IRawElementProviderSimple2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetMetadataValue: GetMetadataValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawElementProviderSimple3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderWindowlessSiteImpl: Sized {
    fn GetAdjacentFragment();
    fn GetRuntimeIdPrefix();
}
#[cfg(feature = "Win32_System_Com")]
impl IRawElementProviderWindowlessSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawElementProviderWindowlessSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawElementProviderWindowlessSiteVtbl {
        unsafe extern "system" fn GetAdjacentFragment<Impl: IRawElementProviderWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRuntimeIdPrefix<Impl: IRawElementProviderWindowlessSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRichEditUiaInformationImpl: Sized {
    fn GetBoundaryRectangle();
    fn IsVisible();
}
impl IRichEditUiaInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditUiaInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditUiaInformationVtbl {
        unsafe extern "system" fn GetBoundaryRectangle<Impl: IRichEditUiaInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puiarect: *mut UiaRect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsVisible<Impl: IRichEditUiaInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IRicheditWindowlessAccessibilityImpl: Sized {
    fn CreateProvider();
}
impl IRicheditWindowlessAccessibilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRicheditWindowlessAccessibilityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRicheditWindowlessAccessibilityVtbl {
        unsafe extern "system" fn CreateProvider<Impl: IRicheditWindowlessAccessibilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psite: ::windows::core::RawPtr, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateProvider: CreateProvider::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRicheditWindowlessAccessibility as ::windows::core::Interface>::IID
    }
}
pub trait IScrollItemProviderImpl: Sized {
    fn ScrollIntoView();
}
impl IScrollItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollItemProviderVtbl {
        unsafe extern "system" fn ScrollIntoView<Impl: IScrollItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ScrollIntoView: ScrollIntoView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollItemProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IScrollProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollProviderVtbl {
        unsafe extern "system" fn Scroll<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScrollPercent<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HorizontalScrollPercent<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VerticalScrollPercent<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HorizontalViewSize<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VerticalViewSize<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HorizontallyScrollable<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VerticallyScrollable<Impl: IScrollProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ISelectionItemProviderImpl: Sized {
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn IsSelected();
    fn SelectionContainer();
}
#[cfg(feature = "Win32_Foundation")]
impl ISelectionItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionItemProviderVtbl {
        unsafe extern "system" fn Select<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddToSelection<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSelected<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SelectionContainer<Impl: ISelectionItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ISelectionProviderImpl: Sized {
    fn GetSelection();
    fn CanSelectMultiple();
    fn IsSelectionRequired();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISelectionProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionProviderVtbl {
        unsafe extern "system" fn GetSelection<Impl: ISelectionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanSelectMultiple<Impl: ISelectionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSelectionRequired<Impl: ISelectionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ISelectionProvider2Impl: Sized + ISelectionProviderImpl {
    fn FirstSelectedItem();
    fn LastSelectedItem();
    fn CurrentSelectedItem();
    fn ItemCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISelectionProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectionProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectionProvider2Vtbl {
        unsafe extern "system" fn FirstSelectedItem<Impl: ISelectionProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastSelectedItem<Impl: ISelectionProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentSelectedItem<Impl: ISelectionProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ItemCount<Impl: ISelectionProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ISelectionProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait ISpreadsheetItemProviderImpl: Sized {
    fn Formula();
    fn GetAnnotationObjects();
    fn GetAnnotationTypes();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpreadsheetItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetItemProviderVtbl {
        unsafe extern "system" fn Formula<Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnnotationObjects<Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAnnotationTypes<Impl: ISpreadsheetItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ISpreadsheetProviderImpl: Sized {
    fn GetItemByName();
}
#[cfg(feature = "Win32_Foundation")]
impl ISpreadsheetProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpreadsheetProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpreadsheetProviderVtbl {
        unsafe extern "system" fn GetItemByName<Impl: ISpreadsheetProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetItemByName: GetItemByName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpreadsheetProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStylesProviderImpl: Sized {
    fn StyleId();
    fn StyleName();
    fn FillColor();
    fn FillPatternStyle();
    fn Shape();
    fn FillPatternColor();
    fn ExtendedProperties();
}
#[cfg(feature = "Win32_Foundation")]
impl IStylesProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStylesProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStylesProviderVtbl {
        unsafe extern "system" fn StyleId<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StyleName<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillColor<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillPatternStyle<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shape<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FillPatternColor<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtendedProperties<Impl: IStylesProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ISynchronizedInputProviderImpl: Sized {
    fn StartListening();
    fn Cancel();
}
impl ISynchronizedInputProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizedInputProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronizedInputProviderVtbl {
        unsafe extern "system" fn StartListening<Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: ISynchronizedInputProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITableItemProviderImpl: Sized {
    fn GetRowHeaderItems();
    fn GetColumnHeaderItems();
}
#[cfg(feature = "Win32_System_Com")]
impl ITableItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableItemProviderVtbl {
        unsafe extern "system" fn GetRowHeaderItems<Impl: ITableItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnHeaderItems<Impl: ITableItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITableProviderImpl: Sized {
    fn GetRowHeaders();
    fn GetColumnHeaders();
    fn RowOrColumnMajor();
}
#[cfg(feature = "Win32_System_Com")]
impl ITableProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITableProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITableProviderVtbl {
        unsafe extern "system" fn GetRowHeaders<Impl: ITableProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnHeaders<Impl: ITableProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RowOrColumnMajor<Impl: ITableProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITextChildProviderImpl: Sized {
    fn TextContainer();
    fn TextRange();
}
impl ITextChildProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextChildProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextChildProviderVtbl {
        unsafe extern "system" fn TextContainer<Impl: ITextChildProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TextRange<Impl: ITextChildProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITextEditProviderImpl: Sized + ITextProviderImpl {
    fn GetActiveComposition();
    fn GetConversionTarget();
}
#[cfg(feature = "Win32_System_Com")]
impl ITextEditProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextEditProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextEditProviderVtbl {
        unsafe extern "system" fn GetActiveComposition<Impl: ITextEditProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionTarget<Impl: ITextEditProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ITextProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Impl, IMPL_OFFSET>,
            GetConversionTarget: GetConversionTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextEditProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextProviderImpl: Sized {
    fn GetSelection();
    fn GetVisibleRanges();
    fn RangeFromChild();
    fn RangeFromPoint();
    fn DocumentRange();
    fn SupportedTextSelection();
}
#[cfg(feature = "Win32_System_Com")]
impl ITextProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextProviderVtbl {
        unsafe extern "system" fn GetSelection<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVisibleRanges<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RangeFromChild<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childelement: ::windows::core::RawPtr, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RangeFromPoint<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: UiaPoint, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DocumentRange<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedTextSelection<Impl: ITextProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITextProvider2Impl: Sized + ITextProviderImpl {
    fn RangeFromAnnotation();
    fn GetCaretRange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ITextProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextProvider2Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Impl: ITextProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotationelement: ::windows::core::RawPtr, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaretRange<Impl: ITextProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ITextProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Impl, IMPL_OFFSET>,
            GetCaretRange: GetCaretRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextProvider2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRangeProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRangeProviderVtbl {
        unsafe extern "system" fn Clone<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Compare<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareEndpoints<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindAttribute<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, val: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, backward: super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindText<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeValue<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeid: i32, pretval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoundingRectangles<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnclosingElement<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetText<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveEndpointByUnit<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveEndpointByRange<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Select<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddToSelection<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScrollIntoView<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChildren<Impl: ITextRangeProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITextRangeProvider2Impl: Sized + ITextRangeProviderImpl {
    fn ShowContextMenu();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITextRangeProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextRangeProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextRangeProvider2Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: ITextRangeProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ITextRangeProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ShowContextMenu: ShowContextMenu::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextRangeProvider2 as ::windows::core::Interface>::IID
    }
}
pub trait IToggleProviderImpl: Sized {
    fn Toggle();
    fn ToggleState();
}
impl IToggleProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleProviderVtbl {
        unsafe extern "system" fn Toggle<Impl: IToggleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ToggleState<Impl: IToggleProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransformProviderImpl: Sized {
    fn Move();
    fn Resize();
    fn Rotate();
    fn CanMove();
    fn CanResize();
    fn CanRotate();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransformProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformProviderVtbl {
        unsafe extern "system" fn Move<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resize<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rotate<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanMove<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanResize<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanRotate<Impl: ITransformProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait ITransformProvider2Impl: Sized + ITransformProviderImpl {
    fn Zoom();
    fn CanZoom();
    fn ZoomLevel();
    fn ZoomMinimum();
    fn ZoomMaximum();
    fn ZoomByUnit();
}
#[cfg(feature = "Win32_Foundation")]
impl ITransformProvider2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransformProvider2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransformProvider2Vtbl {
        unsafe extern "system" fn Zoom<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanZoom<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ZoomLevel<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ZoomMinimum<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ZoomMaximum<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ZoomByUnit<Impl: ITransformProvider2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ITransformProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationVtbl {
        unsafe extern "system" fn CompareElements<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, el1: ::windows::core::RawPtr, el2: ::windows::core::RawPtr, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareRuntimeIds<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRootElement<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElementFromHandle<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElementFromPoint<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFocusedElement<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRootElementBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, root: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElementFromHandleBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElementFromPointBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFocusedElementBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTreeWalker<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcondition: ::windows::core::RawPtr, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ControlViewWalker<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContentViewWalker<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawViewWalker<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, walker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RawViewCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ControlViewCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContentViewCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateCacheRequest<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTrueCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFalseCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePropertyCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePropertyConditionEx<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, flags: PropertyConditionFlags, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAndCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: ::windows::core::RawPtr, condition2: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAndConditionFromArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAndConditionFromNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const ::windows::core::RawPtr, conditioncount: i32, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateOrCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition1: ::windows::core::RawPtr, condition2: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateOrConditionFromArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateOrConditionFromNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, conditions: *const ::windows::core::RawPtr, conditioncount: i32, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNotCondition<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: ::windows::core::RawPtr, newcondition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAutomationEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAutomationEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandlerNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const i32, propertycount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemovePropertyChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStructureChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFocusChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFocusChangedEventHandler<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllEventHandlers<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IntNativeArrayToSafeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, array: *const i32, arraycount: i32, safearray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IntSafeArrayToNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RectToVariant<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, var: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VariantToRect<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, rc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SafeArrayToRectNativeArray<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateProxyFactoryEntry<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, factoryentry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProxyFactoryMapping<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorymapping: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyProgrammaticName<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPatternProgrammaticName<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PollForPotentialSupportedPatterns<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PollForPotentialSupportedProperties<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pelement: ::windows::core::RawPtr, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckNotSupported<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, isnotsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReservedNotSupportedValue<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notsupportedvalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReservedMixedAttributeValue<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mixedattributevalue: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElementFromIAccessible<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: ::windows::core::RawPtr, childid: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElementFromIAccessibleBuildCache<Impl: IUIAutomationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accessible: ::windows::core::RawPtr, childid: i32, cacherequest: ::windows::core::RawPtr, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomation2Impl: Sized + IUIAutomationImpl {
    fn AutoSetFocus();
    fn SetAutoSetFocus();
    fn ConnectionTimeout();
    fn SetConnectionTimeout();
    fn TransactionTimeout();
    fn SetTransactionTimeout();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation2Vtbl {
        unsafe extern "system" fn AutoSetFocus<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoSetFocus<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autosetfocus: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectionTimeout<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectionTimeout<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionTimeout<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransactionTimeout<Impl: IUIAutomation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IUIAutomation3Impl: Sized + IUIAutomationImpl + IUIAutomation2Impl {
    fn AddTextEditTextChangedEventHandler();
    fn RemoveTextEditTextChangedEventHandler();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation3Vtbl {
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Impl: IUIAutomation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveTextEditTextChangedEventHandler<Impl: IUIAutomation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomation2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Impl, IMPL_OFFSET>,
            RemoveTextEditTextChangedEventHandler: RemoveTextEditTextChangedEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation4Impl: Sized + IUIAutomationImpl + IUIAutomation2Impl + IUIAutomation3Impl {
    fn AddChangesEventHandler();
    fn RemoveChangesEventHandler();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation4Vtbl {
        unsafe extern "system" fn AddChangesEventHandler<Impl: IUIAutomation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveChangesEventHandler<Impl: IUIAutomation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomation3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddChangesEventHandler: AddChangesEventHandler::<Impl, IMPL_OFFSET>,
            RemoveChangesEventHandler: RemoveChangesEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation5Impl: Sized + IUIAutomationImpl + IUIAutomation2Impl + IUIAutomation3Impl + IUIAutomation4Impl {
    fn AddNotificationEventHandler();
    fn RemoveNotificationEventHandler();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation5Vtbl {
        unsafe extern "system" fn AddNotificationEventHandler<Impl: IUIAutomation5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveNotificationEventHandler<Impl: IUIAutomation5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomation4Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddNotificationEventHandler: AddNotificationEventHandler::<Impl, IMPL_OFFSET>,
            RemoveNotificationEventHandler: RemoveNotificationEventHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomation5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomation6Impl: Sized + IUIAutomationImpl + IUIAutomation2Impl + IUIAutomation3Impl + IUIAutomation4Impl + IUIAutomation5Impl {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomation6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomation6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomation6Vtbl {
        unsafe extern "system" fn CreateEventHandlerGroup<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handlergroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEventHandlerGroup<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handlergroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEventHandlerGroup<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handlergroup: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ConnectionRecoveryBehavior<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: *mut ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectionRecoveryBehavior<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CoalesceEvents<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: *mut CoalesceEventsOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCoalesceEvents<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveActiveTextPositionChangedEventHandler<Impl: IUIAutomation6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomation5Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IUIAutomationActiveTextPositionChangedEventHandlerImpl: Sized {
    fn HandleActiveTextPositionChangedEvent();
}
impl IUIAutomationActiveTextPositionChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationActiveTextPositionChangedEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationActiveTextPositionChangedEventHandlerVtbl {
        unsafe extern "system" fn HandleActiveTextPositionChangedEvent<Impl: IUIAutomationActiveTextPositionChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, range: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationAndConditionImpl: Sized + IUIAutomationConditionImpl {
    fn ChildCount();
    fn GetChildrenAsNativeArray();
    fn GetChildren();
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationAndConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAndConditionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationAndConditionVtbl {
        unsafe extern "system" fn ChildCount<Impl: IUIAutomationAndConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Impl: IUIAutomationAndConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::windows::core::RawPtr, childarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChildren<Impl: IUIAutomationAndConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationConditionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationAnnotationPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationAnnotationPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationAnnotationPatternVtbl {
        unsafe extern "system" fn CurrentAnnotationTypeId<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAnnotationTypeName<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAuthor<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentDateTime<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentTarget<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAnnotationTypeId<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAnnotationTypeName<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAuthor<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedDateTime<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedTarget<Impl: IUIAutomationAnnotationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationBoolConditionImpl: Sized + IUIAutomationConditionImpl {
    fn BooleanValue();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationBoolConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationBoolConditionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationBoolConditionVtbl {
        unsafe extern "system" fn BooleanValue<Impl: IUIAutomationBoolConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boolval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IUIAutomationConditionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), BooleanValue: BooleanValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationBoolCondition as ::windows::core::Interface>::IID
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
impl IUIAutomationCacheRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCacheRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationCacheRequestVtbl {
        unsafe extern "system" fn AddProperty<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPattern<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TreeScope<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: *mut TreeScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTreeScope<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TreeFilter<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTreeFilter<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AutomationElementMode<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: *mut AutomationElementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutomationElementMode<Impl: IUIAutomationCacheRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: AutomationElementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationChangesEventHandlerImpl: Sized {
    fn HandleChangesEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationChangesEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationChangesEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationChangesEventHandlerVtbl {
        unsafe extern "system" fn HandleChangesEvent<Impl: IUIAutomationChangesEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleChangesEvent: HandleChangesEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationChangesEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationConditionImpl: Sized {}
impl IUIAutomationConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationConditionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationConditionVtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationCondition as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationCustomNavigationPatternImpl: Sized {
    fn Navigate();
}
impl IUIAutomationCustomNavigationPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationCustomNavigationPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationCustomNavigationPatternVtbl {
        unsafe extern "system" fn Navigate<Impl: IUIAutomationCustomNavigationPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Navigate: Navigate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationCustomNavigationPattern as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationDockPatternImpl: Sized {
    fn SetDockPosition();
    fn CurrentDockPosition();
    fn CachedDockPosition();
}
impl IUIAutomationDockPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDockPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationDockPatternVtbl {
        unsafe extern "system" fn SetDockPosition<Impl: IUIAutomationDockPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dockpos: DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentDockPosition<Impl: IUIAutomationDockPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedDockPosition<Impl: IUIAutomationDockPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationDragPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDragPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationDragPatternVtbl {
        unsafe extern "system" fn CurrentIsGrabbed<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsGrabbed<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentDropEffect<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedDropEffect<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentDropEffects<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedDropEffects<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentGrabbedItems<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedGrabbedItems<Impl: IUIAutomationDragPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationDropTargetPatternImpl: Sized {
    fn CurrentDropTargetEffect();
    fn CachedDropTargetEffect();
    fn CurrentDropTargetEffects();
    fn CachedDropTargetEffects();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationDropTargetPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationDropTargetPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationDropTargetPatternVtbl {
        unsafe extern "system" fn CurrentDropTargetEffect<Impl: IUIAutomationDropTargetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedDropTargetEffect<Impl: IUIAutomationDropTargetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentDropTargetEffects<Impl: IUIAutomationDropTargetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedDropTargetEffects<Impl: IUIAutomationDropTargetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElementVtbl {
        unsafe extern "system" fn SetFocus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRuntimeId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirst<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindAll<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstBuildCache<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindAllBuildCache<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BuildUpdatedCache<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, updatedelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPropertyValue<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPropertyValueEx<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedPropertyValue<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedPropertyValueEx<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: i32, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPatternAs<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedPatternAs<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, riid: *const ::windows::core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPattern<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedPattern<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patternid: i32, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedParent<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedChildren<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentProcessId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentControlType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentLocalizedControlType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentName<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAcceleratorKey<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAccessKey<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentHasKeyboardFocus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsKeyboardFocusable<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsEnabled<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAutomationId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentClassName<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentHelpText<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCulture<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsControlElement<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsContentElement<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsPassword<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentNativeWindowHandle<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentItemType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsOffscreen<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentOrientation<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentFrameworkId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsRequiredForForm<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentItemStatus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentBoundingRectangle<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentLabeledBy<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAriaRole<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAriaProperties<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsDataValidForForm<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentControllerFor<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentDescribedBy<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentFlowsTo<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentProviderDescription<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedProcessId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedControlType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedLocalizedControlType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedName<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAcceleratorKey<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAccessKey<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedHasKeyboardFocus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsKeyboardFocusable<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsEnabled<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAutomationId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedClassName<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedHelpText<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCulture<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsControlElement<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsContentElement<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsPassword<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedNativeWindowHandle<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedItemType<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsOffscreen<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedOrientation<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedFrameworkId<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsRequiredForForm<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedItemStatus<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedBoundingRectangle<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedLabeledBy<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAriaRole<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAriaProperties<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsDataValidForForm<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedControllerFor<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedDescribedBy<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedFlowsTo<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedProviderDescription<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClickablePoint<Impl: IUIAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationElement2Impl: Sized + IUIAutomationElementImpl {
    fn CurrentOptimizeForVisualContent();
    fn CachedOptimizeForVisualContent();
    fn CurrentLiveSetting();
    fn CachedLiveSetting();
    fn CurrentFlowsFrom();
    fn CachedFlowsFrom();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement2Vtbl {
        unsafe extern "system" fn CurrentOptimizeForVisualContent<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedOptimizeForVisualContent<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentLiveSetting<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedLiveSetting<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentFlowsFrom<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedFlowsFrom<Impl: IUIAutomationElement2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationElementVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IUIAutomationElement3Impl: Sized + IUIAutomationElementImpl + IUIAutomationElement2Impl {
    fn ShowContextMenu();
    fn CurrentIsPeripheral();
    fn CachedIsPeripheral();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement3Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: IUIAutomationElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsPeripheral<Impl: IUIAutomationElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsPeripheral<Impl: IUIAutomationElement3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationElement2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IUIAutomationElement4Impl: Sized + IUIAutomationElementImpl + IUIAutomationElement2Impl + IUIAutomationElement3Impl {
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement4Vtbl {
        unsafe extern "system" fn CurrentPositionInSet<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentSizeOfSet<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentLevel<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAnnotationTypes<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAnnotationObjects<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedPositionInSet<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedSizeOfSet<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedLevel<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAnnotationTypes<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedAnnotationObjects<Impl: IUIAutomationElement4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationElement3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IUIAutomationElement5Impl: Sized + IUIAutomationElementImpl + IUIAutomationElement2Impl + IUIAutomationElement3Impl + IUIAutomationElement4Impl {
    fn CurrentLandmarkType();
    fn CurrentLocalizedLandmarkType();
    fn CachedLandmarkType();
    fn CachedLocalizedLandmarkType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement5Vtbl {
        unsafe extern "system" fn CurrentLandmarkType<Impl: IUIAutomationElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentLocalizedLandmarkType<Impl: IUIAutomationElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedLandmarkType<Impl: IUIAutomationElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedLocalizedLandmarkType<Impl: IUIAutomationElement5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationElement4Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IUIAutomationElement6Impl: Sized + IUIAutomationElementImpl + IUIAutomationElement2Impl + IUIAutomationElement3Impl + IUIAutomationElement4Impl + IUIAutomationElement5Impl {
    fn CurrentFullDescription();
    fn CachedFullDescription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement6Vtbl {
        unsafe extern "system" fn CurrentFullDescription<Impl: IUIAutomationElement6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedFullDescription<Impl: IUIAutomationElement6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationElement5Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentFullDescription: CurrentFullDescription::<Impl, IMPL_OFFSET>,
            CachedFullDescription: CachedFullDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement7Impl: Sized + IUIAutomationElementImpl + IUIAutomationElement2Impl + IUIAutomationElement3Impl + IUIAutomationElement4Impl + IUIAutomationElement5Impl + IUIAutomationElement6Impl {
    fn FindFirstWithOptions();
    fn FindAllWithOptions();
    fn FindFirstWithOptionsBuildCache();
    fn FindAllWithOptionsBuildCache();
    fn GetCurrentMetadataValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement7Vtbl {
        unsafe extern "system" fn FindFirstWithOptions<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindAllWithOptions<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindFirstWithOptionsBuildCache<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindAllWithOptionsBuildCache<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, traversaloptions: TreeTraversalOptions, root: ::windows::core::RawPtr, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentMetadataValue<Impl: IUIAutomationElement7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: i32, returnval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationElement6Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IUIAutomationElement8Impl: Sized + IUIAutomationElementImpl + IUIAutomationElement2Impl + IUIAutomationElement3Impl + IUIAutomationElement4Impl + IUIAutomationElement5Impl + IUIAutomationElement6Impl + IUIAutomationElement7Impl {
    fn CurrentHeadingLevel();
    fn CachedHeadingLevel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement8Vtbl {
        unsafe extern "system" fn CurrentHeadingLevel<Impl: IUIAutomationElement8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedHeadingLevel<Impl: IUIAutomationElement8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationElement7Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentHeadingLevel: CurrentHeadingLevel::<Impl, IMPL_OFFSET>,
            CachedHeadingLevel: CachedHeadingLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationElement9Impl: Sized + IUIAutomationElementImpl + IUIAutomationElement2Impl + IUIAutomationElement3Impl + IUIAutomationElement4Impl + IUIAutomationElement5Impl + IUIAutomationElement6Impl + IUIAutomationElement7Impl + IUIAutomationElement8Impl {
    fn CurrentIsDialog();
    fn CachedIsDialog();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationElement9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElement9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElement9Vtbl {
        unsafe extern "system" fn CurrentIsDialog<Impl: IUIAutomationElement9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsDialog<Impl: IUIAutomationElement9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationElement8Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentIsDialog: CurrentIsDialog::<Impl, IMPL_OFFSET>,
            CachedIsDialog: CachedIsDialog::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationElement9 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationElementArrayImpl: Sized {
    fn Length();
    fn GetElement();
}
impl IUIAutomationElementArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationElementArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationElementArrayVtbl {
        unsafe extern "system" fn Length<Impl: IUIAutomationElementArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetElement<Impl: IUIAutomationElementArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationEventHandlerImpl: Sized {
    fn HandleAutomationEvent();
}
impl IUIAutomationEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationEventHandlerVtbl {
        unsafe extern "system" fn HandleAutomationEvent<Impl: IUIAutomationEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, eventid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleAutomationEvent: HandleAutomationEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationEventHandler as ::windows::core::Interface>::IID
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
impl IUIAutomationEventHandlerGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationEventHandlerGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationEventHandlerGroupVtbl {
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAutomationEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddChangesEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddNotificationEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, propertyarray: *const i32, propertycount: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Impl: IUIAutomationEventHandlerGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationExpandCollapsePatternImpl: Sized {
    fn Expand();
    fn Collapse();
    fn CurrentExpandCollapseState();
    fn CachedExpandCollapseState();
}
impl IUIAutomationExpandCollapsePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationExpandCollapsePatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationExpandCollapsePatternVtbl {
        unsafe extern "system" fn Expand<Impl: IUIAutomationExpandCollapsePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Collapse<Impl: IUIAutomationExpandCollapsePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentExpandCollapseState<Impl: IUIAutomationExpandCollapsePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedExpandCollapseState<Impl: IUIAutomationExpandCollapsePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationFocusChangedEventHandlerImpl: Sized {
    fn HandleFocusChangedEvent();
}
impl IUIAutomationFocusChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationFocusChangedEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationFocusChangedEventHandlerVtbl {
        unsafe extern "system" fn HandleFocusChangedEvent<Impl: IUIAutomationFocusChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleFocusChangedEvent: HandleFocusChangedEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationFocusChangedEventHandler as ::windows::core::Interface>::IID
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
impl IUIAutomationGridItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridItemPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationGridItemPatternVtbl {
        unsafe extern "system" fn CurrentContainingGrid<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentRow<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentColumn<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentRowSpan<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentColumnSpan<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedContainingGrid<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedRow<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedColumn<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedRowSpan<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedColumnSpan<Impl: IUIAutomationGridItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationGridPatternImpl: Sized {
    fn GetItem();
    fn CurrentRowCount();
    fn CurrentColumnCount();
    fn CachedRowCount();
    fn CachedColumnCount();
}
impl IUIAutomationGridPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationGridPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationGridPatternVtbl {
        unsafe extern "system" fn GetItem<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentRowCount<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentColumnCount<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedRowCount<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedColumnCount<Impl: IUIAutomationGridPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationInvokePatternImpl: Sized {
    fn Invoke();
}
impl IUIAutomationInvokePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationInvokePatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationInvokePatternVtbl {
        unsafe extern "system" fn Invoke<Impl: IUIAutomationInvokePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Invoke: Invoke::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationInvokePattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationItemContainerPatternImpl: Sized {
    fn FindItemByProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationItemContainerPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationItemContainerPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationItemContainerPatternVtbl {
        unsafe extern "system" fn FindItemByProperty<Impl: IUIAutomationItemContainerPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstartafter: ::windows::core::RawPtr, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FindItemByProperty: FindItemByProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationItemContainerPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationLegacyIAccessiblePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationLegacyIAccessiblePatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationLegacyIAccessiblePatternVtbl {
        unsafe extern "system" fn Select<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoDefaultAction<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentChildId<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentName<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentValue<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentDescription<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentRole<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentState<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentHelp<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentKeyboardShortcut<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentSelection<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentDefaultAction<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedChildId<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedName<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedValue<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedDescription<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedRole<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedState<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedHelp<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszhelp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedKeyboardShortcut<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedSelection<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedDefaultAction<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIAccessible<Impl: IUIAutomationLegacyIAccessiblePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationMultipleViewPatternImpl: Sized {
    fn GetViewName();
    fn SetCurrentView();
    fn CurrentCurrentView();
    fn GetCurrentSupportedViews();
    fn CachedCurrentView();
    fn GetCachedSupportedViews();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationMultipleViewPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationMultipleViewPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationMultipleViewPatternVtbl {
        unsafe extern "system" fn GetViewName<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentView<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCurrentView<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentSupportedViews<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCurrentView<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedSupportedViews<Impl: IUIAutomationMultipleViewPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationNotConditionImpl: Sized + IUIAutomationConditionImpl {
    fn GetChild();
}
impl IUIAutomationNotConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotConditionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationNotConditionVtbl {
        unsafe extern "system" fn GetChild<Impl: IUIAutomationNotConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IUIAutomationConditionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetChild: GetChild::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationNotCondition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationNotificationEventHandlerImpl: Sized {
    fn HandleNotificationEvent();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationNotificationEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationNotificationEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationNotificationEventHandlerVtbl {
        unsafe extern "system" fn HandleNotificationEvent<Impl: IUIAutomationNotificationEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, activityid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandleNotificationEvent: HandleNotificationEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationNotificationEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationObjectModelPatternImpl: Sized {
    fn GetUnderlyingObjectModel();
}
impl IUIAutomationObjectModelPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationObjectModelPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationObjectModelPatternVtbl {
        unsafe extern "system" fn GetUnderlyingObjectModel<Impl: IUIAutomationObjectModelPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationObjectModelPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationOrConditionImpl: Sized + IUIAutomationConditionImpl {
    fn ChildCount();
    fn GetChildrenAsNativeArray();
    fn GetChildren();
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationOrConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationOrConditionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationOrConditionVtbl {
        unsafe extern "system" fn ChildCount<Impl: IUIAutomationOrConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Impl: IUIAutomationOrConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::windows::core::RawPtr, childarraycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChildren<Impl: IUIAutomationOrConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationConditionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ChildCount: ChildCount::<Impl, IMPL_OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Impl, IMPL_OFFSET>,
            GetChildren: GetChildren::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationOrCondition as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationPatternHandlerImpl: Sized {
    fn CreateClientWrapper();
    fn Dispatch();
}
impl IUIAutomationPatternHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationPatternHandlerVtbl {
        unsafe extern "system" fn CreateClientWrapper<Impl: IUIAutomationPatternHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppatterninstance: ::windows::core::RawPtr, pclientwrapper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Dispatch<Impl: IUIAutomationPatternHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationPatternInstanceImpl: Sized {
    fn GetProperty();
    fn CallMethod();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationPatternInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPatternInstanceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationPatternInstanceVtbl {
        unsafe extern "system" fn GetProperty<Impl: IUIAutomationPatternInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CallMethod<Impl: IUIAutomationPatternInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationPropertyChangedEventHandlerImpl: Sized {
    fn HandlePropertyChangedEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationPropertyChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyChangedEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationPropertyChangedEventHandlerVtbl {
        unsafe extern "system" fn HandlePropertyChangedEvent<Impl: IUIAutomationPropertyChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, propertyid: i32, newvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), HandlePropertyChangedEvent: HandlePropertyChangedEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationPropertyChangedEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationPropertyConditionImpl: Sized + IUIAutomationConditionImpl {
    fn PropertyId();
    fn PropertyValue();
    fn PropertyConditionFlags();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationPropertyConditionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationPropertyConditionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationPropertyConditionVtbl {
        unsafe extern "system" fn PropertyId<Impl: IUIAutomationPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertyValue<Impl: IUIAutomationPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertyConditionFlags<Impl: IUIAutomationPropertyConditionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut PropertyConditionFlags) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationConditionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IUIAutomationProxyFactoryImpl: Sized {
    fn CreateProvider();
    fn ProxyFactoryId();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationProxyFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationProxyFactoryVtbl {
        unsafe extern "system" fn CreateProvider<Impl: IUIAutomationProxyFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, provider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProxyFactoryId<Impl: IUIAutomationProxyFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factoryid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationProxyFactoryEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationProxyFactoryEntryVtbl {
        unsafe extern "system" fn ProxyFactory<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClassName<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImageName<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowSubstringMatch<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanCheckBaseClass<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NeedsAdviseEvents<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClassName<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetImageName<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowSubstringMatch<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCanCheckBaseClass<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNeedsAdviseEvents<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adviseevents: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWinEventsForAutomationEvent<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyid: i32, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWinEventsForAutomationEvent<Impl: IUIAutomationProxyFactoryEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: i32, propertyid: i32, winevents: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationProxyFactoryMappingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationProxyFactoryMappingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationProxyFactoryMappingVtbl {
        unsafe extern "system" fn Count<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTable<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, table: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEntry<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, entry: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTable<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertEntries<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InsertEntry<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, before: u32, factory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEntry<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearTable<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreDefaultTable<Impl: IUIAutomationProxyFactoryMappingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationRangeValuePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRangeValuePatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationRangeValuePatternVtbl {
        unsafe extern "system" fn SetValue<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentValue<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsReadOnly<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentMaximum<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentMinimum<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentLargeChange<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentSmallChange<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedValue<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsReadOnly<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedMaximum<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedMinimum<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedLargeChange<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedSmallChange<Impl: IUIAutomationRangeValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationRegistrarImpl: Sized {
    fn RegisterProperty();
    fn RegisterEvent();
    fn RegisterPattern();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationRegistrarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationRegistrarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationRegistrarVtbl {
        unsafe extern "system" fn RegisterProperty<Impl: IUIAutomationRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const UIAutomationPropertyInfo, propertyid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEvent<Impl: IUIAutomationRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const UIAutomationEventInfo, eventid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterPattern<Impl: IUIAutomationRegistrarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationScrollItemPatternImpl: Sized {
    fn ScrollIntoView();
}
impl IUIAutomationScrollItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollItemPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationScrollItemPatternVtbl {
        unsafe extern "system" fn ScrollIntoView<Impl: IUIAutomationScrollItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ScrollIntoView: ScrollIntoView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationScrollItemPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationScrollPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationScrollPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationScrollPatternVtbl {
        unsafe extern "system" fn Scroll<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentHorizontalScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentVerticalScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentHorizontalViewSize<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentVerticalViewSize<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentHorizontallyScrollable<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentVerticallyScrollable<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedHorizontalScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedVerticalScrollPercent<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedHorizontalViewSize<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedVerticalViewSize<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedHorizontallyScrollable<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedVerticallyScrollable<Impl: IUIAutomationScrollPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationSelectionItemPatternImpl: Sized {
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn CurrentIsSelected();
    fn CurrentSelectionContainer();
    fn CachedIsSelected();
    fn CachedSelectionContainer();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationSelectionItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionItemPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSelectionItemPatternVtbl {
        unsafe extern "system" fn Select<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddToSelection<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsSelected<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentSelectionContainer<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsSelected<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedSelectionContainer<Impl: IUIAutomationSelectionItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationSelectionPatternImpl: Sized {
    fn GetCurrentSelection();
    fn CurrentCanSelectMultiple();
    fn CurrentIsSelectionRequired();
    fn GetCachedSelection();
    fn CachedCanSelectMultiple();
    fn CachedIsSelectionRequired();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationSelectionPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSelectionPatternVtbl {
        unsafe extern "system" fn GetCurrentSelection<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCanSelectMultiple<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsSelectionRequired<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedSelection<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCanSelectMultiple<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsSelectionRequired<Impl: IUIAutomationSelectionPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationSelectionPattern2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSelectionPattern2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSelectionPattern2Vtbl {
        unsafe extern "system" fn CurrentFirstSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentLastSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCurrentSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentItemCount<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedFirstSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedLastSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCurrentSelectedItem<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedItemCount<Impl: IUIAutomationSelectionPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationSelectionPatternVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IUIAutomationSpreadsheetItemPatternImpl: Sized {
    fn CurrentFormula();
    fn GetCurrentAnnotationObjects();
    fn GetCurrentAnnotationTypes();
    fn CachedFormula();
    fn GetCachedAnnotationObjects();
    fn GetCachedAnnotationTypes();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IUIAutomationSpreadsheetItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetItemPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSpreadsheetItemPatternVtbl {
        unsafe extern "system" fn CurrentFormula<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentAnnotationObjects<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentAnnotationTypes<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedFormula<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedAnnotationObjects<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedAnnotationTypes<Impl: IUIAutomationSpreadsheetItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationSpreadsheetPatternImpl: Sized {
    fn GetItemByName();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationSpreadsheetPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSpreadsheetPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSpreadsheetPatternVtbl {
        unsafe extern "system" fn GetItemByName<Impl: IUIAutomationSpreadsheetPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetItemByName: GetItemByName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationStructureChangedEventHandlerImpl: Sized {
    fn HandleStructureChangedEvent();
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationStructureChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStructureChangedEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationStructureChangedEventHandlerVtbl {
        unsafe extern "system" fn HandleStructureChangedEvent<Impl: IUIAutomationStructureChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationStylesPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationStylesPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationStylesPatternVtbl {
        unsafe extern "system" fn CurrentStyleId<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentStyleName<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentFillColor<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentFillPatternStyle<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentShape<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentFillPatternColor<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentExtendedProperties<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentExtendedPropertiesAsArray<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedStyleId<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedStyleName<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedFillColor<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedFillPatternStyle<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedShape<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedFillPatternColor<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedExtendedProperties<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedExtendedPropertiesAsArray<Impl: IUIAutomationStylesPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationSynchronizedInputPatternImpl: Sized {
    fn StartListening();
    fn Cancel();
}
impl IUIAutomationSynchronizedInputPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationSynchronizedInputPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationSynchronizedInputPatternVtbl {
        unsafe extern "system" fn StartListening<Impl: IUIAutomationSynchronizedInputPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IUIAutomationSynchronizedInputPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationTableItemPatternImpl: Sized {
    fn GetCurrentRowHeaderItems();
    fn GetCurrentColumnHeaderItems();
    fn GetCachedRowHeaderItems();
    fn GetCachedColumnHeaderItems();
}
impl IUIAutomationTableItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTableItemPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTableItemPatternVtbl {
        unsafe extern "system" fn GetCurrentRowHeaderItems<Impl: IUIAutomationTableItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentColumnHeaderItems<Impl: IUIAutomationTableItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedRowHeaderItems<Impl: IUIAutomationTableItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedColumnHeaderItems<Impl: IUIAutomationTableItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationTablePatternImpl: Sized {
    fn GetCurrentRowHeaders();
    fn GetCurrentColumnHeaders();
    fn CurrentRowOrColumnMajor();
    fn GetCachedRowHeaders();
    fn GetCachedColumnHeaders();
    fn CachedRowOrColumnMajor();
}
impl IUIAutomationTablePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTablePatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTablePatternVtbl {
        unsafe extern "system" fn GetCurrentRowHeaders<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentColumnHeaders<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentRowOrColumnMajor<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedRowHeaders<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCachedColumnHeaders<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedRowOrColumnMajor<Impl: IUIAutomationTablePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationTextChildPatternImpl: Sized {
    fn TextContainer();
    fn TextRange();
}
impl IUIAutomationTextChildPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextChildPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextChildPatternVtbl {
        unsafe extern "system" fn TextContainer<Impl: IUIAutomationTextChildPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, container: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TextRange<Impl: IUIAutomationTextChildPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationTextEditPatternImpl: Sized + IUIAutomationTextPatternImpl {
    fn GetActiveComposition();
    fn GetConversionTarget();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTextEditPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextEditPatternVtbl {
        unsafe extern "system" fn GetActiveComposition<Impl: IUIAutomationTextEditPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConversionTarget<Impl: IUIAutomationTextEditPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationTextPatternVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Impl, IMPL_OFFSET>,
            GetConversionTarget: GetConversionTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextEditPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationTextEditTextChangedEventHandlerImpl: Sized {
    fn HandleTextEditTextChangedEvent();
}
#[cfg(feature = "Win32_System_Com")]
impl IUIAutomationTextEditTextChangedEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextEditTextChangedEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextEditTextChangedEventHandlerVtbl {
        unsafe extern "system" fn HandleTextEditTextChangedEvent<Impl: IUIAutomationTextEditTextChangedEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationTextPatternImpl: Sized {
    fn RangeFromPoint();
    fn RangeFromChild();
    fn GetSelection();
    fn GetVisibleRanges();
    fn DocumentRange();
    fn SupportedTextSelection();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTextPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextPatternVtbl {
        unsafe extern "system" fn RangeFromPoint<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RangeFromChild<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, child: ::windows::core::RawPtr, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelection<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVisibleRanges<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ranges: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DocumentRange<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedTextSelection<Impl: IUIAutomationTextPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedtextselection: *mut SupportedTextSelection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationTextPattern2Impl: Sized + IUIAutomationTextPatternImpl {
    fn RangeFromAnnotation();
    fn GetCaretRange();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTextPattern2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextPattern2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextPattern2Vtbl {
        unsafe extern "system" fn RangeFromAnnotation<Impl: IUIAutomationTextPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, annotation: ::windows::core::RawPtr, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaretRange<Impl: IUIAutomationTextPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, range: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationTextPatternVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Impl, IMPL_OFFSET>,
            GetCaretRange: GetCaretRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationTextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextRangeVtbl {
        unsafe extern "system" fn Clone<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clonedrange: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Compare<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, range: ::windows::core::RawPtr, aresame: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompareEndpoints<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint, compvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textunit: TextUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindAttribute<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: i32, val: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, backward: super::super::Foundation::BOOL, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindText<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, found: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeValue<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attr: i32, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBoundingRectangles<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundingrects: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnclosingElement<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enclosingelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetText<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlength: i32, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Move<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveEndpointByUnit<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveEndpointByRange<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: ::windows::core::RawPtr, targetendpoint: TextPatternRangeEndpoint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Select<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddToSelection<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveFromSelection<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScrollIntoView<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChildren<Impl: IUIAutomationTextRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationTextRange2Impl: Sized + IUIAutomationTextRangeImpl {
    fn ShowContextMenu();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationTextRange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextRange2Vtbl {
        unsafe extern "system" fn ShowContextMenu<Impl: IUIAutomationTextRange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IUIAutomationTextRangeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ShowContextMenu: ShowContextMenu::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextRange2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IUIAutomationTextRange3Impl: Sized + IUIAutomationTextRangeImpl + IUIAutomationTextRange2Impl {
    fn GetEnclosingElementBuildCache();
    fn GetChildrenBuildCache();
    fn GetAttributeValues();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IUIAutomationTextRange3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRange3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextRange3Vtbl {
        unsafe extern "system" fn GetEnclosingElementBuildCache<Impl: IUIAutomationTextRange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, enclosingelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChildrenBuildCache<Impl: IUIAutomationTextRange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cacherequest: ::windows::core::RawPtr, children: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeValues<Impl: IUIAutomationTextRange3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeids: *const i32, attributeidcount: i32, attributevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationTextRange2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetEnclosingElementBuildCache: GetEnclosingElementBuildCache::<Impl, IMPL_OFFSET>,
            GetChildrenBuildCache: GetChildrenBuildCache::<Impl, IMPL_OFFSET>,
            GetAttributeValues: GetAttributeValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationTextRange3 as ::windows::core::Interface>::IID
    }
}
pub trait IUIAutomationTextRangeArrayImpl: Sized {
    fn Length();
    fn GetElement();
}
impl IUIAutomationTextRangeArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTextRangeArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTextRangeArrayVtbl {
        unsafe extern "system" fn Length<Impl: IUIAutomationTextRangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetElement<Impl: IUIAutomationTextRangeArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationTogglePatternImpl: Sized {
    fn Toggle();
    fn CurrentToggleState();
    fn CachedToggleState();
}
impl IUIAutomationTogglePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTogglePatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTogglePatternVtbl {
        unsafe extern "system" fn Toggle<Impl: IUIAutomationTogglePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentToggleState<Impl: IUIAutomationTogglePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedToggleState<Impl: IUIAutomationTogglePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTransformPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTransformPatternVtbl {
        unsafe extern "system" fn Move<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resize<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Rotate<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCanMove<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCanResize<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCanRotate<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCanMove<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCanResize<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCanRotate<Impl: IUIAutomationTransformPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationTransformPattern2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTransformPattern2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTransformPattern2Vtbl {
        unsafe extern "system" fn Zoom<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ZoomByUnit<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCanZoom<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCanZoom<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentZoomLevel<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedZoomLevel<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentZoomMinimum<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedZoomMinimum<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentZoomMaximum<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedZoomMaximum<Impl: IUIAutomationTransformPattern2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IUIAutomationTransformPatternVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
impl IUIAutomationTreeWalkerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationTreeWalkerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationTreeWalkerVtbl {
        unsafe extern "system" fn GetParentElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstChildElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, first: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastChildElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, last: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextSiblingElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousSiblingElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, previous: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NormalizeElement<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, normalized: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParentElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstChildElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, first: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastChildElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, last: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextSiblingElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, next: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreviousSiblingElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, previous: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NormalizeElementBuildCache<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, cacherequest: ::windows::core::RawPtr, normalized: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Condition<Impl: IUIAutomationTreeWalkerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, condition: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationValuePatternImpl: Sized {
    fn SetValue();
    fn CurrentValue();
    fn CurrentIsReadOnly();
    fn CachedValue();
    fn CachedIsReadOnly();
}
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationValuePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationValuePatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationValuePatternVtbl {
        unsafe extern "system" fn SetValue<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentValue<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsReadOnly<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedValue<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsReadOnly<Impl: IUIAutomationValuePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IUIAutomationVirtualizedItemPatternImpl: Sized {
    fn Realize();
}
impl IUIAutomationVirtualizedItemPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationVirtualizedItemPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationVirtualizedItemPatternVtbl {
        unsafe extern "system" fn Realize<Impl: IUIAutomationVirtualizedItemPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Realize: Realize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIAutomationVirtualizedItemPattern as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IUIAutomationWindowPatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIAutomationWindowPatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIAutomationWindowPatternVtbl {
        unsafe extern "system" fn Close<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForInputIdle<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, success: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWindowVisualState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCanMaximize<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentCanMinimize<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsModal<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentIsTopmost<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentWindowVisualState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentWindowInteractionState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCanMaximize<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedCanMinimize<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsModal<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedIsTopmost<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedWindowVisualState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CachedWindowInteractionState<Impl: IUIAutomationWindowPatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IValueProviderImpl: Sized {
    fn SetValue();
    fn Value();
    fn IsReadOnly();
}
#[cfg(feature = "Win32_Foundation")]
impl IValueProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IValueProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IValueProviderVtbl {
        unsafe extern "system" fn SetValue<Impl: IValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsReadOnly<Impl: IValueProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IVirtualizedItemProviderImpl: Sized {
    fn Realize();
}
impl IVirtualizedItemProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualizedItemProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualizedItemProviderVtbl {
        unsafe extern "system" fn Realize<Impl: IVirtualizedItemProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Realize: Realize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualizedItemProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IWindowProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowProviderVtbl {
        unsafe extern "system" fn SetVisualState<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForInputIdle<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, milliseconds: i32, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanMaximize<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanMinimize<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsModal<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WindowVisualState<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowVisualState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WindowInteractionState<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowInteractionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTopmost<Impl: IWindowProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
