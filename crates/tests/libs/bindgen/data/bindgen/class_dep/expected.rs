windows_core::imp::define_interface!(
    IWwwFormUrlDecoderEntry,
    IWwwFormUrlDecoderEntry_Vtbl,
    0x125e7431_f678_4e8e_b670_20a9b06c512d
);
impl windows_core::RuntimeType for IWwwFormUrlDecoderEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IWwwFormUrlDecoderEntry");
}
windows_core::imp::interface_hierarchy!(
    IWwwFormUrlDecoderEntry,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWwwFormUrlDecoderEntry {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IWwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderEntry";
}
pub trait IWwwFormUrlDecoderEntry_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Value(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IWwwFormUrlDecoderEntry_Vtbl {
    pub const fn new<Identity: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<
            Identity: IWwwFormUrlDecoderEntry_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWwwFormUrlDecoderEntry_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<
            Identity: IWwwFormUrlDecoderEntry_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWwwFormUrlDecoderEntry_Impl::Value(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWwwFormUrlDecoderEntry, OFFSET>(
            ),
            Name: Name::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWwwFormUrlDecoderEntry as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IWwwFormUrlDecoderEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWwwFormUrlDecoderRuntimeClass,
    IWwwFormUrlDecoderRuntimeClass_Vtbl,
    0xd45a0451_f225_4542_9296_0e1df5d254df
);
impl windows_core::RuntimeType for IWwwFormUrlDecoderRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Foundation.IWwwFormUrlDecoderRuntimeClass",
    );
}
#[repr(C)]
pub struct IWwwFormUrlDecoderRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFirstValueByName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWwwFormUrlDecoderRuntimeClassFactory,
    IWwwFormUrlDecoderRuntimeClassFactory_Vtbl,
    0x5b8c6b3d_24ae_41b5_a1bf_f0c3d544845b
);
impl windows_core::RuntimeType for IWwwFormUrlDecoderRuntimeClassFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory",
    );
}
#[repr(C)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWwwFormUrlDecoder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WwwFormUrlDecoder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WwwFormUrlDecoder,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    WwwFormUrlDecoder,
    windows_collections::IIterable<IWwwFormUrlDecoderEntry>,
    windows_collections::IVectorView<IWwwFormUrlDecoderEntry>
);
impl WwwFormUrlDecoder {
    pub fn First(
        &self,
    ) -> windows_core::Result<windows_collections::IIterator<IWwwFormUrlDecoderEntry>> {
        let this = &windows_core::Interface::cast::<
            windows_collections::IIterable<IWwwFormUrlDecoderEntry>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<IWwwFormUrlDecoderEntry> {
        let this = &windows_core::Interface::cast::<
            windows_collections::IVectorView<IWwwFormUrlDecoderEntry>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(
                windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<
            windows_collections::IVectorView<IWwwFormUrlDecoderEntry>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<IWwwFormUrlDecoderEntry>,
    {
        let this = &windows_core::Interface::cast::<
            windows_collections::IVectorView<IWwwFormUrlDecoderEntry>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [Option<IWwwFormUrlDecoderEntry>],
    ) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<
            windows_collections::IVectorView<IWwwFormUrlDecoderEntry>,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(
                windows_core::Interface::as_raw(this),
                startindex,
                items.len().try_into().unwrap(),
                core::mem::transmute_copy(&items),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetFirstValueByName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstValueByName)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateWwwFormUrlDecoder(
        query: &windows_core::HSTRING,
    ) -> windows_core::Result<WwwFormUrlDecoder> {
        Self::IWwwFormUrlDecoderRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWwwFormUrlDecoder)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(query),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWwwFormUrlDecoderRuntimeClassFactory<
        R,
        F: FnOnce(&IWwwFormUrlDecoderRuntimeClassFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            WwwFormUrlDecoder,
            IWwwFormUrlDecoderRuntimeClassFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WwwFormUrlDecoder {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWwwFormUrlDecoderRuntimeClass>();
}
unsafe impl windows_core::Interface for WwwFormUrlDecoder {
    type Vtable = <IWwwFormUrlDecoderRuntimeClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IWwwFormUrlDecoderRuntimeClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WwwFormUrlDecoder {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoder";
}
unsafe impl Send for WwwFormUrlDecoder {}
unsafe impl Sync for WwwFormUrlDecoder {}
impl IntoIterator for WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
