#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IIterable<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IIterable<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IIterable<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IIterable<T>
{
}
impl<T: windows_core::RuntimeType + 'static> IIterable<T> {
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = {
        windows_core::imp::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"{faa585ea-6214-4217-afda-7f46de5869b3}")
            .push_slice(b";")
            .push_other(T::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterable<T> {
    type Vtable = IIterable_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IIterable_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IIterator<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IIterator<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IIterator<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IIterator<T>
{
}
impl<T: windows_core::RuntimeType + 'static> IIterator<T> {
    pub fn Current(&self) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetMany(
        &self,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(
                windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                core::mem::transmute_copy(&items),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl<T: windows_core::RuntimeType> Iterator for IIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.Current().ok();
        if result.is_some() {
            self.MoveNext().ok()?;
        }
        result
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterator<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = {
        windows_core::imp::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"{6a79e863-4300-459a-9966-cbb660963ee1}")
            .push_slice(b";")
            .push_other(T::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterator<T> {
    type Vtable = IIterator_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IIterator_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub HasCurrent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoveNext:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::AbiType<T>,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(
    IStringable,
    IStringable_Vtbl,
    0x96369f54_8eb6_48f0_abce_c1b211e627c3
);
impl core::ops::Deref for IStringable {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IStringable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IStringable {
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IStringable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IStringable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUriEscapeStatics,
    IUriEscapeStatics_Vtbl,
    0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1
);
impl windows_core::RuntimeType for IUriEscapeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriEscapeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnescapeComponent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub EscapeComponent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUriRuntimeClass,
    IUriRuntimeClass_Vtbl,
    0x9e365e57_48b2_4160_956f_c7385120bbfc
);
impl windows_core::RuntimeType for IUriRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub DisplayUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Extension: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Fragment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Host: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub QueryParsed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RawUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub SchemeName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Suspicious:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub CombineUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUriRuntimeClassFactory,
    IUriRuntimeClassFactory_Vtbl,
    0x44a9796f_723e_4fdf_a218_033e75b0c084
);
impl windows_core::RuntimeType for IUriRuntimeClassFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateWithRelativeUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IUriRuntimeClassWithAbsoluteCanonicalUri,
    IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl,
    0x758d9661_221c_480f_a339_50656673f46f
);
impl windows_core::RuntimeType for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteCanonicalUri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub DisplayIri: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IVectorView<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IVectorView<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IVectorView<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IVectorView<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>>
    for IVectorView<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVectorView<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = self;
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
        let this = self;
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
        P0: windows_core::Param<T>,
    {
        let this = self;
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
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        let this = self;
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
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
pub struct VectorViewIterator<T: windows_core::RuntimeType + 'static> {
    vector: Option<IVectorView<T>>,
    current: u32,
}
impl<T: windows_core::RuntimeType> VectorViewIterator<T> {
    pub fn new(vector: Option<IVectorView<T>>) -> Self {
        Self { vector, current: 0 }
    }
}
impl<T: windows_core::RuntimeType> Iterator for VectorViewIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.vector
            .as_ref()
            .and_then(|vector| vector.GetAt(self.current).ok())
            .and_then(|result| {
                self.current += 1;
                Some(result)
            })
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for IVectorView<T> {
    type Item = T;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for &IVectorView<T> {
    type Item = T;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorViewIterator::new(Some(Clone::clone(self)))
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVectorView<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = {
        windows_core::imp::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"{bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}")
            .push_slice(b";")
            .push_other(T::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVectorView<T> {
    type Vtable = IVectorView_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IVectorView_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<T>,
        *mut u32,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut windows_core::AbiType<T>,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(
    IWwwFormUrlDecoderEntry,
    IWwwFormUrlDecoderEntry_Vtbl,
    0x125e7431_f678_4e8e_b670_20a9b06c512d
);
impl core::ops::Deref for IWwwFormUrlDecoderEntry {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IWwwFormUrlDecoderEntry,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWwwFormUrlDecoderEntry {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IWwwFormUrlDecoderEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWwwFormUrlDecoderEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
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
}
#[repr(C)]
pub struct IWwwFormUrlDecoderRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFirstValueByName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
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
}
#[repr(C)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWwwFormUrlDecoder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Uri(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Uri, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Uri, IStringable);
impl Uri {
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UnescapeComponent(
        tounescape: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnescapeComponent)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(tounescape),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn EscapeComponent(
        toescape: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EscapeComponent)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(toescape),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AbsoluteUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Domain(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Domain)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Extension(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Extension)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Fragment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Fragment)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Host(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Host)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Password(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Password)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Query(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Query)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn QueryParsed(&self) -> windows_core::Result<WwwFormUrlDecoder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryParsed)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RawUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SchemeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SchemeName)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn UserName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserName)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Port(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Port)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Suspicious(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Suspicious)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Equals<P0>(&self, puri: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Equals)(
                windows_core::Interface::as_raw(this),
                puri.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn CombineUri(&self, relativeuri: &windows_core::HSTRING) -> windows_core::Result<Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CombineUri)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(relativeuri),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateUri(uri: &windows_core::HSTRING) -> windows_core::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUri)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(uri),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithRelativeUri(
        baseuri: &windows_core::HSTRING,
        relativeuri: &windows_core::HSTRING,
    ) -> windows_core::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithRelativeUri)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(baseuri),
                core::mem::transmute_copy(relativeuri),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AbsoluteCanonicalUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this =
            &windows_core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteCanonicalUri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayIri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this =
            &windows_core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayIri)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Uri, IUriEscapeStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUriRuntimeClassFactory<
        R,
        F: FnOnce(&IUriRuntimeClassFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Uri, IUriRuntimeClassFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Uri {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
}
unsafe impl windows_core::Interface for Uri {
    type Vtable = IUriRuntimeClass_Vtbl;
    const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Uri {
    const NAME: &'static str = "Windows.Foundation.Uri";
}
unsafe impl Send for Uri {}
unsafe impl Sync for Uri {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WwwFormUrlDecoder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WwwFormUrlDecoder,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    WwwFormUrlDecoder,
    IIterable<IWwwFormUrlDecoderEntry>,
    IVectorView<IWwwFormUrlDecoderEntry>
);
impl WwwFormUrlDecoder {
    pub fn First(&self) -> windows_core::Result<IIterator<IWwwFormUrlDecoderEntry>> {
        let this = &windows_core::Interface::cast::<IIterable<IWwwFormUrlDecoderEntry>>(self)?;
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
        let this = &windows_core::Interface::cast::<IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
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
        let this = &windows_core::Interface::cast::<IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
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
        let this = &windows_core::Interface::cast::<IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
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
        let this = &windows_core::Interface::cast::<IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
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
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFirstValueByName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
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
    #[doc(hidden)]
    pub fn IWwwFormUrlDecoderRuntimeClassFactory<
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
    type Vtable = IWwwFormUrlDecoderRuntimeClass_Vtbl;
    const IID: windows_core::GUID =
        <IWwwFormUrlDecoderRuntimeClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WwwFormUrlDecoder {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoder";
}
impl IntoIterator for WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
unsafe impl Send for WwwFormUrlDecoder {}
unsafe impl Sync for WwwFormUrlDecoder {}