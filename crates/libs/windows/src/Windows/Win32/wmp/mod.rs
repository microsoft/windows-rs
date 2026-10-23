pub const CLSID_WMPSkinManager: windows_core::GUID = windows_core::GUID::from_u128(0xb2a7fd52_301f_4348_b93a_638c6de49229);
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPCdrom, IWMPCdrom_Vtbl, 0xcfab6e98_8730_11d3_b388_00c04f68574b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPCdrom {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPCdrom, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPCdrom {
    pub unsafe fn get_driveSpecifier(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_driveSpecifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_playlist(&self) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_playlist)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn eject(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).eject)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdrom_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_driveSpecifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_playlist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub eject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPCdrom_Impl: super::IDispatch_Impl {
    fn get_driveSpecifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_playlist(&self) -> windows_core::Result<IWMPPlaylist>;
    fn eject(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPCdrom_Vtbl {
    pub const fn new<Identity: IWMPCdrom_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_driveSpecifier<Identity: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdrive: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdrom_Impl::get_driveSpecifier(this) {
                    Ok(ok__) => {
                        pbstrdrive.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_playlist<Identity: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplaylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdrom_Impl::get_playlist(this) {
                    Ok(ok__) => {
                        ppplaylist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn eject<Identity: IWMPCdrom_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdrom_Impl::eject(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_driveSpecifier: get_driveSpecifier::<Identity, OFFSET>,
            get_playlist: get_playlist::<Identity, OFFSET>,
            eject: eject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCdrom as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPCdrom {}
windows_core::imp::define_interface!(IWMPCdromBurn, IWMPCdromBurn_Vtbl, 0xbd94dbeb_417f_4928_aa06_087d56ed9b59);
windows_core::imp::interface_hierarchy!(IWMPCdromBurn, windows_core::IUnknown);
impl IWMPCdromBurn {
    #[cfg(feature = "wtypes")]
    pub unsafe fn isAvailable(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isAvailable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritem), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getItemInfo(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getItemInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritem), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_label(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_label(&self, bstrlabel: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_label)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrlabel)) }
    }
    pub unsafe fn get_burnFormat(&self) -> windows_core::Result<WMPBurnFormat> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_burnFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_burnFormat(&self, wmpbf: WMPBurnFormat) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_burnFormat)(windows_core::Interface::as_raw(self), wmpbf) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn get_burnPlaylist(&self) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_burnPlaylist)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn put_burnPlaylist<P0>(&self, pplaylist: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPPlaylist>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_burnPlaylist)(windows_core::Interface::as_raw(self), pplaylist.param().abi()) }
    }
    pub unsafe fn refreshStatus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).refreshStatus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_burnState(&self) -> windows_core::Result<WMPBurnState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_burnState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_burnProgress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_burnProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn startBurn(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startBurn)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn stopBurn(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).stopBurn)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn erase(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).erase)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdromBurn_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub isAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isAvailable: usize,
    pub getItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_burnFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMPBurnFormat) -> windows_core::HRESULT,
    pub put_burnFormat: unsafe extern "system" fn(*mut core::ffi::c_void, WMPBurnFormat) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub get_burnPlaylist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    get_burnPlaylist: usize,
    #[cfg(feature = "oaidl")]
    pub put_burnPlaylist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    put_burnPlaylist: usize,
    pub refreshStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_burnState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMPBurnState) -> windows_core::HRESULT,
    pub get_burnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub startBurn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub stopBurn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub erase: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
pub trait IWMPCdromBurn_Impl: windows_core::IUnknownImpl {
    fn isAvailable(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn getItemInfo(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn get_label(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_label(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_burnFormat(&self) -> windows_core::Result<WMPBurnFormat>;
    fn put_burnFormat(&self, wmpbf: WMPBurnFormat) -> windows_core::Result<()>;
    fn get_burnPlaylist(&self) -> windows_core::Result<IWMPPlaylist>;
    fn put_burnPlaylist(&self, pplaylist: windows_core::Ref<IWMPPlaylist>) -> windows_core::Result<()>;
    fn refreshStatus(&self) -> windows_core::Result<()>;
    fn get_burnState(&self) -> windows_core::Result<WMPBurnState>;
    fn get_burnProgress(&self) -> windows_core::Result<i32>;
    fn startBurn(&self) -> windows_core::Result<()>;
    fn stopBurn(&self) -> windows_core::Result<()>;
    fn erase(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl IWMPCdromBurn_Vtbl {
    pub const fn new<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn isAvailable<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: *mut core::ffi::c_void, pisavailable: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromBurn_Impl::isAvailable(this, core::mem::transmute(&bstritem)) {
                    Ok(ok__) => {
                        pisavailable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getItemInfo<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: *mut core::ffi::c_void, pbstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromBurn_Impl::getItemInfo(this, core::mem::transmute(&bstritem)) {
                    Ok(ok__) => {
                        pbstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_label<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromBurn_Impl::get_label(this) {
                    Ok(ok__) => {
                        pbstrlabel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_label<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdromBurn_Impl::put_label(this, core::mem::transmute(&bstrlabel)).into()
            }
        }
        unsafe extern "system" fn get_burnFormat<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromBurn_Impl::get_burnFormat(this) {
                    Ok(ok__) => {
                        pwmpbf.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_burnFormat<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmpbf: WMPBurnFormat) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdromBurn_Impl::put_burnFormat(this, core::mem::transmute_copy(&wmpbf)).into()
            }
        }
        unsafe extern "system" fn get_burnPlaylist<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplaylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromBurn_Impl::get_burnPlaylist(this) {
                    Ok(ok__) => {
                        ppplaylist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_burnPlaylist<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplaylist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdromBurn_Impl::put_burnPlaylist(this, core::mem::transmute_copy(&pplaylist)).into()
            }
        }
        unsafe extern "system" fn refreshStatus<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdromBurn_Impl::refreshStatus(this).into()
            }
        }
        unsafe extern "system" fn get_burnState<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromBurn_Impl::get_burnState(this) {
                    Ok(ok__) => {
                        pwmpbs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_burnProgress<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprogress: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromBurn_Impl::get_burnProgress(this) {
                    Ok(ok__) => {
                        plprogress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn startBurn<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdromBurn_Impl::startBurn(this).into()
            }
        }
        unsafe extern "system" fn stopBurn<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdromBurn_Impl::stopBurn(this).into()
            }
        }
        unsafe extern "system" fn erase<Identity: IWMPCdromBurn_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdromBurn_Impl::erase(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            isAvailable: isAvailable::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            get_label: get_label::<Identity, OFFSET>,
            put_label: put_label::<Identity, OFFSET>,
            get_burnFormat: get_burnFormat::<Identity, OFFSET>,
            put_burnFormat: put_burnFormat::<Identity, OFFSET>,
            get_burnPlaylist: get_burnPlaylist::<Identity, OFFSET>,
            put_burnPlaylist: put_burnPlaylist::<Identity, OFFSET>,
            refreshStatus: refreshStatus::<Identity, OFFSET>,
            get_burnState: get_burnState::<Identity, OFFSET>,
            get_burnProgress: get_burnProgress::<Identity, OFFSET>,
            startBurn: startBurn::<Identity, OFFSET>,
            stopBurn: stopBurn::<Identity, OFFSET>,
            erase: erase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCdromBurn as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl windows_core::RuntimeName for IWMPCdromBurn {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPCdromCollection, IWMPCdromCollection_Vtbl, 0xee4c8fe2_34b2_11d3_a3bf_006097c9b344);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPCdromCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPCdromCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPCdromCollection {
    pub unsafe fn get_count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn item(&self, lindex: i32) -> windows_core::Result<IWMPCdrom> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getByDriveSpecifier(&self, bstrdrivespecifier: &windows_core::BSTR) -> windows_core::Result<IWMPCdrom> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getByDriveSpecifier)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdrivespecifier), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdromCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getByDriveSpecifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPCdromCollection_Impl: super::IDispatch_Impl {
    fn get_count(&self) -> windows_core::Result<i32>;
    fn item(&self, lindex: i32) -> windows_core::Result<IWMPCdrom>;
    fn getByDriveSpecifier(&self, bstrdrivespecifier: &windows_core::BSTR) -> windows_core::Result<IWMPCdrom>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPCdromCollection_Vtbl {
    pub const fn new<Identity: IWMPCdromCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_count<Identity: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromCollection_Impl::get_count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn item<Identity: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromCollection_Impl::item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getByDriveSpecifier<Identity: IWMPCdromCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdrivespecifier: *mut core::ffi::c_void, ppcdrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromCollection_Impl::getByDriveSpecifier(this, core::mem::transmute(&bstrdrivespecifier)) {
                    Ok(ok__) => {
                        ppcdrom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_count: get_count::<Identity, OFFSET>,
            item: item::<Identity, OFFSET>,
            getByDriveSpecifier: getByDriveSpecifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCdromCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPCdromCollection {}
windows_core::imp::define_interface!(IWMPCdromRip, IWMPCdromRip_Vtbl, 0x56e2294f_69ed_4629_a869_aea72c0dcc2c);
windows_core::imp::interface_hierarchy!(IWMPCdromRip, windows_core::IUnknown);
impl IWMPCdromRip {
    pub unsafe fn get_ripState(&self) -> windows_core::Result<WMPRipState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ripState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ripProgress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ripProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn startRip(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startRip)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn stopRip(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).stopRip)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCdromRip_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_ripState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMPRipState) -> windows_core::HRESULT,
    pub get_ripProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub startRip: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub stopRip: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMPCdromRip_Impl: windows_core::IUnknownImpl {
    fn get_ripState(&self) -> windows_core::Result<WMPRipState>;
    fn get_ripProgress(&self) -> windows_core::Result<i32>;
    fn startRip(&self) -> windows_core::Result<()>;
    fn stopRip(&self) -> windows_core::Result<()>;
}
impl IWMPCdromRip_Vtbl {
    pub const fn new<Identity: IWMPCdromRip_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ripState<Identity: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmprs: *mut WMPRipState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromRip_Impl::get_ripState(this) {
                    Ok(ok__) => {
                        pwmprs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ripProgress<Identity: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprogress: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCdromRip_Impl::get_ripProgress(this) {
                    Ok(ok__) => {
                        plprogress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn startRip<Identity: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdromRip_Impl::startRip(this).into()
            }
        }
        unsafe extern "system" fn stopRip<Identity: IWMPCdromRip_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCdromRip_Impl::stopRip(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ripState: get_ripState::<Identity, OFFSET>,
            get_ripProgress: get_ripProgress::<Identity, OFFSET>,
            startRip: startRip::<Identity, OFFSET>,
            stopRip: stopRip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCdromRip as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMPCdromRip {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPClosedCaption, IWMPClosedCaption_Vtbl, 0x4f2df574_c588_11d3_9ed0_00c04fb6e937);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPClosedCaption {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPClosedCaption, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPClosedCaption {
    pub unsafe fn get_SAMIStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SAMIStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SAMIStyle(&self, bstrsamistyle: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SAMIStyle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsamistyle)) }
    }
    pub unsafe fn get_SAMILang(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SAMILang)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SAMILang(&self, bstrsamilang: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SAMILang)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsamilang)) }
    }
    pub unsafe fn get_SAMIFileName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SAMIFileName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_SAMIFileName(&self, bstrsamifilename: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SAMIFileName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsamifilename)) }
    }
    pub unsafe fn get_captioningId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_captioningId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_captioningId(&self, bstrcaptioningid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_captioningId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcaptioningid)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPClosedCaption_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_SAMIStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SAMIStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SAMILang: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SAMILang: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SAMIFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_SAMIFileName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_captioningId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_captioningId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPClosedCaption_Impl: super::IDispatch_Impl {
    fn get_SAMIStyle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SAMIStyle(&self, bstrsamistyle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SAMILang(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SAMILang(&self, bstrsamilang: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SAMIFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_SAMIFileName(&self, bstrsamifilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_captioningId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_captioningId(&self, bstrcaptioningid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPClosedCaption_Vtbl {
    pub const fn new<Identity: IWMPClosedCaption_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_SAMIStyle<Identity: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsamistyle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPClosedCaption_Impl::get_SAMIStyle(this) {
                    Ok(ok__) => {
                        pbstrsamistyle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SAMIStyle<Identity: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsamistyle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPClosedCaption_Impl::put_SAMIStyle(this, core::mem::transmute(&bstrsamistyle)).into()
            }
        }
        unsafe extern "system" fn get_SAMILang<Identity: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsamilang: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPClosedCaption_Impl::get_SAMILang(this) {
                    Ok(ok__) => {
                        pbstrsamilang.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SAMILang<Identity: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsamilang: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPClosedCaption_Impl::put_SAMILang(this, core::mem::transmute(&bstrsamilang)).into()
            }
        }
        unsafe extern "system" fn get_SAMIFileName<Identity: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsamifilename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPClosedCaption_Impl::get_SAMIFileName(this) {
                    Ok(ok__) => {
                        pbstrsamifilename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SAMIFileName<Identity: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsamifilename: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPClosedCaption_Impl::put_SAMIFileName(this, core::mem::transmute(&bstrsamifilename)).into()
            }
        }
        unsafe extern "system" fn get_captioningId<Identity: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcaptioningid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPClosedCaption_Impl::get_captioningId(this) {
                    Ok(ok__) => {
                        pbstrcaptioningid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_captioningId<Identity: IWMPClosedCaption_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaptioningid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPClosedCaption_Impl::put_captioningId(this, core::mem::transmute(&bstrcaptioningid)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_SAMIStyle: get_SAMIStyle::<Identity, OFFSET>,
            put_SAMIStyle: put_SAMIStyle::<Identity, OFFSET>,
            get_SAMILang: get_SAMILang::<Identity, OFFSET>,
            put_SAMILang: put_SAMILang::<Identity, OFFSET>,
            get_SAMIFileName: get_SAMIFileName::<Identity, OFFSET>,
            put_SAMIFileName: put_SAMIFileName::<Identity, OFFSET>,
            get_captioningId: get_captioningId::<Identity, OFFSET>,
            put_captioningId: put_captioningId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPClosedCaption as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPClosedCaption {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPClosedCaption2, IWMPClosedCaption2_Vtbl, 0x350ba78b_6bc8_4113_a5f5_312056934eb6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPClosedCaption2 {
    type Target = IWMPClosedCaption;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPClosedCaption2, windows_core::IUnknown, super::IDispatch, IWMPClosedCaption);
#[cfg(feature = "oaidl")]
impl IWMPClosedCaption2 {
    pub unsafe fn get_SAMILangCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SAMILangCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getSAMILangName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getSAMILangName)(windows_core::Interface::as_raw(self), nindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getSAMILangID(&self, nindex: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getSAMILangID)(windows_core::Interface::as_raw(self), nindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_SAMIStyleCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SAMIStyleCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getSAMIStyleName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getSAMIStyleName)(windows_core::Interface::as_raw(self), nindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPClosedCaption2_Vtbl {
    pub base__: IWMPClosedCaption_Vtbl,
    pub get_SAMILangCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getSAMILangName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getSAMILangID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub get_SAMIStyleCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getSAMIStyleName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPClosedCaption2_Impl: IWMPClosedCaption_Impl {
    fn get_SAMILangCount(&self) -> windows_core::Result<i32>;
    fn getSAMILangName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getSAMILangID(&self, nindex: i32) -> windows_core::Result<i32>;
    fn get_SAMIStyleCount(&self) -> windows_core::Result<i32>;
    fn getSAMIStyleName(&self, nindex: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPClosedCaption2_Vtbl {
    pub const fn new<Identity: IWMPClosedCaption2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_SAMILangCount<Identity: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPClosedCaption2_Impl::get_SAMILangCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getSAMILangName<Identity: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPClosedCaption2_Impl::getSAMILangName(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getSAMILangID<Identity: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPClosedCaption2_Impl::getSAMILangID(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        pllangid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SAMIStyleCount<Identity: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPClosedCaption2_Impl::get_SAMIStyleCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getSAMIStyleName<Identity: IWMPClosedCaption2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPClosedCaption2_Impl::getSAMIStyleName(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMPClosedCaption_Vtbl::new::<Identity, OFFSET>(),
            get_SAMILangCount: get_SAMILangCount::<Identity, OFFSET>,
            getSAMILangName: getSAMILangName::<Identity, OFFSET>,
            getSAMILangID: getSAMILangID::<Identity, OFFSET>,
            get_SAMIStyleCount: get_SAMIStyleCount::<Identity, OFFSET>,
            getSAMIStyleName: getSAMIStyleName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPClosedCaption2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPClosedCaption as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPClosedCaption2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPControls, IWMPControls_Vtbl, 0x74c09e02_f828_11d2_a74b_00a0c905f36e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPControls {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPControls, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPControls {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_isAvailable(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_isAvailable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritem), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn play(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).play)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn fastForward(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).fastForward)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn fastReverse(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).fastReverse)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_currentPosition(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_currentPosition(&self, dcurrentposition: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_currentPosition)(windows_core::Interface::as_raw(self), dcurrentposition) }
    }
    pub unsafe fn get_currentPositionString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentPositionString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn next(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).next)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn previous(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).previous)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_currentItem(&self) -> windows_core::Result<IWMPMedia> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_currentItem<P0>(&self, piwmpmedia: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPMedia>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_currentItem)(windows_core::Interface::as_raw(self), piwmpmedia.param().abi()) }
    }
    pub unsafe fn get_currentMarker(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentMarker)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_currentMarker(&self, lmarker: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_currentMarker)(windows_core::Interface::as_raw(self), lmarker) }
    }
    pub unsafe fn playItem<P0>(&self, piwmpmedia: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPMedia>,
    {
        unsafe { (windows_core::Interface::vtable(self).playItem)(windows_core::Interface::as_raw(self), piwmpmedia.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPControls_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_isAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_isAvailable: usize,
    pub play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fastForward: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub fastReverse: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_currentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub put_currentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub get_currentPositionString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub next: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub previous: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_currentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_currentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_currentMarker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_currentMarker: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub playItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPControls_Impl: super::IDispatch_Impl {
    fn get_isAvailable(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn play(&self) -> windows_core::Result<()>;
    fn stop(&self) -> windows_core::Result<()>;
    fn pause(&self) -> windows_core::Result<()>;
    fn fastForward(&self) -> windows_core::Result<()>;
    fn fastReverse(&self) -> windows_core::Result<()>;
    fn get_currentPosition(&self) -> windows_core::Result<f64>;
    fn put_currentPosition(&self, dcurrentposition: f64) -> windows_core::Result<()>;
    fn get_currentPositionString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn next(&self) -> windows_core::Result<()>;
    fn previous(&self) -> windows_core::Result<()>;
    fn get_currentItem(&self) -> windows_core::Result<IWMPMedia>;
    fn put_currentItem(&self, piwmpmedia: windows_core::Ref<IWMPMedia>) -> windows_core::Result<()>;
    fn get_currentMarker(&self) -> windows_core::Result<i32>;
    fn put_currentMarker(&self, lmarker: i32) -> windows_core::Result<()>;
    fn playItem(&self, piwmpmedia: windows_core::Ref<IWMPMedia>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPControls_Vtbl {
    pub const fn new<Identity: IWMPControls_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_isAvailable<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: *mut core::ffi::c_void, pisavailable: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls_Impl::get_isAvailable(this, core::mem::transmute(&bstritem)) {
                    Ok(ok__) => {
                        pisavailable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn play<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::play(this).into()
            }
        }
        unsafe extern "system" fn stop<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::stop(this).into()
            }
        }
        unsafe extern "system" fn pause<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::pause(this).into()
            }
        }
        unsafe extern "system" fn fastForward<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::fastForward(this).into()
            }
        }
        unsafe extern "system" fn fastReverse<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::fastReverse(this).into()
            }
        }
        unsafe extern "system" fn get_currentPosition<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcurrentposition: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls_Impl::get_currentPosition(this) {
                    Ok(ok__) => {
                        pdcurrentposition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_currentPosition<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dcurrentposition: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::put_currentPosition(this, core::mem::transmute_copy(&dcurrentposition)).into()
            }
        }
        unsafe extern "system" fn get_currentPositionString<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcurrentposition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls_Impl::get_currentPositionString(this) {
                    Ok(ok__) => {
                        pbstrcurrentposition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn next<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::next(this).into()
            }
        }
        unsafe extern "system" fn previous<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::previous(this).into()
            }
        }
        unsafe extern "system" fn get_currentItem<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwmpmedia: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls_Impl::get_currentItem(this) {
                    Ok(ok__) => {
                        ppiwmpmedia.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_currentItem<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::put_currentItem(this, core::mem::transmute_copy(&piwmpmedia)).into()
            }
        }
        unsafe extern "system" fn get_currentMarker<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmarker: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls_Impl::get_currentMarker(this) {
                    Ok(ok__) => {
                        plmarker.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_currentMarker<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmarker: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::put_currentMarker(this, core::mem::transmute_copy(&lmarker)).into()
            }
        }
        unsafe extern "system" fn playItem<Identity: IWMPControls_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls_Impl::playItem(this, core::mem::transmute_copy(&piwmpmedia)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_isAvailable: get_isAvailable::<Identity, OFFSET>,
            play: play::<Identity, OFFSET>,
            stop: stop::<Identity, OFFSET>,
            pause: pause::<Identity, OFFSET>,
            fastForward: fastForward::<Identity, OFFSET>,
            fastReverse: fastReverse::<Identity, OFFSET>,
            get_currentPosition: get_currentPosition::<Identity, OFFSET>,
            put_currentPosition: put_currentPosition::<Identity, OFFSET>,
            get_currentPositionString: get_currentPositionString::<Identity, OFFSET>,
            next: next::<Identity, OFFSET>,
            previous: previous::<Identity, OFFSET>,
            get_currentItem: get_currentItem::<Identity, OFFSET>,
            put_currentItem: put_currentItem::<Identity, OFFSET>,
            get_currentMarker: get_currentMarker::<Identity, OFFSET>,
            put_currentMarker: put_currentMarker::<Identity, OFFSET>,
            playItem: playItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPControls as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPControls {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPControls2, IWMPControls2_Vtbl, 0x6f030d25_0890_480f_9775_1f7e40ab5b8e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPControls2 {
    type Target = IWMPControls;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPControls2, windows_core::IUnknown, super::IDispatch, IWMPControls);
#[cfg(feature = "oaidl")]
impl IWMPControls2 {
    pub unsafe fn step(&self, lstep: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).step)(windows_core::Interface::as_raw(self), lstep) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPControls2_Vtbl {
    pub base__: IWMPControls_Vtbl,
    pub step: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPControls2_Impl: IWMPControls_Impl {
    fn step(&self, lstep: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPControls2_Vtbl {
    pub const fn new<Identity: IWMPControls2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn step<Identity: IWMPControls2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstep: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls2_Impl::step(this, core::mem::transmute_copy(&lstep)).into()
            }
        }
        Self { base__: IWMPControls_Vtbl::new::<Identity, OFFSET>(), step: step::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPControls2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPControls as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPControls2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPControls3, IWMPControls3_Vtbl, 0xa1d1110e_d545_476a_9a78_ac3e4cb1e6bd);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPControls3 {
    type Target = IWMPControls2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPControls3, windows_core::IUnknown, super::IDispatch, IWMPControls, IWMPControls2);
#[cfg(feature = "oaidl")]
impl IWMPControls3 {
    pub unsafe fn get_audioLanguageCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_audioLanguageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getAudioLanguageID(&self, lindex: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAudioLanguageID)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getAudioLanguageDescription(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAudioLanguageDescription)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_currentAudioLanguage(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentAudioLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_currentAudioLanguage(&self, llangid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_currentAudioLanguage)(windows_core::Interface::as_raw(self), llangid) }
    }
    pub unsafe fn get_currentAudioLanguageIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentAudioLanguageIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_currentAudioLanguageIndex(&self, lindex: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_currentAudioLanguageIndex)(windows_core::Interface::as_raw(self), lindex) }
    }
    pub unsafe fn getLanguageName(&self, llangid: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getLanguageName)(windows_core::Interface::as_raw(self), llangid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_currentPositionTimecode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentPositionTimecode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_currentPositionTimecode(&self, bstrtimecode: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_currentPositionTimecode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtimecode)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPControls3_Vtbl {
    pub base__: IWMPControls2_Vtbl,
    pub get_audioLanguageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getAudioLanguageID: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub getAudioLanguageDescription: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_currentAudioLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_currentAudioLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_currentAudioLanguageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_currentAudioLanguageIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub getLanguageName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_currentPositionTimecode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_currentPositionTimecode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPControls3_Impl: IWMPControls2_Impl {
    fn get_audioLanguageCount(&self) -> windows_core::Result<i32>;
    fn getAudioLanguageID(&self, lindex: i32) -> windows_core::Result<i32>;
    fn getAudioLanguageDescription(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn get_currentAudioLanguage(&self) -> windows_core::Result<i32>;
    fn put_currentAudioLanguage(&self, llangid: i32) -> windows_core::Result<()>;
    fn get_currentAudioLanguageIndex(&self) -> windows_core::Result<i32>;
    fn put_currentAudioLanguageIndex(&self, lindex: i32) -> windows_core::Result<()>;
    fn getLanguageName(&self, llangid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn get_currentPositionTimecode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_currentPositionTimecode(&self, bstrtimecode: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPControls3_Vtbl {
    pub const fn new<Identity: IWMPControls3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_audioLanguageCount<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls3_Impl::get_audioLanguageCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAudioLanguageID<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls3_Impl::getAudioLanguageID(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pllangid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAudioLanguageDescription<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls3_Impl::getAudioLanguageDescription(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pbstrlangdesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_currentAudioLanguage<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllangid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls3_Impl::get_currentAudioLanguage(this) {
                    Ok(ok__) => {
                        pllangid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_currentAudioLanguage<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llangid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls3_Impl::put_currentAudioLanguage(this, core::mem::transmute_copy(&llangid)).into()
            }
        }
        unsafe extern "system" fn get_currentAudioLanguageIndex<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls3_Impl::get_currentAudioLanguageIndex(this) {
                    Ok(ok__) => {
                        plindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_currentAudioLanguageIndex<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls3_Impl::put_currentAudioLanguageIndex(this, core::mem::transmute_copy(&lindex)).into()
            }
        }
        unsafe extern "system" fn getLanguageName<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llangid: i32, pbstrlangname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls3_Impl::getLanguageName(this, core::mem::transmute_copy(&llangid)) {
                    Ok(ok__) => {
                        pbstrlangname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_currentPositionTimecode<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtimecode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPControls3_Impl::get_currentPositionTimecode(this) {
                    Ok(ok__) => {
                        bstrtimecode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_currentPositionTimecode<Identity: IWMPControls3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtimecode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPControls3_Impl::put_currentPositionTimecode(this, core::mem::transmute(&bstrtimecode)).into()
            }
        }
        Self {
            base__: IWMPControls2_Vtbl::new::<Identity, OFFSET>(),
            get_audioLanguageCount: get_audioLanguageCount::<Identity, OFFSET>,
            getAudioLanguageID: getAudioLanguageID::<Identity, OFFSET>,
            getAudioLanguageDescription: getAudioLanguageDescription::<Identity, OFFSET>,
            get_currentAudioLanguage: get_currentAudioLanguage::<Identity, OFFSET>,
            put_currentAudioLanguage: put_currentAudioLanguage::<Identity, OFFSET>,
            get_currentAudioLanguageIndex: get_currentAudioLanguageIndex::<Identity, OFFSET>,
            put_currentAudioLanguageIndex: put_currentAudioLanguageIndex::<Identity, OFFSET>,
            getLanguageName: getLanguageName::<Identity, OFFSET>,
            get_currentPositionTimecode: get_currentPositionTimecode::<Identity, OFFSET>,
            put_currentPositionTimecode: put_currentPositionTimecode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPControls3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPControls as windows_core::Interface>::IID || iid == &<IWMPControls2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPControls3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPCore, IWMPCore_Vtbl, 0xd84cca99_cce2_11d2_9ecc_0000f8085981);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPCore {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPCore, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPCore {
    pub unsafe fn close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_URL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_URL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_URL(&self, bstrurl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_URL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl)) }
    }
    pub unsafe fn get_openState(&self) -> windows_core::Result<WMPOpenState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_openState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_playState(&self) -> windows_core::Result<WMPPlayState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_playState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_controls(&self) -> windows_core::Result<IWMPControls> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_controls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_settings(&self) -> windows_core::Result<IWMPSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_currentMedia(&self) -> windows_core::Result<IWMPMedia> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentMedia)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_currentMedia<P0>(&self, pmedia: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPMedia>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_currentMedia)(windows_core::Interface::as_raw(self), pmedia.param().abi()) }
    }
    pub unsafe fn get_mediaCollection(&self) -> windows_core::Result<IWMPMediaCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_mediaCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_playlistCollection(&self) -> windows_core::Result<IWMPPlaylistCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_playlistCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_versionInfo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_versionInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn launchURL(&self, bstrurl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).launchURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl)) }
    }
    pub unsafe fn get_network(&self) -> windows_core::Result<IWMPNetwork> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_network)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_currentPlaylist(&self) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentPlaylist)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_currentPlaylist<P0>(&self, ppl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPPlaylist>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_currentPlaylist)(windows_core::Interface::as_raw(self), ppl.param().abi()) }
    }
    pub unsafe fn get_cdromCollection(&self) -> windows_core::Result<IWMPCdromCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_cdromCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_closedCaption(&self) -> windows_core::Result<IWMPClosedCaption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_closedCaption)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_isOnline(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_isOnline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_error(&self) -> windows_core::Result<IWMPError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_error)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_status(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_status)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCore_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_URL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_URL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_openState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMPOpenState) -> windows_core::HRESULT,
    pub get_playState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMPPlayState) -> windows_core::HRESULT,
    pub get_controls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_currentMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_currentMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_mediaCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_playlistCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_versionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub launchURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_network: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_currentPlaylist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_currentPlaylist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_cdromCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_closedCaption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_isOnline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_isOnline: usize,
    pub get_error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPCore_Impl: super::IDispatch_Impl {
    fn close(&self) -> windows_core::Result<()>;
    fn get_URL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_URL(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_openState(&self) -> windows_core::Result<WMPOpenState>;
    fn get_playState(&self) -> windows_core::Result<WMPPlayState>;
    fn get_controls(&self) -> windows_core::Result<IWMPControls>;
    fn get_settings(&self) -> windows_core::Result<IWMPSettings>;
    fn get_currentMedia(&self) -> windows_core::Result<IWMPMedia>;
    fn put_currentMedia(&self, pmedia: windows_core::Ref<IWMPMedia>) -> windows_core::Result<()>;
    fn get_mediaCollection(&self) -> windows_core::Result<IWMPMediaCollection>;
    fn get_playlistCollection(&self) -> windows_core::Result<IWMPPlaylistCollection>;
    fn get_versionInfo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn launchURL(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_network(&self) -> windows_core::Result<IWMPNetwork>;
    fn get_currentPlaylist(&self) -> windows_core::Result<IWMPPlaylist>;
    fn put_currentPlaylist(&self, ppl: windows_core::Ref<IWMPPlaylist>) -> windows_core::Result<()>;
    fn get_cdromCollection(&self) -> windows_core::Result<IWMPCdromCollection>;
    fn get_closedCaption(&self) -> windows_core::Result<IWMPClosedCaption>;
    fn get_isOnline(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_error(&self) -> windows_core::Result<IWMPError>;
    fn get_status(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPCore_Vtbl {
    pub const fn new<Identity: IWMPCore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn close<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCore_Impl::close(this).into()
            }
        }
        unsafe extern "system" fn get_URL<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_URL(this) {
                    Ok(ok__) => {
                        pbstrurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_URL<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCore_Impl::put_URL(this, core::mem::transmute(&bstrurl)).into()
            }
        }
        unsafe extern "system" fn get_openState<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpos: *mut WMPOpenState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_openState(this) {
                    Ok(ok__) => {
                        pwmpos.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_playState<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpps: *mut WMPPlayState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_playState(this) {
                    Ok(ok__) => {
                        pwmpps.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_controls<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_controls(this) {
                    Ok(ok__) => {
                        ppcontrol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_settings<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_settings(this) {
                    Ok(ok__) => {
                        ppsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_currentMedia<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmedia: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_currentMedia(this) {
                    Ok(ok__) => {
                        ppmedia.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_currentMedia<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCore_Impl::put_currentMedia(this, core::mem::transmute_copy(&pmedia)).into()
            }
        }
        unsafe extern "system" fn get_mediaCollection<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediacollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_mediaCollection(this) {
                    Ok(ok__) => {
                        ppmediacollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_playlistCollection<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplaylistcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_playlistCollection(this) {
                    Ok(ok__) => {
                        ppplaylistcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_versionInfo<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrversioninfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_versionInfo(this) {
                    Ok(ok__) => {
                        pbstrversioninfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn launchURL<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCore_Impl::launchURL(this, core::mem::transmute(&bstrurl)).into()
            }
        }
        unsafe extern "system" fn get_network<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqni: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_network(this) {
                    Ok(ok__) => {
                        ppqni.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_currentPlaylist<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_currentPlaylist(this) {
                    Ok(ok__) => {
                        pppl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_currentPlaylist<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPCore_Impl::put_currentPlaylist(this, core::mem::transmute_copy(&ppl)).into()
            }
        }
        unsafe extern "system" fn get_cdromCollection<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcdromcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_cdromCollection(this) {
                    Ok(ok__) => {
                        ppcdromcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_closedCaption<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclosedcaption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_closedCaption(this) {
                    Ok(ok__) => {
                        ppclosedcaption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_isOnline<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfonline: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_isOnline(this) {
                    Ok(ok__) => {
                        pfonline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_error<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_error(this) {
                    Ok(ok__) => {
                        pperror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_status<Identity: IWMPCore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore_Impl::get_status(this) {
                    Ok(ok__) => {
                        pbstrstatus.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            close: close::<Identity, OFFSET>,
            get_URL: get_URL::<Identity, OFFSET>,
            put_URL: put_URL::<Identity, OFFSET>,
            get_openState: get_openState::<Identity, OFFSET>,
            get_playState: get_playState::<Identity, OFFSET>,
            get_controls: get_controls::<Identity, OFFSET>,
            get_settings: get_settings::<Identity, OFFSET>,
            get_currentMedia: get_currentMedia::<Identity, OFFSET>,
            put_currentMedia: put_currentMedia::<Identity, OFFSET>,
            get_mediaCollection: get_mediaCollection::<Identity, OFFSET>,
            get_playlistCollection: get_playlistCollection::<Identity, OFFSET>,
            get_versionInfo: get_versionInfo::<Identity, OFFSET>,
            launchURL: launchURL::<Identity, OFFSET>,
            get_network: get_network::<Identity, OFFSET>,
            get_currentPlaylist: get_currentPlaylist::<Identity, OFFSET>,
            put_currentPlaylist: put_currentPlaylist::<Identity, OFFSET>,
            get_cdromCollection: get_cdromCollection::<Identity, OFFSET>,
            get_closedCaption: get_closedCaption::<Identity, OFFSET>,
            get_isOnline: get_isOnline::<Identity, OFFSET>,
            get_error: get_error::<Identity, OFFSET>,
            get_status: get_status::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCore as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPCore {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPCore2, IWMPCore2_Vtbl, 0xbc17e5b7_7561_4c18_bb90_17d485775659);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPCore2 {
    type Target = IWMPCore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPCore2, windows_core::IUnknown, super::IDispatch, IWMPCore);
#[cfg(feature = "oaidl")]
impl IWMPCore2 {
    pub unsafe fn get_dvd(&self) -> windows_core::Result<IWMPDVD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_dvd)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCore2_Vtbl {
    pub base__: IWMPCore_Vtbl,
    pub get_dvd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPCore2_Impl: IWMPCore_Impl {
    fn get_dvd(&self) -> windows_core::Result<IWMPDVD>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPCore2_Vtbl {
    pub const fn new<Identity: IWMPCore2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_dvd<Identity: IWMPCore2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdvd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore2_Impl::get_dvd(this) {
                    Ok(ok__) => {
                        ppdvd.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWMPCore_Vtbl::new::<Identity, OFFSET>(), get_dvd: get_dvd::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCore2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPCore2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPCore3, IWMPCore3_Vtbl, 0x7587c667_628f_499f_88e7_6a6f4e888464);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPCore3 {
    type Target = IWMPCore2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPCore3, windows_core::IUnknown, super::IDispatch, IWMPCore, IWMPCore2);
#[cfg(feature = "oaidl")]
impl IWMPCore3 {
    pub unsafe fn newPlaylist(&self, bstrname: &windows_core::BSTR, bstrurl: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).newPlaylist)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrurl), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn newMedia(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<IWMPMedia> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).newMedia)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPCore3_Vtbl {
    pub base__: IWMPCore2_Vtbl,
    pub newPlaylist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub newMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPCore3_Impl: IWMPCore2_Impl {
    fn newPlaylist(&self, bstrname: &windows_core::BSTR, bstrurl: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn newMedia(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<IWMPMedia>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPCore3_Vtbl {
    pub const fn new<Identity: IWMPCore3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn newPlaylist<Identity: IWMPCore3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, ppplaylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore3_Impl::newPlaylist(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrurl)) {
                    Ok(ok__) => {
                        ppplaylist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn newMedia<Identity: IWMPCore3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, ppmedia: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPCore3_Impl::newMedia(this, core::mem::transmute(&bstrurl)) {
                    Ok(ok__) => {
                        ppmedia.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWMPCore2_Vtbl::new::<Identity, OFFSET>(), newPlaylist: newPlaylist::<Identity, OFFSET>, newMedia: newMedia::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCore3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID || iid == &<IWMPCore2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPCore3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPDVD, IWMPDVD_Vtbl, 0x8da61686_4668_4a5c_ae5d_803193293dbe);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPDVD {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPDVD, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPDVD {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_isAvailable(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_isAvailable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritem), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_domain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_domain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn topMenu(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).topMenu)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn titleMenu(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).titleMenu)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn back(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).back)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).resume)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPDVD_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_isAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_isAvailable: usize,
    pub get_domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub topMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub titleMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub back: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPDVD_Impl: super::IDispatch_Impl {
    fn get_isAvailable(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_domain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn topMenu(&self) -> windows_core::Result<()>;
    fn titleMenu(&self) -> windows_core::Result<()>;
    fn back(&self) -> windows_core::Result<()>;
    fn resume(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPDVD_Vtbl {
    pub const fn new<Identity: IWMPDVD_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_isAvailable<Identity: IWMPDVD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: *mut core::ffi::c_void, pisavailable: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPDVD_Impl::get_isAvailable(this, core::mem::transmute(&bstritem)) {
                    Ok(ok__) => {
                        pisavailable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_domain<Identity: IWMPDVD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPDVD_Impl::get_domain(this) {
                    Ok(ok__) => {
                        strdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn topMenu<Identity: IWMPDVD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPDVD_Impl::topMenu(this).into()
            }
        }
        unsafe extern "system" fn titleMenu<Identity: IWMPDVD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPDVD_Impl::titleMenu(this).into()
            }
        }
        unsafe extern "system" fn back<Identity: IWMPDVD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPDVD_Impl::back(this).into()
            }
        }
        unsafe extern "system" fn resume<Identity: IWMPDVD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPDVD_Impl::resume(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_isAvailable: get_isAvailable::<Identity, OFFSET>,
            get_domain: get_domain::<Identity, OFFSET>,
            topMenu: topMenu::<Identity, OFFSET>,
            titleMenu: titleMenu::<Identity, OFFSET>,
            back: back::<Identity, OFFSET>,
            resume: resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPDVD as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPDVD {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPError, IWMPError_Vtbl, 0xa12dcf7d_14ab_4c1b_a8cd_63909f06025b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPError {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPError, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPError {
    pub unsafe fn clearErrorQueue(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).clearErrorQueue)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_errorCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_errorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_item(&self, dwindex: i32) -> windows_core::Result<IWMPErrorItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_item)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn webHelp(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).webHelp)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPError_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub clearErrorQueue: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_errorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub webHelp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPError_Impl: super::IDispatch_Impl {
    fn clearErrorQueue(&self) -> windows_core::Result<()>;
    fn get_errorCount(&self) -> windows_core::Result<i32>;
    fn get_item(&self, dwindex: i32) -> windows_core::Result<IWMPErrorItem>;
    fn webHelp(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPError_Vtbl {
    pub const fn new<Identity: IWMPError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn clearErrorQueue<Identity: IWMPError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPError_Impl::clearErrorQueue(this).into()
            }
        }
        unsafe extern "system" fn get_errorCount<Identity: IWMPError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plnumerrors: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPError_Impl::get_errorCount(this) {
                    Ok(ok__) => {
                        plnumerrors.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_item<Identity: IWMPError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: i32, pperroritem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPError_Impl::get_item(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pperroritem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn webHelp<Identity: IWMPError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPError_Impl::webHelp(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            clearErrorQueue: clearErrorQueue::<Identity, OFFSET>,
            get_errorCount: get_errorCount::<Identity, OFFSET>,
            get_item: get_item::<Identity, OFFSET>,
            webHelp: webHelp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPError as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPError {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPErrorItem, IWMPErrorItem_Vtbl, 0x3614c646_3b3b_4de7_a81e_930e3f2127b3);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPErrorItem {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPErrorItem, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPErrorItem {
    pub unsafe fn get_errorCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_errorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_errorDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_errorDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_errorContext(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_errorContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_remedy(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_remedy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_customUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_customUrl)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPErrorItem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_errorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_errorDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_errorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_errorContext: usize,
    pub get_remedy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_customUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPErrorItem_Impl: super::IDispatch_Impl {
    fn get_errorCode(&self) -> windows_core::Result<i32>;
    fn get_errorDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_errorContext(&self) -> windows_core::Result<super::VARIANT>;
    fn get_remedy(&self) -> windows_core::Result<i32>;
    fn get_customUrl(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPErrorItem_Vtbl {
    pub const fn new<Identity: IWMPErrorItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_errorCode<Identity: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPErrorItem_Impl::get_errorCode(this) {
                    Ok(ok__) => {
                        phr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_errorDescription<Identity: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPErrorItem_Impl::get_errorDescription(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_errorContext<Identity: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcontext: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPErrorItem_Impl::get_errorContext(this) {
                    Ok(ok__) => {
                        pvarcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_remedy<Identity: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plremedy: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPErrorItem_Impl::get_remedy(this) {
                    Ok(ok__) => {
                        plremedy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_customUrl<Identity: IWMPErrorItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcustomurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPErrorItem_Impl::get_customUrl(this) {
                    Ok(ok__) => {
                        pbstrcustomurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_errorCode: get_errorCode::<Identity, OFFSET>,
            get_errorDescription: get_errorDescription::<Identity, OFFSET>,
            get_errorContext: get_errorContext::<Identity, OFFSET>,
            get_remedy: get_remedy::<Identity, OFFSET>,
            get_customUrl: get_customUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPErrorItem as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPErrorItem {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPErrorItem2, IWMPErrorItem2_Vtbl, 0xf75ccec0_c67c_475c_931e_8719870bee7d);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPErrorItem2 {
    type Target = IWMPErrorItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPErrorItem2, windows_core::IUnknown, super::IDispatch, IWMPErrorItem);
#[cfg(feature = "oaidl")]
impl IWMPErrorItem2 {
    pub unsafe fn get_condition(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_condition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPErrorItem2_Vtbl {
    pub base__: IWMPErrorItem_Vtbl,
    pub get_condition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPErrorItem2_Impl: IWMPErrorItem_Impl {
    fn get_condition(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPErrorItem2_Vtbl {
    pub const fn new<Identity: IWMPErrorItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_condition<Identity: IWMPErrorItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcondition: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPErrorItem2_Impl::get_condition(this) {
                    Ok(ok__) => {
                        plcondition.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWMPErrorItem_Vtbl::new::<Identity, OFFSET>(), get_condition: get_condition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPErrorItem2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPErrorItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPErrorItem2 {}
windows_core::imp::define_interface!(IWMPEvents, IWMPEvents_Vtbl, 0x19a6627b_da9e_47c1_bb23_00b5e668236a);
windows_core::imp::interface_hierarchy!(IWMPEvents, windows_core::IUnknown);
impl IWMPEvents {
    pub unsafe fn OpenStateChange(&self, newstate: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).OpenStateChange)(windows_core::Interface::as_raw(self), newstate);
        }
    }
    pub unsafe fn PlayStateChange(&self, newstate: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).PlayStateChange)(windows_core::Interface::as_raw(self), newstate);
        }
    }
    pub unsafe fn AudioLanguageChange(&self, langid: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).AudioLanguageChange)(windows_core::Interface::as_raw(self), langid);
        }
    }
    pub unsafe fn StatusChange(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).StatusChange)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn ScriptCommand(&self, sctype: &windows_core::BSTR, param: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).ScriptCommand)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sctype), core::mem::transmute_copy(param));
        }
    }
    pub unsafe fn NewStream(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).NewStream)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Disconnect(&self, result: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self), result);
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Buffering(&self, start: super::VARIANT_BOOL) {
        unsafe {
            (windows_core::Interface::vtable(self).Buffering)(windows_core::Interface::as_raw(self), start);
        }
    }
    pub unsafe fn Error(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Warning(&self, warningtype: i32, param: i32, description: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).Warning)(windows_core::Interface::as_raw(self), warningtype, param, core::mem::transmute_copy(description));
        }
    }
    pub unsafe fn EndOfStream(&self, result: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).EndOfStream)(windows_core::Interface::as_raw(self), result);
        }
    }
    pub unsafe fn PositionChange(&self, oldposition: f64, newposition: f64) {
        unsafe {
            (windows_core::Interface::vtable(self).PositionChange)(windows_core::Interface::as_raw(self), oldposition, newposition);
        }
    }
    pub unsafe fn MarkerHit(&self, markernum: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).MarkerHit)(windows_core::Interface::as_raw(self), markernum);
        }
    }
    pub unsafe fn DurationUnitChange(&self, newdurationunit: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).DurationUnitChange)(windows_core::Interface::as_raw(self), newdurationunit);
        }
    }
    pub unsafe fn CdromMediaChange(&self, cdromnum: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).CdromMediaChange)(windows_core::Interface::as_raw(self), cdromnum);
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn PlaylistChange<P0>(&self, playlist: P0, change: WMPPlaylistChangeEventType)
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PlaylistChange)(windows_core::Interface::as_raw(self), playlist.param().abi(), change);
        }
    }
    pub unsafe fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType) {
        unsafe {
            (windows_core::Interface::vtable(self).CurrentPlaylistChange)(windows_core::Interface::as_raw(self), change);
        }
    }
    pub unsafe fn CurrentPlaylistItemAvailable(&self, bstritemname: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).CurrentPlaylistItemAvailable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemname));
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn MediaChange<P0>(&self, item: P0)
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MediaChange)(windows_core::Interface::as_raw(self), item.param().abi());
        }
    }
    pub unsafe fn CurrentMediaItemAvailable(&self, bstritemname: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).CurrentMediaItemAvailable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemname));
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CurrentItemChange<P0>(&self, pdispmedia: P0)
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CurrentItemChange)(windows_core::Interface::as_raw(self), pdispmedia.param().abi());
        }
    }
    pub unsafe fn MediaCollectionChange(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).MediaCollectionChange)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &windows_core::BSTR, bstrattribval: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).MediaCollectionAttributeStringAdded)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrattribname), core::mem::transmute_copy(bstrattribval));
        }
    }
    pub unsafe fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &windows_core::BSTR, bstrattribval: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).MediaCollectionAttributeStringRemoved)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrattribname), core::mem::transmute_copy(bstrattribval));
        }
    }
    pub unsafe fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &windows_core::BSTR, bstroldattribval: &windows_core::BSTR, bstrnewattribval: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).MediaCollectionAttributeStringChanged)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrattribname), core::mem::transmute_copy(bstroldattribval), core::mem::transmute_copy(bstrnewattribval));
        }
    }
    pub unsafe fn PlaylistCollectionChange(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PlaylistCollectionChange)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).PlaylistCollectionPlaylistAdded)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrplaylistname));
        }
    }
    pub unsafe fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).PlaylistCollectionPlaylistRemoved)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrplaylistname));
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn PlaylistCollectionPlaylistSetAsDeleted(&self, bstrplaylistname: &windows_core::BSTR, varfisdeleted: super::VARIANT_BOOL) {
        unsafe {
            (windows_core::Interface::vtable(self).PlaylistCollectionPlaylistSetAsDeleted)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrplaylistname), varfisdeleted);
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ModeChange(&self, modename: &windows_core::BSTR, newvalue: super::VARIANT_BOOL) {
        unsafe {
            (windows_core::Interface::vtable(self).ModeChange)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(modename), newvalue);
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn MediaError<P0>(&self, pmediaobject: P0)
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MediaError)(windows_core::Interface::as_raw(self), pmediaobject.param().abi());
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn OpenPlaylistSwitch<P0>(&self, pitem: P0)
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OpenPlaylistSwitch)(windows_core::Interface::as_raw(self), pitem.param().abi());
        }
    }
    pub unsafe fn DomainChange(&self, strdomain: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).DomainChange)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strdomain));
        }
    }
    pub unsafe fn SwitchedToPlayerApplication(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).SwitchedToPlayerApplication)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn SwitchedToControl(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).SwitchedToControl)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn PlayerDockedStateChange(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PlayerDockedStateChange)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn PlayerReconnect(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PlayerReconnect)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).Click)(windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy);
        }
    }
    pub unsafe fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).DoubleClick)(windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy);
        }
    }
    pub unsafe fn KeyDown(&self, nkeycode: i16, nshiftstate: i16) {
        unsafe {
            (windows_core::Interface::vtable(self).KeyDown)(windows_core::Interface::as_raw(self), nkeycode, nshiftstate);
        }
    }
    pub unsafe fn KeyPress(&self, nkeyascii: i16) {
        unsafe {
            (windows_core::Interface::vtable(self).KeyPress)(windows_core::Interface::as_raw(self), nkeyascii);
        }
    }
    pub unsafe fn KeyUp(&self, nkeycode: i16, nshiftstate: i16) {
        unsafe {
            (windows_core::Interface::vtable(self).KeyUp)(windows_core::Interface::as_raw(self), nkeycode, nshiftstate);
        }
    }
    pub unsafe fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).MouseDown)(windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy);
        }
    }
    pub unsafe fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).MouseMove)(windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy);
        }
    }
    pub unsafe fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).MouseUp)(windows_core::Interface::as_raw(self), nbutton, nshiftstate, fx, fy);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub PlayStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub AudioLanguageChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub StatusChange: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub ScriptCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub NewStream: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    #[cfg(feature = "wtypes")]
    pub Buffering: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL),
    #[cfg(not(feature = "wtypes"))]
    Buffering: usize,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Warning: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void),
    pub EndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub PositionChange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64),
    pub MarkerHit: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub DurationUnitChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    pub CdromMediaChange: unsafe extern "system" fn(*mut core::ffi::c_void, i32),
    #[cfg(feature = "oaidl")]
    pub PlaylistChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WMPPlaylistChangeEventType),
    #[cfg(not(feature = "oaidl"))]
    PlaylistChange: usize,
    pub CurrentPlaylistChange: unsafe extern "system" fn(*mut core::ffi::c_void, WMPPlaylistChangeEventType),
    pub CurrentPlaylistItemAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "oaidl")]
    pub MediaChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "oaidl"))]
    MediaChange: usize,
    pub CurrentMediaItemAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "oaidl")]
    pub CurrentItemChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "oaidl"))]
    CurrentItemChange: usize,
    pub MediaCollectionChange: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub MediaCollectionAttributeStringAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub MediaCollectionAttributeStringRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub MediaCollectionAttributeStringChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    pub PlaylistCollectionChange: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PlaylistCollectionPlaylistAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub PlaylistCollectionPlaylistRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(feature = "wtypes")]
    pub PlaylistCollectionPlaylistSetAsDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL),
    #[cfg(not(feature = "wtypes"))]
    PlaylistCollectionPlaylistSetAsDeleted: usize,
    #[cfg(feature = "wtypes")]
    pub ModeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL),
    #[cfg(not(feature = "wtypes"))]
    ModeChange: usize,
    #[cfg(feature = "oaidl")]
    pub MediaError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "oaidl"))]
    MediaError: usize,
    #[cfg(feature = "oaidl")]
    pub OpenPlaylistSwitch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "oaidl"))]
    OpenPlaylistSwitch: usize,
    pub DomainChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SwitchedToPlayerApplication: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub SwitchedToControl: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PlayerDockedStateChange: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PlayerReconnect: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Click: unsafe extern "system" fn(*mut core::ffi::c_void, i16, i16, i32, i32),
    pub DoubleClick: unsafe extern "system" fn(*mut core::ffi::c_void, i16, i16, i32, i32),
    pub KeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, i16, i16),
    pub KeyPress: unsafe extern "system" fn(*mut core::ffi::c_void, i16),
    pub KeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, i16, i16),
    pub MouseDown: unsafe extern "system" fn(*mut core::ffi::c_void, i16, i16, i32, i32),
    pub MouseMove: unsafe extern "system" fn(*mut core::ffi::c_void, i16, i16, i32, i32),
    pub MouseUp: unsafe extern "system" fn(*mut core::ffi::c_void, i16, i16, i32, i32),
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
pub trait IWMPEvents_Impl: windows_core::IUnknownImpl {
    fn OpenStateChange(&self, newstate: i32);
    fn PlayStateChange(&self, newstate: i32);
    fn AudioLanguageChange(&self, langid: i32);
    fn StatusChange(&self);
    fn ScriptCommand(&self, sctype: &windows_core::BSTR, param: &windows_core::BSTR);
    fn NewStream(&self);
    fn Disconnect(&self, result: i32);
    fn Buffering(&self, start: super::VARIANT_BOOL);
    fn Error(&self);
    fn Warning(&self, warningtype: i32, param: i32, description: &windows_core::BSTR);
    fn EndOfStream(&self, result: i32);
    fn PositionChange(&self, oldposition: f64, newposition: f64);
    fn MarkerHit(&self, markernum: i32);
    fn DurationUnitChange(&self, newdurationunit: i32);
    fn CdromMediaChange(&self, cdromnum: i32);
    fn PlaylistChange(&self, playlist: windows_core::Ref<super::IDispatch>, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistItemAvailable(&self, bstritemname: &windows_core::BSTR);
    fn MediaChange(&self, item: windows_core::Ref<super::IDispatch>);
    fn CurrentMediaItemAvailable(&self, bstritemname: &windows_core::BSTR);
    fn CurrentItemChange(&self, pdispmedia: windows_core::Ref<super::IDispatch>);
    fn MediaCollectionChange(&self);
    fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &windows_core::BSTR, bstrattribval: &windows_core::BSTR);
    fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &windows_core::BSTR, bstrattribval: &windows_core::BSTR);
    fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &windows_core::BSTR, bstroldattribval: &windows_core::BSTR, bstrnewattribval: &windows_core::BSTR);
    fn PlaylistCollectionChange(&self);
    fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &windows_core::BSTR);
    fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &windows_core::BSTR);
    fn PlaylistCollectionPlaylistSetAsDeleted(&self, bstrplaylistname: &windows_core::BSTR, varfisdeleted: super::VARIANT_BOOL);
    fn ModeChange(&self, modename: &windows_core::BSTR, newvalue: super::VARIANT_BOOL);
    fn MediaError(&self, pmediaobject: windows_core::Ref<super::IDispatch>);
    fn OpenPlaylistSwitch(&self, pitem: windows_core::Ref<super::IDispatch>);
    fn DomainChange(&self, strdomain: &windows_core::BSTR);
    fn SwitchedToPlayerApplication(&self);
    fn SwitchedToControl(&self);
    fn PlayerDockedStateChange(&self);
    fn PlayerReconnect(&self);
    fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn KeyDown(&self, nkeycode: i16, nshiftstate: i16);
    fn KeyPress(&self, nkeyascii: i16);
    fn KeyUp(&self, nkeycode: i16, nshiftstate: i16);
    fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl IWMPEvents_Vtbl {
    pub const fn new<Identity: IWMPEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenStateChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::OpenStateChange(this, core::mem::transmute_copy(&newstate));
            }
        }
        unsafe extern "system" fn PlayStateChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::PlayStateChange(this, core::mem::transmute_copy(&newstate));
            }
        }
        unsafe extern "system" fn AudioLanguageChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::AudioLanguageChange(this, core::mem::transmute_copy(&langid));
            }
        }
        unsafe extern "system" fn StatusChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::StatusChange(this);
            }
        }
        unsafe extern "system" fn ScriptCommand<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sctype: *mut core::ffi::c_void, param: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::ScriptCommand(this, core::mem::transmute(&sctype), core::mem::transmute(&param));
            }
        }
        unsafe extern "system" fn NewStream<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::NewStream(this);
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::Disconnect(this, core::mem::transmute_copy(&result));
            }
        }
        unsafe extern "system" fn Buffering<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: super::VARIANT_BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::Buffering(this, core::mem::transmute_copy(&start));
            }
        }
        unsafe extern "system" fn Error<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::Error(this);
            }
        }
        unsafe extern "system" fn Warning<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, warningtype: i32, param: i32, description: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::Warning(this, core::mem::transmute_copy(&warningtype), core::mem::transmute_copy(&param), core::mem::transmute(&description));
            }
        }
        unsafe extern "system" fn EndOfStream<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::EndOfStream(this, core::mem::transmute_copy(&result));
            }
        }
        unsafe extern "system" fn PositionChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldposition: f64, newposition: f64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::PositionChange(this, core::mem::transmute_copy(&oldposition), core::mem::transmute_copy(&newposition));
            }
        }
        unsafe extern "system" fn MarkerHit<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, markernum: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MarkerHit(this, core::mem::transmute_copy(&markernum));
            }
        }
        unsafe extern "system" fn DurationUnitChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newdurationunit: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::DurationUnitChange(this, core::mem::transmute_copy(&newdurationunit));
            }
        }
        unsafe extern "system" fn CdromMediaChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdromnum: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::CdromMediaChange(this, core::mem::transmute_copy(&cdromnum));
            }
        }
        unsafe extern "system" fn PlaylistChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, playlist: *mut core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::PlaylistChange(this, core::mem::transmute_copy(&playlist), core::mem::transmute_copy(&change));
            }
        }
        unsafe extern "system" fn CurrentPlaylistChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::CurrentPlaylistChange(this, core::mem::transmute_copy(&change));
            }
        }
        unsafe extern "system" fn CurrentPlaylistItemAvailable<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::CurrentPlaylistItemAvailable(this, core::mem::transmute(&bstritemname));
            }
        }
        unsafe extern "system" fn MediaChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MediaChange(this, core::mem::transmute_copy(&item));
            }
        }
        unsafe extern "system" fn CurrentMediaItemAvailable<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::CurrentMediaItemAvailable(this, core::mem::transmute(&bstritemname));
            }
        }
        unsafe extern "system" fn CurrentItemChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdispmedia: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::CurrentItemChange(this, core::mem::transmute_copy(&pdispmedia));
            }
        }
        unsafe extern "system" fn MediaCollectionChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MediaCollectionChange(this);
            }
        }
        unsafe extern "system" fn MediaCollectionAttributeStringAdded<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribname: *mut core::ffi::c_void, bstrattribval: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MediaCollectionAttributeStringAdded(this, core::mem::transmute(&bstrattribname), core::mem::transmute(&bstrattribval));
            }
        }
        unsafe extern "system" fn MediaCollectionAttributeStringRemoved<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribname: *mut core::ffi::c_void, bstrattribval: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MediaCollectionAttributeStringRemoved(this, core::mem::transmute(&bstrattribname), core::mem::transmute(&bstrattribval));
            }
        }
        unsafe extern "system" fn MediaCollectionAttributeStringChanged<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribname: *mut core::ffi::c_void, bstroldattribval: *mut core::ffi::c_void, bstrnewattribval: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MediaCollectionAttributeStringChanged(this, core::mem::transmute(&bstrattribname), core::mem::transmute(&bstroldattribval), core::mem::transmute(&bstrnewattribval));
            }
        }
        unsafe extern "system" fn PlaylistCollectionChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::PlaylistCollectionChange(this);
            }
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistAdded<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrplaylistname: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::PlaylistCollectionPlaylistAdded(this, core::mem::transmute(&bstrplaylistname));
            }
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistRemoved<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrplaylistname: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::PlaylistCollectionPlaylistRemoved(this, core::mem::transmute(&bstrplaylistname));
            }
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistSetAsDeleted<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrplaylistname: *mut core::ffi::c_void, varfisdeleted: super::VARIANT_BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::PlaylistCollectionPlaylistSetAsDeleted(this, core::mem::transmute(&bstrplaylistname), core::mem::transmute_copy(&varfisdeleted));
            }
        }
        unsafe extern "system" fn ModeChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modename: *mut core::ffi::c_void, newvalue: super::VARIANT_BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::ModeChange(this, core::mem::transmute(&modename), core::mem::transmute_copy(&newvalue));
            }
        }
        unsafe extern "system" fn MediaError<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediaobject: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MediaError(this, core::mem::transmute_copy(&pmediaobject));
            }
        }
        unsafe extern "system" fn OpenPlaylistSwitch<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::OpenPlaylistSwitch(this, core::mem::transmute_copy(&pitem));
            }
        }
        unsafe extern "system" fn DomainChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdomain: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::DomainChange(this, core::mem::transmute(&strdomain));
            }
        }
        unsafe extern "system" fn SwitchedToPlayerApplication<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::SwitchedToPlayerApplication(this);
            }
        }
        unsafe extern "system" fn SwitchedToControl<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::SwitchedToControl(this);
            }
        }
        unsafe extern "system" fn PlayerDockedStateChange<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::PlayerDockedStateChange(this);
            }
        }
        unsafe extern "system" fn PlayerReconnect<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::PlayerReconnect(this);
            }
        }
        unsafe extern "system" fn Click<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::Click(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy));
            }
        }
        unsafe extern "system" fn DoubleClick<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::DoubleClick(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy));
            }
        }
        unsafe extern "system" fn KeyDown<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::KeyDown(this, core::mem::transmute_copy(&nkeycode), core::mem::transmute_copy(&nshiftstate));
            }
        }
        unsafe extern "system" fn KeyPress<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nkeyascii: i16) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::KeyPress(this, core::mem::transmute_copy(&nkeyascii));
            }
        }
        unsafe extern "system" fn KeyUp<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::KeyUp(this, core::mem::transmute_copy(&nkeycode), core::mem::transmute_copy(&nshiftstate));
            }
        }
        unsafe extern "system" fn MouseDown<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MouseDown(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy));
            }
        }
        unsafe extern "system" fn MouseMove<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MouseMove(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy));
            }
        }
        unsafe extern "system" fn MouseUp<Identity: IWMPEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents_Impl::MouseUp(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenStateChange: OpenStateChange::<Identity, OFFSET>,
            PlayStateChange: PlayStateChange::<Identity, OFFSET>,
            AudioLanguageChange: AudioLanguageChange::<Identity, OFFSET>,
            StatusChange: StatusChange::<Identity, OFFSET>,
            ScriptCommand: ScriptCommand::<Identity, OFFSET>,
            NewStream: NewStream::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Buffering: Buffering::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
            Warning: Warning::<Identity, OFFSET>,
            EndOfStream: EndOfStream::<Identity, OFFSET>,
            PositionChange: PositionChange::<Identity, OFFSET>,
            MarkerHit: MarkerHit::<Identity, OFFSET>,
            DurationUnitChange: DurationUnitChange::<Identity, OFFSET>,
            CdromMediaChange: CdromMediaChange::<Identity, OFFSET>,
            PlaylistChange: PlaylistChange::<Identity, OFFSET>,
            CurrentPlaylistChange: CurrentPlaylistChange::<Identity, OFFSET>,
            CurrentPlaylistItemAvailable: CurrentPlaylistItemAvailable::<Identity, OFFSET>,
            MediaChange: MediaChange::<Identity, OFFSET>,
            CurrentMediaItemAvailable: CurrentMediaItemAvailable::<Identity, OFFSET>,
            CurrentItemChange: CurrentItemChange::<Identity, OFFSET>,
            MediaCollectionChange: MediaCollectionChange::<Identity, OFFSET>,
            MediaCollectionAttributeStringAdded: MediaCollectionAttributeStringAdded::<Identity, OFFSET>,
            MediaCollectionAttributeStringRemoved: MediaCollectionAttributeStringRemoved::<Identity, OFFSET>,
            MediaCollectionAttributeStringChanged: MediaCollectionAttributeStringChanged::<Identity, OFFSET>,
            PlaylistCollectionChange: PlaylistCollectionChange::<Identity, OFFSET>,
            PlaylistCollectionPlaylistAdded: PlaylistCollectionPlaylistAdded::<Identity, OFFSET>,
            PlaylistCollectionPlaylistRemoved: PlaylistCollectionPlaylistRemoved::<Identity, OFFSET>,
            PlaylistCollectionPlaylistSetAsDeleted: PlaylistCollectionPlaylistSetAsDeleted::<Identity, OFFSET>,
            ModeChange: ModeChange::<Identity, OFFSET>,
            MediaError: MediaError::<Identity, OFFSET>,
            OpenPlaylistSwitch: OpenPlaylistSwitch::<Identity, OFFSET>,
            DomainChange: DomainChange::<Identity, OFFSET>,
            SwitchedToPlayerApplication: SwitchedToPlayerApplication::<Identity, OFFSET>,
            SwitchedToControl: SwitchedToControl::<Identity, OFFSET>,
            PlayerDockedStateChange: PlayerDockedStateChange::<Identity, OFFSET>,
            PlayerReconnect: PlayerReconnect::<Identity, OFFSET>,
            Click: Click::<Identity, OFFSET>,
            DoubleClick: DoubleClick::<Identity, OFFSET>,
            KeyDown: KeyDown::<Identity, OFFSET>,
            KeyPress: KeyPress::<Identity, OFFSET>,
            KeyUp: KeyUp::<Identity, OFFSET>,
            MouseDown: MouseDown::<Identity, OFFSET>,
            MouseMove: MouseMove::<Identity, OFFSET>,
            MouseUp: MouseUp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEvents as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl windows_core::RuntimeName for IWMPEvents {}
windows_core::imp::define_interface!(IWMPEvents2, IWMPEvents2_Vtbl, 0x1e7601fa_47ea_4107_9ea9_9004ed9684ff);
impl core::ops::Deref for IWMPEvents2 {
    type Target = IWMPEvents;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPEvents2, windows_core::IUnknown, IWMPEvents);
impl IWMPEvents2 {
    pub unsafe fn DeviceConnect<P0>(&self, pdevice: P0)
    where
        P0: windows_core::Param<IWMPSyncDevice>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DeviceConnect)(windows_core::Interface::as_raw(self), pdevice.param().abi());
        }
    }
    pub unsafe fn DeviceDisconnect<P0>(&self, pdevice: P0)
    where
        P0: windows_core::Param<IWMPSyncDevice>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DeviceDisconnect)(windows_core::Interface::as_raw(self), pdevice.param().abi());
        }
    }
    pub unsafe fn DeviceStatusChange<P0>(&self, pdevice: P0, newstatus: WMPDeviceStatus)
    where
        P0: windows_core::Param<IWMPSyncDevice>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DeviceStatusChange)(windows_core::Interface::as_raw(self), pdevice.param().abi(), newstatus);
        }
    }
    pub unsafe fn DeviceSyncStateChange<P0>(&self, pdevice: P0, newstate: WMPSyncState)
    where
        P0: windows_core::Param<IWMPSyncDevice>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DeviceSyncStateChange)(windows_core::Interface::as_raw(self), pdevice.param().abi(), newstate);
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn DeviceSyncError<P0, P1>(&self, pdevice: P0, pmedia: P1)
    where
        P0: windows_core::Param<IWMPSyncDevice>,
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DeviceSyncError)(windows_core::Interface::as_raw(self), pdevice.param().abi(), pmedia.param().abi());
        }
    }
    pub unsafe fn CreatePartnershipComplete<P0>(&self, pdevice: P0, hrresult: windows_core::HRESULT)
    where
        P0: windows_core::Param<IWMPSyncDevice>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreatePartnershipComplete)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hrresult);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents2_Vtbl {
    pub base__: IWMPEvents_Vtbl,
    pub DeviceConnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DeviceDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub DeviceStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WMPDeviceStatus),
    pub DeviceSyncStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WMPSyncState),
    #[cfg(feature = "oaidl")]
    pub DeviceSyncError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "oaidl"))]
    DeviceSyncError: usize,
    pub CreatePartnershipComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT),
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
pub trait IWMPEvents2_Impl: IWMPEvents_Impl {
    fn DeviceConnect(&self, pdevice: windows_core::Ref<IWMPSyncDevice>);
    fn DeviceDisconnect(&self, pdevice: windows_core::Ref<IWMPSyncDevice>);
    fn DeviceStatusChange(&self, pdevice: windows_core::Ref<IWMPSyncDevice>, newstatus: WMPDeviceStatus);
    fn DeviceSyncStateChange(&self, pdevice: windows_core::Ref<IWMPSyncDevice>, newstate: WMPSyncState);
    fn DeviceSyncError(&self, pdevice: windows_core::Ref<IWMPSyncDevice>, pmedia: windows_core::Ref<super::IDispatch>);
    fn CreatePartnershipComplete(&self, pdevice: windows_core::Ref<IWMPSyncDevice>, hrresult: windows_core::HRESULT);
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl IWMPEvents2_Vtbl {
    pub const fn new<Identity: IWMPEvents2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceConnect<Identity: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents2_Impl::DeviceConnect(this, core::mem::transmute_copy(&pdevice));
            }
        }
        unsafe extern "system" fn DeviceDisconnect<Identity: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents2_Impl::DeviceDisconnect(this, core::mem::transmute_copy(&pdevice));
            }
        }
        unsafe extern "system" fn DeviceStatusChange<Identity: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, newstatus: WMPDeviceStatus) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents2_Impl::DeviceStatusChange(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&newstatus));
            }
        }
        unsafe extern "system" fn DeviceSyncStateChange<Identity: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, newstate: WMPSyncState) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents2_Impl::DeviceSyncStateChange(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&newstate));
            }
        }
        unsafe extern "system" fn DeviceSyncError<Identity: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents2_Impl::DeviceSyncError(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pmedia));
            }
        }
        unsafe extern "system" fn CreatePartnershipComplete<Identity: IWMPEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hrresult: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents2_Impl::CreatePartnershipComplete(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hrresult));
            }
        }
        Self {
            base__: IWMPEvents_Vtbl::new::<Identity, OFFSET>(),
            DeviceConnect: DeviceConnect::<Identity, OFFSET>,
            DeviceDisconnect: DeviceDisconnect::<Identity, OFFSET>,
            DeviceStatusChange: DeviceStatusChange::<Identity, OFFSET>,
            DeviceSyncStateChange: DeviceSyncStateChange::<Identity, OFFSET>,
            DeviceSyncError: DeviceSyncError::<Identity, OFFSET>,
            CreatePartnershipComplete: CreatePartnershipComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEvents2 as windows_core::Interface>::IID || iid == &<IWMPEvents as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl windows_core::RuntimeName for IWMPEvents2 {}
windows_core::imp::define_interface!(IWMPEvents3, IWMPEvents3_Vtbl, 0x1f504270_a66b_4223_8e96_26a06c63d69f);
impl core::ops::Deref for IWMPEvents3 {
    type Target = IWMPEvents2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPEvents3, windows_core::IUnknown, IWMPEvents, IWMPEvents2);
impl IWMPEvents3 {
    pub unsafe fn CdromRipStateChange<P0>(&self, pcdromrip: P0, wmprs: WMPRipState)
    where
        P0: windows_core::Param<IWMPCdromRip>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CdromRipStateChange)(windows_core::Interface::as_raw(self), pcdromrip.param().abi(), wmprs);
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CdromRipMediaError<P0, P1>(&self, pcdromrip: P0, pmedia: P1)
    where
        P0: windows_core::Param<IWMPCdromRip>,
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CdromRipMediaError)(windows_core::Interface::as_raw(self), pcdromrip.param().abi(), pmedia.param().abi());
        }
    }
    pub unsafe fn CdromBurnStateChange<P0>(&self, pcdromburn: P0, wmpbs: WMPBurnState)
    where
        P0: windows_core::Param<IWMPCdromBurn>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CdromBurnStateChange)(windows_core::Interface::as_raw(self), pcdromburn.param().abi(), wmpbs);
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CdromBurnMediaError<P0, P1>(&self, pcdromburn: P0, pmedia: P1)
    where
        P0: windows_core::Param<IWMPCdromBurn>,
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CdromBurnMediaError)(windows_core::Interface::as_raw(self), pcdromburn.param().abi(), pmedia.param().abi());
        }
    }
    pub unsafe fn CdromBurnError<P0>(&self, pcdromburn: P0, hrerror: windows_core::HRESULT)
    where
        P0: windows_core::Param<IWMPCdromBurn>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CdromBurnError)(windows_core::Interface::as_raw(self), pcdromburn.param().abi(), hrerror);
        }
    }
    pub unsafe fn LibraryConnect<P0>(&self, plibrary: P0)
    where
        P0: windows_core::Param<IWMPLibrary>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).LibraryConnect)(windows_core::Interface::as_raw(self), plibrary.param().abi());
        }
    }
    pub unsafe fn LibraryDisconnect<P0>(&self, plibrary: P0)
    where
        P0: windows_core::Param<IWMPLibrary>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).LibraryDisconnect)(windows_core::Interface::as_raw(self), plibrary.param().abi());
        }
    }
    pub unsafe fn FolderScanStateChange(&self, wmpfss: WMPFolderScanState) {
        unsafe {
            (windows_core::Interface::vtable(self).FolderScanStateChange)(windows_core::Interface::as_raw(self), wmpfss);
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn StringCollectionChange<P0>(&self, pdispstringcollection: P0, change: WMPStringCollectionChangeEventType, lcollectionindex: i32)
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).StringCollectionChange)(windows_core::Interface::as_raw(self), pdispstringcollection.param().abi(), change, lcollectionindex);
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn MediaCollectionMediaAdded<P0>(&self, pdispmedia: P0)
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MediaCollectionMediaAdded)(windows_core::Interface::as_raw(self), pdispmedia.param().abi());
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn MediaCollectionMediaRemoved<P0>(&self, pdispmedia: P0)
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MediaCollectionMediaRemoved)(windows_core::Interface::as_raw(self), pdispmedia.param().abi());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents3_Vtbl {
    pub base__: IWMPEvents2_Vtbl,
    pub CdromRipStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WMPRipState),
    #[cfg(feature = "oaidl")]
    pub CdromRipMediaError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "oaidl"))]
    CdromRipMediaError: usize,
    pub CdromBurnStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WMPBurnState),
    #[cfg(feature = "oaidl")]
    pub CdromBurnMediaError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "oaidl"))]
    CdromBurnMediaError: usize,
    pub CdromBurnError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT),
    pub LibraryConnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub LibraryDisconnect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub FolderScanStateChange: unsafe extern "system" fn(*mut core::ffi::c_void, WMPFolderScanState),
    #[cfg(feature = "oaidl")]
    pub StringCollectionChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WMPStringCollectionChangeEventType, i32),
    #[cfg(not(feature = "oaidl"))]
    StringCollectionChange: usize,
    #[cfg(feature = "oaidl")]
    pub MediaCollectionMediaAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "oaidl"))]
    MediaCollectionMediaAdded: usize,
    #[cfg(feature = "oaidl")]
    pub MediaCollectionMediaRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(not(feature = "oaidl"))]
    MediaCollectionMediaRemoved: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
pub trait IWMPEvents3_Impl: IWMPEvents2_Impl {
    fn CdromRipStateChange(&self, pcdromrip: windows_core::Ref<IWMPCdromRip>, wmprs: WMPRipState);
    fn CdromRipMediaError(&self, pcdromrip: windows_core::Ref<IWMPCdromRip>, pmedia: windows_core::Ref<super::IDispatch>);
    fn CdromBurnStateChange(&self, pcdromburn: windows_core::Ref<IWMPCdromBurn>, wmpbs: WMPBurnState);
    fn CdromBurnMediaError(&self, pcdromburn: windows_core::Ref<IWMPCdromBurn>, pmedia: windows_core::Ref<super::IDispatch>);
    fn CdromBurnError(&self, pcdromburn: windows_core::Ref<IWMPCdromBurn>, hrerror: windows_core::HRESULT);
    fn LibraryConnect(&self, plibrary: windows_core::Ref<IWMPLibrary>);
    fn LibraryDisconnect(&self, plibrary: windows_core::Ref<IWMPLibrary>);
    fn FolderScanStateChange(&self, wmpfss: WMPFolderScanState);
    fn StringCollectionChange(&self, pdispstringcollection: windows_core::Ref<super::IDispatch>, change: WMPStringCollectionChangeEventType, lcollectionindex: i32);
    fn MediaCollectionMediaAdded(&self, pdispmedia: windows_core::Ref<super::IDispatch>);
    fn MediaCollectionMediaRemoved(&self, pdispmedia: windows_core::Ref<super::IDispatch>);
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl IWMPEvents3_Vtbl {
    pub const fn new<Identity: IWMPEvents3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CdromRipStateChange<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromrip: *mut core::ffi::c_void, wmprs: WMPRipState) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::CdromRipStateChange(this, core::mem::transmute_copy(&pcdromrip), core::mem::transmute_copy(&wmprs));
            }
        }
        unsafe extern "system" fn CdromRipMediaError<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromrip: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::CdromRipMediaError(this, core::mem::transmute_copy(&pcdromrip), core::mem::transmute_copy(&pmedia));
            }
        }
        unsafe extern "system" fn CdromBurnStateChange<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromburn: *mut core::ffi::c_void, wmpbs: WMPBurnState) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::CdromBurnStateChange(this, core::mem::transmute_copy(&pcdromburn), core::mem::transmute_copy(&wmpbs));
            }
        }
        unsafe extern "system" fn CdromBurnMediaError<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromburn: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::CdromBurnMediaError(this, core::mem::transmute_copy(&pcdromburn), core::mem::transmute_copy(&pmedia));
            }
        }
        unsafe extern "system" fn CdromBurnError<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromburn: *mut core::ffi::c_void, hrerror: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::CdromBurnError(this, core::mem::transmute_copy(&pcdromburn), core::mem::transmute_copy(&hrerror));
            }
        }
        unsafe extern "system" fn LibraryConnect<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plibrary: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::LibraryConnect(this, core::mem::transmute_copy(&plibrary));
            }
        }
        unsafe extern "system" fn LibraryDisconnect<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plibrary: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::LibraryDisconnect(this, core::mem::transmute_copy(&plibrary));
            }
        }
        unsafe extern "system" fn FolderScanStateChange<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmpfss: WMPFolderScanState) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::FolderScanStateChange(this, core::mem::transmute_copy(&wmpfss));
            }
        }
        unsafe extern "system" fn StringCollectionChange<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdispstringcollection: *mut core::ffi::c_void, change: WMPStringCollectionChangeEventType, lcollectionindex: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::StringCollectionChange(this, core::mem::transmute_copy(&pdispstringcollection), core::mem::transmute_copy(&change), core::mem::transmute_copy(&lcollectionindex));
            }
        }
        unsafe extern "system" fn MediaCollectionMediaAdded<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdispmedia: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::MediaCollectionMediaAdded(this, core::mem::transmute_copy(&pdispmedia));
            }
        }
        unsafe extern "system" fn MediaCollectionMediaRemoved<Identity: IWMPEvents3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdispmedia: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents3_Impl::MediaCollectionMediaRemoved(this, core::mem::transmute_copy(&pdispmedia));
            }
        }
        Self {
            base__: IWMPEvents2_Vtbl::new::<Identity, OFFSET>(),
            CdromRipStateChange: CdromRipStateChange::<Identity, OFFSET>,
            CdromRipMediaError: CdromRipMediaError::<Identity, OFFSET>,
            CdromBurnStateChange: CdromBurnStateChange::<Identity, OFFSET>,
            CdromBurnMediaError: CdromBurnMediaError::<Identity, OFFSET>,
            CdromBurnError: CdromBurnError::<Identity, OFFSET>,
            LibraryConnect: LibraryConnect::<Identity, OFFSET>,
            LibraryDisconnect: LibraryDisconnect::<Identity, OFFSET>,
            FolderScanStateChange: FolderScanStateChange::<Identity, OFFSET>,
            StringCollectionChange: StringCollectionChange::<Identity, OFFSET>,
            MediaCollectionMediaAdded: MediaCollectionMediaAdded::<Identity, OFFSET>,
            MediaCollectionMediaRemoved: MediaCollectionMediaRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEvents3 as windows_core::Interface>::IID || iid == &<IWMPEvents as windows_core::Interface>::IID || iid == &<IWMPEvents2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl windows_core::RuntimeName for IWMPEvents3 {}
windows_core::imp::define_interface!(IWMPEvents4, IWMPEvents4_Vtbl, 0x26dabcfa_306b_404d_9a6f_630a8405048d);
impl core::ops::Deref for IWMPEvents4 {
    type Target = IWMPEvents3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPEvents4, windows_core::IUnknown, IWMPEvents, IWMPEvents2, IWMPEvents3);
impl IWMPEvents4 {
    pub unsafe fn DeviceEstimation<P0>(&self, pdevice: P0, hrresult: windows_core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64)
    where
        P0: windows_core::Param<IWMPSyncDevice>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DeviceEstimation)(windows_core::Interface::as_raw(self), pdevice.param().abi(), hrresult, qwestimatedusedspace, qwestimatedspace);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPEvents4_Vtbl {
    pub base__: IWMPEvents3_Vtbl,
    pub DeviceEstimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT, i64, i64),
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
pub trait IWMPEvents4_Impl: IWMPEvents3_Impl {
    fn DeviceEstimation(&self, pdevice: windows_core::Ref<IWMPSyncDevice>, hrresult: windows_core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64);
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl IWMPEvents4_Vtbl {
    pub const fn new<Identity: IWMPEvents4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceEstimation<Identity: IWMPEvents4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hrresult: windows_core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPEvents4_Impl::DeviceEstimation(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&hrresult), core::mem::transmute_copy(&qwestimatedusedspace), core::mem::transmute_copy(&qwestimatedspace));
            }
        }
        Self { base__: IWMPEvents3_Vtbl::new::<Identity, OFFSET>(), DeviceEstimation: DeviceEstimation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEvents4 as windows_core::Interface>::IID || iid == &<IWMPEvents as windows_core::Interface>::IID || iid == &<IWMPEvents2 as windows_core::Interface>::IID || iid == &<IWMPEvents3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl windows_core::RuntimeName for IWMPEvents4 {}
windows_core::imp::define_interface!(IWMPFolderMonitorServices, IWMPFolderMonitorServices_Vtbl, 0x788c8743_e57f_439d_a468_5bc77f2e59c6);
windows_core::imp::interface_hierarchy!(IWMPFolderMonitorServices, windows_core::IUnknown);
impl IWMPFolderMonitorServices {
    pub unsafe fn get_count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn item(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn add(&self, bstrfolder: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfolder)) }
    }
    pub unsafe fn remove(&self, lindex: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).remove)(windows_core::Interface::as_raw(self), lindex) }
    }
    pub unsafe fn get_scanState(&self) -> windows_core::Result<WMPFolderScanState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_scanState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_currentFolder(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_currentFolder)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_scannedFilesCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_scannedFilesCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_addedFilesCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_addedFilesCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_updateProgress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_updateProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn startScan(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).startScan)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn stopScan(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).stopScan)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPFolderMonitorServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_scanState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMPFolderScanState) -> windows_core::HRESULT,
    pub get_currentFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_scannedFilesCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_addedFilesCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_updateProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub startScan: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub stopScan: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMPFolderMonitorServices_Impl: windows_core::IUnknownImpl {
    fn get_count(&self) -> windows_core::Result<i32>;
    fn item(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn add(&self, bstrfolder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn remove(&self, lindex: i32) -> windows_core::Result<()>;
    fn get_scanState(&self) -> windows_core::Result<WMPFolderScanState>;
    fn get_currentFolder(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_scannedFilesCount(&self) -> windows_core::Result<i32>;
    fn get_addedFilesCount(&self) -> windows_core::Result<i32>;
    fn get_updateProgress(&self) -> windows_core::Result<i32>;
    fn startScan(&self) -> windows_core::Result<()>;
    fn stopScan(&self) -> windows_core::Result<()>;
}
impl IWMPFolderMonitorServices_Vtbl {
    pub const fn new<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_count<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPFolderMonitorServices_Impl::get_count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn item<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstrfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPFolderMonitorServices_Impl::item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pbstrfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn add<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfolder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPFolderMonitorServices_Impl::add(this, core::mem::transmute(&bstrfolder)).into()
            }
        }
        unsafe extern "system" fn remove<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPFolderMonitorServices_Impl::remove(this, core::mem::transmute_copy(&lindex)).into()
            }
        }
        unsafe extern "system" fn get_scanState<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPFolderMonitorServices_Impl::get_scanState(this) {
                    Ok(ok__) => {
                        pwmpfss.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_currentFolder<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPFolderMonitorServices_Impl::get_currentFolder(this) {
                    Ok(ok__) => {
                        pbstrfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_scannedFilesCount<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPFolderMonitorServices_Impl::get_scannedFilesCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_addedFilesCount<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPFolderMonitorServices_Impl::get_addedFilesCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_updateProgress<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprogress: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPFolderMonitorServices_Impl::get_updateProgress(this) {
                    Ok(ok__) => {
                        plprogress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn startScan<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPFolderMonitorServices_Impl::startScan(this).into()
            }
        }
        unsafe extern "system" fn stopScan<Identity: IWMPFolderMonitorServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPFolderMonitorServices_Impl::stopScan(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_count: get_count::<Identity, OFFSET>,
            item: item::<Identity, OFFSET>,
            add: add::<Identity, OFFSET>,
            remove: remove::<Identity, OFFSET>,
            get_scanState: get_scanState::<Identity, OFFSET>,
            get_currentFolder: get_currentFolder::<Identity, OFFSET>,
            get_scannedFilesCount: get_scannedFilesCount::<Identity, OFFSET>,
            get_addedFilesCount: get_addedFilesCount::<Identity, OFFSET>,
            get_updateProgress: get_updateProgress::<Identity, OFFSET>,
            startScan: startScan::<Identity, OFFSET>,
            stopScan: stopScan::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPFolderMonitorServices as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMPFolderMonitorServices {}
windows_core::imp::define_interface!(IWMPLibrary, IWMPLibrary_Vtbl, 0x3df47861_7df1_4c1f_a81b_4c26f0f7a7c6);
windows_core::imp::interface_hierarchy!(IWMPLibrary, windows_core::IUnknown);
impl IWMPLibrary {
    pub unsafe fn get_name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_type(&self) -> windows_core::Result<WMPLibraryType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn get_mediaCollection(&self) -> windows_core::Result<IWMPMediaCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_mediaCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn isIdentical<P0>(&self, piwmplibrary: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isIdentical)(windows_core::Interface::as_raw(self), piwmplibrary.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibrary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMPLibraryType) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub get_mediaCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    get_mediaCollection: usize,
    #[cfg(feature = "wtypes")]
    pub isIdentical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isIdentical: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
pub trait IWMPLibrary_Impl: windows_core::IUnknownImpl {
    fn get_name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_type(&self) -> windows_core::Result<WMPLibraryType>;
    fn get_mediaCollection(&self) -> windows_core::Result<IWMPMediaCollection>;
    fn isIdentical(&self, piwmplibrary: windows_core::Ref<IWMPLibrary>) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl IWMPLibrary_Vtbl {
    pub const fn new<Identity: IWMPLibrary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_name<Identity: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPLibrary_Impl::get_name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_type<Identity: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPLibrary_Impl::get_type(this) {
                    Ok(ok__) => {
                        pwmplt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_mediaCollection<Identity: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwmpmediacollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPLibrary_Impl::get_mediaCollection(this) {
                    Ok(ok__) => {
                        ppiwmpmediacollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isIdentical<Identity: IWMPLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmplibrary: *mut core::ffi::c_void, pvbool: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPLibrary_Impl::isIdentical(this, core::mem::transmute_copy(&piwmplibrary)) {
                    Ok(ok__) => {
                        pvbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_name: get_name::<Identity, OFFSET>,
            get_type: get_type::<Identity, OFFSET>,
            get_mediaCollection: get_mediaCollection::<Identity, OFFSET>,
            isIdentical: isIdentical::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPLibrary as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl windows_core::RuntimeName for IWMPLibrary {}
windows_core::imp::define_interface!(IWMPLibrary2, IWMPLibrary2_Vtbl, 0xdd578a4e_79b1_426c_bf8f_3add9072500b);
impl core::ops::Deref for IWMPLibrary2 {
    type Target = IWMPLibrary;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPLibrary2, windows_core::IUnknown, IWMPLibrary);
impl IWMPLibrary2 {
    pub unsafe fn getItemInfo(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getItemInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibrary2_Vtbl {
    pub base__: IWMPLibrary_Vtbl,
    pub getItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
pub trait IWMPLibrary2_Impl: IWMPLibrary_Impl {
    fn getItemInfo(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl IWMPLibrary2_Vtbl {
    pub const fn new<Identity: IWMPLibrary2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getItemInfo<Identity: IWMPLibrary2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: *mut core::ffi::c_void, pbstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPLibrary2_Impl::getItemInfo(this, core::mem::transmute(&bstritemname)) {
                    Ok(ok__) => {
                        pbstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWMPLibrary_Vtbl::new::<Identity, OFFSET>(), getItemInfo: getItemInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPLibrary2 as windows_core::Interface>::IID || iid == &<IWMPLibrary as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl windows_core::RuntimeName for IWMPLibrary2 {}
windows_core::imp::define_interface!(IWMPLibraryServices, IWMPLibraryServices_Vtbl, 0x39c2f8d5_1cf2_4d5e_ae09_d73492cf9eaa);
windows_core::imp::interface_hierarchy!(IWMPLibraryServices, windows_core::IUnknown);
impl IWMPLibraryServices {
    pub unsafe fn getCountByType(&self, wmplt: WMPLibraryType) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getCountByType)(windows_core::Interface::as_raw(self), wmplt, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getLibraryByType(&self, wmplt: WMPLibraryType, lindex: i32) -> windows_core::Result<IWMPLibrary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getLibraryByType)(windows_core::Interface::as_raw(self), wmplt, lindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibraryServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub getCountByType: unsafe extern "system" fn(*mut core::ffi::c_void, WMPLibraryType, *mut i32) -> windows_core::HRESULT,
    pub getLibraryByType: unsafe extern "system" fn(*mut core::ffi::c_void, WMPLibraryType, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMPLibraryServices_Impl: windows_core::IUnknownImpl {
    fn getCountByType(&self, wmplt: WMPLibraryType) -> windows_core::Result<i32>;
    fn getLibraryByType(&self, wmplt: WMPLibraryType, lindex: i32) -> windows_core::Result<IWMPLibrary>;
}
impl IWMPLibraryServices_Vtbl {
    pub const fn new<Identity: IWMPLibraryServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getCountByType<Identity: IWMPLibraryServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPLibraryServices_Impl::getCountByType(this, core::mem::transmute_copy(&wmplt)) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getLibraryByType<Identity: IWMPLibraryServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPLibraryServices_Impl::getLibraryByType(this, core::mem::transmute_copy(&wmplt), core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        ppiwmplibrary.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getCountByType: getCountByType::<Identity, OFFSET>,
            getLibraryByType: getLibraryByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPLibraryServices as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMPLibraryServices {}
windows_core::imp::define_interface!(IWMPLibrarySharingServices, IWMPLibrarySharingServices_Vtbl, 0x82cba86b_9f04_474b_a365_d6dd1466e541);
windows_core::imp::interface_hierarchy!(IWMPLibrarySharingServices, windows_core::IUnknown);
impl IWMPLibrarySharingServices {
    #[cfg(feature = "wtypes")]
    pub unsafe fn isLibraryShared(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isLibraryShared)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn isLibrarySharingEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isLibrarySharingEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn showLibrarySharing(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).showLibrarySharing)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPLibrarySharingServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub isLibraryShared: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isLibraryShared: usize,
    #[cfg(feature = "wtypes")]
    pub isLibrarySharingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isLibrarySharingEnabled: usize,
    pub showLibrarySharing: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypes")]
pub trait IWMPLibrarySharingServices_Impl: windows_core::IUnknownImpl {
    fn isLibraryShared(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn isLibrarySharingEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn showLibrarySharing(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IWMPLibrarySharingServices_Vtbl {
    pub const fn new<Identity: IWMPLibrarySharingServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn isLibraryShared<Identity: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbshared: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPLibrarySharingServices_Impl::isLibraryShared(this) {
                    Ok(ok__) => {
                        pvbshared.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isLibrarySharingEnabled<Identity: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPLibrarySharingServices_Impl::isLibrarySharingEnabled(this) {
                    Ok(ok__) => {
                        pvbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn showLibrarySharing<Identity: IWMPLibrarySharingServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPLibrarySharingServices_Impl::showLibrarySharing(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            isLibraryShared: isLibraryShared::<Identity, OFFSET>,
            isLibrarySharingEnabled: isLibrarySharingEnabled::<Identity, OFFSET>,
            showLibrarySharing: showLibrarySharing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPLibrarySharingServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IWMPLibrarySharingServices {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPMedia, IWMPMedia_Vtbl, 0x94d55e95_3fac_11d3_b155_00c04f79faa6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPMedia {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPMedia, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPMedia {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_isIdentical<P0>(&self, piwmpmedia: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_isIdentical)(windows_core::Interface::as_raw(self), piwmpmedia.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_sourceURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_sourceURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_name(&self, bstrname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)) }
    }
    pub unsafe fn get_imageSourceWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_imageSourceWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_imageSourceHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_imageSourceHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_markerCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_markerCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getMarkerTime(&self, markernum: i32) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getMarkerTime)(windows_core::Interface::as_raw(self), markernum, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getMarkerName(&self, markernum: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getMarkerName)(windows_core::Interface::as_raw(self), markernum, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_duration(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_durationString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_durationString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_attributeCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_attributeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getAttributeName(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttributeName)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getItemInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn setItemInfo(&self, bstritemname: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setItemInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemname), core::mem::transmute_copy(bstrval)) }
    }
    pub unsafe fn getItemInfoByAtom(&self, latom: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getItemInfoByAtom)(windows_core::Interface::as_raw(self), latom, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn isMemberOf<P0>(&self, pplaylist: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<IWMPPlaylist>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isMemberOf)(windows_core::Interface::as_raw(self), pplaylist.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn isReadOnlyItem(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isReadOnlyItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemname), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMedia_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_isIdentical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_isIdentical: usize,
    pub get_sourceURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_imageSourceWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_imageSourceHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_markerCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getMarkerTime: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut f64) -> windows_core::HRESULT,
    pub getMarkerName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_durationString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_attributeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getAttributeName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getItemInfoByAtom: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub isMemberOf: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isMemberOf: usize,
    #[cfg(feature = "wtypes")]
    pub isReadOnlyItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isReadOnlyItem: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPMedia_Impl: super::IDispatch_Impl {
    fn get_isIdentical(&self, piwmpmedia: windows_core::Ref<IWMPMedia>) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_sourceURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_name(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_imageSourceWidth(&self) -> windows_core::Result<i32>;
    fn get_imageSourceHeight(&self) -> windows_core::Result<i32>;
    fn get_markerCount(&self) -> windows_core::Result<i32>;
    fn getMarkerTime(&self, markernum: i32) -> windows_core::Result<f64>;
    fn getMarkerName(&self, markernum: i32) -> windows_core::Result<windows_core::BSTR>;
    fn get_duration(&self) -> windows_core::Result<f64>;
    fn get_durationString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_attributeCount(&self) -> windows_core::Result<i32>;
    fn getAttributeName(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn getItemInfo(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn setItemInfo(&self, bstritemname: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getItemInfoByAtom(&self, latom: i32) -> windows_core::Result<windows_core::BSTR>;
    fn isMemberOf(&self, pplaylist: windows_core::Ref<IWMPPlaylist>) -> windows_core::Result<super::VARIANT_BOOL>;
    fn isReadOnlyItem(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPMedia_Vtbl {
    pub const fn new<Identity: IWMPMedia_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_isIdentical<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void, pvbool: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::get_isIdentical(this, core::mem::transmute_copy(&piwmpmedia)) {
                    Ok(ok__) => {
                        pvbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_sourceURL<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsourceurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::get_sourceURL(this) {
                    Ok(ok__) => {
                        pbstrsourceurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_name<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::get_name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_name<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPMedia_Impl::put_name(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn get_imageSourceWidth<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::get_imageSourceWidth(this) {
                    Ok(ok__) => {
                        pwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_imageSourceHeight<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheight: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::get_imageSourceHeight(this) {
                    Ok(ok__) => {
                        pheight.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_markerCount<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmarkercount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::get_markerCount(this) {
                    Ok(ok__) => {
                        pmarkercount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getMarkerTime<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::getMarkerTime(this, core::mem::transmute_copy(&markernum)) {
                    Ok(ok__) => {
                        pmarkertime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getMarkerName<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, markernum: i32, pbstrmarkername: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::getMarkerName(this, core::mem::transmute_copy(&markernum)) {
                    Ok(ok__) => {
                        pbstrmarkername.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_duration<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pduration: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::get_duration(this) {
                    Ok(ok__) => {
                        pduration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_durationString<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrduration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::get_durationString(this) {
                    Ok(ok__) => {
                        pbstrduration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_attributeCount<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::get_attributeCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAttributeName<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstritemname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::getAttributeName(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pbstritemname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getItemInfo<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: *mut core::ffi::c_void, pbstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::getItemInfo(this, core::mem::transmute(&bstritemname)) {
                    Ok(ok__) => {
                        pbstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setItemInfo<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPMedia_Impl::setItemInfo(this, core::mem::transmute(&bstritemname), core::mem::transmute(&bstrval)).into()
            }
        }
        unsafe extern "system" fn getItemInfoByAtom<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, latom: i32, pbstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::getItemInfoByAtom(this, core::mem::transmute_copy(&latom)) {
                    Ok(ok__) => {
                        pbstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isMemberOf<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplaylist: *mut core::ffi::c_void, pvarfismemberof: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::isMemberOf(this, core::mem::transmute_copy(&pplaylist)) {
                    Ok(ok__) => {
                        pvarfismemberof.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn isReadOnlyItem<Identity: IWMPMedia_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: *mut core::ffi::c_void, pvarfisreadonly: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia_Impl::isReadOnlyItem(this, core::mem::transmute(&bstritemname)) {
                    Ok(ok__) => {
                        pvarfisreadonly.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_isIdentical: get_isIdentical::<Identity, OFFSET>,
            get_sourceURL: get_sourceURL::<Identity, OFFSET>,
            get_name: get_name::<Identity, OFFSET>,
            put_name: put_name::<Identity, OFFSET>,
            get_imageSourceWidth: get_imageSourceWidth::<Identity, OFFSET>,
            get_imageSourceHeight: get_imageSourceHeight::<Identity, OFFSET>,
            get_markerCount: get_markerCount::<Identity, OFFSET>,
            getMarkerTime: getMarkerTime::<Identity, OFFSET>,
            getMarkerName: getMarkerName::<Identity, OFFSET>,
            get_duration: get_duration::<Identity, OFFSET>,
            get_durationString: get_durationString::<Identity, OFFSET>,
            get_attributeCount: get_attributeCount::<Identity, OFFSET>,
            getAttributeName: getAttributeName::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            setItemInfo: setItemInfo::<Identity, OFFSET>,
            getItemInfoByAtom: getItemInfoByAtom::<Identity, OFFSET>,
            isMemberOf: isMemberOf::<Identity, OFFSET>,
            isReadOnlyItem: isReadOnlyItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMedia as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPMedia {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPMedia2, IWMPMedia2_Vtbl, 0xab7c88bb_143e_4ea4_acc3_e4350b2106c3);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPMedia2 {
    type Target = IWMPMedia;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPMedia2, windows_core::IUnknown, super::IDispatch, IWMPMedia);
#[cfg(feature = "oaidl")]
impl IWMPMedia2 {
    pub unsafe fn get_error(&self) -> windows_core::Result<IWMPErrorItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_error)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMedia2_Vtbl {
    pub base__: IWMPMedia_Vtbl,
    pub get_error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPMedia2_Impl: IWMPMedia_Impl {
    fn get_error(&self) -> windows_core::Result<IWMPErrorItem>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPMedia2_Vtbl {
    pub const fn new<Identity: IWMPMedia2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_error<Identity: IWMPMedia2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwmperroritem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia2_Impl::get_error(this) {
                    Ok(ok__) => {
                        ppiwmperroritem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWMPMedia_Vtbl::new::<Identity, OFFSET>(), get_error: get_error::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMedia2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPMedia as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPMedia2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPMedia3, IWMPMedia3_Vtbl, 0xf118efc7_f03a_4fb4_99c9_1c02a5c1065b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPMedia3 {
    type Target = IWMPMedia2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPMedia3, windows_core::IUnknown, super::IDispatch, IWMPMedia, IWMPMedia2);
#[cfg(feature = "oaidl")]
impl IWMPMedia3 {
    pub unsafe fn getAttributeCountByType(&self, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttributeCountByType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtype), core::mem::transmute_copy(bstrlanguage), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn getItemInfoByType(&self, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR, lindex: i32) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getItemInfoByType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtype), core::mem::transmute_copy(bstrlanguage), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMedia3_Vtbl {
    pub base__: IWMPMedia2_Vtbl,
    pub getAttributeCountByType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub getItemInfoByType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    getItemInfoByType: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPMedia3_Impl: IWMPMedia2_Impl {
    fn getAttributeCountByType(&self, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn getItemInfoByType(&self, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR, lindex: i32) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPMedia3_Vtbl {
    pub const fn new<Identity: IWMPMedia3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn getAttributeCountByType<Identity: IWMPMedia3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtype: *mut core::ffi::c_void, bstrlanguage: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia3_Impl::getAttributeCountByType(this, core::mem::transmute(&bstrtype), core::mem::transmute(&bstrlanguage)) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getItemInfoByType<Identity: IWMPMedia3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtype: *mut core::ffi::c_void, bstrlanguage: *mut core::ffi::c_void, lindex: i32, pvarvalue: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMedia3_Impl::getItemInfoByType(this, core::mem::transmute(&bstrtype), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMPMedia2_Vtbl::new::<Identity, OFFSET>(),
            getAttributeCountByType: getAttributeCountByType::<Identity, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMedia3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPMedia as windows_core::Interface>::IID || iid == &<IWMPMedia2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPMedia3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPMediaCollection, IWMPMediaCollection_Vtbl, 0x8363bc22_b4b4_4b19_989d_1cd765749dd1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPMediaCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPMediaCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPMediaCollection {
    pub unsafe fn add(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<IWMPMedia> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getAll(&self) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAll)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getByGenre(&self, bstrgenre: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getByGenre)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgenre), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getByAuthor(&self, bstrauthor: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getByAuthor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrauthor), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getByAlbum(&self, bstralbum: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getByAlbum)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstralbum), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getByAttribute(&self, bstrattribute: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getByAttribute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrattribute), core::mem::transmute_copy(bstrvalue), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn remove<P0>(&self, pitem: P0, varfdeletefile: super::VARIANT_BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPMedia>,
    {
        unsafe { (windows_core::Interface::vtable(self).remove)(windows_core::Interface::as_raw(self), pitem.param().abi(), varfdeletefile) }
    }
    pub unsafe fn getAttributeStringCollection(&self, bstrattribute: &windows_core::BSTR, bstrmediatype: &windows_core::BSTR) -> windows_core::Result<IWMPStringCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttributeStringCollection)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrattribute), core::mem::transmute_copy(bstrmediatype), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getMediaAtom(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getMediaAtom)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemname), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn setDeleted<P0>(&self, pitem: P0, varfisdeleted: super::VARIANT_BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPMedia>,
    {
        unsafe { (windows_core::Interface::vtable(self).setDeleted)(windows_core::Interface::as_raw(self), pitem.param().abi(), varfisdeleted) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn isDeleted<P0>(&self, pitem: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<IWMPMedia>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isDeleted)(windows_core::Interface::as_raw(self), pitem.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMediaCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getByGenre: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getByAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getByAlbum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getByAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    remove: usize,
    pub getAttributeStringCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getMediaAtom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub setDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    setDeleted: usize,
    #[cfg(feature = "wtypes")]
    pub isDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isDeleted: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPMediaCollection_Impl: super::IDispatch_Impl {
    fn add(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<IWMPMedia>;
    fn getAll(&self) -> windows_core::Result<IWMPPlaylist>;
    fn getByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getByGenre(&self, bstrgenre: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getByAuthor(&self, bstrauthor: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getByAlbum(&self, bstralbum: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getByAttribute(&self, bstrattribute: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn remove(&self, pitem: windows_core::Ref<IWMPMedia>, varfdeletefile: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getAttributeStringCollection(&self, bstrattribute: &windows_core::BSTR, bstrmediatype: &windows_core::BSTR) -> windows_core::Result<IWMPStringCollection>;
    fn getMediaAtom(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn setDeleted(&self, pitem: windows_core::Ref<IWMPMedia>, varfisdeleted: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn isDeleted(&self, pitem: windows_core::Ref<IWMPMedia>) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPMediaCollection_Vtbl {
    pub const fn new<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn add<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::add(this, core::mem::transmute(&bstrurl)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAll<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::getAll(this) {
                    Ok(ok__) => {
                        ppmediaitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getByName<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::getByName(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        ppmediaitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getByGenre<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgenre: *mut core::ffi::c_void, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::getByGenre(this, core::mem::transmute(&bstrgenre)) {
                    Ok(ok__) => {
                        ppmediaitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getByAuthor<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthor: *mut core::ffi::c_void, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::getByAuthor(this, core::mem::transmute(&bstrauthor)) {
                    Ok(ok__) => {
                        ppmediaitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getByAlbum<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstralbum: *mut core::ffi::c_void, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::getByAlbum(this, core::mem::transmute(&bstralbum)) {
                    Ok(ok__) => {
                        ppmediaitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getByAttribute<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::getByAttribute(this, core::mem::transmute(&bstrattribute), core::mem::transmute(&bstrvalue)) {
                    Ok(ok__) => {
                        ppmediaitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn remove<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, varfdeletefile: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPMediaCollection_Impl::remove(this, core::mem::transmute_copy(&pitem), core::mem::transmute_copy(&varfdeletefile)).into()
            }
        }
        unsafe extern "system" fn getAttributeStringCollection<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: *mut core::ffi::c_void, bstrmediatype: *mut core::ffi::c_void, ppstringcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::getAttributeStringCollection(this, core::mem::transmute(&bstrattribute), core::mem::transmute(&bstrmediatype)) {
                    Ok(ok__) => {
                        ppstringcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getMediaAtom<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: *mut core::ffi::c_void, platom: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::getMediaAtom(this, core::mem::transmute(&bstritemname)) {
                    Ok(ok__) => {
                        platom.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setDeleted<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, varfisdeleted: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPMediaCollection_Impl::setDeleted(this, core::mem::transmute_copy(&pitem), core::mem::transmute_copy(&varfisdeleted)).into()
            }
        }
        unsafe extern "system" fn isDeleted<Identity: IWMPMediaCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, pvarfisdeleted: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection_Impl::isDeleted(this, core::mem::transmute_copy(&pitem)) {
                    Ok(ok__) => {
                        pvarfisdeleted.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            add: add::<Identity, OFFSET>,
            getAll: getAll::<Identity, OFFSET>,
            getByName: getByName::<Identity, OFFSET>,
            getByGenre: getByGenre::<Identity, OFFSET>,
            getByAuthor: getByAuthor::<Identity, OFFSET>,
            getByAlbum: getByAlbum::<Identity, OFFSET>,
            getByAttribute: getByAttribute::<Identity, OFFSET>,
            remove: remove::<Identity, OFFSET>,
            getAttributeStringCollection: getAttributeStringCollection::<Identity, OFFSET>,
            getMediaAtom: getMediaAtom::<Identity, OFFSET>,
            setDeleted: setDeleted::<Identity, OFFSET>,
            isDeleted: isDeleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMediaCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPMediaCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPMediaCollection2, IWMPMediaCollection2_Vtbl, 0x8ba957f5_fd8c_4791_b82d_f840401ee474);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPMediaCollection2 {
    type Target = IWMPMediaCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPMediaCollection2, windows_core::IUnknown, super::IDispatch, IWMPMediaCollection);
#[cfg(feature = "oaidl")]
impl IWMPMediaCollection2 {
    pub unsafe fn createQuery(&self) -> windows_core::Result<IWMPQuery> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).createQuery)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn getPlaylistByQuery<P0>(&self, pquery: P0, bstrmediatype: &windows_core::BSTR, bstrsortattribute: &windows_core::BSTR, fsortascending: super::VARIANT_BOOL) -> windows_core::Result<IWMPPlaylist>
    where
        P0: windows_core::Param<IWMPQuery>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getPlaylistByQuery)(windows_core::Interface::as_raw(self), pquery.param().abi(), core::mem::transmute_copy(bstrmediatype), core::mem::transmute_copy(bstrsortattribute), fsortascending, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn getStringCollectionByQuery<P1>(&self, bstrattribute: &windows_core::BSTR, pquery: P1, bstrmediatype: &windows_core::BSTR, bstrsortattribute: &windows_core::BSTR, fsortascending: super::VARIANT_BOOL) -> windows_core::Result<IWMPStringCollection>
    where
        P1: windows_core::Param<IWMPQuery>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getStringCollectionByQuery)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrattribute), pquery.param().abi(), core::mem::transmute_copy(bstrmediatype), core::mem::transmute_copy(bstrsortattribute), fsortascending, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getByAttributeAndMediaType(&self, bstrattribute: &windows_core::BSTR, bstrvalue: &windows_core::BSTR, bstrmediatype: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getByAttributeAndMediaType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrattribute), core::mem::transmute_copy(bstrvalue), core::mem::transmute_copy(bstrmediatype), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMediaCollection2_Vtbl {
    pub base__: IWMPMediaCollection_Vtbl,
    pub createQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub getPlaylistByQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    getPlaylistByQuery: usize,
    #[cfg(feature = "wtypes")]
    pub getStringCollectionByQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    getStringCollectionByQuery: usize,
    pub getByAttributeAndMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPMediaCollection2_Impl: IWMPMediaCollection_Impl {
    fn createQuery(&self) -> windows_core::Result<IWMPQuery>;
    fn getPlaylistByQuery(&self, pquery: windows_core::Ref<IWMPQuery>, bstrmediatype: &windows_core::BSTR, bstrsortattribute: &windows_core::BSTR, fsortascending: super::VARIANT_BOOL) -> windows_core::Result<IWMPPlaylist>;
    fn getStringCollectionByQuery(&self, bstrattribute: &windows_core::BSTR, pquery: windows_core::Ref<IWMPQuery>, bstrmediatype: &windows_core::BSTR, bstrsortattribute: &windows_core::BSTR, fsortascending: super::VARIANT_BOOL) -> windows_core::Result<IWMPStringCollection>;
    fn getByAttributeAndMediaType(&self, bstrattribute: &windows_core::BSTR, bstrvalue: &windows_core::BSTR, bstrmediatype: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPMediaCollection2_Vtbl {
    pub const fn new<Identity: IWMPMediaCollection2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn createQuery<Identity: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection2_Impl::createQuery(this) {
                    Ok(ok__) => {
                        ppquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getPlaylistByQuery<Identity: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquery: *mut core::ffi::c_void, bstrmediatype: *mut core::ffi::c_void, bstrsortattribute: *mut core::ffi::c_void, fsortascending: super::VARIANT_BOOL, ppplaylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection2_Impl::getPlaylistByQuery(this, core::mem::transmute_copy(&pquery), core::mem::transmute(&bstrmediatype), core::mem::transmute(&bstrsortattribute), core::mem::transmute_copy(&fsortascending)) {
                    Ok(ok__) => {
                        ppplaylist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getStringCollectionByQuery<Identity: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: *mut core::ffi::c_void, pquery: *mut core::ffi::c_void, bstrmediatype: *mut core::ffi::c_void, bstrsortattribute: *mut core::ffi::c_void, fsortascending: super::VARIANT_BOOL, ppstringcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection2_Impl::getStringCollectionByQuery(this, core::mem::transmute(&bstrattribute), core::mem::transmute_copy(&pquery), core::mem::transmute(&bstrmediatype), core::mem::transmute(&bstrsortattribute), core::mem::transmute_copy(&fsortascending)) {
                    Ok(ok__) => {
                        ppstringcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getByAttributeAndMediaType<Identity: IWMPMediaCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void, bstrmediatype: *mut core::ffi::c_void, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMediaCollection2_Impl::getByAttributeAndMediaType(this, core::mem::transmute(&bstrattribute), core::mem::transmute(&bstrvalue), core::mem::transmute(&bstrmediatype)) {
                    Ok(ok__) => {
                        ppmediaitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMPMediaCollection_Vtbl::new::<Identity, OFFSET>(),
            createQuery: createQuery::<Identity, OFFSET>,
            getPlaylistByQuery: getPlaylistByQuery::<Identity, OFFSET>,
            getStringCollectionByQuery: getStringCollectionByQuery::<Identity, OFFSET>,
            getByAttributeAndMediaType: getByAttributeAndMediaType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMediaCollection2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPMediaCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPMediaCollection2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPMetadataPicture, IWMPMetadataPicture_Vtbl, 0x5c29bbe0_f87d_4c45_aa28_a70f0230ffa9);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPMetadataPicture {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPMetadataPicture, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPMetadataPicture {
    pub unsafe fn get_mimeType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_mimeType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_pictureType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_pictureType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_URL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_URL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMetadataPicture_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_mimeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_pictureType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_URL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPMetadataPicture_Impl: super::IDispatch_Impl {
    fn get_mimeType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_pictureType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_URL(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPMetadataPicture_Vtbl {
    pub const fn new<Identity: IWMPMetadataPicture_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_mimeType<Identity: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmimetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMetadataPicture_Impl::get_mimeType(this) {
                    Ok(ok__) => {
                        pbstrmimetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_pictureType<Identity: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpicturetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMetadataPicture_Impl::get_pictureType(this) {
                    Ok(ok__) => {
                        pbstrpicturetype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_description<Identity: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMetadataPicture_Impl::get_description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_URL<Identity: IWMPMetadataPicture_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMetadataPicture_Impl::get_URL(this) {
                    Ok(ok__) => {
                        pbstrurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_mimeType: get_mimeType::<Identity, OFFSET>,
            get_pictureType: get_pictureType::<Identity, OFFSET>,
            get_description: get_description::<Identity, OFFSET>,
            get_URL: get_URL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMetadataPicture as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPMetadataPicture {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPMetadataText, IWMPMetadataText_Vtbl, 0x769a72db_13d2_45e2_9c48_53ca9d5b7450);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPMetadataText {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPMetadataText, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPMetadataText {
    pub unsafe fn get_description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_text(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_text)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPMetadataText_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPMetadataText_Impl: super::IDispatch_Impl {
    fn get_description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_text(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPMetadataText_Vtbl {
    pub const fn new<Identity: IWMPMetadataText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_description<Identity: IWMPMetadataText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMetadataText_Impl::get_description(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_text<Identity: IWMPMetadataText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPMetadataText_Impl::get_text(this) {
                    Ok(ok__) => {
                        pbstrtext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_description: get_description::<Identity, OFFSET>,
            get_text: get_text::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMetadataText as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPMetadataText {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPNetwork, IWMPNetwork_Vtbl, 0xec21b779_edef_462d_bba4_ad9dde2b29a7);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPNetwork {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPNetwork, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPNetwork {
    pub unsafe fn get_bandWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_bandWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_recoveredPackets(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_recoveredPackets)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_sourceProtocol(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_sourceProtocol)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_receivedPackets(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_receivedPackets)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_lostPackets(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_lostPackets)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_receptionQuality(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_receptionQuality)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_bufferingCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_bufferingCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_bufferingProgress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_bufferingProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_bufferingTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_bufferingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_bufferingTime(&self, lbufferingtime: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_bufferingTime)(windows_core::Interface::as_raw(self), lbufferingtime) }
    }
    pub unsafe fn get_frameRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_frameRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_maxBitRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_maxBitRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_bitRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_bitRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getProxySettings(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProxySettings)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn setProxySettings(&self, bstrprotocol: &windows_core::BSTR, lproxysetting: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setProxySettings)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), lproxysetting) }
    }
    pub unsafe fn getProxyName(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProxyName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn setProxyName(&self, bstrprotocol: &windows_core::BSTR, bstrproxyname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setProxyName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), core::mem::transmute_copy(bstrproxyname)) }
    }
    pub unsafe fn getProxyPort(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProxyPort)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn setProxyPort(&self, bstrprotocol: &windows_core::BSTR, lproxyport: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setProxyPort)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), lproxyport) }
    }
    pub unsafe fn getProxyExceptionList(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProxyExceptionList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn setProxyExceptionList(&self, bstrprotocol: &windows_core::BSTR, pbstrexceptionlist: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setProxyExceptionList)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), core::mem::transmute_copy(pbstrexceptionlist)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn getProxyBypassForLocal(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getProxyBypassForLocal)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn setProxyBypassForLocal(&self, bstrprotocol: &windows_core::BSTR, fbypassforlocal: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setProxyBypassForLocal)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrprotocol), fbypassforlocal) }
    }
    pub unsafe fn get_maxBandwidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_maxBandwidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_maxBandwidth(&self, lmaxbandwidth: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_maxBandwidth)(windows_core::Interface::as_raw(self), lmaxbandwidth) }
    }
    pub unsafe fn get_downloadProgress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_downloadProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_encodedFrameRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_encodedFrameRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_framesSkipped(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_framesSkipped)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPNetwork_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_bandWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_recoveredPackets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_sourceProtocol: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_receivedPackets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_lostPackets: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_receptionQuality: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_bufferingCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_bufferingProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_bufferingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_bufferingTime: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_frameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_maxBitRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_bitRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub setProxySettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub getProxyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setProxyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getProxyPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub setProxyPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub getProxyExceptionList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setProxyExceptionList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub getProxyBypassForLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    getProxyBypassForLocal: usize,
    #[cfg(feature = "wtypes")]
    pub setProxyBypassForLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    setProxyBypassForLocal: usize,
    pub get_maxBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_maxBandwidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_downloadProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_encodedFrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_framesSkipped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPNetwork_Impl: super::IDispatch_Impl {
    fn get_bandWidth(&self) -> windows_core::Result<i32>;
    fn get_recoveredPackets(&self) -> windows_core::Result<i32>;
    fn get_sourceProtocol(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_receivedPackets(&self) -> windows_core::Result<i32>;
    fn get_lostPackets(&self) -> windows_core::Result<i32>;
    fn get_receptionQuality(&self) -> windows_core::Result<i32>;
    fn get_bufferingCount(&self) -> windows_core::Result<i32>;
    fn get_bufferingProgress(&self) -> windows_core::Result<i32>;
    fn get_bufferingTime(&self) -> windows_core::Result<i32>;
    fn put_bufferingTime(&self, lbufferingtime: i32) -> windows_core::Result<()>;
    fn get_frameRate(&self) -> windows_core::Result<i32>;
    fn get_maxBitRate(&self) -> windows_core::Result<i32>;
    fn get_bitRate(&self) -> windows_core::Result<i32>;
    fn getProxySettings(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn setProxySettings(&self, bstrprotocol: &windows_core::BSTR, lproxysetting: i32) -> windows_core::Result<()>;
    fn getProxyName(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn setProxyName(&self, bstrprotocol: &windows_core::BSTR, bstrproxyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getProxyPort(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn setProxyPort(&self, bstrprotocol: &windows_core::BSTR, lproxyport: i32) -> windows_core::Result<()>;
    fn getProxyExceptionList(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn setProxyExceptionList(&self, bstrprotocol: &windows_core::BSTR, pbstrexceptionlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getProxyBypassForLocal(&self, bstrprotocol: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn setProxyBypassForLocal(&self, bstrprotocol: &windows_core::BSTR, fbypassforlocal: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_maxBandwidth(&self) -> windows_core::Result<i32>;
    fn put_maxBandwidth(&self, lmaxbandwidth: i32) -> windows_core::Result<()>;
    fn get_downloadProgress(&self) -> windows_core::Result<i32>;
    fn get_encodedFrameRate(&self) -> windows_core::Result<i32>;
    fn get_framesSkipped(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPNetwork_Vtbl {
    pub const fn new<Identity: IWMPNetwork_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_bandWidth<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbandwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_bandWidth(this) {
                    Ok(ok__) => {
                        plbandwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_recoveredPackets<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plrecoveredpackets: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_recoveredPackets(this) {
                    Ok(ok__) => {
                        plrecoveredpackets.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_sourceProtocol<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsourceprotocol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_sourceProtocol(this) {
                    Ok(ok__) => {
                        pbstrsourceprotocol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_receivedPackets<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plreceivedpackets: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_receivedPackets(this) {
                    Ok(ok__) => {
                        plreceivedpackets.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_lostPackets<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllostpackets: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_lostPackets(this) {
                    Ok(ok__) => {
                        pllostpackets.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_receptionQuality<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plreceptionquality: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_receptionQuality(this) {
                    Ok(ok__) => {
                        plreceptionquality.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_bufferingCount<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbufferingcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_bufferingCount(this) {
                    Ok(ok__) => {
                        plbufferingcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_bufferingProgress<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbufferingprogress: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_bufferingProgress(this) {
                    Ok(ok__) => {
                        plbufferingprogress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_bufferingTime<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbufferingtime: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_bufferingTime(this) {
                    Ok(ok__) => {
                        plbufferingtime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_bufferingTime<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbufferingtime: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPNetwork_Impl::put_bufferingTime(this, core::mem::transmute_copy(&lbufferingtime)).into()
            }
        }
        unsafe extern "system" fn get_frameRate<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plframerate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_frameRate(this) {
                    Ok(ok__) => {
                        plframerate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_maxBitRate<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbitrate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_maxBitRate(this) {
                    Ok(ok__) => {
                        plbitrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_bitRate<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbitrate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_bitRate(this) {
                    Ok(ok__) => {
                        plbitrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getProxySettings<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, plproxysetting: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::getProxySettings(this, core::mem::transmute(&bstrprotocol)) {
                    Ok(ok__) => {
                        plproxysetting.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setProxySettings<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, lproxysetting: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPNetwork_Impl::setProxySettings(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&lproxysetting)).into()
            }
        }
        unsafe extern "system" fn getProxyName<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, pbstrproxyname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::getProxyName(this, core::mem::transmute(&bstrprotocol)) {
                    Ok(ok__) => {
                        pbstrproxyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setProxyName<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, bstrproxyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPNetwork_Impl::setProxyName(this, core::mem::transmute(&bstrprotocol), core::mem::transmute(&bstrproxyname)).into()
            }
        }
        unsafe extern "system" fn getProxyPort<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, lproxyport: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::getProxyPort(this, core::mem::transmute(&bstrprotocol)) {
                    Ok(ok__) => {
                        lproxyport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setProxyPort<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, lproxyport: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPNetwork_Impl::setProxyPort(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&lproxyport)).into()
            }
        }
        unsafe extern "system" fn getProxyExceptionList<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, pbstrexceptionlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::getProxyExceptionList(this, core::mem::transmute(&bstrprotocol)) {
                    Ok(ok__) => {
                        pbstrexceptionlist.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setProxyExceptionList<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, pbstrexceptionlist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPNetwork_Impl::setProxyExceptionList(this, core::mem::transmute(&bstrprotocol), core::mem::transmute(&pbstrexceptionlist)).into()
            }
        }
        unsafe extern "system" fn getProxyBypassForLocal<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, pfbypassforlocal: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::getProxyBypassForLocal(this, core::mem::transmute(&bstrprotocol)) {
                    Ok(ok__) => {
                        pfbypassforlocal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setProxyBypassForLocal<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: *mut core::ffi::c_void, fbypassforlocal: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPNetwork_Impl::setProxyBypassForLocal(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&fbypassforlocal)).into()
            }
        }
        unsafe extern "system" fn get_maxBandwidth<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxbandwidth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_maxBandwidth(this) {
                    Ok(ok__) => {
                        lmaxbandwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_maxBandwidth<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxbandwidth: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPNetwork_Impl::put_maxBandwidth(this, core::mem::transmute_copy(&lmaxbandwidth)).into()
            }
        }
        unsafe extern "system" fn get_downloadProgress<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldownloadprogress: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_downloadProgress(this) {
                    Ok(ok__) => {
                        pldownloadprogress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_encodedFrameRate<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plframerate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_encodedFrameRate(this) {
                    Ok(ok__) => {
                        plframerate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_framesSkipped<Identity: IWMPNetwork_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plframes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPNetwork_Impl::get_framesSkipped(this) {
                    Ok(ok__) => {
                        plframes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_bandWidth: get_bandWidth::<Identity, OFFSET>,
            get_recoveredPackets: get_recoveredPackets::<Identity, OFFSET>,
            get_sourceProtocol: get_sourceProtocol::<Identity, OFFSET>,
            get_receivedPackets: get_receivedPackets::<Identity, OFFSET>,
            get_lostPackets: get_lostPackets::<Identity, OFFSET>,
            get_receptionQuality: get_receptionQuality::<Identity, OFFSET>,
            get_bufferingCount: get_bufferingCount::<Identity, OFFSET>,
            get_bufferingProgress: get_bufferingProgress::<Identity, OFFSET>,
            get_bufferingTime: get_bufferingTime::<Identity, OFFSET>,
            put_bufferingTime: put_bufferingTime::<Identity, OFFSET>,
            get_frameRate: get_frameRate::<Identity, OFFSET>,
            get_maxBitRate: get_maxBitRate::<Identity, OFFSET>,
            get_bitRate: get_bitRate::<Identity, OFFSET>,
            getProxySettings: getProxySettings::<Identity, OFFSET>,
            setProxySettings: setProxySettings::<Identity, OFFSET>,
            getProxyName: getProxyName::<Identity, OFFSET>,
            setProxyName: setProxyName::<Identity, OFFSET>,
            getProxyPort: getProxyPort::<Identity, OFFSET>,
            setProxyPort: setProxyPort::<Identity, OFFSET>,
            getProxyExceptionList: getProxyExceptionList::<Identity, OFFSET>,
            setProxyExceptionList: setProxyExceptionList::<Identity, OFFSET>,
            getProxyBypassForLocal: getProxyBypassForLocal::<Identity, OFFSET>,
            setProxyBypassForLocal: setProxyBypassForLocal::<Identity, OFFSET>,
            get_maxBandwidth: get_maxBandwidth::<Identity, OFFSET>,
            put_maxBandwidth: put_maxBandwidth::<Identity, OFFSET>,
            get_downloadProgress: get_downloadProgress::<Identity, OFFSET>,
            get_encodedFrameRate: get_encodedFrameRate::<Identity, OFFSET>,
            get_framesSkipped: get_framesSkipped::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPNetwork as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPNetwork {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPPlayer, IWMPPlayer_Vtbl, 0x6bf52a4f_394a_11d3_b153_00c04f79faa6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPPlayer {
    type Target = IWMPCore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPPlayer, windows_core::IUnknown, super::IDispatch, IWMPCore);
#[cfg(feature = "oaidl")]
impl IWMPPlayer {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_enabled(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_enabled)(windows_core::Interface::as_raw(self), benabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_fullScreen(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_fullScreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_fullScreen(&self, bfullscreen: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_fullScreen)(windows_core::Interface::as_raw(self), bfullscreen) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_enableContextMenu(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_enableContextMenu)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_enableContextMenu(&self, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_enableContextMenu)(windows_core::Interface::as_raw(self), benablecontextmenu) }
    }
    pub unsafe fn put_uiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_uiMode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmode)) }
    }
    pub unsafe fn get_uiMode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_uiMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer_Vtbl {
    pub base__: IWMPCore_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_enabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_fullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_fullScreen: usize,
    #[cfg(feature = "wtypes")]
    pub put_fullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_fullScreen: usize,
    #[cfg(feature = "wtypes")]
    pub get_enableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_enableContextMenu: usize,
    #[cfg(feature = "wtypes")]
    pub put_enableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_enableContextMenu: usize,
    pub put_uiMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_uiMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPPlayer_Impl: IWMPCore_Impl {
    fn get_enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_enabled(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_fullScreen(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_fullScreen(&self, bfullscreen: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_enableContextMenu(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_enableContextMenu(&self, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn put_uiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_uiMode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPPlayer_Vtbl {
    pub const fn new<Identity: IWMPPlayer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_enabled<Identity: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer_Impl::get_enabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_enabled<Identity: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer_Impl::put_enabled(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn get_fullScreen<Identity: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfullscreen: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer_Impl::get_fullScreen(this) {
                    Ok(ok__) => {
                        pbfullscreen.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_fullScreen<Identity: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullscreen: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer_Impl::put_fullScreen(this, core::mem::transmute_copy(&bfullscreen)).into()
            }
        }
        unsafe extern "system" fn get_enableContextMenu<Identity: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenablecontextmenu: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer_Impl::get_enableContextMenu(this) {
                    Ok(ok__) => {
                        pbenablecontextmenu.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_enableContextMenu<Identity: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer_Impl::put_enableContextMenu(this, core::mem::transmute_copy(&benablecontextmenu)).into()
            }
        }
        unsafe extern "system" fn put_uiMode<Identity: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer_Impl::put_uiMode(this, core::mem::transmute(&bstrmode)).into()
            }
        }
        unsafe extern "system" fn get_uiMode<Identity: IWMPPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer_Impl::get_uiMode(this) {
                    Ok(ok__) => {
                        pbstrmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMPCore_Vtbl::new::<Identity, OFFSET>(),
            get_enabled: get_enabled::<Identity, OFFSET>,
            put_enabled: put_enabled::<Identity, OFFSET>,
            get_fullScreen: get_fullScreen::<Identity, OFFSET>,
            put_fullScreen: put_fullScreen::<Identity, OFFSET>,
            get_enableContextMenu: get_enableContextMenu::<Identity, OFFSET>,
            put_enableContextMenu: put_enableContextMenu::<Identity, OFFSET>,
            put_uiMode: put_uiMode::<Identity, OFFSET>,
            get_uiMode: get_uiMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayer as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPPlayer {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPPlayer2, IWMPPlayer2_Vtbl, 0x0e6b01d1_d407_4c85_bf5f_1c01f6150280);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPPlayer2 {
    type Target = IWMPCore;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPPlayer2, windows_core::IUnknown, super::IDispatch, IWMPCore);
#[cfg(feature = "oaidl")]
impl IWMPPlayer2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_enabled(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_enabled)(windows_core::Interface::as_raw(self), benabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_fullScreen(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_fullScreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_fullScreen(&self, bfullscreen: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_fullScreen)(windows_core::Interface::as_raw(self), bfullscreen) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_enableContextMenu(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_enableContextMenu)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_enableContextMenu(&self, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_enableContextMenu)(windows_core::Interface::as_raw(self), benablecontextmenu) }
    }
    pub unsafe fn put_uiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_uiMode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmode)) }
    }
    pub unsafe fn get_uiMode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_uiMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_stretchToFit(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_stretchToFit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_stretchToFit(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_stretchToFit)(windows_core::Interface::as_raw(self), benabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_windowlessVideo(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_windowlessVideo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_windowlessVideo(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_windowlessVideo)(windows_core::Interface::as_raw(self), benabled) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer2_Vtbl {
    pub base__: IWMPCore_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_enabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_fullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_fullScreen: usize,
    #[cfg(feature = "wtypes")]
    pub put_fullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_fullScreen: usize,
    #[cfg(feature = "wtypes")]
    pub get_enableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_enableContextMenu: usize,
    #[cfg(feature = "wtypes")]
    pub put_enableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_enableContextMenu: usize,
    pub put_uiMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_uiMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_stretchToFit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_stretchToFit: usize,
    #[cfg(feature = "wtypes")]
    pub put_stretchToFit: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_stretchToFit: usize,
    #[cfg(feature = "wtypes")]
    pub get_windowlessVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_windowlessVideo: usize,
    #[cfg(feature = "wtypes")]
    pub put_windowlessVideo: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_windowlessVideo: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPPlayer2_Impl: IWMPCore_Impl {
    fn get_enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_enabled(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_fullScreen(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_fullScreen(&self, bfullscreen: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_enableContextMenu(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_enableContextMenu(&self, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn put_uiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_uiMode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_stretchToFit(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_stretchToFit(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_windowlessVideo(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_windowlessVideo(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPPlayer2_Vtbl {
    pub const fn new<Identity: IWMPPlayer2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_enabled<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer2_Impl::get_enabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_enabled<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer2_Impl::put_enabled(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn get_fullScreen<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfullscreen: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer2_Impl::get_fullScreen(this) {
                    Ok(ok__) => {
                        pbfullscreen.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_fullScreen<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullscreen: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer2_Impl::put_fullScreen(this, core::mem::transmute_copy(&bfullscreen)).into()
            }
        }
        unsafe extern "system" fn get_enableContextMenu<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenablecontextmenu: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer2_Impl::get_enableContextMenu(this) {
                    Ok(ok__) => {
                        pbenablecontextmenu.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_enableContextMenu<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer2_Impl::put_enableContextMenu(this, core::mem::transmute_copy(&benablecontextmenu)).into()
            }
        }
        unsafe extern "system" fn put_uiMode<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer2_Impl::put_uiMode(this, core::mem::transmute(&bstrmode)).into()
            }
        }
        unsafe extern "system" fn get_uiMode<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer2_Impl::get_uiMode(this) {
                    Ok(ok__) => {
                        pbstrmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_stretchToFit<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer2_Impl::get_stretchToFit(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_stretchToFit<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer2_Impl::put_stretchToFit(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn get_windowlessVideo<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer2_Impl::get_windowlessVideo(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_windowlessVideo<Identity: IWMPPlayer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer2_Impl::put_windowlessVideo(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        Self {
            base__: IWMPCore_Vtbl::new::<Identity, OFFSET>(),
            get_enabled: get_enabled::<Identity, OFFSET>,
            put_enabled: put_enabled::<Identity, OFFSET>,
            get_fullScreen: get_fullScreen::<Identity, OFFSET>,
            put_fullScreen: put_fullScreen::<Identity, OFFSET>,
            get_enableContextMenu: get_enableContextMenu::<Identity, OFFSET>,
            put_enableContextMenu: put_enableContextMenu::<Identity, OFFSET>,
            put_uiMode: put_uiMode::<Identity, OFFSET>,
            get_uiMode: get_uiMode::<Identity, OFFSET>,
            get_stretchToFit: get_stretchToFit::<Identity, OFFSET>,
            put_stretchToFit: put_stretchToFit::<Identity, OFFSET>,
            get_windowlessVideo: get_windowlessVideo::<Identity, OFFSET>,
            put_windowlessVideo: put_windowlessVideo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayer2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPPlayer2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPPlayer3, IWMPPlayer3_Vtbl, 0x54062b68_052a_4c25_a39f_8b63346511d4);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPPlayer3 {
    type Target = IWMPCore2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPPlayer3, windows_core::IUnknown, super::IDispatch, IWMPCore, IWMPCore2);
#[cfg(feature = "oaidl")]
impl IWMPPlayer3 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_enabled(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_enabled)(windows_core::Interface::as_raw(self), benabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_fullScreen(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_fullScreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_fullScreen(&self, bfullscreen: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_fullScreen)(windows_core::Interface::as_raw(self), bfullscreen) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_enableContextMenu(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_enableContextMenu)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_enableContextMenu(&self, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_enableContextMenu)(windows_core::Interface::as_raw(self), benablecontextmenu) }
    }
    pub unsafe fn put_uiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_uiMode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmode)) }
    }
    pub unsafe fn get_uiMode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_uiMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_stretchToFit(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_stretchToFit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_stretchToFit(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_stretchToFit)(windows_core::Interface::as_raw(self), benabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_windowlessVideo(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_windowlessVideo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_windowlessVideo(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_windowlessVideo)(windows_core::Interface::as_raw(self), benabled) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer3_Vtbl {
    pub base__: IWMPCore2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_enabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_fullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_fullScreen: usize,
    #[cfg(feature = "wtypes")]
    pub put_fullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_fullScreen: usize,
    #[cfg(feature = "wtypes")]
    pub get_enableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_enableContextMenu: usize,
    #[cfg(feature = "wtypes")]
    pub put_enableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_enableContextMenu: usize,
    pub put_uiMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_uiMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_stretchToFit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_stretchToFit: usize,
    #[cfg(feature = "wtypes")]
    pub put_stretchToFit: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_stretchToFit: usize,
    #[cfg(feature = "wtypes")]
    pub get_windowlessVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_windowlessVideo: usize,
    #[cfg(feature = "wtypes")]
    pub put_windowlessVideo: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_windowlessVideo: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPPlayer3_Impl: IWMPCore2_Impl {
    fn get_enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_enabled(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_fullScreen(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_fullScreen(&self, bfullscreen: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_enableContextMenu(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_enableContextMenu(&self, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn put_uiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_uiMode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_stretchToFit(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_stretchToFit(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_windowlessVideo(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_windowlessVideo(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPPlayer3_Vtbl {
    pub const fn new<Identity: IWMPPlayer3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_enabled<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer3_Impl::get_enabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_enabled<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer3_Impl::put_enabled(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn get_fullScreen<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfullscreen: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer3_Impl::get_fullScreen(this) {
                    Ok(ok__) => {
                        pbfullscreen.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_fullScreen<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullscreen: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer3_Impl::put_fullScreen(this, core::mem::transmute_copy(&bfullscreen)).into()
            }
        }
        unsafe extern "system" fn get_enableContextMenu<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenablecontextmenu: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer3_Impl::get_enableContextMenu(this) {
                    Ok(ok__) => {
                        pbenablecontextmenu.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_enableContextMenu<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer3_Impl::put_enableContextMenu(this, core::mem::transmute_copy(&benablecontextmenu)).into()
            }
        }
        unsafe extern "system" fn put_uiMode<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer3_Impl::put_uiMode(this, core::mem::transmute(&bstrmode)).into()
            }
        }
        unsafe extern "system" fn get_uiMode<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer3_Impl::get_uiMode(this) {
                    Ok(ok__) => {
                        pbstrmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_stretchToFit<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer3_Impl::get_stretchToFit(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_stretchToFit<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer3_Impl::put_stretchToFit(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn get_windowlessVideo<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer3_Impl::get_windowlessVideo(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_windowlessVideo<Identity: IWMPPlayer3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer3_Impl::put_windowlessVideo(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        Self {
            base__: IWMPCore2_Vtbl::new::<Identity, OFFSET>(),
            get_enabled: get_enabled::<Identity, OFFSET>,
            put_enabled: put_enabled::<Identity, OFFSET>,
            get_fullScreen: get_fullScreen::<Identity, OFFSET>,
            put_fullScreen: put_fullScreen::<Identity, OFFSET>,
            get_enableContextMenu: get_enableContextMenu::<Identity, OFFSET>,
            put_enableContextMenu: put_enableContextMenu::<Identity, OFFSET>,
            put_uiMode: put_uiMode::<Identity, OFFSET>,
            get_uiMode: get_uiMode::<Identity, OFFSET>,
            get_stretchToFit: get_stretchToFit::<Identity, OFFSET>,
            put_stretchToFit: put_stretchToFit::<Identity, OFFSET>,
            get_windowlessVideo: get_windowlessVideo::<Identity, OFFSET>,
            put_windowlessVideo: put_windowlessVideo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayer3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID || iid == &<IWMPCore2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPPlayer3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPPlayer4, IWMPPlayer4_Vtbl, 0x6c497d62_8919_413c_82db_e935fb3ec584);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPPlayer4 {
    type Target = IWMPCore3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPPlayer4, windows_core::IUnknown, super::IDispatch, IWMPCore, IWMPCore2, IWMPCore3);
#[cfg(feature = "oaidl")]
impl IWMPPlayer4 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_enabled(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_enabled)(windows_core::Interface::as_raw(self), benabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_fullScreen(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_fullScreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_fullScreen(&self, bfullscreen: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_fullScreen)(windows_core::Interface::as_raw(self), bfullscreen) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_enableContextMenu(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_enableContextMenu)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_enableContextMenu(&self, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_enableContextMenu)(windows_core::Interface::as_raw(self), benablecontextmenu) }
    }
    pub unsafe fn put_uiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_uiMode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmode)) }
    }
    pub unsafe fn get_uiMode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_uiMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_stretchToFit(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_stretchToFit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_stretchToFit(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_stretchToFit)(windows_core::Interface::as_raw(self), benabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_windowlessVideo(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_windowlessVideo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_windowlessVideo(&self, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_windowlessVideo)(windows_core::Interface::as_raw(self), benabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_isRemote(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_isRemote)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_playerApplication(&self) -> windows_core::Result<IWMPPlayerApplication> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_playerApplication)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn openPlayer(&self, bstrurl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).openPlayer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayer4_Vtbl {
    pub base__: IWMPCore3_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_enabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_fullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_fullScreen: usize,
    #[cfg(feature = "wtypes")]
    pub put_fullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_fullScreen: usize,
    #[cfg(feature = "wtypes")]
    pub get_enableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_enableContextMenu: usize,
    #[cfg(feature = "wtypes")]
    pub put_enableContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_enableContextMenu: usize,
    pub put_uiMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_uiMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_stretchToFit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_stretchToFit: usize,
    #[cfg(feature = "wtypes")]
    pub put_stretchToFit: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_stretchToFit: usize,
    #[cfg(feature = "wtypes")]
    pub get_windowlessVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_windowlessVideo: usize,
    #[cfg(feature = "wtypes")]
    pub put_windowlessVideo: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_windowlessVideo: usize,
    #[cfg(feature = "wtypes")]
    pub get_isRemote: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_isRemote: usize,
    pub get_playerApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub openPlayer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPPlayer4_Impl: IWMPCore3_Impl {
    fn get_enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_enabled(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_fullScreen(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_fullScreen(&self, bfullscreen: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_enableContextMenu(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_enableContextMenu(&self, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn put_uiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_uiMode(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_stretchToFit(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_stretchToFit(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_windowlessVideo(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_windowlessVideo(&self, benabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_isRemote(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_playerApplication(&self) -> windows_core::Result<IWMPPlayerApplication>;
    fn openPlayer(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPPlayer4_Vtbl {
    pub const fn new<Identity: IWMPPlayer4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_enabled<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer4_Impl::get_enabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_enabled<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer4_Impl::put_enabled(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn get_fullScreen<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfullscreen: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer4_Impl::get_fullScreen(this) {
                    Ok(ok__) => {
                        pbfullscreen.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_fullScreen<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullscreen: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer4_Impl::put_fullScreen(this, core::mem::transmute_copy(&bfullscreen)).into()
            }
        }
        unsafe extern "system" fn get_enableContextMenu<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenablecontextmenu: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer4_Impl::get_enableContextMenu(this) {
                    Ok(ok__) => {
                        pbenablecontextmenu.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_enableContextMenu<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benablecontextmenu: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer4_Impl::put_enableContextMenu(this, core::mem::transmute_copy(&benablecontextmenu)).into()
            }
        }
        unsafe extern "system" fn put_uiMode<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer4_Impl::put_uiMode(this, core::mem::transmute(&bstrmode)).into()
            }
        }
        unsafe extern "system" fn get_uiMode<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer4_Impl::get_uiMode(this) {
                    Ok(ok__) => {
                        pbstrmode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_stretchToFit<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer4_Impl::get_stretchToFit(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_stretchToFit<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer4_Impl::put_stretchToFit(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn get_windowlessVideo<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer4_Impl::get_windowlessVideo(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_windowlessVideo<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer4_Impl::put_windowlessVideo(this, core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn get_isRemote<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarfisremote: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer4_Impl::get_isRemote(this) {
                    Ok(ok__) => {
                        pvarfisremote.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_playerApplication<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwmpplayerapplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayer4_Impl::get_playerApplication(this) {
                    Ok(ok__) => {
                        ppiwmpplayerapplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn openPlayer<Identity: IWMPPlayer4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayer4_Impl::openPlayer(this, core::mem::transmute(&bstrurl)).into()
            }
        }
        Self {
            base__: IWMPCore3_Vtbl::new::<Identity, OFFSET>(),
            get_enabled: get_enabled::<Identity, OFFSET>,
            put_enabled: put_enabled::<Identity, OFFSET>,
            get_fullScreen: get_fullScreen::<Identity, OFFSET>,
            put_fullScreen: put_fullScreen::<Identity, OFFSET>,
            get_enableContextMenu: get_enableContextMenu::<Identity, OFFSET>,
            put_enableContextMenu: put_enableContextMenu::<Identity, OFFSET>,
            put_uiMode: put_uiMode::<Identity, OFFSET>,
            get_uiMode: get_uiMode::<Identity, OFFSET>,
            get_stretchToFit: get_stretchToFit::<Identity, OFFSET>,
            put_stretchToFit: put_stretchToFit::<Identity, OFFSET>,
            get_windowlessVideo: get_windowlessVideo::<Identity, OFFSET>,
            put_windowlessVideo: put_windowlessVideo::<Identity, OFFSET>,
            get_isRemote: get_isRemote::<Identity, OFFSET>,
            get_playerApplication: get_playerApplication::<Identity, OFFSET>,
            openPlayer: openPlayer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayer4 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID || iid == &<IWMPCore2 as windows_core::Interface>::IID || iid == &<IWMPCore3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPPlayer4 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPPlayerApplication, IWMPPlayerApplication_Vtbl, 0x40897764_ceab_47be_ad4a_8e28537f9bbf);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPPlayerApplication {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPPlayerApplication, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPPlayerApplication {
    pub unsafe fn switchToPlayerApplication(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).switchToPlayerApplication)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn switchToControl(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).switchToControl)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_playerDocked(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_playerDocked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_hasDisplay(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_hasDisplay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayerApplication_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub switchToPlayerApplication: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub switchToControl: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_playerDocked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_playerDocked: usize,
    #[cfg(feature = "wtypes")]
    pub get_hasDisplay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_hasDisplay: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPPlayerApplication_Impl: super::IDispatch_Impl {
    fn switchToPlayerApplication(&self) -> windows_core::Result<()>;
    fn switchToControl(&self) -> windows_core::Result<()>;
    fn get_playerDocked(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_hasDisplay(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPPlayerApplication_Vtbl {
    pub const fn new<Identity: IWMPPlayerApplication_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn switchToPlayerApplication<Identity: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayerApplication_Impl::switchToPlayerApplication(this).into()
            }
        }
        unsafe extern "system" fn switchToControl<Identity: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayerApplication_Impl::switchToControl(this).into()
            }
        }
        unsafe extern "system" fn get_playerDocked<Identity: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbplayerdocked: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayerApplication_Impl::get_playerDocked(this) {
                    Ok(ok__) => {
                        pbplayerdocked.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_hasDisplay<Identity: IWMPPlayerApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbhasdisplay: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlayerApplication_Impl::get_hasDisplay(this) {
                    Ok(ok__) => {
                        pbhasdisplay.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            switchToPlayerApplication: switchToPlayerApplication::<Identity, OFFSET>,
            switchToControl: switchToControl::<Identity, OFFSET>,
            get_playerDocked: get_playerDocked::<Identity, OFFSET>,
            get_hasDisplay: get_hasDisplay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayerApplication as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPPlayerApplication {}
windows_core::imp::define_interface!(IWMPPlayerServices, IWMPPlayerServices_Vtbl, 0x1d01fbdb_ade2_4c8d_9842_c190b95c3306);
windows_core::imp::interface_hierarchy!(IWMPPlayerServices, windows_core::IUnknown);
impl IWMPPlayerServices {
    pub unsafe fn activateUIPlugin(&self, bstrplugin: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).activateUIPlugin)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrplugin)) }
    }
    pub unsafe fn setTaskPane(&self, bstrtaskpane: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setTaskPane)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtaskpane)) }
    }
    pub unsafe fn setTaskPaneURL(&self, bstrtaskpane: &windows_core::BSTR, bstrurl: &windows_core::BSTR, bstrfriendlyname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setTaskPaneURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrtaskpane), core::mem::transmute_copy(bstrurl), core::mem::transmute_copy(bstrfriendlyname)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayerServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub activateUIPlugin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setTaskPane: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setTaskPaneURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMPPlayerServices_Impl: windows_core::IUnknownImpl {
    fn activateUIPlugin(&self, bstrplugin: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setTaskPane(&self, bstrtaskpane: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setTaskPaneURL(&self, bstrtaskpane: &windows_core::BSTR, bstrurl: &windows_core::BSTR, bstrfriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IWMPPlayerServices_Vtbl {
    pub const fn new<Identity: IWMPPlayerServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn activateUIPlugin<Identity: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrplugin: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayerServices_Impl::activateUIPlugin(this, core::mem::transmute(&bstrplugin)).into()
            }
        }
        unsafe extern "system" fn setTaskPane<Identity: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskpane: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayerServices_Impl::setTaskPane(this, core::mem::transmute(&bstrtaskpane)).into()
            }
        }
        unsafe extern "system" fn setTaskPaneURL<Identity: IWMPPlayerServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskpane: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, bstrfriendlyname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayerServices_Impl::setTaskPaneURL(this, core::mem::transmute(&bstrtaskpane), core::mem::transmute(&bstrurl), core::mem::transmute(&bstrfriendlyname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            activateUIPlugin: activateUIPlugin::<Identity, OFFSET>,
            setTaskPane: setTaskPane::<Identity, OFFSET>,
            setTaskPaneURL: setTaskPaneURL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayerServices as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMPPlayerServices {}
windows_core::imp::define_interface!(IWMPPlayerServices2, IWMPPlayerServices2_Vtbl, 0x1bb1592f_f040_418a_9f71_17c7512b4d70);
impl core::ops::Deref for IWMPPlayerServices2 {
    type Target = IWMPPlayerServices;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPPlayerServices2, windows_core::IUnknown, IWMPPlayerServices);
impl IWMPPlayerServices2 {
    pub unsafe fn setBackgroundProcessingPriority(&self, bstrpriority: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setBackgroundProcessingPriority)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpriority)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlayerServices2_Vtbl {
    pub base__: IWMPPlayerServices_Vtbl,
    pub setBackgroundProcessingPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMPPlayerServices2_Impl: IWMPPlayerServices_Impl {
    fn setBackgroundProcessingPriority(&self, bstrpriority: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IWMPPlayerServices2_Vtbl {
    pub const fn new<Identity: IWMPPlayerServices2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn setBackgroundProcessingPriority<Identity: IWMPPlayerServices2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpriority: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlayerServices2_Impl::setBackgroundProcessingPriority(this, core::mem::transmute(&bstrpriority)).into()
            }
        }
        Self { base__: IWMPPlayerServices_Vtbl::new::<Identity, OFFSET>(), setBackgroundProcessingPriority: setBackgroundProcessingPriority::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayerServices2 as windows_core::Interface>::IID || iid == &<IWMPPlayerServices as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMPPlayerServices2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPPlaylist, IWMPPlaylist_Vtbl, 0xd5f0f4f1_130c_11d3_b14e_00c04f79faa6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPPlaylist {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPPlaylist, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPPlaylist {
    pub unsafe fn get_count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_name(&self, bstrname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)) }
    }
    pub unsafe fn get_attributeCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_attributeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_attributeName(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_attributeName)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_item(&self, lindex: i32) -> windows_core::Result<IWMPMedia> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getItemInfo(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getItemInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn setItemInfo(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setItemInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), core::mem::transmute_copy(bstrvalue)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_isIdentical<P0>(&self, piwmpplaylist: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_isIdentical)(windows_core::Interface::as_raw(self), piwmpplaylist.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn insertItem<P1>(&self, lindex: i32, piwmpmedia: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IWMPMedia>,
    {
        unsafe { (windows_core::Interface::vtable(self).insertItem)(windows_core::Interface::as_raw(self), lindex, piwmpmedia.param().abi()) }
    }
    pub unsafe fn appendItem<P0>(&self, piwmpmedia: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPMedia>,
    {
        unsafe { (windows_core::Interface::vtable(self).appendItem)(windows_core::Interface::as_raw(self), piwmpmedia.param().abi()) }
    }
    pub unsafe fn removeItem<P0>(&self, piwmpmedia: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPMedia>,
    {
        unsafe { (windows_core::Interface::vtable(self).removeItem)(windows_core::Interface::as_raw(self), piwmpmedia.param().abi()) }
    }
    pub unsafe fn moveItem(&self, lindexold: i32, lindexnew: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).moveItem)(windows_core::Interface::as_raw(self), lindexold, lindexnew) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlaylist_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_attributeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_attributeName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub setItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_isIdentical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_isIdentical: usize,
    pub clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub insertItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub appendItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub removeItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub moveItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPPlaylist_Impl: super::IDispatch_Impl {
    fn get_count(&self) -> windows_core::Result<i32>;
    fn get_name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_name(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_attributeCount(&self) -> windows_core::Result<i32>;
    fn get_attributeName(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn get_item(&self, lindex: i32) -> windows_core::Result<IWMPMedia>;
    fn getItemInfo(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn setItemInfo(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_isIdentical(&self, piwmpplaylist: windows_core::Ref<IWMPPlaylist>) -> windows_core::Result<super::VARIANT_BOOL>;
    fn clear(&self) -> windows_core::Result<()>;
    fn insertItem(&self, lindex: i32, piwmpmedia: windows_core::Ref<IWMPMedia>) -> windows_core::Result<()>;
    fn appendItem(&self, piwmpmedia: windows_core::Ref<IWMPMedia>) -> windows_core::Result<()>;
    fn removeItem(&self, piwmpmedia: windows_core::Ref<IWMPMedia>) -> windows_core::Result<()>;
    fn moveItem(&self, lindexold: i32, lindexnew: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPPlaylist_Vtbl {
    pub const fn new<Identity: IWMPPlaylist_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_count<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylist_Impl::get_count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_name<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylist_Impl::get_name(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_name<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlaylist_Impl::put_name(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn get_attributeCount<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylist_Impl::get_attributeCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_attributeName<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstrattributename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylist_Impl::get_attributeName(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pbstrattributename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_item<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylist_Impl::get_item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        ppiwmpmedia.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getItemInfo<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, pbstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylist_Impl::getItemInfo(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        pbstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setItemInfo<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlaylist_Impl::setItemInfo(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
            }
        }
        unsafe extern "system" fn get_isIdentical<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpplaylist: *mut core::ffi::c_void, pvbool: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylist_Impl::get_isIdentical(this, core::mem::transmute_copy(&piwmpplaylist)) {
                    Ok(ok__) => {
                        pvbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn clear<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlaylist_Impl::clear(this).into()
            }
        }
        unsafe extern "system" fn insertItem<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlaylist_Impl::insertItem(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&piwmpmedia)).into()
            }
        }
        unsafe extern "system" fn appendItem<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlaylist_Impl::appendItem(this, core::mem::transmute_copy(&piwmpmedia)).into()
            }
        }
        unsafe extern "system" fn removeItem<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlaylist_Impl::removeItem(this, core::mem::transmute_copy(&piwmpmedia)).into()
            }
        }
        unsafe extern "system" fn moveItem<Identity: IWMPPlaylist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindexold: i32, lindexnew: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlaylist_Impl::moveItem(this, core::mem::transmute_copy(&lindexold), core::mem::transmute_copy(&lindexnew)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_count: get_count::<Identity, OFFSET>,
            get_name: get_name::<Identity, OFFSET>,
            put_name: put_name::<Identity, OFFSET>,
            get_attributeCount: get_attributeCount::<Identity, OFFSET>,
            get_attributeName: get_attributeName::<Identity, OFFSET>,
            get_item: get_item::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            setItemInfo: setItemInfo::<Identity, OFFSET>,
            get_isIdentical: get_isIdentical::<Identity, OFFSET>,
            clear: clear::<Identity, OFFSET>,
            insertItem: insertItem::<Identity, OFFSET>,
            appendItem: appendItem::<Identity, OFFSET>,
            removeItem: removeItem::<Identity, OFFSET>,
            moveItem: moveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlaylist as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPPlaylist {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPPlaylistArray, IWMPPlaylistArray_Vtbl, 0x679409c0_99f7_11d3_9fb7_00105aa620bb);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPPlaylistArray {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPPlaylistArray, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPPlaylistArray {
    pub unsafe fn get_count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn item(&self, lindex: i32) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).item)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlaylistArray_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPPlaylistArray_Impl: super::IDispatch_Impl {
    fn get_count(&self) -> windows_core::Result<i32>;
    fn item(&self, lindex: i32) -> windows_core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPPlaylistArray_Vtbl {
    pub const fn new<Identity: IWMPPlaylistArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_count<Identity: IWMPPlaylistArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylistArray_Impl::get_count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn item<Identity: IWMPPlaylistArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylistArray_Impl::item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_count: get_count::<Identity, OFFSET>, item: item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlaylistArray as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPPlaylistArray {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPPlaylistCollection, IWMPPlaylistCollection_Vtbl, 0x10a13217_23a7_439b_b1c0_d847c79b7774);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPPlaylistCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPPlaylistCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPPlaylistCollection {
    pub unsafe fn newPlaylist(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).newPlaylist)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getAll(&self) -> windows_core::Result<IWMPPlaylistArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAll)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn getByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylistArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn remove<P0>(&self, pitem: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPPlaylist>,
    {
        unsafe { (windows_core::Interface::vtable(self).remove)(windows_core::Interface::as_raw(self), pitem.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn setDeleted<P0>(&self, pitem: P0, varfisdeleted: super::VARIANT_BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPPlaylist>,
    {
        unsafe { (windows_core::Interface::vtable(self).setDeleted)(windows_core::Interface::as_raw(self), pitem.param().abi(), varfisdeleted) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn isDeleted<P0>(&self, pitem: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<IWMPPlaylist>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isDeleted)(windows_core::Interface::as_raw(self), pitem.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn importPlaylist<P0>(&self, pitem: P0) -> windows_core::Result<IWMPPlaylist>
    where
        P0: windows_core::Param<IWMPPlaylist>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).importPlaylist)(windows_core::Interface::as_raw(self), pitem.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPPlaylistCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub newPlaylist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub setDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    setDeleted: usize,
    #[cfg(feature = "wtypes")]
    pub isDeleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isDeleted: usize,
    pub importPlaylist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPPlaylistCollection_Impl: super::IDispatch_Impl {
    fn newPlaylist(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getAll(&self) -> windows_core::Result<IWMPPlaylistArray>;
    fn getByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylistArray>;
    fn remove(&self, pitem: windows_core::Ref<IWMPPlaylist>) -> windows_core::Result<()>;
    fn setDeleted(&self, pitem: windows_core::Ref<IWMPPlaylist>, varfisdeleted: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn isDeleted(&self, pitem: windows_core::Ref<IWMPPlaylist>) -> windows_core::Result<super::VARIANT_BOOL>;
    fn importPlaylist(&self, pitem: windows_core::Ref<IWMPPlaylist>) -> windows_core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPPlaylistCollection_Vtbl {
    pub const fn new<Identity: IWMPPlaylistCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn newPlaylist<Identity: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylistCollection_Impl::newPlaylist(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAll<Identity: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplaylistarray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylistCollection_Impl::getAll(this) {
                    Ok(ok__) => {
                        ppplaylistarray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getByName<Identity: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, ppplaylistarray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylistCollection_Impl::getByName(this, core::mem::transmute(&bstrname)) {
                    Ok(ok__) => {
                        ppplaylistarray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn remove<Identity: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlaylistCollection_Impl::remove(this, core::mem::transmute_copy(&pitem)).into()
            }
        }
        unsafe extern "system" fn setDeleted<Identity: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, varfisdeleted: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPPlaylistCollection_Impl::setDeleted(this, core::mem::transmute_copy(&pitem), core::mem::transmute_copy(&varfisdeleted)).into()
            }
        }
        unsafe extern "system" fn isDeleted<Identity: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, pvarfisdeleted: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylistCollection_Impl::isDeleted(this, core::mem::transmute_copy(&pitem)) {
                    Ok(ok__) => {
                        pvarfisdeleted.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn importPlaylist<Identity: IWMPPlaylistCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, ppimporteditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPPlaylistCollection_Impl::importPlaylist(this, core::mem::transmute_copy(&pitem)) {
                    Ok(ok__) => {
                        ppimporteditem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            newPlaylist: newPlaylist::<Identity, OFFSET>,
            getAll: getAll::<Identity, OFFSET>,
            getByName: getByName::<Identity, OFFSET>,
            remove: remove::<Identity, OFFSET>,
            setDeleted: setDeleted::<Identity, OFFSET>,
            isDeleted: isDeleted::<Identity, OFFSET>,
            importPlaylist: importPlaylist::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlaylistCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPPlaylistCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPQuery, IWMPQuery_Vtbl, 0xa00918f3_a6b0_4bfb_9189_fd834c7bc5a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPQuery {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPQuery, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPQuery {
    pub unsafe fn addCondition(&self, bstrattribute: &windows_core::BSTR, bstroperator: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).addCondition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrattribute), core::mem::transmute_copy(bstroperator), core::mem::transmute_copy(bstrvalue)) }
    }
    pub unsafe fn beginNextGroup(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).beginNextGroup)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPQuery_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub addCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub beginNextGroup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPQuery_Impl: super::IDispatch_Impl {
    fn addCondition(&self, bstrattribute: &windows_core::BSTR, bstroperator: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn beginNextGroup(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPQuery_Vtbl {
    pub const fn new<Identity: IWMPQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn addCondition<Identity: IWMPQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: *mut core::ffi::c_void, bstroperator: *mut core::ffi::c_void, bstrvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPQuery_Impl::addCondition(this, core::mem::transmute(&bstrattribute), core::mem::transmute(&bstroperator), core::mem::transmute(&bstrvalue)).into()
            }
        }
        unsafe extern "system" fn beginNextGroup<Identity: IWMPQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPQuery_Impl::beginNextGroup(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            addCondition: addCondition::<Identity, OFFSET>,
            beginNextGroup: beginNextGroup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPQuery as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPQuery {}
windows_core::imp::define_interface!(IWMPRemoteMediaServices, IWMPRemoteMediaServices_Vtbl, 0xcbb92747_741f_44fe_ab5b_f1a48f3b2a59);
windows_core::imp::interface_hierarchy!(IWMPRemoteMediaServices, windows_core::IUnknown);
impl IWMPRemoteMediaServices {
    pub unsafe fn GetServiceType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetApplicationName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplicationName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetScriptableObject(&self, pbstrname: *mut windows_core::BSTR, ppdispatch: *mut Option<super::IDispatch>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScriptableObject)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrname), core::mem::transmute(ppdispatch)) }
    }
    pub unsafe fn GetCustomUIMode(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCustomUIMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPRemoteMediaServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetServiceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetApplicationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetScriptableObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetScriptableObject: usize,
    pub GetCustomUIMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IWMPRemoteMediaServices_Impl: windows_core::IUnknownImpl {
    fn GetServiceType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetApplicationName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetScriptableObject(&self, pbstrname: *mut windows_core::BSTR, ppdispatch: windows_core::OutRef<super::IDispatch>) -> windows_core::Result<()>;
    fn GetCustomUIMode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "oaidl")]
impl IWMPRemoteMediaServices_Vtbl {
    pub const fn new<Identity: IWMPRemoteMediaServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetServiceType<Identity: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPRemoteMediaServices_Impl::GetServiceType(this) {
                    Ok(ok__) => {
                        pbstrtype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplicationName<Identity: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPRemoteMediaServices_Impl::GetApplicationName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScriptableObject<Identity: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void, ppdispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPRemoteMediaServices_Impl::GetScriptableObject(this, core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&ppdispatch)).into()
            }
        }
        unsafe extern "system" fn GetCustomUIMode<Identity: IWMPRemoteMediaServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPRemoteMediaServices_Impl::GetCustomUIMode(this) {
                    Ok(ok__) => {
                        pbstrfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetServiceType: GetServiceType::<Identity, OFFSET>,
            GetApplicationName: GetApplicationName::<Identity, OFFSET>,
            GetScriptableObject: GetScriptableObject::<Identity, OFFSET>,
            GetCustomUIMode: GetCustomUIMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPRemoteMediaServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IWMPRemoteMediaServices {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPSettings, IWMPSettings_Vtbl, 0x9104d1ab_80c9_4fed_abf0_2e6417a6df14);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPSettings {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPSettings, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPSettings {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_isAvailable(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_isAvailable)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritem), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_autoStart(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_autoStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_autoStart(&self, fautostart: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_autoStart)(windows_core::Interface::as_raw(self), fautostart) }
    }
    pub unsafe fn get_baseURL(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_baseURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_baseURL(&self, bstrbaseurl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_baseURL)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrbaseurl)) }
    }
    pub unsafe fn get_defaultFrame(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_defaultFrame)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_defaultFrame(&self, bstrdefaultframe: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_defaultFrame)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdefaultframe)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_invokeURLs(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_invokeURLs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_invokeURLs(&self, finvokeurls: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_invokeURLs)(windows_core::Interface::as_raw(self), finvokeurls) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_mute(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_mute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_mute(&self, fmute: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_mute)(windows_core::Interface::as_raw(self), fmute) }
    }
    pub unsafe fn get_playCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_playCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_playCount(&self, lcount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_playCount)(windows_core::Interface::as_raw(self), lcount) }
    }
    pub unsafe fn get_rate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_rate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_rate(&self, drate: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_rate)(windows_core::Interface::as_raw(self), drate) }
    }
    pub unsafe fn get_balance(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_balance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_balance(&self, lbalance: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_balance)(windows_core::Interface::as_raw(self), lbalance) }
    }
    pub unsafe fn get_volume(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_volume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_volume(&self, lvolume: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_volume)(windows_core::Interface::as_raw(self), lvolume) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn getMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getMode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmode), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn setMode(&self, bstrmode: &windows_core::BSTR, varfmode: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setMode)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrmode), varfmode) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_enableErrorDialogs(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_enableErrorDialogs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_enableErrorDialogs(&self, fenableerrordialogs: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_enableErrorDialogs)(windows_core::Interface::as_raw(self), fenableerrordialogs) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_isAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_isAvailable: usize,
    #[cfg(feature = "wtypes")]
    pub get_autoStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_autoStart: usize,
    #[cfg(feature = "wtypes")]
    pub put_autoStart: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_autoStart: usize,
    pub get_baseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_baseURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_defaultFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_defaultFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_invokeURLs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_invokeURLs: usize,
    #[cfg(feature = "wtypes")]
    pub put_invokeURLs: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_invokeURLs: usize,
    #[cfg(feature = "wtypes")]
    pub get_mute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_mute: usize,
    #[cfg(feature = "wtypes")]
    pub put_mute: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_mute: usize,
    pub get_playCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_playCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_rate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub put_rate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub get_balance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_balance: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_volume: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub getMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    getMode: usize,
    #[cfg(feature = "wtypes")]
    pub setMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    setMode: usize,
    #[cfg(feature = "wtypes")]
    pub get_enableErrorDialogs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_enableErrorDialogs: usize,
    #[cfg(feature = "wtypes")]
    pub put_enableErrorDialogs: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_enableErrorDialogs: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPSettings_Impl: super::IDispatch_Impl {
    fn get_isAvailable(&self, bstritem: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_autoStart(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_autoStart(&self, fautostart: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_baseURL(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_baseURL(&self, bstrbaseurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_defaultFrame(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_defaultFrame(&self, bstrdefaultframe: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_invokeURLs(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_invokeURLs(&self, finvokeurls: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_mute(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_mute(&self, fmute: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_playCount(&self) -> windows_core::Result<i32>;
    fn put_playCount(&self, lcount: i32) -> windows_core::Result<()>;
    fn get_rate(&self) -> windows_core::Result<f64>;
    fn put_rate(&self, drate: f64) -> windows_core::Result<()>;
    fn get_balance(&self) -> windows_core::Result<i32>;
    fn put_balance(&self, lbalance: i32) -> windows_core::Result<()>;
    fn get_volume(&self) -> windows_core::Result<i32>;
    fn put_volume(&self, lvolume: i32) -> windows_core::Result<()>;
    fn getMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn setMode(&self, bstrmode: &windows_core::BSTR, varfmode: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_enableErrorDialogs(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_enableErrorDialogs(&self, fenableerrordialogs: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPSettings_Vtbl {
    pub const fn new<Identity: IWMPSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_isAvailable<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: *mut core::ffi::c_void, pisavailable: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_isAvailable(this, core::mem::transmute(&bstritem)) {
                    Ok(ok__) => {
                        pisavailable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_autoStart<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfautostart: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_autoStart(this) {
                    Ok(ok__) => {
                        pfautostart.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_autoStart<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fautostart: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_autoStart(this, core::mem::transmute_copy(&fautostart)).into()
            }
        }
        unsafe extern "system" fn get_baseURL<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbaseurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_baseURL(this) {
                    Ok(ok__) => {
                        pbstrbaseurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_baseURL<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbaseurl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_baseURL(this, core::mem::transmute(&bstrbaseurl)).into()
            }
        }
        unsafe extern "system" fn get_defaultFrame<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdefaultframe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_defaultFrame(this) {
                    Ok(ok__) => {
                        pbstrdefaultframe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_defaultFrame<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdefaultframe: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_defaultFrame(this, core::mem::transmute(&bstrdefaultframe)).into()
            }
        }
        unsafe extern "system" fn get_invokeURLs<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfinvokeurls: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_invokeURLs(this) {
                    Ok(ok__) => {
                        pfinvokeurls.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_invokeURLs<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finvokeurls: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_invokeURLs(this, core::mem::transmute_copy(&finvokeurls)).into()
            }
        }
        unsafe extern "system" fn get_mute<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmute: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_mute(this) {
                    Ok(ok__) => {
                        pfmute.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_mute<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmute: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_mute(this, core::mem::transmute_copy(&fmute)).into()
            }
        }
        unsafe extern "system" fn get_playCount<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_playCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_playCount<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_playCount(this, core::mem::transmute_copy(&lcount)).into()
            }
        }
        unsafe extern "system" fn get_rate<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdrate: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_rate(this) {
                    Ok(ok__) => {
                        pdrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_rate<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, drate: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_rate(this, core::mem::transmute_copy(&drate)).into()
            }
        }
        unsafe extern "system" fn get_balance<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbalance: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_balance(this) {
                    Ok(ok__) => {
                        plbalance.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_balance<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbalance: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_balance(this, core::mem::transmute_copy(&lbalance)).into()
            }
        }
        unsafe extern "system" fn get_volume<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_volume(this) {
                    Ok(ok__) => {
                        plvolume.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_volume<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_volume(this, core::mem::transmute_copy(&lvolume)).into()
            }
        }
        unsafe extern "system" fn getMode<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: *mut core::ffi::c_void, pvarfmode: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::getMode(this, core::mem::transmute(&bstrmode)) {
                    Ok(ok__) => {
                        pvarfmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn setMode<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: *mut core::ffi::c_void, varfmode: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::setMode(this, core::mem::transmute(&bstrmode), core::mem::transmute_copy(&varfmode)).into()
            }
        }
        unsafe extern "system" fn get_enableErrorDialogs<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenableerrordialogs: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings_Impl::get_enableErrorDialogs(this) {
                    Ok(ok__) => {
                        pfenableerrordialogs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_enableErrorDialogs<Identity: IWMPSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenableerrordialogs: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSettings_Impl::put_enableErrorDialogs(this, core::mem::transmute_copy(&fenableerrordialogs)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_isAvailable: get_isAvailable::<Identity, OFFSET>,
            get_autoStart: get_autoStart::<Identity, OFFSET>,
            put_autoStart: put_autoStart::<Identity, OFFSET>,
            get_baseURL: get_baseURL::<Identity, OFFSET>,
            put_baseURL: put_baseURL::<Identity, OFFSET>,
            get_defaultFrame: get_defaultFrame::<Identity, OFFSET>,
            put_defaultFrame: put_defaultFrame::<Identity, OFFSET>,
            get_invokeURLs: get_invokeURLs::<Identity, OFFSET>,
            put_invokeURLs: put_invokeURLs::<Identity, OFFSET>,
            get_mute: get_mute::<Identity, OFFSET>,
            put_mute: put_mute::<Identity, OFFSET>,
            get_playCount: get_playCount::<Identity, OFFSET>,
            put_playCount: put_playCount::<Identity, OFFSET>,
            get_rate: get_rate::<Identity, OFFSET>,
            put_rate: put_rate::<Identity, OFFSET>,
            get_balance: get_balance::<Identity, OFFSET>,
            put_balance: put_balance::<Identity, OFFSET>,
            get_volume: get_volume::<Identity, OFFSET>,
            put_volume: put_volume::<Identity, OFFSET>,
            getMode: getMode::<Identity, OFFSET>,
            setMode: setMode::<Identity, OFFSET>,
            get_enableErrorDialogs: get_enableErrorDialogs::<Identity, OFFSET>,
            put_enableErrorDialogs: put_enableErrorDialogs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPSettings {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPSettings2, IWMPSettings2_Vtbl, 0xfda937a4_eece_4da5_a0b6_39bf89ade2c2);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPSettings2 {
    type Target = IWMPSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPSettings2, windows_core::IUnknown, super::IDispatch, IWMPSettings);
#[cfg(feature = "oaidl")]
impl IWMPSettings2 {
    pub unsafe fn get_defaultAudioLanguage(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_defaultAudioLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_mediaAccessRights(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_mediaAccessRights)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn requestMediaAccessRights(&self, bstrdesiredaccess: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).requestMediaAccessRights)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdesiredaccess), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSettings2_Vtbl {
    pub base__: IWMPSettings_Vtbl,
    pub get_defaultAudioLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_mediaAccessRights: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub requestMediaAccessRights: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    requestMediaAccessRights: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPSettings2_Impl: IWMPSettings_Impl {
    fn get_defaultAudioLanguage(&self) -> windows_core::Result<i32>;
    fn get_mediaAccessRights(&self) -> windows_core::Result<windows_core::BSTR>;
    fn requestMediaAccessRights(&self, bstrdesiredaccess: &windows_core::BSTR) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPSettings2_Vtbl {
    pub const fn new<Identity: IWMPSettings2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_defaultAudioLanguage<Identity: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllangid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings2_Impl::get_defaultAudioLanguage(this) {
                    Ok(ok__) => {
                        pllangid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_mediaAccessRights<Identity: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrights: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings2_Impl::get_mediaAccessRights(this) {
                    Ok(ok__) => {
                        pbstrrights.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn requestMediaAccessRights<Identity: IWMPSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdesiredaccess: *mut core::ffi::c_void, pvbaccepted: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSettings2_Impl::requestMediaAccessRights(this, core::mem::transmute(&bstrdesiredaccess)) {
                    Ok(ok__) => {
                        pvbaccepted.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMPSettings_Vtbl::new::<Identity, OFFSET>(),
            get_defaultAudioLanguage: get_defaultAudioLanguage::<Identity, OFFSET>,
            get_mediaAccessRights: get_mediaAccessRights::<Identity, OFFSET>,
            requestMediaAccessRights: requestMediaAccessRights::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSettings2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPSettings as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPSettings2 {}
windows_core::imp::define_interface!(IWMPSkinManager, IWMPSkinManager_Vtbl, 0x076f2fa6_ed30_448b_8cc5_3f3ef3529c7a);
windows_core::imp::interface_hierarchy!(IWMPSkinManager, windows_core::IUnknown);
impl IWMPSkinManager {
    pub unsafe fn SetVisualStyle(&self, bstrpath: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVisualStyle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpath)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSkinManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetVisualStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMPSkinManager_Impl: windows_core::IUnknownImpl {
    fn SetVisualStyle(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl IWMPSkinManager_Vtbl {
    pub const fn new<Identity: IWMPSkinManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetVisualStyle<Identity: IWMPSkinManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSkinManager_Impl::SetVisualStyle(this, core::mem::transmute(&bstrpath)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetVisualStyle: SetVisualStyle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSkinManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMPSkinManager {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPStringCollection, IWMPStringCollection_Vtbl, 0x4a976298_8c0d_11d3_b389_00c04f68574b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPStringCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPStringCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWMPStringCollection {
    pub unsafe fn get_count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn item(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).item)(windows_core::Interface::as_raw(self), lindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPStringCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPStringCollection_Impl: super::IDispatch_Impl {
    fn get_count(&self) -> windows_core::Result<i32>;
    fn item(&self, lindex: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPStringCollection_Vtbl {
    pub const fn new<Identity: IWMPStringCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_count<Identity: IWMPStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPStringCollection_Impl::get_count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn item<Identity: IWMPStringCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstrstring: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPStringCollection_Impl::item(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        pbstrstring.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_count: get_count::<Identity, OFFSET>, item: item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPStringCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPStringCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWMPStringCollection2, IWMPStringCollection2_Vtbl, 0x46ad648d_53f1_4a74_92e2_2a1b68d63fd4);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWMPStringCollection2 {
    type Target = IWMPStringCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWMPStringCollection2, windows_core::IUnknown, super::IDispatch, IWMPStringCollection);
#[cfg(feature = "oaidl")]
impl IWMPStringCollection2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn isIdentical<P0>(&self, piwmpstringcollection2: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isIdentical)(windows_core::Interface::as_raw(self), piwmpstringcollection2.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getItemInfo(&self, lcollectionindex: i32, bstritemname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getItemInfo)(windows_core::Interface::as_raw(self), lcollectionindex, core::mem::transmute_copy(bstritemname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn getAttributeCountByType(&self, lcollectionindex: i32, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getAttributeCountByType)(windows_core::Interface::as_raw(self), lcollectionindex, core::mem::transmute_copy(bstrtype), core::mem::transmute_copy(bstrlanguage), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn getItemInfoByType(&self, lcollectionindex: i32, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR, lattributeindex: i32) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getItemInfoByType)(windows_core::Interface::as_raw(self), lcollectionindex, core::mem::transmute_copy(bstrtype), core::mem::transmute_copy(bstrlanguage), lattributeindex, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWMPStringCollection2_Vtbl {
    pub base__: IWMPStringCollection_Vtbl,
    #[cfg(feature = "wtypes")]
    pub isIdentical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isIdentical: usize,
    pub getItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub getAttributeCountByType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub getItemInfoByType: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    getItemInfoByType: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWMPStringCollection2_Impl: IWMPStringCollection_Impl {
    fn isIdentical(&self, piwmpstringcollection2: windows_core::Ref<IWMPStringCollection2>) -> windows_core::Result<super::VARIANT_BOOL>;
    fn getItemInfo(&self, lcollectionindex: i32, bstritemname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getAttributeCountByType(&self, lcollectionindex: i32, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn getItemInfoByType(&self, lcollectionindex: i32, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR, lattributeindex: i32) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWMPStringCollection2_Vtbl {
    pub const fn new<Identity: IWMPStringCollection2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn isIdentical<Identity: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpstringcollection2: *mut core::ffi::c_void, pvbool: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPStringCollection2_Impl::isIdentical(this, core::mem::transmute_copy(&piwmpstringcollection2)) {
                    Ok(ok__) => {
                        pvbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getItemInfo<Identity: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcollectionindex: i32, bstritemname: *mut core::ffi::c_void, pbstrvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPStringCollection2_Impl::getItemInfo(this, core::mem::transmute_copy(&lcollectionindex), core::mem::transmute(&bstritemname)) {
                    Ok(ok__) => {
                        pbstrvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getAttributeCountByType<Identity: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcollectionindex: i32, bstrtype: *mut core::ffi::c_void, bstrlanguage: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPStringCollection2_Impl::getAttributeCountByType(this, core::mem::transmute_copy(&lcollectionindex), core::mem::transmute(&bstrtype), core::mem::transmute(&bstrlanguage)) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getItemInfoByType<Identity: IWMPStringCollection2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcollectionindex: i32, bstrtype: *mut core::ffi::c_void, bstrlanguage: *mut core::ffi::c_void, lattributeindex: i32, pvarvalue: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPStringCollection2_Impl::getItemInfoByType(this, core::mem::transmute_copy(&lcollectionindex), core::mem::transmute(&bstrtype), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&lattributeindex)) {
                    Ok(ok__) => {
                        pvarvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWMPStringCollection_Vtbl::new::<Identity, OFFSET>(),
            isIdentical: isIdentical::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            getAttributeCountByType: getAttributeCountByType::<Identity, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPStringCollection2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWMPStringCollection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWMPStringCollection2 {}
windows_core::imp::define_interface!(IWMPSyncDevice, IWMPSyncDevice_Vtbl, 0x82a2986c_0293_4fd0_b279_b21b86c058be);
windows_core::imp::interface_hierarchy!(IWMPSyncDevice, windows_core::IUnknown);
impl IWMPSyncDevice {
    pub unsafe fn get_friendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_friendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_friendlyName(&self, bstrname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_friendlyName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname)) }
    }
    pub unsafe fn get_deviceName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_deviceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_deviceId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_deviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_partnershipIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_partnershipIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_connected(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_connected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_status(&self) -> windows_core::Result<WMPDeviceStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_syncState(&self) -> windows_core::Result<WMPSyncState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_syncState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_progress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_progress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getItemInfo(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getItemInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn createPartnership(&self, vbshowui: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).createPartnership)(windows_core::Interface::as_raw(self), vbshowui) }
    }
    pub unsafe fn deletePartnership(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).deletePartnership)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn start(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).start)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn showSettings(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).showSettings)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn isIdentical<P0>(&self, pdevice: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).isIdentical)(windows_core::Interface::as_raw(self), pdevice.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_friendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_friendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_deviceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_deviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_partnershipIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_connected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_connected: usize,
    pub get_status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMPDeviceStatus) -> windows_core::HRESULT,
    pub get_syncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WMPSyncState) -> windows_core::HRESULT,
    pub get_progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub createPartnership: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    createPartnership: usize,
    pub deletePartnership: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub showSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub isIdentical: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    isIdentical: usize,
}
#[cfg(feature = "wtypes")]
pub trait IWMPSyncDevice_Impl: windows_core::IUnknownImpl {
    fn get_friendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_friendlyName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_deviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_deviceId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_partnershipIndex(&self) -> windows_core::Result<i32>;
    fn get_connected(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_status(&self) -> windows_core::Result<WMPDeviceStatus>;
    fn get_syncState(&self) -> windows_core::Result<WMPSyncState>;
    fn get_progress(&self) -> windows_core::Result<i32>;
    fn getItemInfo(&self, bstritemname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn createPartnership(&self, vbshowui: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn deletePartnership(&self) -> windows_core::Result<()>;
    fn start(&self) -> windows_core::Result<()>;
    fn stop(&self) -> windows_core::Result<()>;
    fn showSettings(&self) -> windows_core::Result<()>;
    fn isIdentical(&self, pdevice: windows_core::Ref<IWMPSyncDevice>) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(feature = "wtypes")]
impl IWMPSyncDevice_Vtbl {
    pub const fn new<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_friendlyName<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::get_friendlyName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_friendlyName<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSyncDevice_Impl::put_friendlyName(this, core::mem::transmute(&bstrname)).into()
            }
        }
        unsafe extern "system" fn get_deviceName<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::get_deviceName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_deviceId<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdeviceid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::get_deviceId(this) {
                    Ok(ok__) => {
                        pbstrdeviceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_partnershipIndex<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::get_partnershipIndex(this) {
                    Ok(ok__) => {
                        plindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_connected<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbconnected: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::get_connected(this) {
                    Ok(ok__) => {
                        pvbconnected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_status<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::get_status(this) {
                    Ok(ok__) => {
                        pwmpds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_syncState<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpss: *mut WMPSyncState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::get_syncState(this) {
                    Ok(ok__) => {
                        pwmpss.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_progress<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprogress: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::get_progress(this) {
                    Ok(ok__) => {
                        plprogress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getItemInfo<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: *mut core::ffi::c_void, pbstrval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::getItemInfo(this, core::mem::transmute(&bstritemname)) {
                    Ok(ok__) => {
                        pbstrval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn createPartnership<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbshowui: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSyncDevice_Impl::createPartnership(this, core::mem::transmute_copy(&vbshowui)).into()
            }
        }
        unsafe extern "system" fn deletePartnership<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSyncDevice_Impl::deletePartnership(this).into()
            }
        }
        unsafe extern "system" fn start<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSyncDevice_Impl::start(this).into()
            }
        }
        unsafe extern "system" fn stop<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSyncDevice_Impl::stop(this).into()
            }
        }
        unsafe extern "system" fn showSettings<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSyncDevice_Impl::showSettings(this).into()
            }
        }
        unsafe extern "system" fn isIdentical<Identity: IWMPSyncDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pvbool: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncDevice_Impl::isIdentical(this, core::mem::transmute_copy(&pdevice)) {
                    Ok(ok__) => {
                        pvbool.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_friendlyName: get_friendlyName::<Identity, OFFSET>,
            put_friendlyName: put_friendlyName::<Identity, OFFSET>,
            get_deviceName: get_deviceName::<Identity, OFFSET>,
            get_deviceId: get_deviceId::<Identity, OFFSET>,
            get_partnershipIndex: get_partnershipIndex::<Identity, OFFSET>,
            get_connected: get_connected::<Identity, OFFSET>,
            get_status: get_status::<Identity, OFFSET>,
            get_syncState: get_syncState::<Identity, OFFSET>,
            get_progress: get_progress::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            createPartnership: createPartnership::<Identity, OFFSET>,
            deletePartnership: deletePartnership::<Identity, OFFSET>,
            start: start::<Identity, OFFSET>,
            stop: stop::<Identity, OFFSET>,
            showSettings: showSettings::<Identity, OFFSET>,
            isIdentical: isIdentical::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSyncDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IWMPSyncDevice {}
windows_core::imp::define_interface!(IWMPSyncDevice2, IWMPSyncDevice2_Vtbl, 0x88afb4b2_140a_44d2_91e6_4543da467cd1);
impl core::ops::Deref for IWMPSyncDevice2 {
    type Target = IWMPSyncDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPSyncDevice2, windows_core::IUnknown, IWMPSyncDevice);
impl IWMPSyncDevice2 {
    pub unsafe fn setItemInfo(&self, bstritemname: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).setItemInfo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstritemname), core::mem::transmute_copy(bstrval)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncDevice2_Vtbl {
    pub base__: IWMPSyncDevice_Vtbl,
    pub setItemInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypes")]
pub trait IWMPSyncDevice2_Impl: IWMPSyncDevice_Impl {
    fn setItemInfo(&self, bstritemname: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IWMPSyncDevice2_Vtbl {
    pub const fn new<Identity: IWMPSyncDevice2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn setItemInfo<Identity: IWMPSyncDevice2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: *mut core::ffi::c_void, bstrval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSyncDevice2_Impl::setItemInfo(this, core::mem::transmute(&bstritemname), core::mem::transmute(&bstrval)).into()
            }
        }
        Self { base__: IWMPSyncDevice_Vtbl::new::<Identity, OFFSET>(), setItemInfo: setItemInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSyncDevice2 as windows_core::Interface>::IID || iid == &<IWMPSyncDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IWMPSyncDevice2 {}
windows_core::imp::define_interface!(IWMPSyncDevice3, IWMPSyncDevice3_Vtbl, 0xb22c85f9_263c_4372_a0da_b518db9b4098);
impl core::ops::Deref for IWMPSyncDevice3 {
    type Target = IWMPSyncDevice2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWMPSyncDevice3, windows_core::IUnknown, IWMPSyncDevice, IWMPSyncDevice2);
impl IWMPSyncDevice3 {
    #[cfg(feature = "oaidl")]
    pub unsafe fn estimateSyncSize<P0, P1>(&self, pnonruleplaylist: P0, prulesplaylist: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWMPPlaylist>,
        P1: windows_core::Param<IWMPPlaylist>,
    {
        unsafe { (windows_core::Interface::vtable(self).estimateSyncSize)(windows_core::Interface::as_raw(self), pnonruleplaylist.param().abi(), prulesplaylist.param().abi()) }
    }
    pub unsafe fn cancelEstimation(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).cancelEstimation)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncDevice3_Vtbl {
    pub base__: IWMPSyncDevice2_Vtbl,
    #[cfg(feature = "oaidl")]
    pub estimateSyncSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    estimateSyncSize: usize,
    pub cancelEstimation: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
pub trait IWMPSyncDevice3_Impl: IWMPSyncDevice2_Impl {
    fn estimateSyncSize(&self, pnonruleplaylist: windows_core::Ref<IWMPPlaylist>, prulesplaylist: windows_core::Ref<IWMPPlaylist>) -> windows_core::Result<()>;
    fn cancelEstimation(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl IWMPSyncDevice3_Vtbl {
    pub const fn new<Identity: IWMPSyncDevice3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn estimateSyncSize<Identity: IWMPSyncDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnonruleplaylist: *mut core::ffi::c_void, prulesplaylist: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSyncDevice3_Impl::estimateSyncSize(this, core::mem::transmute_copy(&pnonruleplaylist), core::mem::transmute_copy(&prulesplaylist)).into()
            }
        }
        unsafe extern "system" fn cancelEstimation<Identity: IWMPSyncDevice3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWMPSyncDevice3_Impl::cancelEstimation(this).into()
            }
        }
        Self {
            base__: IWMPSyncDevice2_Vtbl::new::<Identity, OFFSET>(),
            estimateSyncSize: estimateSyncSize::<Identity, OFFSET>,
            cancelEstimation: cancelEstimation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSyncDevice3 as windows_core::Interface>::IID || iid == &<IWMPSyncDevice as windows_core::Interface>::IID || iid == &<IWMPSyncDevice2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes"))]
impl windows_core::RuntimeName for IWMPSyncDevice3 {}
windows_core::imp::define_interface!(IWMPSyncServices, IWMPSyncServices_Vtbl, 0x8b5050ff_e0a4_4808_b3a8_893a9e1ed894);
windows_core::imp::interface_hierarchy!(IWMPSyncServices, windows_core::IUnknown);
impl IWMPSyncServices {
    pub unsafe fn get_deviceCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_deviceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn getDevice(&self, lindex: i32) -> windows_core::Result<IWMPSyncDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).getDevice)(windows_core::Interface::as_raw(self), lindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWMPSyncServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_deviceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub getDevice: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWMPSyncServices_Impl: windows_core::IUnknownImpl {
    fn get_deviceCount(&self) -> windows_core::Result<i32>;
    fn getDevice(&self, lindex: i32) -> windows_core::Result<IWMPSyncDevice>;
}
impl IWMPSyncServices_Vtbl {
    pub const fn new<Identity: IWMPSyncServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_deviceCount<Identity: IWMPSyncServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncServices_Impl::get_deviceCount(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn getDevice<Identity: IWMPSyncServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWMPSyncServices_Impl::getDevice(this, core::mem::transmute_copy(&lindex)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_deviceCount: get_deviceCount::<Identity, OFFSET>,
            getDevice: getDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSyncServices as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWMPSyncServices {}
pub type WMPBurnFormat = i32;
pub type WMPBurnState = i32;
pub type WMPDeviceStatus = i32;
pub type WMPFolderScanState = i32;
pub const WMPLib: windows_core::GUID = windows_core::GUID::from_u128(0x6bf52a50_394a_11d3_b153_00c04f79faa6);
pub type WMPLibraryType = i32;
pub type WMPOpenState = i32;
pub type WMPPlayState = i32;
pub type WMPPlaylistChangeEventType = i32;
pub const WMPRemoteMediaServices: windows_core::GUID = windows_core::GUID::from_u128(0xdf333473_2cf7_4be2_907f_9aad5661364f);
pub type WMPRipState = i32;
pub type WMPStringCollectionChangeEventType = i32;
pub type WMPSyncState = i32;
pub const WindowsMediaPlayer: windows_core::GUID = windows_core::GUID::from_u128(0x6bf52a52_394a_11d3_b153_00c04f79faa6);
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(_WMPOCXEvents, _WMPOCXEvents_Vtbl, 0x6bf52a51_394a_11d3_b153_00c04f79faa6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for _WMPOCXEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(_WMPOCXEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct _WMPOCXEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait _WMPOCXEvents_Impl: super::IDispatch_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl _WMPOCXEvents_Vtbl {
    pub const fn new<Identity: _WMPOCXEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_WMPOCXEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for _WMPOCXEvents {}
pub const wmpbfAudioCD: WMPBurnFormat = 0;
pub const wmpbfDataCD: WMPBurnFormat = 1;
pub const wmpbsBurning: WMPBurnState = 6;
pub const wmpbsBusy: WMPBurnState = 1;
pub const wmpbsDownloading: WMPBurnState = 9;
pub const wmpbsErasing: WMPBurnState = 8;
pub const wmpbsPreparingToBurn: WMPBurnState = 5;
pub const wmpbsReady: WMPBurnState = 2;
pub const wmpbsRefreshStatusPending: WMPBurnState = 4;
pub const wmpbsStopped: WMPBurnState = 7;
pub const wmpbsUnknown: WMPBurnState = 0;
pub const wmpbsWaitingForDisc: WMPBurnState = 3;
pub const wmpdsLast: WMPDeviceStatus = 6;
pub const wmpdsManualDevice: WMPDeviceStatus = 4;
pub const wmpdsNewDevice: WMPDeviceStatus = 5;
pub const wmpdsPartnershipAnother: WMPDeviceStatus = 3;
pub const wmpdsPartnershipDeclined: WMPDeviceStatus = 2;
pub const wmpdsPartnershipExists: WMPDeviceStatus = 1;
pub const wmpdsUnknown: WMPDeviceStatus = 0;
pub const wmpfssScanning: WMPFolderScanState = 1;
pub const wmpfssStopped: WMPFolderScanState = 3;
pub const wmpfssUnknown: WMPFolderScanState = 0;
pub const wmpfssUpdating: WMPFolderScanState = 2;
pub const wmplcAppend: WMPPlaylistChangeEventType = 6;
pub const wmplcClear: WMPPlaylistChangeEventType = 1;
pub const wmplcDelete: WMPPlaylistChangeEventType = 4;
pub const wmplcInfoChange: WMPPlaylistChangeEventType = 2;
pub const wmplcInsert: WMPPlaylistChangeEventType = 5;
pub const wmplcLast: WMPPlaylistChangeEventType = 11;
pub const wmplcMorph: WMPPlaylistChangeEventType = 9;
pub const wmplcMove: WMPPlaylistChangeEventType = 3;
pub const wmplcNameChange: WMPPlaylistChangeEventType = 8;
pub const wmplcPrivate: WMPPlaylistChangeEventType = 7;
pub const wmplcSort: WMPPlaylistChangeEventType = 10;
pub const wmplcUnknown: WMPPlaylistChangeEventType = 0;
pub const wmpltAll: WMPLibraryType = 1;
pub const wmpltDisc: WMPLibraryType = 4;
pub const wmpltLocal: WMPLibraryType = 2;
pub const wmpltPortableDevice: WMPLibraryType = 5;
pub const wmpltRemote: WMPLibraryType = 3;
pub const wmpltUnknown: WMPLibraryType = 0;
pub const wmposBeginCodecAcquisition: WMPOpenState = 14;
pub const wmposBeginIndividualization: WMPOpenState = 18;
pub const wmposBeginLicenseAcquisition: WMPOpenState = 16;
pub const wmposEndCodecAcquisition: WMPOpenState = 15;
pub const wmposEndIndividualization: WMPOpenState = 19;
pub const wmposEndLicenseAcquisition: WMPOpenState = 17;
pub const wmposMediaChanging: WMPOpenState = 8;
pub const wmposMediaConnecting: WMPOpenState = 10;
pub const wmposMediaLoading: WMPOpenState = 11;
pub const wmposMediaLocating: WMPOpenState = 9;
pub const wmposMediaOpen: WMPOpenState = 13;
pub const wmposMediaOpening: WMPOpenState = 12;
pub const wmposMediaWaiting: WMPOpenState = 20;
pub const wmposOpeningUnknownURL: WMPOpenState = 21;
pub const wmposPlaylistChanged: WMPOpenState = 7;
pub const wmposPlaylistChanging: WMPOpenState = 1;
pub const wmposPlaylistConnecting: WMPOpenState = 3;
pub const wmposPlaylistLoading: WMPOpenState = 4;
pub const wmposPlaylistLocating: WMPOpenState = 2;
pub const wmposPlaylistOpenNoMedia: WMPOpenState = 6;
pub const wmposPlaylistOpening: WMPOpenState = 5;
pub const wmposUndefined: WMPOpenState = 0;
pub const wmppsBuffering: WMPPlayState = 6;
pub const wmppsLast: WMPPlayState = 12;
pub const wmppsMediaEnded: WMPPlayState = 8;
pub const wmppsPaused: WMPPlayState = 2;
pub const wmppsPlaying: WMPPlayState = 3;
pub const wmppsReady: WMPPlayState = 10;
pub const wmppsReconnecting: WMPPlayState = 11;
pub const wmppsScanForward: WMPPlayState = 4;
pub const wmppsScanReverse: WMPPlayState = 5;
pub const wmppsStopped: WMPPlayState = 1;
pub const wmppsTransitioning: WMPPlayState = 9;
pub const wmppsUndefined: WMPPlayState = 0;
pub const wmppsWaiting: WMPPlayState = 7;
pub const wmprsRipping: WMPRipState = 1;
pub const wmprsStopped: WMPRipState = 2;
pub const wmprsUnknown: WMPRipState = 0;
pub const wmpsccetBeginUpdates: WMPStringCollectionChangeEventType = 5;
pub const wmpsccetChange: WMPStringCollectionChangeEventType = 2;
pub const wmpsccetClear: WMPStringCollectionChangeEventType = 4;
pub const wmpsccetDelete: WMPStringCollectionChangeEventType = 3;
pub const wmpsccetEndUpdates: WMPStringCollectionChangeEventType = 6;
pub const wmpsccetInsert: WMPStringCollectionChangeEventType = 1;
pub const wmpsccetUnknown: WMPStringCollectionChangeEventType = 0;
pub const wmpssEstimating: WMPSyncState = 3;
pub const wmpssLast: WMPSyncState = 4;
pub const wmpssStopped: WMPSyncState = 2;
pub const wmpssSynchronizing: WMPSyncState = 1;
pub const wmpssUnknown: WMPSyncState = 0;
