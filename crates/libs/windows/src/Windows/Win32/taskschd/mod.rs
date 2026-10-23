#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IAction, IAction_Vtbl, 0xbae54997_48b1_4cbe_9965_d6be263ebea4);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IAction {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IAction, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IAction {
    pub unsafe fn get_Id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Id(&self, id: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Id)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id)) }
    }
    pub unsafe fn get_Type(&self) -> windows_core::Result<TASK_ACTION_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IAction_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_ACTION_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IAction_Impl: super::IDispatch_Impl {
    fn get_Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Id(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Type(&self) -> windows_core::Result<TASK_ACTION_TYPE>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IAction_Vtbl {
    pub const fn new<Identity: IAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Id<Identity: IAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAction_Impl::get_Id(this) {
                    Ok(ok__) => {
                        pid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Id<Identity: IAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAction_Impl::put_Id(this, core::mem::transmute(&id)).into()
            }
        }
        unsafe extern "system" fn get_Type<Identity: IAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAction_Impl::get_Type(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Id: get_Id::<Identity, OFFSET>,
            put_Id: put_Id::<Identity, OFFSET>,
            get_Type: get_Type::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAction as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IAction {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IActionCollection, IActionCollection_Vtbl, 0x02820e19_7b98_4ed2_b2e8_fdccceff619b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IActionCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IActionCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IActionCollection {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_XmlText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_XmlText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_XmlText(&self, text: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)) }
    }
    pub unsafe fn Create(&self, r#type: TASK_ACTION_TYPE) -> windows_core::Result<IAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, index: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index)) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_Context(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Context)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Context(&self, context: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Context)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(context)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IActionCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_ACTION_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IActionCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: i32) -> windows_core::Result<IAction>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_XmlText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_XmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Create(&self, r#type: TASK_ACTION_TYPE) -> windows_core::Result<IAction>;
    fn Remove(&self, index: &super::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn get_Context(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Context(&self, context: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IActionCollection_Vtbl {
    pub const fn new<Identity: IActionCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, ppaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_XmlText<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionCollection_Impl::get_XmlText(this) {
                    Ok(ok__) => {
                        ptext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_XmlText<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::put_XmlText(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn Create<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionCollection_Impl::Create(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        ppaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::Remove(this, core::mem::transmute(&index)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn get_Context<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActionCollection_Impl::get_Context(this) {
                    Ok(ok__) => {
                        pcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Context<Identity: IActionCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActionCollection_Impl::put_Context(this, core::mem::transmute(&context)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_XmlText: get_XmlText::<Identity, OFFSET>,
            put_XmlText: put_XmlText::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            get_Context: get_Context::<Identity, OFFSET>,
            put_Context: put_Context::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActionCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IActionCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IBootTrigger, IBootTrigger_Vtbl, 0x2a9c35da_d357_41f4_bbc1_207ac1b1f3cb);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IBootTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IBootTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl IBootTrigger {
    pub unsafe fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Delay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Delay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBootTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBootTrigger_Impl: ITrigger_Impl {
    fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBootTrigger_Vtbl {
    pub const fn new<Identity: IBootTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Delay<Identity: IBootTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootTrigger_Impl::get_Delay(this) {
                    Ok(ok__) => {
                        pdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Delay<Identity: IBootTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootTrigger_Impl::put_Delay(this, core::mem::transmute(&delay)).into()
            }
        }
        Self { base__: ITrigger_Vtbl::new::<Identity, OFFSET>(), get_Delay: get_Delay::<Identity, OFFSET>, put_Delay: put_Delay::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBootTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBootTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IComHandlerAction, IComHandlerAction_Vtbl, 0x6d2fd252_75c5_4f66_90ba_2a7d8cc3039f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IComHandlerAction {
    type Target = IAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IComHandlerAction, windows_core::IUnknown, super::IDispatch, IAction);
#[cfg(feature = "oaidl")]
impl IComHandlerAction {
    pub unsafe fn get_ClassId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClassId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ClassId(&self, clsid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClassId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(clsid)) }
    }
    pub unsafe fn get_Data(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Data)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Data(&self, data: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Data)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IComHandlerAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub get_ClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IComHandlerAction_Impl: IAction_Impl {
    fn get_ClassId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ClassId(&self, clsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Data(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IComHandlerAction_Vtbl {
    pub const fn new<Identity: IComHandlerAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ClassId<Identity: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComHandlerAction_Impl::get_ClassId(this) {
                    Ok(ok__) => {
                        pclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClassId<Identity: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComHandlerAction_Impl::put_ClassId(this, core::mem::transmute(&clsid)).into()
            }
        }
        unsafe extern "system" fn get_Data<Identity: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IComHandlerAction_Impl::get_Data(this) {
                    Ok(ok__) => {
                        pdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Data<Identity: IComHandlerAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IComHandlerAction_Impl::put_Data(this, core::mem::transmute(&data)).into()
            }
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            get_ClassId: get_ClassId::<Identity, OFFSET>,
            put_ClassId: put_ClassId::<Identity, OFFSET>,
            get_Data: get_Data::<Identity, OFFSET>,
            put_Data: put_Data::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IComHandlerAction as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IComHandlerAction {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDailyTrigger, IDailyTrigger_Vtbl, 0x126c5cd8_b288_41d5_8dbf_e491446adc5c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDailyTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDailyTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl IDailyTrigger {
    pub unsafe fn get_DaysInterval(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DaysInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DaysInterval(&self, days: i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DaysInterval)(windows_core::Interface::as_raw(self), days) }
    }
    pub unsafe fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RandomDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDailyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_DaysInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub put_DaysInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub get_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDailyTrigger_Impl: ITrigger_Impl {
    fn get_DaysInterval(&self) -> windows_core::Result<i16>;
    fn put_DaysInterval(&self, days: i16) -> windows_core::Result<()>;
    fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDailyTrigger_Vtbl {
    pub const fn new<Identity: IDailyTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DaysInterval<Identity: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDailyTrigger_Impl::get_DaysInterval(this) {
                    Ok(ok__) => {
                        pdays.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DaysInterval<Identity: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDailyTrigger_Impl::put_DaysInterval(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn get_RandomDelay<Identity: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDailyTrigger_Impl::get_RandomDelay(this) {
                    Ok(ok__) => {
                        prandomdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RandomDelay<Identity: IDailyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDailyTrigger_Impl::put_RandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            get_DaysInterval: get_DaysInterval::<Identity, OFFSET>,
            put_DaysInterval: put_DaysInterval::<Identity, OFFSET>,
            get_RandomDelay: get_RandomDelay::<Identity, OFFSET>,
            put_RandomDelay: put_RandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDailyTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDailyTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IEmailAction, IEmailAction_Vtbl, 0x10f62c64_7e16_4314_a0c2_0c3683f99d40);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IEmailAction {
    type Target = IAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IEmailAction, windows_core::IUnknown, super::IDispatch, IAction);
#[cfg(feature = "oaidl")]
impl IEmailAction {
    pub unsafe fn get_Server(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Server)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Server(&self, server: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Server)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(server)) }
    }
    pub unsafe fn get_Subject(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Subject)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Subject(&self, subject: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Subject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(subject)) }
    }
    pub unsafe fn get_To(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_To)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_To(&self, to: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_To)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(to)) }
    }
    pub unsafe fn get_Cc(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Cc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Cc(&self, cc: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Cc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(cc)) }
    }
    pub unsafe fn get_Bcc(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Bcc)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Bcc(&self, bcc: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Bcc)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bcc)) }
    }
    pub unsafe fn get_ReplyTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReplyTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ReplyTo(&self, replyto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ReplyTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(replyto)) }
    }
    pub unsafe fn get_From(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_From)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_From(&self, from: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_From)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(from)) }
    }
    pub unsafe fn get_HeaderFields(&self) -> windows_core::Result<ITaskNamedValueCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HeaderFields)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_HeaderFields<P0>(&self, pheaderfields: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITaskNamedValueCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_HeaderFields)(windows_core::Interface::as_raw(self), pheaderfields.param().abi()) }
    }
    pub unsafe fn get_Body(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Body)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Body(&self, body: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Body)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(body)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_Attachments(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Attachments)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_Attachments(&self, pattachements: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Attachments)(windows_core::Interface::as_raw(self), pattachements) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub get_Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Server: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_To: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Cc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Cc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Bcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Bcc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ReplyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_From: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_HeaderFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_HeaderFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Body: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_Attachments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_Attachments: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_Attachments: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_Attachments: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IEmailAction_Impl: IAction_Impl {
    fn get_Server(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Server(&self, server: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Subject(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Subject(&self, subject: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_To(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_To(&self, to: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Cc(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Cc(&self, cc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Bcc(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Bcc(&self, bcc: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ReplyTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ReplyTo(&self, replyto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_From(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_From(&self, from: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_HeaderFields(&self) -> windows_core::Result<ITaskNamedValueCollection>;
    fn put_HeaderFields(&self, pheaderfields: windows_core::Ref<ITaskNamedValueCollection>) -> windows_core::Result<()>;
    fn get_Body(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Body(&self, body: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Attachments(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_Attachments(&self, pattachements: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IEmailAction_Vtbl {
    pub const fn new<Identity: IEmailAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Server<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_Server(this) {
                    Ok(ok__) => {
                        pserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Server<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, server: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_Server(this, core::mem::transmute(&server)).into()
            }
        }
        unsafe extern "system" fn get_Subject<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_Subject(this) {
                    Ok(ok__) => {
                        psubject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Subject<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_Subject(this, core::mem::transmute(&subject)).into()
            }
        }
        unsafe extern "system" fn get_To<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_To(this) {
                    Ok(ok__) => {
                        pto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_To<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, to: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_To(this, core::mem::transmute(&to)).into()
            }
        }
        unsafe extern "system" fn get_Cc<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_Cc(this) {
                    Ok(ok__) => {
                        pcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Cc<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_Cc(this, core::mem::transmute(&cc)).into()
            }
        }
        unsafe extern "system" fn get_Bcc<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_Bcc(this) {
                    Ok(ok__) => {
                        pbcc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Bcc<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_Bcc(this, core::mem::transmute(&bcc)).into()
            }
        }
        unsafe extern "system" fn get_ReplyTo<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preplyto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_ReplyTo(this) {
                    Ok(ok__) => {
                        preplyto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ReplyTo<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, replyto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_ReplyTo(this, core::mem::transmute(&replyto)).into()
            }
        }
        unsafe extern "system" fn get_From<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_From(this) {
                    Ok(ok__) => {
                        pfrom.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_From<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, from: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_From(this, core::mem::transmute(&from)).into()
            }
        }
        unsafe extern "system" fn get_HeaderFields<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppheaderfields: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_HeaderFields(this) {
                    Ok(ok__) => {
                        ppheaderfields.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HeaderFields<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheaderfields: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_HeaderFields(this, core::mem::transmute_copy(&pheaderfields)).into()
            }
        }
        unsafe extern "system" fn get_Body<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_Body(this) {
                    Ok(ok__) => {
                        pbody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Body<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, body: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_Body(this, core::mem::transmute(&body)).into()
            }
        }
        unsafe extern "system" fn get_Attachments<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattachements: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEmailAction_Impl::get_Attachments(this) {
                    Ok(ok__) => {
                        pattachements.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Attachments<Identity: IEmailAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattachements: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEmailAction_Impl::put_Attachments(this, core::mem::transmute_copy(&pattachements)).into()
            }
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            get_Server: get_Server::<Identity, OFFSET>,
            put_Server: put_Server::<Identity, OFFSET>,
            get_Subject: get_Subject::<Identity, OFFSET>,
            put_Subject: put_Subject::<Identity, OFFSET>,
            get_To: get_To::<Identity, OFFSET>,
            put_To: put_To::<Identity, OFFSET>,
            get_Cc: get_Cc::<Identity, OFFSET>,
            put_Cc: put_Cc::<Identity, OFFSET>,
            get_Bcc: get_Bcc::<Identity, OFFSET>,
            put_Bcc: put_Bcc::<Identity, OFFSET>,
            get_ReplyTo: get_ReplyTo::<Identity, OFFSET>,
            put_ReplyTo: put_ReplyTo::<Identity, OFFSET>,
            get_From: get_From::<Identity, OFFSET>,
            put_From: put_From::<Identity, OFFSET>,
            get_HeaderFields: get_HeaderFields::<Identity, OFFSET>,
            put_HeaderFields: put_HeaderFields::<Identity, OFFSET>,
            get_Body: get_Body::<Identity, OFFSET>,
            put_Body: put_Body::<Identity, OFFSET>,
            get_Attachments: get_Attachments::<Identity, OFFSET>,
            put_Attachments: put_Attachments::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEmailAction as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IEmailAction {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IEventTrigger, IEventTrigger_Vtbl, 0xd45b0167_9653_4eef_b94f_0732ca7af251);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IEventTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IEventTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl IEventTrigger {
    pub unsafe fn get_Subscription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Subscription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Subscription(&self, query: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Subscription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(query)) }
    }
    pub unsafe fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Delay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Delay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)) }
    }
    pub unsafe fn get_ValueQueries(&self) -> windows_core::Result<ITaskNamedValueCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ValueQueries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_ValueQueries<P0>(&self, pnamedxpaths: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITaskNamedValueCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_ValueQueries)(windows_core::Interface::as_raw(self), pnamedxpaths.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IEventTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_Subscription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Subscription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ValueQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ValueQueries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IEventTrigger_Impl: ITrigger_Impl {
    fn get_Subscription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Subscription(&self, query: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ValueQueries(&self) -> windows_core::Result<ITaskNamedValueCollection>;
    fn put_ValueQueries(&self, pnamedxpaths: windows_core::Ref<ITaskNamedValueCollection>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IEventTrigger_Vtbl {
    pub const fn new<Identity: IEventTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Subscription<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEventTrigger_Impl::get_Subscription(this) {
                    Ok(ok__) => {
                        pquery.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Subscription<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, query: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventTrigger_Impl::put_Subscription(this, core::mem::transmute(&query)).into()
            }
        }
        unsafe extern "system" fn get_Delay<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEventTrigger_Impl::get_Delay(this) {
                    Ok(ok__) => {
                        pdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Delay<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventTrigger_Impl::put_Delay(this, core::mem::transmute(&delay)).into()
            }
        }
        unsafe extern "system" fn get_ValueQueries<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnamedxpaths: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEventTrigger_Impl::get_ValueQueries(this) {
                    Ok(ok__) => {
                        ppnamedxpaths.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ValueQueries<Identity: IEventTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamedxpaths: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEventTrigger_Impl::put_ValueQueries(this, core::mem::transmute_copy(&pnamedxpaths)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            get_Subscription: get_Subscription::<Identity, OFFSET>,
            put_Subscription: put_Subscription::<Identity, OFFSET>,
            get_Delay: get_Delay::<Identity, OFFSET>,
            put_Delay: put_Delay::<Identity, OFFSET>,
            get_ValueQueries: get_ValueQueries::<Identity, OFFSET>,
            put_ValueQueries: put_ValueQueries::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IEventTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IExecAction, IExecAction_Vtbl, 0x4c3d624d_fd6b_49a3_b9b7_09cb3cd3f047);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IExecAction {
    type Target = IAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IExecAction, windows_core::IUnknown, super::IDispatch, IAction);
#[cfg(feature = "oaidl")]
impl IExecAction {
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Path(&self, path: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Path)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)) }
    }
    pub unsafe fn get_Arguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Arguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Arguments(&self, argument: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Arguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(argument)) }
    }
    pub unsafe fn get_WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_WorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(workingdirectory)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IExecAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IExecAction_Impl: IAction_Impl {
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Path(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Arguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Arguments(&self, argument: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_WorkingDirectory(&self, workingdirectory: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IExecAction_Vtbl {
    pub const fn new<Identity: IExecAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Path<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExecAction_Impl::get_Path(this) {
                    Ok(ok__) => {
                        ppath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Path<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction_Impl::put_Path(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn get_Arguments<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pargument: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExecAction_Impl::get_Arguments(this) {
                    Ok(ok__) => {
                        pargument.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Arguments<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, argument: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction_Impl::put_Arguments(this, core::mem::transmute(&argument)).into()
            }
        }
        unsafe extern "system" fn get_WorkingDirectory<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pworkingdirectory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExecAction_Impl::get_WorkingDirectory(this) {
                    Ok(ok__) => {
                        pworkingdirectory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WorkingDirectory<Identity: IExecAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, workingdirectory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction_Impl::put_WorkingDirectory(this, core::mem::transmute(&workingdirectory)).into()
            }
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            get_Path: get_Path::<Identity, OFFSET>,
            put_Path: put_Path::<Identity, OFFSET>,
            get_Arguments: get_Arguments::<Identity, OFFSET>,
            put_Arguments: put_Arguments::<Identity, OFFSET>,
            get_WorkingDirectory: get_WorkingDirectory::<Identity, OFFSET>,
            put_WorkingDirectory: put_WorkingDirectory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExecAction as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IExecAction {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IExecAction2, IExecAction2_Vtbl, 0xf2a82542_bda5_4e6b_9143_e2bf4f8987b6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IExecAction2 {
    type Target = IExecAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IExecAction2, windows_core::IUnknown, super::IDispatch, IAction, IExecAction);
#[cfg(feature = "oaidl")]
impl IExecAction2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_HideAppWindow(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HideAppWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_HideAppWindow(&self, hideappwindow: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_HideAppWindow)(windows_core::Interface::as_raw(self), hideappwindow) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IExecAction2_Vtbl {
    pub base__: IExecAction_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_HideAppWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_HideAppWindow: usize,
    #[cfg(feature = "wtypes")]
    pub put_HideAppWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_HideAppWindow: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IExecAction2_Impl: IExecAction_Impl {
    fn get_HideAppWindow(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_HideAppWindow(&self, hideappwindow: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IExecAction2_Vtbl {
    pub const fn new<Identity: IExecAction2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_HideAppWindow<Identity: IExecAction2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phideappwindow: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExecAction2_Impl::get_HideAppWindow(this) {
                    Ok(ok__) => {
                        phideappwindow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_HideAppWindow<Identity: IExecAction2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hideappwindow: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExecAction2_Impl::put_HideAppWindow(this, core::mem::transmute_copy(&hideappwindow)).into()
            }
        }
        Self {
            base__: IExecAction_Vtbl::new::<Identity, OFFSET>(),
            get_HideAppWindow: get_HideAppWindow::<Identity, OFFSET>,
            put_HideAppWindow: put_HideAppWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExecAction2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID || iid == &<IExecAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IExecAction2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IIdleSettings, IIdleSettings_Vtbl, 0x84594461_0053_4342_a8fd_088fabf11f32);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IIdleSettings {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IIdleSettings, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IIdleSettings {
    pub unsafe fn get_IdleDuration(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IdleDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_IdleDuration(&self, delay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IdleDuration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)) }
    }
    pub unsafe fn get_WaitTimeout(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WaitTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_WaitTimeout(&self, timeout: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WaitTimeout)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(timeout)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_StopOnIdleEnd(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StopOnIdleEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_StopOnIdleEnd(&self, stop: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StopOnIdleEnd)(windows_core::Interface::as_raw(self), stop) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RestartOnIdle(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RestartOnIdle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_RestartOnIdle(&self, restart: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RestartOnIdle)(windows_core::Interface::as_raw(self), restart) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IIdleSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_IdleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_IdleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_WaitTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_WaitTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_StopOnIdleEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_StopOnIdleEnd: usize,
    #[cfg(feature = "wtypes")]
    pub put_StopOnIdleEnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_StopOnIdleEnd: usize,
    #[cfg(feature = "wtypes")]
    pub get_RestartOnIdle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RestartOnIdle: usize,
    #[cfg(feature = "wtypes")]
    pub put_RestartOnIdle: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_RestartOnIdle: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IIdleSettings_Impl: super::IDispatch_Impl {
    fn get_IdleDuration(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_IdleDuration(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_WaitTimeout(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_WaitTimeout(&self, timeout: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_StopOnIdleEnd(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_StopOnIdleEnd(&self, stop: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RestartOnIdle(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_RestartOnIdle(&self, restart: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IIdleSettings_Vtbl {
    pub const fn new<Identity: IIdleSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IdleDuration<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdleSettings_Impl::get_IdleDuration(this) {
                    Ok(ok__) => {
                        pdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IdleDuration<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::put_IdleDuration(this, core::mem::transmute(&delay)).into()
            }
        }
        unsafe extern "system" fn get_WaitTimeout<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimeout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdleSettings_Impl::get_WaitTimeout(this) {
                    Ok(ok__) => {
                        ptimeout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WaitTimeout<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::put_WaitTimeout(this, core::mem::transmute(&timeout)).into()
            }
        }
        unsafe extern "system" fn get_StopOnIdleEnd<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstop: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdleSettings_Impl::get_StopOnIdleEnd(this) {
                    Ok(ok__) => {
                        pstop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StopOnIdleEnd<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::put_StopOnIdleEnd(this, core::mem::transmute_copy(&stop)).into()
            }
        }
        unsafe extern "system" fn get_RestartOnIdle<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestart: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIdleSettings_Impl::get_RestartOnIdle(this) {
                    Ok(ok__) => {
                        prestart.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RestartOnIdle<Identity: IIdleSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restart: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIdleSettings_Impl::put_RestartOnIdle(this, core::mem::transmute_copy(&restart)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_IdleDuration: get_IdleDuration::<Identity, OFFSET>,
            put_IdleDuration: put_IdleDuration::<Identity, OFFSET>,
            get_WaitTimeout: get_WaitTimeout::<Identity, OFFSET>,
            put_WaitTimeout: put_WaitTimeout::<Identity, OFFSET>,
            get_StopOnIdleEnd: get_StopOnIdleEnd::<Identity, OFFSET>,
            put_StopOnIdleEnd: put_StopOnIdleEnd::<Identity, OFFSET>,
            get_RestartOnIdle: get_RestartOnIdle::<Identity, OFFSET>,
            put_RestartOnIdle: put_RestartOnIdle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdleSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IIdleSettings {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IIdleTrigger, IIdleTrigger_Vtbl, 0xd537d2b0_9fb3_4d34_9739_1ff5ce7b1ef3);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IIdleTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IIdleTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IIdleTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IIdleTrigger_Impl: ITrigger_Impl {}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IIdleTrigger_Vtbl {
    pub const fn new<Identity: IIdleTrigger_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ITrigger_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIdleTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IIdleTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ILogonTrigger, ILogonTrigger_Vtbl, 0x72dade38_fae4_4b3e_baf4_5d009af02b1c);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ILogonTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ILogonTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl ILogonTrigger {
    pub unsafe fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Delay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Delay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)) }
    }
    pub unsafe fn get_UserId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_UserId(&self, user: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UserId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(user)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ILogonTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ILogonTrigger_Impl: ITrigger_Impl {
    fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_UserId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_UserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ILogonTrigger_Vtbl {
    pub const fn new<Identity: ILogonTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Delay<Identity: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILogonTrigger_Impl::get_Delay(this) {
                    Ok(ok__) => {
                        pdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Delay<Identity: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILogonTrigger_Impl::put_Delay(this, core::mem::transmute(&delay)).into()
            }
        }
        unsafe extern "system" fn get_UserId<Identity: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILogonTrigger_Impl::get_UserId(this) {
                    Ok(ok__) => {
                        puser.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UserId<Identity: ILogonTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILogonTrigger_Impl::put_UserId(this, core::mem::transmute(&user)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            get_Delay: get_Delay::<Identity, OFFSET>,
            put_Delay: put_Delay::<Identity, OFFSET>,
            get_UserId: get_UserId::<Identity, OFFSET>,
            put_UserId: put_UserId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILogonTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ILogonTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMaintenanceSettings, IMaintenanceSettings_Vtbl, 0xa6024fa8_9652_4adb_a6bf_5cfcd877a7ba);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMaintenanceSettings {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMaintenanceSettings, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IMaintenanceSettings {
    pub unsafe fn put_Period(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Period)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_Period(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Period)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Deadline(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Deadline)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_Deadline(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Deadline)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Exclusive(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Exclusive)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Exclusive(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Exclusive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub put_Period: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Period: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub put_Exclusive: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Exclusive: usize,
    #[cfg(feature = "wtypes")]
    pub get_Exclusive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Exclusive: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMaintenanceSettings_Impl: super::IDispatch_Impl {
    fn put_Period(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Period(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Deadline(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Deadline(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Exclusive(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Exclusive(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMaintenanceSettings_Vtbl {
    pub const fn new<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_Period<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMaintenanceSettings_Impl::put_Period(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_Period<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMaintenanceSettings_Impl::get_Period(this) {
                    Ok(ok__) => {
                        target.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Deadline<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMaintenanceSettings_Impl::put_Deadline(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_Deadline<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMaintenanceSettings_Impl::get_Deadline(this) {
                    Ok(ok__) => {
                        target.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Exclusive<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMaintenanceSettings_Impl::put_Exclusive(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Exclusive<Identity: IMaintenanceSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMaintenanceSettings_Impl::get_Exclusive(this) {
                    Ok(ok__) => {
                        target.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            put_Period: put_Period::<Identity, OFFSET>,
            get_Period: get_Period::<Identity, OFFSET>,
            put_Deadline: put_Deadline::<Identity, OFFSET>,
            get_Deadline: get_Deadline::<Identity, OFFSET>,
            put_Exclusive: put_Exclusive::<Identity, OFFSET>,
            get_Exclusive: get_Exclusive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMaintenanceSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMaintenanceSettings {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMonthlyDOWTrigger, IMonthlyDOWTrigger_Vtbl, 0x77d025a3_90fa_43aa_b52e_cda5499b946a);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMonthlyDOWTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMonthlyDOWTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl IMonthlyDOWTrigger {
    pub unsafe fn get_DaysOfWeek(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DaysOfWeek)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DaysOfWeek(&self, days: i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DaysOfWeek)(windows_core::Interface::as_raw(self), days) }
    }
    pub unsafe fn get_WeeksOfMonth(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WeeksOfMonth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_WeeksOfMonth(&self, weeks: i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WeeksOfMonth)(windows_core::Interface::as_raw(self), weeks) }
    }
    pub unsafe fn get_MonthsOfYear(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MonthsOfYear)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MonthsOfYear(&self, months: i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MonthsOfYear)(windows_core::Interface::as_raw(self), months) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RunOnLastWeekOfMonth(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RunOnLastWeekOfMonth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_RunOnLastWeekOfMonth(&self, lastweek: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RunOnLastWeekOfMonth)(windows_core::Interface::as_raw(self), lastweek) }
    }
    pub unsafe fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RandomDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMonthlyDOWTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_DaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub put_DaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub get_WeeksOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub put_WeeksOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub get_MonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub put_MonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_RunOnLastWeekOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RunOnLastWeekOfMonth: usize,
    #[cfg(feature = "wtypes")]
    pub put_RunOnLastWeekOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_RunOnLastWeekOfMonth: usize,
    pub get_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMonthlyDOWTrigger_Impl: ITrigger_Impl {
    fn get_DaysOfWeek(&self) -> windows_core::Result<i16>;
    fn put_DaysOfWeek(&self, days: i16) -> windows_core::Result<()>;
    fn get_WeeksOfMonth(&self) -> windows_core::Result<i16>;
    fn put_WeeksOfMonth(&self, weeks: i16) -> windows_core::Result<()>;
    fn get_MonthsOfYear(&self) -> windows_core::Result<i16>;
    fn put_MonthsOfYear(&self, months: i16) -> windows_core::Result<()>;
    fn get_RunOnLastWeekOfMonth(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_RunOnLastWeekOfMonth(&self, lastweek: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMonthlyDOWTrigger_Vtbl {
    pub const fn new<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DaysOfWeek<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMonthlyDOWTrigger_Impl::get_DaysOfWeek(this) {
                    Ok(ok__) => {
                        pdays.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DaysOfWeek<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::put_DaysOfWeek(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn get_WeeksOfMonth<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pweeks: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMonthlyDOWTrigger_Impl::get_WeeksOfMonth(this) {
                    Ok(ok__) => {
                        pweeks.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WeeksOfMonth<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weeks: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::put_WeeksOfMonth(this, core::mem::transmute_copy(&weeks)).into()
            }
        }
        unsafe extern "system" fn get_MonthsOfYear<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmonths: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMonthlyDOWTrigger_Impl::get_MonthsOfYear(this) {
                    Ok(ok__) => {
                        pmonths.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MonthsOfYear<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, months: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::put_MonthsOfYear(this, core::mem::transmute_copy(&months)).into()
            }
        }
        unsafe extern "system" fn get_RunOnLastWeekOfMonth<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastweek: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMonthlyDOWTrigger_Impl::get_RunOnLastWeekOfMonth(this) {
                    Ok(ok__) => {
                        plastweek.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RunOnLastWeekOfMonth<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastweek: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::put_RunOnLastWeekOfMonth(this, core::mem::transmute_copy(&lastweek)).into()
            }
        }
        unsafe extern "system" fn get_RandomDelay<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMonthlyDOWTrigger_Impl::get_RandomDelay(this) {
                    Ok(ok__) => {
                        prandomdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RandomDelay<Identity: IMonthlyDOWTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyDOWTrigger_Impl::put_RandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            get_DaysOfWeek: get_DaysOfWeek::<Identity, OFFSET>,
            put_DaysOfWeek: put_DaysOfWeek::<Identity, OFFSET>,
            get_WeeksOfMonth: get_WeeksOfMonth::<Identity, OFFSET>,
            put_WeeksOfMonth: put_WeeksOfMonth::<Identity, OFFSET>,
            get_MonthsOfYear: get_MonthsOfYear::<Identity, OFFSET>,
            put_MonthsOfYear: put_MonthsOfYear::<Identity, OFFSET>,
            get_RunOnLastWeekOfMonth: get_RunOnLastWeekOfMonth::<Identity, OFFSET>,
            put_RunOnLastWeekOfMonth: put_RunOnLastWeekOfMonth::<Identity, OFFSET>,
            get_RandomDelay: get_RandomDelay::<Identity, OFFSET>,
            put_RandomDelay: put_RandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMonthlyDOWTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMonthlyDOWTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMonthlyTrigger, IMonthlyTrigger_Vtbl, 0x97c45ef1_6b02_4a1a_9c0e_1ebfba1500ac);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMonthlyTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMonthlyTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl IMonthlyTrigger {
    pub unsafe fn get_DaysOfMonth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DaysOfMonth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DaysOfMonth(&self, days: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DaysOfMonth)(windows_core::Interface::as_raw(self), days) }
    }
    pub unsafe fn get_MonthsOfYear(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MonthsOfYear)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MonthsOfYear(&self, months: i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MonthsOfYear)(windows_core::Interface::as_raw(self), months) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RunOnLastDayOfMonth(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RunOnLastDayOfMonth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_RunOnLastDayOfMonth(&self, lastday: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RunOnLastDayOfMonth)(windows_core::Interface::as_raw(self), lastday) }
    }
    pub unsafe fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RandomDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMonthlyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_DaysOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_DaysOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_MonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub put_MonthsOfYear: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_RunOnLastDayOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RunOnLastDayOfMonth: usize,
    #[cfg(feature = "wtypes")]
    pub put_RunOnLastDayOfMonth: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_RunOnLastDayOfMonth: usize,
    pub get_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMonthlyTrigger_Impl: ITrigger_Impl {
    fn get_DaysOfMonth(&self) -> windows_core::Result<i32>;
    fn put_DaysOfMonth(&self, days: i32) -> windows_core::Result<()>;
    fn get_MonthsOfYear(&self) -> windows_core::Result<i16>;
    fn put_MonthsOfYear(&self, months: i16) -> windows_core::Result<()>;
    fn get_RunOnLastDayOfMonth(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_RunOnLastDayOfMonth(&self, lastday: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMonthlyTrigger_Vtbl {
    pub const fn new<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DaysOfMonth<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMonthlyTrigger_Impl::get_DaysOfMonth(this) {
                    Ok(ok__) => {
                        pdays.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DaysOfMonth<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::put_DaysOfMonth(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn get_MonthsOfYear<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmonths: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMonthlyTrigger_Impl::get_MonthsOfYear(this) {
                    Ok(ok__) => {
                        pmonths.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MonthsOfYear<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, months: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::put_MonthsOfYear(this, core::mem::transmute_copy(&months)).into()
            }
        }
        unsafe extern "system" fn get_RunOnLastDayOfMonth<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastday: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMonthlyTrigger_Impl::get_RunOnLastDayOfMonth(this) {
                    Ok(ok__) => {
                        plastday.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RunOnLastDayOfMonth<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastday: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::put_RunOnLastDayOfMonth(this, core::mem::transmute_copy(&lastday)).into()
            }
        }
        unsafe extern "system" fn get_RandomDelay<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMonthlyTrigger_Impl::get_RandomDelay(this) {
                    Ok(ok__) => {
                        prandomdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RandomDelay<Identity: IMonthlyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMonthlyTrigger_Impl::put_RandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            get_DaysOfMonth: get_DaysOfMonth::<Identity, OFFSET>,
            put_DaysOfMonth: put_DaysOfMonth::<Identity, OFFSET>,
            get_MonthsOfYear: get_MonthsOfYear::<Identity, OFFSET>,
            put_MonthsOfYear: put_MonthsOfYear::<Identity, OFFSET>,
            get_RunOnLastDayOfMonth: get_RunOnLastDayOfMonth::<Identity, OFFSET>,
            put_RunOnLastDayOfMonth: put_RunOnLastDayOfMonth::<Identity, OFFSET>,
            get_RandomDelay: get_RandomDelay::<Identity, OFFSET>,
            put_RandomDelay: put_RandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMonthlyTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMonthlyTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INetworkSettings, INetworkSettings_Vtbl, 0x9f7dea84_c30b_4245_80b6_00e9f646f1b4);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INetworkSettings {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INetworkSettings, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl INetworkSettings {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_Id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Id(&self, id: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Id)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INetworkSettings_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Id(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INetworkSettings_Vtbl {
    pub const fn new<Identity: INetworkSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: INetworkSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkSettings_Impl::get_Name(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: INetworkSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkSettings_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_Id<Identity: INetworkSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INetworkSettings_Impl::get_Id(this) {
                    Ok(ok__) => {
                        pid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Id<Identity: INetworkSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INetworkSettings_Impl::put_Id(this, core::mem::transmute(&id)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_Id: get_Id::<Identity, OFFSET>,
            put_Id: put_Id::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INetworkSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INetworkSettings {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IPrincipal, IPrincipal_Vtbl, 0xd98d51e5_c9b4_496a_a9c1_18980261cf0f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IPrincipal {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IPrincipal, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IPrincipal {
    pub unsafe fn get_Id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Id(&self, id: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Id)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id)) }
    }
    pub unsafe fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DisplayName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_UserId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_UserId(&self, user: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UserId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(user)) }
    }
    pub unsafe fn get_LogonType(&self) -> windows_core::Result<TASK_LOGON_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LogonType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_LogonType(&self, logon: TASK_LOGON_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LogonType)(windows_core::Interface::as_raw(self), logon) }
    }
    pub unsafe fn get_GroupId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_GroupId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_GroupId(&self, group: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_GroupId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(group)) }
    }
    pub unsafe fn get_RunLevel(&self) -> windows_core::Result<TASK_RUNLEVEL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RunLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_RunLevel(&self, runlevel: TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RunLevel)(windows_core::Interface::as_raw(self), runlevel) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrincipal_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LogonType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_LOGON_TYPE) -> windows_core::HRESULT,
    pub put_LogonType: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_LOGON_TYPE) -> windows_core::HRESULT,
    pub get_GroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_GroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RunLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT,
    pub put_RunLevel: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPrincipal_Impl: super::IDispatch_Impl {
    fn get_Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Id(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_UserId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_UserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_LogonType(&self) -> windows_core::Result<TASK_LOGON_TYPE>;
    fn put_LogonType(&self, logon: TASK_LOGON_TYPE) -> windows_core::Result<()>;
    fn get_GroupId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_GroupId(&self, group: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RunLevel(&self) -> windows_core::Result<TASK_RUNLEVEL_TYPE>;
    fn put_RunLevel(&self, runlevel: TASK_RUNLEVEL_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPrincipal_Vtbl {
    pub const fn new<Identity: IPrincipal_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Id<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrincipal_Impl::get_Id(this) {
                    Ok(ok__) => {
                        pid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Id<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::put_Id(this, core::mem::transmute(&id)).into()
            }
        }
        unsafe extern "system" fn get_DisplayName<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrincipal_Impl::get_DisplayName(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisplayName<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::put_DisplayName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_UserId<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrincipal_Impl::get_UserId(this) {
                    Ok(ok__) => {
                        puser.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UserId<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::put_UserId(this, core::mem::transmute(&user)).into()
            }
        }
        unsafe extern "system" fn get_LogonType<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrincipal_Impl::get_LogonType(this) {
                    Ok(ok__) => {
                        plogon.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LogonType<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logon: TASK_LOGON_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::put_LogonType(this, core::mem::transmute_copy(&logon)).into()
            }
        }
        unsafe extern "system" fn get_GroupId<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrincipal_Impl::get_GroupId(this) {
                    Ok(ok__) => {
                        pgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_GroupId<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::put_GroupId(this, core::mem::transmute(&group)).into()
            }
        }
        unsafe extern "system" fn get_RunLevel<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrincipal_Impl::get_RunLevel(this) {
                    Ok(ok__) => {
                        prunlevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RunLevel<Identity: IPrincipal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal_Impl::put_RunLevel(this, core::mem::transmute_copy(&runlevel)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Id: get_Id::<Identity, OFFSET>,
            put_Id: put_Id::<Identity, OFFSET>,
            get_DisplayName: get_DisplayName::<Identity, OFFSET>,
            put_DisplayName: put_DisplayName::<Identity, OFFSET>,
            get_UserId: get_UserId::<Identity, OFFSET>,
            put_UserId: put_UserId::<Identity, OFFSET>,
            get_LogonType: get_LogonType::<Identity, OFFSET>,
            put_LogonType: put_LogonType::<Identity, OFFSET>,
            get_GroupId: get_GroupId::<Identity, OFFSET>,
            put_GroupId: put_GroupId::<Identity, OFFSET>,
            get_RunLevel: get_RunLevel::<Identity, OFFSET>,
            put_RunLevel: put_RunLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrincipal as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPrincipal {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IPrincipal2, IPrincipal2_Vtbl, 0x248919ae_e345_4a6d_8aeb_e0d3165c904e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IPrincipal2 {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IPrincipal2, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IPrincipal2 {
    pub unsafe fn get_ProcessTokenSidType(&self) -> windows_core::Result<TASK_PROCESSTOKENSID_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProcessTokenSidType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ProcessTokenSidType(&self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ProcessTokenSidType)(windows_core::Interface::as_raw(self), processtokensidtype) }
    }
    pub unsafe fn get_RequiredPrivilegeCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequiredPrivilegeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RequiredPrivilege(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequiredPrivilege)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddRequiredPrivilege(&self, privilege: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRequiredPrivilege)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(privilege)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrincipal2_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ProcessTokenSidType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT,
    pub put_ProcessTokenSidType: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT,
    pub get_RequiredPrivilegeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_RequiredPrivilege: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddRequiredPrivilege: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPrincipal2_Impl: super::IDispatch_Impl {
    fn get_ProcessTokenSidType(&self) -> windows_core::Result<TASK_PROCESSTOKENSID_TYPE>;
    fn put_ProcessTokenSidType(&self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> windows_core::Result<()>;
    fn get_RequiredPrivilegeCount(&self) -> windows_core::Result<i32>;
    fn get_RequiredPrivilege(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn AddRequiredPrivilege(&self, privilege: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPrincipal2_Vtbl {
    pub const fn new<Identity: IPrincipal2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ProcessTokenSidType<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrincipal2_Impl::get_ProcessTokenSidType(this) {
                    Ok(ok__) => {
                        pprocesstokensidtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ProcessTokenSidType<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal2_Impl::put_ProcessTokenSidType(this, core::mem::transmute_copy(&processtokensidtype)).into()
            }
        }
        unsafe extern "system" fn get_RequiredPrivilegeCount<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrincipal2_Impl::get_RequiredPrivilegeCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RequiredPrivilege<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pprivilege: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrincipal2_Impl::get_RequiredPrivilege(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pprivilege.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddRequiredPrivilege<Identity: IPrincipal2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, privilege: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrincipal2_Impl::AddRequiredPrivilege(this, core::mem::transmute(&privilege)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ProcessTokenSidType: get_ProcessTokenSidType::<Identity, OFFSET>,
            put_ProcessTokenSidType: put_ProcessTokenSidType::<Identity, OFFSET>,
            get_RequiredPrivilegeCount: get_RequiredPrivilegeCount::<Identity, OFFSET>,
            get_RequiredPrivilege: get_RequiredPrivilege::<Identity, OFFSET>,
            AddRequiredPrivilege: AddRequiredPrivilege::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrincipal2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPrincipal2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRegisteredTask, IRegisteredTask_Vtbl, 0x9c86f320_dee3_4dd1_b972_a303f26b061e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRegisteredTask {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRegisteredTask, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IRegisteredTask {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_State(&self) -> windows_core::Result<TASK_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Run(&self, params: &super::VARIANT) -> windows_core::Result<IRunningTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(params), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RunEx(&self, params: &super::VARIANT, flags: super::LONG, sessionid: super::LONG, user: &windows_core::BSTR) -> windows_core::Result<IRunningTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RunEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(params), flags, sessionid, core::mem::transmute_copy(user), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetInstances(&self, flags: super::LONG) -> windows_core::Result<IRunningTaskCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstances)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LastRunTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LastTaskResult(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastTaskResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_NumberOfMissedRuns(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NumberOfMissedRuns)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_NextRunTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NextRunTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Definition(&self) -> windows_core::Result<ITaskDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Definition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Xml(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Xml)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: super::LONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), securityinformation, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sddl), flags) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Stop(&self, flags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), flags) }
    }
    #[cfg(all(feature = "minwinbase", feature = "shared"))]
    pub unsafe fn GetRunTimes(&self, pststart: super::LPSYSTEMTIME, pstend: super::LPSYSTEMTIME, pcount: *mut super::DWORD, pruntimes: *mut super::LPSYSTEMTIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRunTimes)(windows_core::Interface::as_raw(self), pststart, pstend, pcount as _, pruntimes as _) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredTask_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_STATE) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Run: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub RunEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, super::LONG, super::LONG, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    RunEx: usize,
    #[cfg(feature = "shared")]
    pub GetInstances: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetInstances: usize,
    #[cfg(feature = "wtypes")]
    pub get_LastRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LastRunTime: usize,
    #[cfg(feature = "shared")]
    pub get_LastTaskResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LastTaskResult: usize,
    #[cfg(feature = "shared")]
    pub get_NumberOfMissedRuns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_NumberOfMissedRuns: usize,
    #[cfg(feature = "wtypes")]
    pub get_NextRunTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_NextRunTime: usize,
    pub get_Definition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Xml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSecurityDescriptor: usize,
    #[cfg(feature = "shared")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "shared")]
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Stop: usize,
    #[cfg(all(feature = "minwinbase", feature = "shared"))]
    pub GetRunTimes: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPSYSTEMTIME, super::LPSYSTEMTIME, *mut super::DWORD, *mut super::LPSYSTEMTIME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwinbase", feature = "shared")))]
    GetRunTimes: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRegisteredTask_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_State(&self) -> windows_core::Result<TASK_STATE>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Run(&self, params: &super::VARIANT) -> windows_core::Result<IRunningTask>;
    fn RunEx(&self, params: &super::VARIANT, flags: super::LONG, sessionid: super::LONG, user: &windows_core::BSTR) -> windows_core::Result<IRunningTask>;
    fn GetInstances(&self, flags: super::LONG) -> windows_core::Result<IRunningTaskCollection>;
    fn get_LastRunTime(&self) -> windows_core::Result<super::DATE>;
    fn get_LastTaskResult(&self) -> windows_core::Result<super::LONG>;
    fn get_NumberOfMissedRuns(&self) -> windows_core::Result<super::LONG>;
    fn get_NextRunTime(&self) -> windows_core::Result<super::DATE>;
    fn get_Definition(&self) -> windows_core::Result<ITaskDefinition>;
    fn get_Xml(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSecurityDescriptor(&self, securityinformation: super::LONG) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<()>;
    fn Stop(&self, flags: super::LONG) -> windows_core::Result<()>;
    fn GetRunTimes(&self, pststart: super::LPSYSTEMTIME, pstend: super::LPSYSTEMTIME, pcount: *mut super::DWORD, pruntimes: *mut super::LPSYSTEMTIME) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRegisteredTask_Vtbl {
    pub const fn new<Identity: IRegisteredTask_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_Name(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Path<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_Path(this) {
                    Ok(ok__) => {
                        ppath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_State<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TASK_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_State(this) {
                    Ok(ok__) => {
                        pstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        penabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisteredTask_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn Run<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, params: super::VARIANT, pprunningtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::Run(this, core::mem::transmute(&params)) {
                    Ok(ok__) => {
                        pprunningtask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RunEx<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, params: super::VARIANT, flags: super::LONG, sessionid: super::LONG, user: *mut core::ffi::c_void, pprunningtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::RunEx(this, core::mem::transmute(&params), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&sessionid), core::mem::transmute(&user)) {
                    Ok(ok__) => {
                        pprunningtask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInstances<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::LONG, pprunningtasks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::GetInstances(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pprunningtasks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastRunTime<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plastruntime: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_LastRunTime(this) {
                    Ok(ok__) => {
                        plastruntime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastTaskResult<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plasttaskresult: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_LastTaskResult(this) {
                    Ok(ok__) => {
                        plasttaskresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_NumberOfMissedRuns<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumberofmissedruns: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_NumberOfMissedRuns(this) {
                    Ok(ok__) => {
                        pnumberofmissedruns.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_NextRunTime<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnextruntime: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_NextRunTime(this) {
                    Ok(ok__) => {
                        pnextruntime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Definition<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_Definition(this) {
                    Ok(ok__) => {
                        ppdefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Xml<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::get_Xml(this) {
                    Ok(ok__) => {
                        pxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinformation: super::LONG, psddl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTask_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&securityinformation)) {
                    Ok(ok__) => {
                        psddl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: *mut core::ffi::c_void, flags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisteredTask_Impl::SetSecurityDescriptor(this, core::mem::transmute(&sddl), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisteredTask_Impl::Stop(this, core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetRunTimes<Identity: IRegisteredTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pststart: super::LPSYSTEMTIME, pstend: super::LPSYSTEMTIME, pcount: *mut super::DWORD, pruntimes: *mut super::LPSYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegisteredTask_Impl::GetRunTimes(this, core::mem::transmute_copy(&pststart), core::mem::transmute_copy(&pstend), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&pruntimes)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_Path: get_Path::<Identity, OFFSET>,
            get_State: get_State::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            RunEx: RunEx::<Identity, OFFSET>,
            GetInstances: GetInstances::<Identity, OFFSET>,
            get_LastRunTime: get_LastRunTime::<Identity, OFFSET>,
            get_LastTaskResult: get_LastTaskResult::<Identity, OFFSET>,
            get_NumberOfMissedRuns: get_NumberOfMissedRuns::<Identity, OFFSET>,
            get_NextRunTime: get_NextRunTime::<Identity, OFFSET>,
            get_Definition: get_Definition::<Identity, OFFSET>,
            get_Xml: get_Xml::<Identity, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            GetRunTimes: GetRunTimes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisteredTask as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRegisteredTask {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRegisteredTaskCollection, IRegisteredTaskCollection_Vtbl, 0x86627eb4_42a7_41e4_a4d9_ac33a72f2d52);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRegisteredTaskCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRegisteredTaskCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IRegisteredTaskCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IRegisteredTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredTaskCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRegisteredTaskCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IRegisteredTask>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRegisteredTaskCollection_Vtbl {
    pub const fn new<Identity: IRegisteredTaskCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTaskCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT, ppregisteredtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTaskCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppregisteredtask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IRegisteredTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegisteredTaskCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisteredTaskCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRegisteredTaskCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRegistrationInfo, IRegistrationInfo_Vtbl, 0x416d8b73_cb41_4ea1_805c_9be9a5ac4a74);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRegistrationInfo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRegistrationInfo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IRegistrationInfo {
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Description)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description)) }
    }
    pub unsafe fn get_Author(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Author)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Author(&self, author: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Author)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(author)) }
    }
    pub unsafe fn get_Version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Version(&self, version: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Version)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(version)) }
    }
    pub unsafe fn get_Date(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Date)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Date(&self, date: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Date)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(date)) }
    }
    pub unsafe fn get_Documentation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Documentation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Documentation(&self, documentation: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Documentation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(documentation)) }
    }
    pub unsafe fn get_XmlText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_XmlText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_XmlText(&self, text: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)) }
    }
    pub unsafe fn get_URI(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_URI)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_URI(&self, uri: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_URI)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(uri)) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_SecurityDescriptor(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SecurityDescriptor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn put_SecurityDescriptor(&self, sddl: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sddl)) }
    }
    pub unsafe fn get_Source(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Source)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Source(&self, source: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Source)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(source)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegistrationInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Documentation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Documentation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_URI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_URI: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_SecurityDescriptor: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub put_SecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    put_SecurityDescriptor: usize,
    pub get_Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRegistrationInfo_Impl: super::IDispatch_Impl {
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Description(&self, description: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Author(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Author(&self, author: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Version(&self, version: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Date(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Date(&self, date: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Documentation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Documentation(&self, documentation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_XmlText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_XmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_URI(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_URI(&self, uri: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SecurityDescriptor(&self) -> windows_core::Result<super::VARIANT>;
    fn put_SecurityDescriptor(&self, sddl: &super::VARIANT) -> windows_core::Result<()>;
    fn get_Source(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Source(&self, source: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRegistrationInfo_Vtbl {
    pub const fn new<Identity: IRegistrationInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Description<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationInfo_Impl::get_Description(this) {
                    Ok(ok__) => {
                        pdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Description<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::put_Description(this, core::mem::transmute(&description)).into()
            }
        }
        unsafe extern "system" fn get_Author<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationInfo_Impl::get_Author(this) {
                    Ok(ok__) => {
                        pauthor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Author<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, author: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::put_Author(this, core::mem::transmute(&author)).into()
            }
        }
        unsafe extern "system" fn get_Version<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationInfo_Impl::get_Version(this) {
                    Ok(ok__) => {
                        pversion.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Version<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::put_Version(this, core::mem::transmute(&version)).into()
            }
        }
        unsafe extern "system" fn get_Date<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationInfo_Impl::get_Date(this) {
                    Ok(ok__) => {
                        pdate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Date<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, date: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::put_Date(this, core::mem::transmute(&date)).into()
            }
        }
        unsafe extern "system" fn get_Documentation<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdocumentation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationInfo_Impl::get_Documentation(this) {
                    Ok(ok__) => {
                        pdocumentation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Documentation<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, documentation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::put_Documentation(this, core::mem::transmute(&documentation)).into()
            }
        }
        unsafe extern "system" fn get_XmlText<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationInfo_Impl::get_XmlText(this) {
                    Ok(ok__) => {
                        ptext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_XmlText<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::put_XmlText(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn get_URI<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puri: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationInfo_Impl::get_URI(this) {
                    Ok(ok__) => {
                        puri.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_URI<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::put_URI(this, core::mem::transmute(&uri)).into()
            }
        }
        unsafe extern "system" fn get_SecurityDescriptor<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psddl: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationInfo_Impl::get_SecurityDescriptor(this) {
                    Ok(ok__) => {
                        psddl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SecurityDescriptor<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::put_SecurityDescriptor(this, core::mem::transmute(&sddl)).into()
            }
        }
        unsafe extern "system" fn get_Source<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationInfo_Impl::get_Source(this) {
                    Ok(ok__) => {
                        psource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Source<Identity: IRegistrationInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationInfo_Impl::put_Source(this, core::mem::transmute(&source)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, OFFSET>,
            put_Description: put_Description::<Identity, OFFSET>,
            get_Author: get_Author::<Identity, OFFSET>,
            put_Author: put_Author::<Identity, OFFSET>,
            get_Version: get_Version::<Identity, OFFSET>,
            put_Version: put_Version::<Identity, OFFSET>,
            get_Date: get_Date::<Identity, OFFSET>,
            put_Date: put_Date::<Identity, OFFSET>,
            get_Documentation: get_Documentation::<Identity, OFFSET>,
            put_Documentation: put_Documentation::<Identity, OFFSET>,
            get_XmlText: get_XmlText::<Identity, OFFSET>,
            put_XmlText: put_XmlText::<Identity, OFFSET>,
            get_URI: get_URI::<Identity, OFFSET>,
            put_URI: put_URI::<Identity, OFFSET>,
            get_SecurityDescriptor: get_SecurityDescriptor::<Identity, OFFSET>,
            put_SecurityDescriptor: put_SecurityDescriptor::<Identity, OFFSET>,
            get_Source: get_Source::<Identity, OFFSET>,
            put_Source: put_Source::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegistrationInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRegistrationInfo {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRegistrationTrigger, IRegistrationTrigger_Vtbl, 0x4c8fec3a_c218_4e0c_b23d_629024db91a2);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRegistrationTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRegistrationTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl IRegistrationTrigger {
    pub unsafe fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Delay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Delay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegistrationTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRegistrationTrigger_Impl: ITrigger_Impl {
    fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRegistrationTrigger_Vtbl {
    pub const fn new<Identity: IRegistrationTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Delay<Identity: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRegistrationTrigger_Impl::get_Delay(this) {
                    Ok(ok__) => {
                        pdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Delay<Identity: IRegistrationTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRegistrationTrigger_Impl::put_Delay(this, core::mem::transmute(&delay)).into()
            }
        }
        Self { base__: ITrigger_Vtbl::new::<Identity, OFFSET>(), get_Delay: get_Delay::<Identity, OFFSET>, put_Delay: put_Delay::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegistrationTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRegistrationTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRepetitionPattern, IRepetitionPattern_Vtbl, 0x7fb9acf1_26be_400e_85b5_294b9c75dfd6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRepetitionPattern {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRepetitionPattern, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IRepetitionPattern {
    pub unsafe fn get_Interval(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Interval)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Interval(&self, interval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Interval)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(interval)) }
    }
    pub unsafe fn get_Duration(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Duration(&self, duration: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Duration)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(duration)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_StopAtDurationEnd(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StopAtDurationEnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_StopAtDurationEnd(&self, stop: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StopAtDurationEnd)(windows_core::Interface::as_raw(self), stop) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRepetitionPattern_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Interval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_StopAtDurationEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_StopAtDurationEnd: usize,
    #[cfg(feature = "wtypes")]
    pub put_StopAtDurationEnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_StopAtDurationEnd: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRepetitionPattern_Impl: super::IDispatch_Impl {
    fn get_Interval(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Interval(&self, interval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Duration(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Duration(&self, duration: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_StopAtDurationEnd(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_StopAtDurationEnd(&self, stop: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRepetitionPattern_Vtbl {
    pub const fn new<Identity: IRepetitionPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Interval<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinterval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRepetitionPattern_Impl::get_Interval(this) {
                    Ok(ok__) => {
                        pinterval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Interval<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRepetitionPattern_Impl::put_Interval(this, core::mem::transmute(&interval)).into()
            }
        }
        unsafe extern "system" fn get_Duration<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pduration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRepetitionPattern_Impl::get_Duration(this) {
                    Ok(ok__) => {
                        pduration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Duration<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRepetitionPattern_Impl::put_Duration(this, core::mem::transmute(&duration)).into()
            }
        }
        unsafe extern "system" fn get_StopAtDurationEnd<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstop: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRepetitionPattern_Impl::get_StopAtDurationEnd(this) {
                    Ok(ok__) => {
                        pstop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StopAtDurationEnd<Identity: IRepetitionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stop: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRepetitionPattern_Impl::put_StopAtDurationEnd(this, core::mem::transmute_copy(&stop)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Interval: get_Interval::<Identity, OFFSET>,
            put_Interval: put_Interval::<Identity, OFFSET>,
            get_Duration: get_Duration::<Identity, OFFSET>,
            put_Duration: put_Duration::<Identity, OFFSET>,
            get_StopAtDurationEnd: get_StopAtDurationEnd::<Identity, OFFSET>,
            put_StopAtDurationEnd: put_StopAtDurationEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRepetitionPattern as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRepetitionPattern {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRunningTask, IRunningTask_Vtbl, 0x653758fb_7b9a_4f1e_a471_beeb8e9b834e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRunningTask {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRunningTask, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IRunningTask {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_InstanceGuid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InstanceGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_State(&self) -> windows_core::Result<TASK_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentAction(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Refresh(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_EnginePID(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EnginePID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRunningTask_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_InstanceGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_STATE) -> windows_core::HRESULT,
    pub get_CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_EnginePID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_EnginePID: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRunningTask_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_InstanceGuid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_State(&self) -> windows_core::Result<TASK_STATE>;
    fn get_CurrentAction(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_EnginePID(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRunningTask_Vtbl {
    pub const fn new<Identity: IRunningTask_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::get_Name(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_InstanceGuid<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::get_InstanceGuid(this) {
                    Ok(ok__) => {
                        pguid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Path<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::get_Path(this) {
                    Ok(ok__) => {
                        ppath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_State<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut TASK_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::get_State(this) {
                    Ok(ok__) => {
                        pstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAction<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::get_CurrentAction(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Stop<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningTask_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Refresh<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningTask_Impl::Refresh(this).into()
            }
        }
        unsafe extern "system" fn get_EnginePID<Identity: IRunningTask_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTask_Impl::get_EnginePID(this) {
                    Ok(ok__) => {
                        ppid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_InstanceGuid: get_InstanceGuid::<Identity, OFFSET>,
            get_Path: get_Path::<Identity, OFFSET>,
            get_State: get_State::<Identity, OFFSET>,
            get_CurrentAction: get_CurrentAction::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_EnginePID: get_EnginePID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunningTask as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRunningTask {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRunningTaskCollection, IRunningTaskCollection_Vtbl, 0x6a67614b_6828_4fec_aa54_6d52e8f1f2db);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRunningTaskCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRunningTaskCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IRunningTaskCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IRunningTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRunningTaskCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRunningTaskCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<IRunningTask>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRunningTaskCollection_Vtbl {
    pub const fn new<Identity: IRunningTaskCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTaskCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT, pprunningtask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTaskCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        pprunningtask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: IRunningTaskCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningTaskCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunningTaskCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRunningTaskCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ISessionStateChangeTrigger, ISessionStateChangeTrigger_Vtbl, 0x754da71b_4385_4475_9dd9_598294fa3641);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ISessionStateChangeTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ISessionStateChangeTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl ISessionStateChangeTrigger {
    pub unsafe fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Delay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Delay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(delay)) }
    }
    pub unsafe fn get_UserId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UserId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_UserId(&self, user: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UserId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(user)) }
    }
    pub unsafe fn get_StateChange(&self) -> windows_core::Result<TASK_SESSION_STATE_CHANGE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StateChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_StateChange(&self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StateChange)(windows_core::Interface::as_raw(self), r#type) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ISessionStateChangeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Delay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_UserId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_StateChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT,
    pub put_StateChange: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ISessionStateChangeTrigger_Impl: ITrigger_Impl {
    fn get_Delay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Delay(&self, delay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_UserId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_UserId(&self, user: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_StateChange(&self) -> windows_core::Result<TASK_SESSION_STATE_CHANGE_TYPE>;
    fn put_StateChange(&self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ISessionStateChangeTrigger_Vtbl {
    pub const fn new<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Delay<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISessionStateChangeTrigger_Impl::get_Delay(this) {
                    Ok(ok__) => {
                        pdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Delay<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, delay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionStateChangeTrigger_Impl::put_Delay(this, core::mem::transmute(&delay)).into()
            }
        }
        unsafe extern "system" fn get_UserId<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISessionStateChangeTrigger_Impl::get_UserId(this) {
                    Ok(ok__) => {
                        puser.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UserId<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, user: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionStateChangeTrigger_Impl::put_UserId(this, core::mem::transmute(&user)).into()
            }
        }
        unsafe extern "system" fn get_StateChange<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISessionStateChangeTrigger_Impl::get_StateChange(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StateChange<Identity: ISessionStateChangeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISessionStateChangeTrigger_Impl::put_StateChange(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            get_Delay: get_Delay::<Identity, OFFSET>,
            put_Delay: put_Delay::<Identity, OFFSET>,
            get_UserId: get_UserId::<Identity, OFFSET>,
            put_UserId: put_UserId::<Identity, OFFSET>,
            get_StateChange: get_StateChange::<Identity, OFFSET>,
            put_StateChange: put_StateChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISessionStateChangeTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ISessionStateChangeTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShowMessageAction, IShowMessageAction_Vtbl, 0x505e9e68_af89_46b8_a30f_56162a83d537);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShowMessageAction {
    type Target = IAction;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShowMessageAction, windows_core::IUnknown, super::IDispatch, IAction);
#[cfg(feature = "oaidl")]
impl IShowMessageAction {
    pub unsafe fn get_Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Title(&self, title: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Title)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(title)) }
    }
    pub unsafe fn get_MessageBody(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MessageBody)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_MessageBody(&self, messagebody: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MessageBody)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(messagebody)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShowMessageAction_Vtbl {
    pub base__: IAction_Vtbl,
    pub get_Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MessageBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MessageBody: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShowMessageAction_Impl: IAction_Impl {
    fn get_Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Title(&self, title: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MessageBody(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_MessageBody(&self, messagebody: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShowMessageAction_Vtbl {
    pub const fn new<Identity: IShowMessageAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Title<Identity: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptitle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShowMessageAction_Impl::get_Title(this) {
                    Ok(ok__) => {
                        ptitle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Title<Identity: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShowMessageAction_Impl::put_Title(this, core::mem::transmute(&title)).into()
            }
        }
        unsafe extern "system" fn get_MessageBody<Identity: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessagebody: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShowMessageAction_Impl::get_MessageBody(this) {
                    Ok(ok__) => {
                        pmessagebody.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MessageBody<Identity: IShowMessageAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagebody: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShowMessageAction_Impl::put_MessageBody(this, core::mem::transmute(&messagebody)).into()
            }
        }
        Self {
            base__: IAction_Vtbl::new::<Identity, OFFSET>(),
            get_Title: get_Title::<Identity, OFFSET>,
            put_Title: put_Title::<Identity, OFFSET>,
            get_MessageBody: get_MessageBody::<Identity, OFFSET>,
            put_MessageBody: put_MessageBody::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShowMessageAction as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IAction as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShowMessageAction {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITaskDefinition, ITaskDefinition_Vtbl, 0xf5bc8fc5_536d_4f77_b852_fbc1356fdeb6);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITaskDefinition {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITaskDefinition, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITaskDefinition {
    pub unsafe fn get_RegistrationInfo(&self) -> windows_core::Result<IRegistrationInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RegistrationInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_RegistrationInfo<P0>(&self, pregistrationinfo: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRegistrationInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_RegistrationInfo)(windows_core::Interface::as_raw(self), pregistrationinfo.param().abi()) }
    }
    pub unsafe fn get_Triggers(&self) -> windows_core::Result<ITriggerCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Triggers)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_Triggers<P0>(&self, ptriggers: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITriggerCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Triggers)(windows_core::Interface::as_raw(self), ptriggers.param().abi()) }
    }
    pub unsafe fn get_Settings(&self) -> windows_core::Result<ITaskSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_Settings<P0>(&self, psettings: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ITaskSettings>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Settings)(windows_core::Interface::as_raw(self), psettings.param().abi()) }
    }
    pub unsafe fn get_Data(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Data)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Data(&self, data: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Data)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(data)) }
    }
    pub unsafe fn get_Principal(&self) -> windows_core::Result<IPrincipal> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Principal)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_Principal<P0>(&self, pprincipal: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPrincipal>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Principal)(windows_core::Interface::as_raw(self), pprincipal.param().abi()) }
    }
    pub unsafe fn get_Actions(&self) -> windows_core::Result<IActionCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Actions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_Actions<P0>(&self, pactions: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IActionCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Actions)(windows_core::Interface::as_raw(self), pactions.param().abi()) }
    }
    pub unsafe fn get_XmlText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_XmlText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_XmlText(&self, xml: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(xml)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskDefinition_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_RegistrationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RegistrationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Triggers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Triggers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Principal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Principal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Actions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITaskDefinition_Impl: super::IDispatch_Impl {
    fn get_RegistrationInfo(&self) -> windows_core::Result<IRegistrationInfo>;
    fn put_RegistrationInfo(&self, pregistrationinfo: windows_core::Ref<IRegistrationInfo>) -> windows_core::Result<()>;
    fn get_Triggers(&self) -> windows_core::Result<ITriggerCollection>;
    fn put_Triggers(&self, ptriggers: windows_core::Ref<ITriggerCollection>) -> windows_core::Result<()>;
    fn get_Settings(&self) -> windows_core::Result<ITaskSettings>;
    fn put_Settings(&self, psettings: windows_core::Ref<ITaskSettings>) -> windows_core::Result<()>;
    fn get_Data(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Data(&self, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Principal(&self) -> windows_core::Result<IPrincipal>;
    fn put_Principal(&self, pprincipal: windows_core::Ref<IPrincipal>) -> windows_core::Result<()>;
    fn get_Actions(&self) -> windows_core::Result<IActionCollection>;
    fn put_Actions(&self, pactions: windows_core::Ref<IActionCollection>) -> windows_core::Result<()>;
    fn get_XmlText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_XmlText(&self, xml: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITaskDefinition_Vtbl {
    pub const fn new<Identity: ITaskDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_RegistrationInfo<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppregistrationinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::get_RegistrationInfo(this) {
                    Ok(ok__) => {
                        ppregistrationinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RegistrationInfo<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pregistrationinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::put_RegistrationInfo(this, core::mem::transmute_copy(&pregistrationinfo)).into()
            }
        }
        unsafe extern "system" fn get_Triggers<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptriggers: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::get_Triggers(this) {
                    Ok(ok__) => {
                        pptriggers.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Triggers<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptriggers: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::put_Triggers(this, core::mem::transmute_copy(&ptriggers)).into()
            }
        }
        unsafe extern "system" fn get_Settings<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::get_Settings(this) {
                    Ok(ok__) => {
                        ppsettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Settings<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psettings: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::put_Settings(this, core::mem::transmute_copy(&psettings)).into()
            }
        }
        unsafe extern "system" fn get_Data<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::get_Data(this) {
                    Ok(ok__) => {
                        pdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Data<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::put_Data(this, core::mem::transmute(&data)).into()
            }
        }
        unsafe extern "system" fn get_Principal<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppprincipal: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::get_Principal(this) {
                    Ok(ok__) => {
                        ppprincipal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Principal<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprincipal: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::put_Principal(this, core::mem::transmute_copy(&pprincipal)).into()
            }
        }
        unsafe extern "system" fn get_Actions<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppactions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::get_Actions(this) {
                    Ok(ok__) => {
                        ppactions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Actions<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::put_Actions(this, core::mem::transmute_copy(&pactions)).into()
            }
        }
        unsafe extern "system" fn get_XmlText<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskDefinition_Impl::get_XmlText(this) {
                    Ok(ok__) => {
                        pxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_XmlText<Identity: ITaskDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskDefinition_Impl::put_XmlText(this, core::mem::transmute(&xml)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_RegistrationInfo: get_RegistrationInfo::<Identity, OFFSET>,
            put_RegistrationInfo: put_RegistrationInfo::<Identity, OFFSET>,
            get_Triggers: get_Triggers::<Identity, OFFSET>,
            put_Triggers: put_Triggers::<Identity, OFFSET>,
            get_Settings: get_Settings::<Identity, OFFSET>,
            put_Settings: put_Settings::<Identity, OFFSET>,
            get_Data: get_Data::<Identity, OFFSET>,
            put_Data: put_Data::<Identity, OFFSET>,
            get_Principal: get_Principal::<Identity, OFFSET>,
            put_Principal: put_Principal::<Identity, OFFSET>,
            get_Actions: get_Actions::<Identity, OFFSET>,
            put_Actions: put_Actions::<Identity, OFFSET>,
            get_XmlText: get_XmlText::<Identity, OFFSET>,
            put_XmlText: put_XmlText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskDefinition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITaskDefinition {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITaskFolder, ITaskFolder_Vtbl, 0x8cfac062_a080_4c15_9a88_aa7c2af80dfc);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITaskFolder {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITaskFolder, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITaskFolder {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFolders(&self, flags: super::LONG) -> windows_core::Result<ITaskFolderCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolders)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreateFolder(&self, subfoldername: &windows_core::BSTR, sddl: &super::VARIANT) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(subfoldername), core::mem::transmute_copy(sddl), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteFolder(&self, subfoldername: &windows_core::BSTR, flags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(subfoldername), flags) }
    }
    pub unsafe fn GetTask(&self, path: &windows_core::BSTR) -> windows_core::Result<IRegisteredTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTasks(&self, flags: super::LONG) -> windows_core::Result<IRegisteredTaskCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTasks)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteTask(&self, name: &windows_core::BSTR, flags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), flags) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RegisterTask(&self, path: &windows_core::BSTR, xmltext: &windows_core::BSTR, flags: super::LONG, userid: &super::VARIANT, password: &super::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::VARIANT) -> windows_core::Result<IRegisteredTask> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterTask)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), core::mem::transmute_copy(xmltext), flags, core::mem::transmute_copy(userid), core::mem::transmute_copy(password), logontype, core::mem::transmute_copy(sddl), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RegisterTaskDefinition<P1>(&self, path: &windows_core::BSTR, pdefinition: P1, flags: super::LONG, userid: &super::VARIANT, password: &super::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::VARIANT) -> windows_core::Result<IRegisteredTask>
    where
        P1: windows_core::Param<ITaskDefinition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterTaskDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), pdefinition.param().abi(), flags, core::mem::transmute_copy(userid), core::mem::transmute_copy(password), logontype, core::mem::transmute_copy(sddl), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: super::LONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSecurityDescriptor)(windows_core::Interface::as_raw(self), securityinformation, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecurityDescriptor)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sddl), flags) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskFolder_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetFolders: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFolders: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub CreateFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    CreateFolder: usize,
    #[cfg(feature = "shared")]
    pub DeleteFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteFolder: usize,
    pub GetTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetTasks: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTasks: usize,
    #[cfg(feature = "shared")]
    pub DeleteTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteTask: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub RegisterTask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::VARIANT, super::VARIANT, TASK_LOGON_TYPE, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    RegisterTask: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub RegisterTaskDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::VARIANT, super::VARIANT, TASK_LOGON_TYPE, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    RegisterTaskDefinition: usize,
    #[cfg(feature = "shared")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSecurityDescriptor: usize,
    #[cfg(feature = "shared")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSecurityDescriptor: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITaskFolder_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<ITaskFolder>;
    fn GetFolders(&self, flags: super::LONG) -> windows_core::Result<ITaskFolderCollection>;
    fn CreateFolder(&self, subfoldername: &windows_core::BSTR, sddl: &super::VARIANT) -> windows_core::Result<ITaskFolder>;
    fn DeleteFolder(&self, subfoldername: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<()>;
    fn GetTask(&self, path: &windows_core::BSTR) -> windows_core::Result<IRegisteredTask>;
    fn GetTasks(&self, flags: super::LONG) -> windows_core::Result<IRegisteredTaskCollection>;
    fn DeleteTask(&self, name: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<()>;
    fn RegisterTask(&self, path: &windows_core::BSTR, xmltext: &windows_core::BSTR, flags: super::LONG, userid: &super::VARIANT, password: &super::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::VARIANT) -> windows_core::Result<IRegisteredTask>;
    fn RegisterTaskDefinition(&self, path: &windows_core::BSTR, pdefinition: windows_core::Ref<ITaskDefinition>, flags: super::LONG, userid: &super::VARIANT, password: &super::VARIANT, logontype: TASK_LOGON_TYPE, sddl: &super::VARIANT) -> windows_core::Result<IRegisteredTask>;
    fn GetSecurityDescriptor(&self, securityinformation: super::LONG) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurityDescriptor(&self, sddl: &windows_core::BSTR, flags: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITaskFolder_Vtbl {
    pub const fn new<Identity: ITaskFolder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::get_Name(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Path<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::get_Path(this) {
                    Ok(ok__) => {
                        ppath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFolder<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetFolder(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        ppfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFolders<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::LONG, ppfolders: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetFolders(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppfolders.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFolder<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subfoldername: *mut core::ffi::c_void, sddl: super::VARIANT, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::CreateFolder(this, core::mem::transmute(&subfoldername), core::mem::transmute(&sddl)) {
                    Ok(ok__) => {
                        ppfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteFolder<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subfoldername: *mut core::ffi::c_void, flags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskFolder_Impl::DeleteFolder(this, core::mem::transmute(&subfoldername), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn GetTask<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetTask(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTasks<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::LONG, pptasks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetTasks(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pptasks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, flags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskFolder_Impl::DeleteTask(this, core::mem::transmute(&name), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn RegisterTask<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, xmltext: *mut core::ffi::c_void, flags: super::LONG, userid: super::VARIANT, password: super::VARIANT, logontype: TASK_LOGON_TYPE, sddl: super::VARIANT, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::RegisterTask(this, core::mem::transmute(&path), core::mem::transmute(&xmltext), core::mem::transmute_copy(&flags), core::mem::transmute(&userid), core::mem::transmute(&password), core::mem::transmute_copy(&logontype), core::mem::transmute(&sddl)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterTaskDefinition<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, pdefinition: *mut core::ffi::c_void, flags: super::LONG, userid: super::VARIANT, password: super::VARIANT, logontype: TASK_LOGON_TYPE, sddl: super::VARIANT, pptask: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::RegisterTaskDefinition(this, core::mem::transmute(&path), core::mem::transmute_copy(&pdefinition), core::mem::transmute_copy(&flags), core::mem::transmute(&userid), core::mem::transmute(&password), core::mem::transmute_copy(&logontype), core::mem::transmute(&sddl)) {
                    Ok(ok__) => {
                        pptask.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, securityinformation: super::LONG, psddl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolder_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&securityinformation)) {
                    Ok(ok__) => {
                        psddl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ITaskFolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sddl: *mut core::ffi::c_void, flags: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskFolder_Impl::SetSecurityDescriptor(this, core::mem::transmute(&sddl), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_Path: get_Path::<Identity, OFFSET>,
            GetFolder: GetFolder::<Identity, OFFSET>,
            GetFolders: GetFolders::<Identity, OFFSET>,
            CreateFolder: CreateFolder::<Identity, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, OFFSET>,
            GetTask: GetTask::<Identity, OFFSET>,
            GetTasks: GetTasks::<Identity, OFFSET>,
            DeleteTask: DeleteTask::<Identity, OFFSET>,
            RegisterTask: RegisterTask::<Identity, OFFSET>,
            RegisterTaskDefinition: RegisterTaskDefinition::<Identity, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskFolder as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITaskFolder {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITaskFolderCollection, ITaskFolderCollection_Vtbl, 0x79184a66_8664_423f_97f1_637356a5d812);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITaskFolderCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITaskFolderCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITaskFolderCollection {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<ITaskFolder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskFolderCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITaskFolderCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_Item(&self, index: &super::VARIANT) -> windows_core::Result<ITaskFolder>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITaskFolderCollection_Vtbl {
    pub const fn new<Identity: ITaskFolderCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolderCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolderCollection_Impl::get_Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: ITaskFolderCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskFolderCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskFolderCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITaskFolderCollection {}
windows_core::imp::define_interface!(ITaskHandler, ITaskHandler_Vtbl, 0x839d7762_5121_4009_9234_4f0d19394f04);
windows_core::imp::interface_hierarchy!(ITaskHandler, windows_core::IUnknown);
impl ITaskHandler {
    pub unsafe fn Start<P0>(&self, phandlerservices: P0, data: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), phandlerservices.param().abi(), core::mem::transmute_copy(data)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Resume(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITaskHandler_Impl: windows_core::IUnknownImpl {
    fn Start(&self, phandlerservices: windows_core::Ref<windows_core::IUnknown>, data: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
}
impl ITaskHandler_Vtbl {
    pub const fn new<Identity: ITaskHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: ITaskHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandlerservices: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandler_Impl::Start(this, core::mem::transmute_copy(&phandlerservices), core::mem::transmute(&data)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: ITaskHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretcode: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskHandler_Impl::Stop(this) {
                    Ok(ok__) => {
                        pretcode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pause<Identity: ITaskHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandler_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Resume<Identity: ITaskHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandler_Impl::Resume(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITaskHandler {}
windows_core::imp::define_interface!(ITaskHandlerStatus, ITaskHandlerStatus_Vtbl, 0xeaec7a8f_27a0_4ddc_8675_14726a01a38a);
windows_core::imp::interface_hierarchy!(ITaskHandlerStatus, windows_core::IUnknown);
impl ITaskHandlerStatus {
    pub unsafe fn UpdateStatus(&self, percentcomplete: i16, statusmessage: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateStatus)(windows_core::Interface::as_raw(self), percentcomplete, core::mem::transmute_copy(statusmessage)) }
    }
    pub unsafe fn TaskCompleted(&self, taskerrcode: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TaskCompleted)(windows_core::Interface::as_raw(self), taskerrcode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskHandlerStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub UpdateStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TaskCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait ITaskHandlerStatus_Impl: windows_core::IUnknownImpl {
    fn UpdateStatus(&self, percentcomplete: i16, statusmessage: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TaskCompleted(&self, taskerrcode: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl ITaskHandlerStatus_Vtbl {
    pub const fn new<Identity: ITaskHandlerStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateStatus<Identity: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, percentcomplete: i16, statusmessage: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandlerStatus_Impl::UpdateStatus(this, core::mem::transmute_copy(&percentcomplete), core::mem::transmute(&statusmessage)).into()
            }
        }
        unsafe extern "system" fn TaskCompleted<Identity: ITaskHandlerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, taskerrcode: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskHandlerStatus_Impl::TaskCompleted(this, core::mem::transmute_copy(&taskerrcode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            UpdateStatus: UpdateStatus::<Identity, OFFSET>,
            TaskCompleted: TaskCompleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskHandlerStatus as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITaskHandlerStatus {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITaskNamedValueCollection, ITaskNamedValueCollection_Vtbl, 0xb4ef826b_63c3_46e4_a504_ef69e4f7ea4d);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITaskNamedValueCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITaskNamedValueCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITaskNamedValueCollection {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<ITaskNamedValuePair> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Create(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<ITaskNamedValuePair> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Remove(&self, index: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskNamedValueCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITaskNamedValueCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<ITaskNamedValuePair>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Create(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<ITaskNamedValuePair>;
    fn Remove(&self, index: super::LONG) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITaskNamedValueCollection_Vtbl {
    pub const fn new<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskNamedValueCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, pppair: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskNamedValueCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pppair.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskNamedValueCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: *mut core::ffi::c_void, pppair: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskNamedValueCollection_Impl::Create(this, core::mem::transmute(&name), core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        pppair.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValueCollection_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ITaskNamedValueCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValueCollection_Impl::Clear(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskNamedValueCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITaskNamedValueCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITaskNamedValuePair, ITaskNamedValuePair_Vtbl, 0x39038068_2b46_4afd_8662_7bb6f868d221);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITaskNamedValuePair {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITaskNamedValuePair, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITaskNamedValuePair {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Value(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskNamedValuePair_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITaskNamedValuePair_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Value(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITaskNamedValuePair_Vtbl {
    pub const fn new<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskNamedValuePair_Impl::get_Name(this) {
                    Ok(ok__) => {
                        pname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValuePair_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_Value<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskNamedValuePair_Impl::get_Value(this) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: ITaskNamedValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskNamedValuePair_Impl::put_Value(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskNamedValuePair as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITaskNamedValuePair {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITaskService, ITaskService_Vtbl, 0x2faba4c7_4da9_4013_9697_20cc3fd40f85);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITaskService {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITaskService, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITaskService {
    pub unsafe fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<ITaskFolder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRunningTasks(&self, flags: super::LONG) -> windows_core::Result<IRunningTaskCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRunningTasks)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn NewTask(&self, flags: super::DWORD) -> windows_core::Result<ITaskDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewTask)(windows_core::Interface::as_raw(self), flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Connect(&self, servername: &super::VARIANT, user: &super::VARIANT, domain: &super::VARIANT, password: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servername), core::mem::transmute_copy(user), core::mem::transmute_copy(domain), core::mem::transmute_copy(password)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Connected(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Connected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_TargetServer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TargetServer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ConnectedUser(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ConnectedUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ConnectedDomain(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ConnectedDomain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_HighestVersion(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HighestVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskService_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetRunningTasks: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRunningTasks: usize,
    #[cfg(feature = "shared")]
    pub NewTask: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    NewTask: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, super::VARIANT, super::VARIANT, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Connect: usize,
    #[cfg(feature = "wtypes")]
    pub get_Connected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Connected: usize,
    pub get_TargetServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ConnectedUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ConnectedDomain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_HighestVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_HighestVersion: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITaskService_Impl: super::IDispatch_Impl {
    fn GetFolder(&self, path: &windows_core::BSTR) -> windows_core::Result<ITaskFolder>;
    fn GetRunningTasks(&self, flags: super::LONG) -> windows_core::Result<IRunningTaskCollection>;
    fn NewTask(&self, flags: super::DWORD) -> windows_core::Result<ITaskDefinition>;
    fn Connect(&self, servername: &super::VARIANT, user: &super::VARIANT, domain: &super::VARIANT, password: &super::VARIANT) -> windows_core::Result<()>;
    fn get_Connected(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_TargetServer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ConnectedUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ConnectedDomain(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_HighestVersion(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITaskService_Vtbl {
    pub const fn new<Identity: ITaskService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFolder<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, ppfolder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::GetFolder(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        ppfolder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRunningTasks<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::LONG, pprunningtasks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::GetRunningTasks(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        pprunningtasks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NewTask<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: super::DWORD, ppdefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::NewTask(this, core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        ppdefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Connect<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servername: super::VARIANT, user: super::VARIANT, domain: super::VARIANT, password: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskService_Impl::Connect(this, core::mem::transmute(&servername), core::mem::transmute(&user), core::mem::transmute(&domain), core::mem::transmute(&password)).into()
            }
        }
        unsafe extern "system" fn get_Connected<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnected: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::get_Connected(this) {
                    Ok(ok__) => {
                        pconnected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TargetServer<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::get_TargetServer(this) {
                    Ok(ok__) => {
                        pserver.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ConnectedUser<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::get_ConnectedUser(this) {
                    Ok(ok__) => {
                        puser.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ConnectedDomain<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdomain: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::get_ConnectedDomain(this) {
                    Ok(ok__) => {
                        pdomain.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HighestVersion<Identity: ITaskService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pversion: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskService_Impl::get_HighestVersion(this) {
                    Ok(ok__) => {
                        pversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetFolder: GetFolder::<Identity, OFFSET>,
            GetRunningTasks: GetRunningTasks::<Identity, OFFSET>,
            NewTask: NewTask::<Identity, OFFSET>,
            Connect: Connect::<Identity, OFFSET>,
            get_Connected: get_Connected::<Identity, OFFSET>,
            get_TargetServer: get_TargetServer::<Identity, OFFSET>,
            get_ConnectedUser: get_ConnectedUser::<Identity, OFFSET>,
            get_ConnectedDomain: get_ConnectedDomain::<Identity, OFFSET>,
            get_HighestVersion: get_HighestVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskService as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITaskService {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITaskSettings, ITaskSettings_Vtbl, 0x8fd4711d_2d02_4c8c_87e3_eff699de127e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITaskSettings {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITaskSettings, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITaskSettings {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowDemandStart(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowDemandStart)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowDemandStart(&self, allowdemandstart: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowDemandStart)(windows_core::Interface::as_raw(self), allowdemandstart) }
    }
    pub unsafe fn get_RestartInterval(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RestartInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RestartInterval(&self, restartinterval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RestartInterval)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(restartinterval)) }
    }
    pub unsafe fn get_RestartCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RestartCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_RestartCount(&self, restartcount: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RestartCount)(windows_core::Interface::as_raw(self), restartcount) }
    }
    pub unsafe fn get_MultipleInstances(&self) -> windows_core::Result<TASK_INSTANCES_POLICY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MultipleInstances)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MultipleInstances)(windows_core::Interface::as_raw(self), policy) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_StopIfGoingOnBatteries(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StopIfGoingOnBatteries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_StopIfGoingOnBatteries(&self, stopifonbatteries: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StopIfGoingOnBatteries)(windows_core::Interface::as_raw(self), stopifonbatteries) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DisallowStartIfOnBatteries(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisallowStartIfOnBatteries)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_DisallowStartIfOnBatteries(&self, disallowstart: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisallowStartIfOnBatteries)(windows_core::Interface::as_raw(self), disallowstart) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AllowHardTerminate(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowHardTerminate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AllowHardTerminate(&self, allowhardterminate: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowHardTerminate)(windows_core::Interface::as_raw(self), allowhardterminate) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_StartWhenAvailable(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartWhenAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_StartWhenAvailable(&self, startwhenavailable: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartWhenAvailable)(windows_core::Interface::as_raw(self), startwhenavailable) }
    }
    pub unsafe fn get_XmlText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_XmlText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_XmlText(&self, text: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_XmlText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RunOnlyIfNetworkAvailable(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RunOnlyIfNetworkAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_RunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RunOnlyIfNetworkAvailable)(windows_core::Interface::as_raw(self), runonlyifnetworkavailable) }
    }
    pub unsafe fn get_ExecutionTimeLimit(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExecutionTimeLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ExecutionTimeLimit(&self, executiontimelimit: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExecutionTimeLimit)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(executiontimelimit)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    pub unsafe fn get_DeleteExpiredTaskAfter(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DeleteExpiredTaskAfter)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DeleteExpiredTaskAfter(&self, expirationdelay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DeleteExpiredTaskAfter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(expirationdelay)) }
    }
    pub unsafe fn get_Priority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Priority(&self, priority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Priority)(windows_core::Interface::as_raw(self), priority) }
    }
    pub unsafe fn get_Compatibility(&self) -> windows_core::Result<TASK_COMPATIBILITY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Compatibility)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Compatibility(&self, compatlevel: TASK_COMPATIBILITY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Compatibility)(windows_core::Interface::as_raw(self), compatlevel) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Hidden(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Hidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Hidden(&self, hidden: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Hidden)(windows_core::Interface::as_raw(self), hidden) }
    }
    pub unsafe fn get_IdleSettings(&self) -> windows_core::Result<IIdleSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IdleSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_IdleSettings<P0>(&self, pidlesettings: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IIdleSettings>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_IdleSettings)(windows_core::Interface::as_raw(self), pidlesettings.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RunOnlyIfIdle(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RunOnlyIfIdle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_RunOnlyIfIdle(&self, runonlyifidle: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RunOnlyIfIdle)(windows_core::Interface::as_raw(self), runonlyifidle) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_WakeToRun(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WakeToRun)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_WakeToRun(&self, wake: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WakeToRun)(windows_core::Interface::as_raw(self), wake) }
    }
    pub unsafe fn get_NetworkSettings(&self) -> windows_core::Result<INetworkSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NetworkSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_NetworkSettings<P0>(&self, pnetworksettings: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<INetworkSettings>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_NetworkSettings)(windows_core::Interface::as_raw(self), pnetworksettings.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_AllowDemandStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowDemandStart: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowDemandStart: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowDemandStart: usize,
    pub get_RestartInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RestartInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RestartCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_RestartCount: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_MultipleInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_INSTANCES_POLICY) -> windows_core::HRESULT,
    pub put_MultipleInstances: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_INSTANCES_POLICY) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_StopIfGoingOnBatteries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_StopIfGoingOnBatteries: usize,
    #[cfg(feature = "wtypes")]
    pub put_StopIfGoingOnBatteries: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_StopIfGoingOnBatteries: usize,
    #[cfg(feature = "wtypes")]
    pub get_DisallowStartIfOnBatteries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DisallowStartIfOnBatteries: usize,
    #[cfg(feature = "wtypes")]
    pub put_DisallowStartIfOnBatteries: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_DisallowStartIfOnBatteries: usize,
    #[cfg(feature = "wtypes")]
    pub get_AllowHardTerminate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AllowHardTerminate: usize,
    #[cfg(feature = "wtypes")]
    pub put_AllowHardTerminate: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AllowHardTerminate: usize,
    #[cfg(feature = "wtypes")]
    pub get_StartWhenAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_StartWhenAvailable: usize,
    #[cfg(feature = "wtypes")]
    pub put_StartWhenAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_StartWhenAvailable: usize,
    pub get_XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_XmlText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_RunOnlyIfNetworkAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RunOnlyIfNetworkAvailable: usize,
    #[cfg(feature = "wtypes")]
    pub put_RunOnlyIfNetworkAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_RunOnlyIfNetworkAvailable: usize,
    pub get_ExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
    pub get_DeleteExpiredTaskAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DeleteExpiredTaskAfter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Priority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_Compatibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_COMPATIBILITY) -> windows_core::HRESULT,
    pub put_Compatibility: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_COMPATIBILITY) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Hidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Hidden: usize,
    #[cfg(feature = "wtypes")]
    pub put_Hidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Hidden: usize,
    pub get_IdleSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_IdleSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_RunOnlyIfIdle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RunOnlyIfIdle: usize,
    #[cfg(feature = "wtypes")]
    pub put_RunOnlyIfIdle: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_RunOnlyIfIdle: usize,
    #[cfg(feature = "wtypes")]
    pub get_WakeToRun: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_WakeToRun: usize,
    #[cfg(feature = "wtypes")]
    pub put_WakeToRun: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_WakeToRun: usize,
    pub get_NetworkSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_NetworkSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITaskSettings_Impl: super::IDispatch_Impl {
    fn get_AllowDemandStart(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowDemandStart(&self, allowdemandstart: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RestartInterval(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RestartInterval(&self, restartinterval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RestartCount(&self) -> windows_core::Result<i32>;
    fn put_RestartCount(&self, restartcount: i32) -> windows_core::Result<()>;
    fn get_MultipleInstances(&self) -> windows_core::Result<TASK_INSTANCES_POLICY>;
    fn put_MultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> windows_core::Result<()>;
    fn get_StopIfGoingOnBatteries(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_StopIfGoingOnBatteries(&self, stopifonbatteries: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_DisallowStartIfOnBatteries(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_DisallowStartIfOnBatteries(&self, disallowstart: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AllowHardTerminate(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AllowHardTerminate(&self, allowhardterminate: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_StartWhenAvailable(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_StartWhenAvailable(&self, startwhenavailable: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_XmlText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_XmlText(&self, text: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RunOnlyIfNetworkAvailable(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_RunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ExecutionTimeLimit(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ExecutionTimeLimit(&self, executiontimelimit: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_DeleteExpiredTaskAfter(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DeleteExpiredTaskAfter(&self, expirationdelay: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Priority(&self) -> windows_core::Result<i32>;
    fn put_Priority(&self, priority: i32) -> windows_core::Result<()>;
    fn get_Compatibility(&self) -> windows_core::Result<TASK_COMPATIBILITY>;
    fn put_Compatibility(&self, compatlevel: TASK_COMPATIBILITY) -> windows_core::Result<()>;
    fn get_Hidden(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Hidden(&self, hidden: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_IdleSettings(&self) -> windows_core::Result<IIdleSettings>;
    fn put_IdleSettings(&self, pidlesettings: windows_core::Ref<IIdleSettings>) -> windows_core::Result<()>;
    fn get_RunOnlyIfIdle(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_RunOnlyIfIdle(&self, runonlyifidle: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_WakeToRun(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_WakeToRun(&self, wake: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_NetworkSettings(&self) -> windows_core::Result<INetworkSettings>;
    fn put_NetworkSettings(&self, pnetworksettings: windows_core::Ref<INetworkSettings>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITaskSettings_Vtbl {
    pub const fn new<Identity: ITaskSettings_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AllowDemandStart<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallowdemandstart: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_AllowDemandStart(this) {
                    Ok(ok__) => {
                        pallowdemandstart.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowDemandStart<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowdemandstart: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_AllowDemandStart(this, core::mem::transmute_copy(&allowdemandstart)).into()
            }
        }
        unsafe extern "system" fn get_RestartInterval<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestartinterval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_RestartInterval(this) {
                    Ok(ok__) => {
                        prestartinterval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RestartInterval<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restartinterval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_RestartInterval(this, core::mem::transmute(&restartinterval)).into()
            }
        }
        unsafe extern "system" fn get_RestartCount<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prestartcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_RestartCount(this) {
                    Ok(ok__) => {
                        prestartcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RestartCount<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restartcount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_RestartCount(this, core::mem::transmute_copy(&restartcount)).into()
            }
        }
        unsafe extern "system" fn get_MultipleInstances<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_MultipleInstances(this) {
                    Ok(ok__) => {
                        ppolicy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MultipleInstances<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_MultipleInstances(this, core::mem::transmute_copy(&policy)).into()
            }
        }
        unsafe extern "system" fn get_StopIfGoingOnBatteries<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstopifonbatteries: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_StopIfGoingOnBatteries(this) {
                    Ok(ok__) => {
                        pstopifonbatteries.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StopIfGoingOnBatteries<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stopifonbatteries: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_StopIfGoingOnBatteries(this, core::mem::transmute_copy(&stopifonbatteries)).into()
            }
        }
        unsafe extern "system" fn get_DisallowStartIfOnBatteries<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisallowstart: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_DisallowStartIfOnBatteries(this) {
                    Ok(ok__) => {
                        pdisallowstart.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisallowStartIfOnBatteries<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowstart: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_DisallowStartIfOnBatteries(this, core::mem::transmute_copy(&disallowstart)).into()
            }
        }
        unsafe extern "system" fn get_AllowHardTerminate<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallowhardterminate: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_AllowHardTerminate(this) {
                    Ok(ok__) => {
                        pallowhardterminate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AllowHardTerminate<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowhardterminate: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_AllowHardTerminate(this, core::mem::transmute_copy(&allowhardterminate)).into()
            }
        }
        unsafe extern "system" fn get_StartWhenAvailable<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstartwhenavailable: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_StartWhenAvailable(this) {
                    Ok(ok__) => {
                        pstartwhenavailable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartWhenAvailable<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startwhenavailable: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_StartWhenAvailable(this, core::mem::transmute_copy(&startwhenavailable)).into()
            }
        }
        unsafe extern "system" fn get_XmlText<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_XmlText(this) {
                    Ok(ok__) => {
                        ptext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_XmlText<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_XmlText(this, core::mem::transmute(&text)).into()
            }
        }
        unsafe extern "system" fn get_RunOnlyIfNetworkAvailable<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prunonlyifnetworkavailable: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_RunOnlyIfNetworkAvailable(this) {
                    Ok(ok__) => {
                        prunonlyifnetworkavailable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RunOnlyIfNetworkAvailable<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runonlyifnetworkavailable: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_RunOnlyIfNetworkAvailable(this, core::mem::transmute_copy(&runonlyifnetworkavailable)).into()
            }
        }
        unsafe extern "system" fn get_ExecutionTimeLimit<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexecutiontimelimit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_ExecutionTimeLimit(this) {
                    Ok(ok__) => {
                        pexecutiontimelimit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExecutionTimeLimit<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executiontimelimit: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_ExecutionTimeLimit(this, core::mem::transmute(&executiontimelimit)).into()
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        penabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_DeleteExpiredTaskAfter<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexpirationdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_DeleteExpiredTaskAfter(this) {
                    Ok(ok__) => {
                        pexpirationdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DeleteExpiredTaskAfter<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expirationdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_DeleteExpiredTaskAfter(this, core::mem::transmute(&expirationdelay)).into()
            }
        }
        unsafe extern "system" fn get_Priority<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_Priority(this) {
                    Ok(ok__) => {
                        ppriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Priority<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_Priority(this, core::mem::transmute_copy(&priority)).into()
            }
        }
        unsafe extern "system" fn get_Compatibility<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_Compatibility(this) {
                    Ok(ok__) => {
                        pcompatlevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Compatibility<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_Compatibility(this, core::mem::transmute_copy(&compatlevel)).into()
            }
        }
        unsafe extern "system" fn get_Hidden<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phidden: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_Hidden(this) {
                    Ok(ok__) => {
                        phidden.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Hidden<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hidden: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_Hidden(this, core::mem::transmute_copy(&hidden)).into()
            }
        }
        unsafe extern "system" fn get_IdleSettings<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppidlesettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_IdleSettings(this) {
                    Ok(ok__) => {
                        ppidlesettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IdleSettings<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidlesettings: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_IdleSettings(this, core::mem::transmute_copy(&pidlesettings)).into()
            }
        }
        unsafe extern "system" fn get_RunOnlyIfIdle<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prunonlyifidle: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_RunOnlyIfIdle(this) {
                    Ok(ok__) => {
                        prunonlyifidle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RunOnlyIfIdle<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runonlyifidle: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_RunOnlyIfIdle(this, core::mem::transmute_copy(&runonlyifidle)).into()
            }
        }
        unsafe extern "system" fn get_WakeToRun<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwake: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_WakeToRun(this) {
                    Ok(ok__) => {
                        pwake.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WakeToRun<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wake: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_WakeToRun(this, core::mem::transmute_copy(&wake)).into()
            }
        }
        unsafe extern "system" fn get_NetworkSettings<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetworksettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings_Impl::get_NetworkSettings(this) {
                    Ok(ok__) => {
                        ppnetworksettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NetworkSettings<Identity: ITaskSettings_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworksettings: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings_Impl::put_NetworkSettings(this, core::mem::transmute_copy(&pnetworksettings)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_AllowDemandStart: get_AllowDemandStart::<Identity, OFFSET>,
            put_AllowDemandStart: put_AllowDemandStart::<Identity, OFFSET>,
            get_RestartInterval: get_RestartInterval::<Identity, OFFSET>,
            put_RestartInterval: put_RestartInterval::<Identity, OFFSET>,
            get_RestartCount: get_RestartCount::<Identity, OFFSET>,
            put_RestartCount: put_RestartCount::<Identity, OFFSET>,
            get_MultipleInstances: get_MultipleInstances::<Identity, OFFSET>,
            put_MultipleInstances: put_MultipleInstances::<Identity, OFFSET>,
            get_StopIfGoingOnBatteries: get_StopIfGoingOnBatteries::<Identity, OFFSET>,
            put_StopIfGoingOnBatteries: put_StopIfGoingOnBatteries::<Identity, OFFSET>,
            get_DisallowStartIfOnBatteries: get_DisallowStartIfOnBatteries::<Identity, OFFSET>,
            put_DisallowStartIfOnBatteries: put_DisallowStartIfOnBatteries::<Identity, OFFSET>,
            get_AllowHardTerminate: get_AllowHardTerminate::<Identity, OFFSET>,
            put_AllowHardTerminate: put_AllowHardTerminate::<Identity, OFFSET>,
            get_StartWhenAvailable: get_StartWhenAvailable::<Identity, OFFSET>,
            put_StartWhenAvailable: put_StartWhenAvailable::<Identity, OFFSET>,
            get_XmlText: get_XmlText::<Identity, OFFSET>,
            put_XmlText: put_XmlText::<Identity, OFFSET>,
            get_RunOnlyIfNetworkAvailable: get_RunOnlyIfNetworkAvailable::<Identity, OFFSET>,
            put_RunOnlyIfNetworkAvailable: put_RunOnlyIfNetworkAvailable::<Identity, OFFSET>,
            get_ExecutionTimeLimit: get_ExecutionTimeLimit::<Identity, OFFSET>,
            put_ExecutionTimeLimit: put_ExecutionTimeLimit::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
            get_DeleteExpiredTaskAfter: get_DeleteExpiredTaskAfter::<Identity, OFFSET>,
            put_DeleteExpiredTaskAfter: put_DeleteExpiredTaskAfter::<Identity, OFFSET>,
            get_Priority: get_Priority::<Identity, OFFSET>,
            put_Priority: put_Priority::<Identity, OFFSET>,
            get_Compatibility: get_Compatibility::<Identity, OFFSET>,
            put_Compatibility: put_Compatibility::<Identity, OFFSET>,
            get_Hidden: get_Hidden::<Identity, OFFSET>,
            put_Hidden: put_Hidden::<Identity, OFFSET>,
            get_IdleSettings: get_IdleSettings::<Identity, OFFSET>,
            put_IdleSettings: put_IdleSettings::<Identity, OFFSET>,
            get_RunOnlyIfIdle: get_RunOnlyIfIdle::<Identity, OFFSET>,
            put_RunOnlyIfIdle: put_RunOnlyIfIdle::<Identity, OFFSET>,
            get_WakeToRun: get_WakeToRun::<Identity, OFFSET>,
            put_WakeToRun: put_WakeToRun::<Identity, OFFSET>,
            get_NetworkSettings: get_NetworkSettings::<Identity, OFFSET>,
            put_NetworkSettings: put_NetworkSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskSettings as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITaskSettings {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITaskSettings2, ITaskSettings2_Vtbl, 0x2c05c3f0_6eed_4c05_a15f_ed7d7a98a369);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITaskSettings2 {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITaskSettings2, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITaskSettings2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DisallowStartOnRemoteAppSession(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_DisallowStartOnRemoteAppSession(&self, disallowstart: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), disallowstart) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UseUnifiedSchedulingEngine(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UseUnifiedSchedulingEngine(&self, useunifiedengine: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), useunifiedengine) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings2_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_DisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DisallowStartOnRemoteAppSession: usize,
    #[cfg(feature = "wtypes")]
    pub put_DisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_DisallowStartOnRemoteAppSession: usize,
    #[cfg(feature = "wtypes")]
    pub get_UseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UseUnifiedSchedulingEngine: usize,
    #[cfg(feature = "wtypes")]
    pub put_UseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UseUnifiedSchedulingEngine: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITaskSettings2_Impl: super::IDispatch_Impl {
    fn get_DisallowStartOnRemoteAppSession(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_DisallowStartOnRemoteAppSession(&self, disallowstart: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UseUnifiedSchedulingEngine(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_UseUnifiedSchedulingEngine(&self, useunifiedengine: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITaskSettings2_Vtbl {
    pub const fn new<Identity: ITaskSettings2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DisallowStartOnRemoteAppSession<Identity: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisallowstart: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings2_Impl::get_DisallowStartOnRemoteAppSession(this) {
                    Ok(ok__) => {
                        pdisallowstart.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisallowStartOnRemoteAppSession<Identity: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowstart: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings2_Impl::put_DisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&disallowstart)).into()
            }
        }
        unsafe extern "system" fn get_UseUnifiedSchedulingEngine<Identity: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puseunifiedengine: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings2_Impl::get_UseUnifiedSchedulingEngine(this) {
                    Ok(ok__) => {
                        puseunifiedengine.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UseUnifiedSchedulingEngine<Identity: ITaskSettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useunifiedengine: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings2_Impl::put_UseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&useunifiedengine)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_DisallowStartOnRemoteAppSession: get_DisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            put_DisallowStartOnRemoteAppSession: put_DisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            get_UseUnifiedSchedulingEngine: get_UseUnifiedSchedulingEngine::<Identity, OFFSET>,
            put_UseUnifiedSchedulingEngine: put_UseUnifiedSchedulingEngine::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskSettings2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITaskSettings2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITaskSettings3, ITaskSettings3_Vtbl, 0x0ad9d0d7_0c7f_4ebb_9a5f_d1c648dca528);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITaskSettings3 {
    type Target = ITaskSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITaskSettings3, windows_core::IUnknown, super::IDispatch, ITaskSettings);
#[cfg(feature = "oaidl")]
impl ITaskSettings3 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DisallowStartOnRemoteAppSession(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_DisallowStartOnRemoteAppSession(&self, disallowstart: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisallowStartOnRemoteAppSession)(windows_core::Interface::as_raw(self), disallowstart) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UseUnifiedSchedulingEngine(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UseUnifiedSchedulingEngine(&self, useunifiedengine: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UseUnifiedSchedulingEngine)(windows_core::Interface::as_raw(self), useunifiedengine) }
    }
    pub unsafe fn get_MaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MaintenanceSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_MaintenanceSettings<P0>(&self, pmaintenancesettings: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMaintenanceSettings>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_MaintenanceSettings)(windows_core::Interface::as_raw(self), pmaintenancesettings.param().abi()) }
    }
    pub unsafe fn CreateMaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMaintenanceSettings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Volatile(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Volatile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Volatile(&self, volatile: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Volatile)(windows_core::Interface::as_raw(self), volatile) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings3_Vtbl {
    pub base__: ITaskSettings_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_DisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DisallowStartOnRemoteAppSession: usize,
    #[cfg(feature = "wtypes")]
    pub put_DisallowStartOnRemoteAppSession: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_DisallowStartOnRemoteAppSession: usize,
    #[cfg(feature = "wtypes")]
    pub get_UseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UseUnifiedSchedulingEngine: usize,
    #[cfg(feature = "wtypes")]
    pub put_UseUnifiedSchedulingEngine: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UseUnifiedSchedulingEngine: usize,
    pub get_MaintenanceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_MaintenanceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMaintenanceSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Volatile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Volatile: usize,
    #[cfg(feature = "wtypes")]
    pub put_Volatile: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Volatile: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITaskSettings3_Impl: ITaskSettings_Impl {
    fn get_DisallowStartOnRemoteAppSession(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_DisallowStartOnRemoteAppSession(&self, disallowstart: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UseUnifiedSchedulingEngine(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_UseUnifiedSchedulingEngine(&self, useunifiedengine: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_MaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings>;
    fn put_MaintenanceSettings(&self, pmaintenancesettings: windows_core::Ref<IMaintenanceSettings>) -> windows_core::Result<()>;
    fn CreateMaintenanceSettings(&self) -> windows_core::Result<IMaintenanceSettings>;
    fn get_Volatile(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Volatile(&self, volatile: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITaskSettings3_Vtbl {
    pub const fn new<Identity: ITaskSettings3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DisallowStartOnRemoteAppSession<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisallowstart: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings3_Impl::get_DisallowStartOnRemoteAppSession(this) {
                    Ok(ok__) => {
                        pdisallowstart.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisallowStartOnRemoteAppSession<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disallowstart: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::put_DisallowStartOnRemoteAppSession(this, core::mem::transmute_copy(&disallowstart)).into()
            }
        }
        unsafe extern "system" fn get_UseUnifiedSchedulingEngine<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puseunifiedengine: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings3_Impl::get_UseUnifiedSchedulingEngine(this) {
                    Ok(ok__) => {
                        puseunifiedengine.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UseUnifiedSchedulingEngine<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useunifiedengine: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::put_UseUnifiedSchedulingEngine(this, core::mem::transmute_copy(&useunifiedengine)).into()
            }
        }
        unsafe extern "system" fn get_MaintenanceSettings<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmaintenancesettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings3_Impl::get_MaintenanceSettings(this) {
                    Ok(ok__) => {
                        ppmaintenancesettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MaintenanceSettings<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaintenancesettings: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::put_MaintenanceSettings(this, core::mem::transmute_copy(&pmaintenancesettings)).into()
            }
        }
        unsafe extern "system" fn CreateMaintenanceSettings<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmaintenancesettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings3_Impl::CreateMaintenanceSettings(this) {
                    Ok(ok__) => {
                        ppmaintenancesettings.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Volatile<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvolatile: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskSettings3_Impl::get_Volatile(this) {
                    Ok(ok__) => {
                        pvolatile.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Volatile<Identity: ITaskSettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volatile: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskSettings3_Impl::put_Volatile(this, core::mem::transmute_copy(&volatile)).into()
            }
        }
        Self {
            base__: ITaskSettings_Vtbl::new::<Identity, OFFSET>(),
            get_DisallowStartOnRemoteAppSession: get_DisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            put_DisallowStartOnRemoteAppSession: put_DisallowStartOnRemoteAppSession::<Identity, OFFSET>,
            get_UseUnifiedSchedulingEngine: get_UseUnifiedSchedulingEngine::<Identity, OFFSET>,
            put_UseUnifiedSchedulingEngine: put_UseUnifiedSchedulingEngine::<Identity, OFFSET>,
            get_MaintenanceSettings: get_MaintenanceSettings::<Identity, OFFSET>,
            put_MaintenanceSettings: put_MaintenanceSettings::<Identity, OFFSET>,
            CreateMaintenanceSettings: CreateMaintenanceSettings::<Identity, OFFSET>,
            get_Volatile: get_Volatile::<Identity, OFFSET>,
            put_Volatile: put_Volatile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskSettings3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITaskSettings as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITaskSettings3 {}
windows_core::imp::define_interface!(ITaskVariables, ITaskVariables_Vtbl, 0x3e4c9351_d966_4b8b_bb87_ceba68bb0107);
windows_core::imp::interface_hierarchy!(ITaskVariables, windows_core::IUnknown);
impl ITaskVariables {
    pub unsafe fn GetInput(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInput)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetOutput(&self, input: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOutput)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(input)) }
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskVariables_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITaskVariables_Impl: windows_core::IUnknownImpl {
    fn GetInput(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOutput(&self, input: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetContext(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl ITaskVariables_Vtbl {
    pub const fn new<Identity: ITaskVariables_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInput<Identity: ITaskVariables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskVariables_Impl::GetInput(this) {
                    Ok(ok__) => {
                        pinput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutput<Identity: ITaskVariables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, input: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITaskVariables_Impl::SetOutput(this, core::mem::transmute(&input)).into()
            }
        }
        unsafe extern "system" fn GetContext<Identity: ITaskVariables_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITaskVariables_Impl::GetContext(this) {
                    Ok(ok__) => {
                        pcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInput: GetInput::<Identity, OFFSET>,
            SetOutput: SetOutput::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITaskVariables as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITaskVariables {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITimeTrigger, ITimeTrigger_Vtbl, 0xb45747e0_eba7_4276_9f29_85c5bb300006);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITimeTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITimeTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl ITimeTrigger {
    pub unsafe fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RandomDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITimeTrigger_Impl: ITrigger_Impl {
    fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITimeTrigger_Vtbl {
    pub const fn new<Identity: ITimeTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_RandomDelay<Identity: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimeTrigger_Impl::get_RandomDelay(this) {
                    Ok(ok__) => {
                        prandomdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RandomDelay<Identity: ITimeTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimeTrigger_Impl::put_RandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            get_RandomDelay: get_RandomDelay::<Identity, OFFSET>,
            put_RandomDelay: put_RandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimeTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITimeTrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITrigger, ITrigger_Vtbl, 0x09941815_ea89_4b5b_89e0_2a773801fac3);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITrigger {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITrigger, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITrigger {
    pub unsafe fn get_Type(&self) -> windows_core::Result<TASK_TRIGGER_TYPE2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Id(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Id(&self, id: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Id)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id)) }
    }
    pub unsafe fn get_Repetition(&self) -> windows_core::Result<IRepetitionPattern> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Repetition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_Repetition<P0>(&self, prepeat: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRepetitionPattern>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Repetition)(windows_core::Interface::as_raw(self), prepeat.param().abi()) }
    }
    pub unsafe fn get_ExecutionTimeLimit(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExecutionTimeLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ExecutionTimeLimit(&self, timelimit: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExecutionTimeLimit)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(timelimit)) }
    }
    pub unsafe fn get_StartBoundary(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartBoundary)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_StartBoundary(&self, start: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartBoundary)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(start)) }
    }
    pub unsafe fn get_EndBoundary(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EndBoundary)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_EndBoundary(&self, end: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EndBoundary)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(end)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITrigger_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TASK_TRIGGER_TYPE2) -> windows_core::HRESULT,
    pub get_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Repetition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Repetition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ExecutionTimeLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_StartBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_StartBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_EndBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_EndBoundary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITrigger_Impl: super::IDispatch_Impl {
    fn get_Type(&self) -> windows_core::Result<TASK_TRIGGER_TYPE2>;
    fn get_Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Id(&self, id: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Repetition(&self) -> windows_core::Result<IRepetitionPattern>;
    fn put_Repetition(&self, prepeat: windows_core::Ref<IRepetitionPattern>) -> windows_core::Result<()>;
    fn get_ExecutionTimeLimit(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ExecutionTimeLimit(&self, timelimit: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_StartBoundary(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_StartBoundary(&self, start: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_EndBoundary(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_EndBoundary(&self, end: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITrigger_Vtbl {
    pub const fn new<Identity: ITrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Type<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrigger_Impl::get_Type(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Id<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrigger_Impl::get_Id(this) {
                    Ok(ok__) => {
                        pid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Id<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::put_Id(this, core::mem::transmute(&id)).into()
            }
        }
        unsafe extern "system" fn get_Repetition<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprepeat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrigger_Impl::get_Repetition(this) {
                    Ok(ok__) => {
                        pprepeat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Repetition<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prepeat: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::put_Repetition(this, core::mem::transmute_copy(&prepeat)).into()
            }
        }
        unsafe extern "system" fn get_ExecutionTimeLimit<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimelimit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrigger_Impl::get_ExecutionTimeLimit(this) {
                    Ok(ok__) => {
                        ptimelimit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExecutionTimeLimit<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timelimit: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::put_ExecutionTimeLimit(this, core::mem::transmute(&timelimit)).into()
            }
        }
        unsafe extern "system" fn get_StartBoundary<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrigger_Impl::get_StartBoundary(this) {
                    Ok(ok__) => {
                        pstart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartBoundary<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::put_StartBoundary(this, core::mem::transmute(&start)).into()
            }
        }
        unsafe extern "system" fn get_EndBoundary<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pend: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrigger_Impl::get_EndBoundary(this) {
                    Ok(ok__) => {
                        pend.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EndBoundary<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, end: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::put_EndBoundary(this, core::mem::transmute(&end)).into()
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITrigger_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        penabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: ITrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITrigger_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Type: get_Type::<Identity, OFFSET>,
            get_Id: get_Id::<Identity, OFFSET>,
            put_Id: put_Id::<Identity, OFFSET>,
            get_Repetition: get_Repetition::<Identity, OFFSET>,
            put_Repetition: put_Repetition::<Identity, OFFSET>,
            get_ExecutionTimeLimit: get_ExecutionTimeLimit::<Identity, OFFSET>,
            put_ExecutionTimeLimit: put_ExecutionTimeLimit::<Identity, OFFSET>,
            get_StartBoundary: get_StartBoundary::<Identity, OFFSET>,
            put_StartBoundary: put_StartBoundary::<Identity, OFFSET>,
            get_EndBoundary: get_EndBoundary::<Identity, OFFSET>,
            put_EndBoundary: put_EndBoundary::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITrigger {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(ITriggerCollection, ITriggerCollection_Vtbl, 0x85df5081_1b24_4f32_878a_d9d14df4cb77);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for ITriggerCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(ITriggerCollection, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl ITriggerCollection {
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<ITrigger> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Create(&self, r#type: TASK_TRIGGER_TYPE2) -> windows_core::Result<ITrigger> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Remove(&self, index: &super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index)) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct ITriggerCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, TASK_TRIGGER_TYPE2, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITriggerCollection_Impl: super::IDispatch_Impl {
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn get_Item(&self, index: i32) -> windows_core::Result<ITrigger>;
    fn get__NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Create(&self, r#type: TASK_TRIGGER_TYPE2) -> windows_core::Result<ITrigger>;
    fn Remove(&self, index: &super::VARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ITriggerCollection_Vtbl {
    pub const fn new<Identity: ITriggerCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITriggerCollection_Impl::get_Count(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITriggerCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pptrigger.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get__NewEnum<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITriggerCollection_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Create<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITriggerCollection_Impl::Create(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        pptrigger.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITriggerCollection_Impl::Remove(this, core::mem::transmute(&index)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ITriggerCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITriggerCollection_Impl::Clear(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITriggerCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITriggerCollection {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWeeklyTrigger, IWeeklyTrigger_Vtbl, 0x5038fc98_82ff_436d_8728_a512a57c9dc1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWeeklyTrigger {
    type Target = ITrigger;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWeeklyTrigger, windows_core::IUnknown, super::IDispatch, ITrigger);
#[cfg(feature = "oaidl")]
impl IWeeklyTrigger {
    pub unsafe fn get_DaysOfWeek(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DaysOfWeek)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DaysOfWeek(&self, days: i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DaysOfWeek)(windows_core::Interface::as_raw(self), days) }
    }
    pub unsafe fn get_WeeksInterval(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WeeksInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_WeeksInterval(&self, weeks: i16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WeeksInterval)(windows_core::Interface::as_raw(self), weeks) }
    }
    pub unsafe fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RandomDelay)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RandomDelay)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(randomdelay)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWeeklyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub get_DaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub put_DaysOfWeek: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub get_WeeksInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub put_WeeksInterval: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub get_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_RandomDelay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWeeklyTrigger_Impl: ITrigger_Impl {
    fn get_DaysOfWeek(&self) -> windows_core::Result<i16>;
    fn put_DaysOfWeek(&self, days: i16) -> windows_core::Result<()>;
    fn get_WeeksInterval(&self) -> windows_core::Result<i16>;
    fn put_WeeksInterval(&self, weeks: i16) -> windows_core::Result<()>;
    fn get_RandomDelay(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_RandomDelay(&self, randomdelay: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWeeklyTrigger_Vtbl {
    pub const fn new<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DaysOfWeek<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdays: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWeeklyTrigger_Impl::get_DaysOfWeek(this) {
                    Ok(ok__) => {
                        pdays.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DaysOfWeek<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, days: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeeklyTrigger_Impl::put_DaysOfWeek(this, core::mem::transmute_copy(&days)).into()
            }
        }
        unsafe extern "system" fn get_WeeksInterval<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pweeks: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWeeklyTrigger_Impl::get_WeeksInterval(this) {
                    Ok(ok__) => {
                        pweeks.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WeeksInterval<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weeks: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeeklyTrigger_Impl::put_WeeksInterval(this, core::mem::transmute_copy(&weeks)).into()
            }
        }
        unsafe extern "system" fn get_RandomDelay<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prandomdelay: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWeeklyTrigger_Impl::get_RandomDelay(this) {
                    Ok(ok__) => {
                        prandomdelay.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RandomDelay<Identity: IWeeklyTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, randomdelay: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWeeklyTrigger_Impl::put_RandomDelay(this, core::mem::transmute(&randomdelay)).into()
            }
        }
        Self {
            base__: ITrigger_Vtbl::new::<Identity, OFFSET>(),
            get_DaysOfWeek: get_DaysOfWeek::<Identity, OFFSET>,
            put_DaysOfWeek: put_DaysOfWeek::<Identity, OFFSET>,
            get_WeeksInterval: get_WeeksInterval::<Identity, OFFSET>,
            put_WeeksInterval: put_WeeksInterval::<Identity, OFFSET>,
            get_RandomDelay: get_RandomDelay::<Identity, OFFSET>,
            put_RandomDelay: put_RandomDelay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWeeklyTrigger as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<ITrigger as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWeeklyTrigger {}
pub const TASK_ACTION_COM_HANDLER: TASK_ACTION_TYPE = 5;
pub const TASK_ACTION_EXEC: TASK_ACTION_TYPE = 0;
pub const TASK_ACTION_SEND_EMAIL: TASK_ACTION_TYPE = 6;
pub const TASK_ACTION_SHOW_MESSAGE: TASK_ACTION_TYPE = 7;
pub type TASK_ACTION_TYPE = i32;
pub type TASK_COMPATIBILITY = i32;
pub const TASK_COMPATIBILITY_AT: TASK_COMPATIBILITY = 0;
pub const TASK_COMPATIBILITY_V1: TASK_COMPATIBILITY = 1;
pub const TASK_COMPATIBILITY_V2: TASK_COMPATIBILITY = 2;
pub const TASK_COMPATIBILITY_V2_1: TASK_COMPATIBILITY = 3;
pub const TASK_COMPATIBILITY_V2_2: TASK_COMPATIBILITY = 4;
pub const TASK_COMPATIBILITY_V2_3: TASK_COMPATIBILITY = 5;
pub const TASK_COMPATIBILITY_V2_4: TASK_COMPATIBILITY = 6;
pub const TASK_CONSOLE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 1;
pub const TASK_CONSOLE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 2;
pub const TASK_CREATE: TASK_CREATION = 2;
pub const TASK_CREATE_OR_UPDATE: TASK_CREATION = 6;
pub type TASK_CREATION = i32;
pub const TASK_DISABLE: TASK_CREATION = 8;
pub const TASK_DONT_ADD_PRINCIPAL_ACE: TASK_CREATION = 16;
pub type TASK_ENUM_FLAGS = i32;
pub const TASK_ENUM_HIDDEN: TASK_ENUM_FLAGS = 1;
pub const TASK_IGNORE_REGISTRATION_TRIGGERS: TASK_CREATION = 32;
pub const TASK_INSTANCES_IGNORE_NEW: TASK_INSTANCES_POLICY = 2;
pub const TASK_INSTANCES_PARALLEL: TASK_INSTANCES_POLICY = 0;
pub type TASK_INSTANCES_POLICY = i32;
pub const TASK_INSTANCES_QUEUE: TASK_INSTANCES_POLICY = 1;
pub const TASK_INSTANCES_STOP_EXISTING: TASK_INSTANCES_POLICY = 3;
pub const TASK_LOGON_GROUP: TASK_LOGON_TYPE = 4;
pub const TASK_LOGON_INTERACTIVE_TOKEN: TASK_LOGON_TYPE = 3;
pub const TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD: TASK_LOGON_TYPE = 6;
pub const TASK_LOGON_NONE: TASK_LOGON_TYPE = 0;
pub const TASK_LOGON_PASSWORD: TASK_LOGON_TYPE = 1;
pub const TASK_LOGON_S4U: TASK_LOGON_TYPE = 2;
pub const TASK_LOGON_SERVICE_ACCOUNT: TASK_LOGON_TYPE = 5;
pub type TASK_LOGON_TYPE = i32;
pub const TASK_PROCESSTOKENSID_DEFAULT: TASK_PROCESSTOKENSID_TYPE = 2;
pub const TASK_PROCESSTOKENSID_NONE: TASK_PROCESSTOKENSID_TYPE = 0;
pub type TASK_PROCESSTOKENSID_TYPE = i32;
pub const TASK_PROCESSTOKENSID_UNRESTRICTED: TASK_PROCESSTOKENSID_TYPE = 1;
pub const TASK_REMOTE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 3;
pub const TASK_REMOTE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = 4;
pub const TASK_RUNLEVEL_HIGHEST: TASK_RUNLEVEL_TYPE = 1;
pub const TASK_RUNLEVEL_LUA: TASK_RUNLEVEL_TYPE = 0;
pub type TASK_RUNLEVEL_TYPE = i32;
pub const TASK_RUN_AS_SELF: TASK_RUN_FLAGS = 1;
pub type TASK_RUN_FLAGS = i32;
pub const TASK_RUN_IGNORE_CONSTRAINTS: TASK_RUN_FLAGS = 2;
pub const TASK_RUN_NO_FLAGS: TASK_RUN_FLAGS = 0;
pub const TASK_RUN_USER_SID: TASK_RUN_FLAGS = 8;
pub const TASK_RUN_USE_SESSION_ID: TASK_RUN_FLAGS = 4;
pub const TASK_SESSION_LOCK: TASK_SESSION_STATE_CHANGE_TYPE = 7;
pub type TASK_SESSION_STATE_CHANGE_TYPE = i32;
pub const TASK_SESSION_UNLOCK: TASK_SESSION_STATE_CHANGE_TYPE = 8;
pub type TASK_STATE = i32;
pub const TASK_STATE_DISABLED: TASK_STATE = 1;
pub const TASK_STATE_QUEUED: TASK_STATE = 2;
pub const TASK_STATE_READY: TASK_STATE = 3;
pub const TASK_STATE_RUNNING: TASK_STATE = 4;
pub const TASK_STATE_UNKNOWN: TASK_STATE = 0;
pub const TASK_TRIGGER_BOOT: TASK_TRIGGER_TYPE2 = 8;
pub const TASK_TRIGGER_CUSTOM_TRIGGER_01: TASK_TRIGGER_TYPE2 = 12;
pub const TASK_TRIGGER_DAILY: TASK_TRIGGER_TYPE2 = 2;
pub const TASK_TRIGGER_EVENT: TASK_TRIGGER_TYPE2 = 0;
pub const TASK_TRIGGER_IDLE: TASK_TRIGGER_TYPE2 = 6;
pub const TASK_TRIGGER_LOGON: TASK_TRIGGER_TYPE2 = 9;
pub const TASK_TRIGGER_MONTHLY: TASK_TRIGGER_TYPE2 = 4;
pub const TASK_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE2 = 5;
pub const TASK_TRIGGER_REGISTRATION: TASK_TRIGGER_TYPE2 = 7;
pub const TASK_TRIGGER_SESSION_STATE_CHANGE: TASK_TRIGGER_TYPE2 = 11;
pub const TASK_TRIGGER_TIME: TASK_TRIGGER_TYPE2 = 1;
pub type TASK_TRIGGER_TYPE2 = i32;
pub const TASK_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE2 = 3;
pub const TASK_UPDATE: TASK_CREATION = 4;
pub const TASK_VALIDATE_ONLY: TASK_CREATION = 1;
pub const TaskHandlerPS: windows_core::GUID = windows_core::GUID::from_u128(0xf2a69db7_da2c_4352_9066_86fee6dacac9);
pub const TaskHandlerStatusPS: windows_core::GUID = windows_core::GUID::from_u128(0x9f15266d_d7ba_48f0_93c1_e6895f6fe5ac);
pub const TaskScheduler: windows_core::GUID = windows_core::GUID::from_u128(0x0f87369f_a4e5_4cfc_bd3e_73e6154572dd);
