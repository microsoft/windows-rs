#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    DependencyObject,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeType for DependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDependencyObject>();
}
unsafe impl windows_core::Interface for DependencyObject {
    type Vtable = <IDependencyObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDependencyObject as windows_core::Interface>::IID;
}
impl core::ops::Deref for DependencyObject {
    type Target = IDependencyObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for DependencyObject {
    const NAME: &'static str = "Microsoft.UI.Xaml.DependencyObject";
}
unsafe impl Send for DependencyObject {}
unsafe impl Sync for DependencyObject {}
windows_core::imp::define_interface!(
    IDependencyObject,
    IDependencyObject_Vtbl,
    0xe7beaee7_160e_50f7_8789_d63463f979fa
);
impl windows_core::RuntimeType for IDependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDependencyObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IImageSource,
    IImageSource_Vtbl,
    0x6c2038f6_d6d5_55e9_9b9e_082f12dbff60
);
impl windows_core::RuntimeType for IImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISurfaceImageSource,
    ISurfaceImageSource_Vtbl,
    0xac078d9c_d0e0_5ff9_b73e_98e82e4c8d36
);
impl windows_core::RuntimeType for ISurfaceImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISurfaceImageSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    ISurfaceImageSourceFactory,
    ISurfaceImageSourceFactory_Vtbl,
    0x09a26ed2_11b3_5ef1_ac56_20d064ccca34
);
impl windows_core::RuntimeType for ISurfaceImageSourceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISurfaceImageSourceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        i32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    ISurfaceImageSourceNativeWithD2D,
    ISurfaceImageSourceNativeWithD2D_Vtbl,
    0xcb833102_d5d1_448b_a31a_52a9509f24e6
);
windows_core::imp::interface_hierarchy!(ISurfaceImageSourceNativeWithD2D, windows_core::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {
    pub(crate) unsafe fn SetDevice(&self, device: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetDevice)(
                windows_core::Interface::as_raw(self),
                device as _,
            )
        }
    }
    pub(crate) unsafe fn BeginDraw(
        &self,
        updaterect: *const RECT,
        iid: *const windows_core::GUID,
        updateobject: *mut *mut core::ffi::c_void,
        offset: *mut POINT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).BeginDraw)(
                windows_core::Interface::as_raw(self),
                updaterect,
                iid,
                updateobject as _,
                offset as _,
            )
        }
    }
    pub(crate) unsafe fn EndDraw(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BeginDraw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const RECT,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
        *mut POINT,
    ) -> windows_core::HRESULT,
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    SuspendDraw: usize,
    ResumeDraw: usize,
}
impl windows_core::RuntimeName for ISurfaceImageSourceNativeWithD2D {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    ImageSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(ImageSource, DependencyObject);
impl windows_core::RuntimeType for ImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IImageSource>();
}
unsafe impl windows_core::Interface for ImageSource {
    type Vtable = <IImageSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImageSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for ImageSource {
    type Target = IImageSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for ImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ImageSource";
}
unsafe impl Send for ImageSource {}
unsafe impl Sync for ImageSource {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceImageSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    SurfaceImageSource,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(SurfaceImageSource, ImageSource, DependencyObject);
impl SurfaceImageSource {
    pub(crate) fn CreateInstanceWithDimensions(
        pixelwidth: i32,
        pixelheight: i32,
    ) -> windows_core::Result<Self> {
        Self::ISurfaceImageSourceFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(
                windows_core::Interface::as_raw(this),
                pixelwidth,
                pixelheight,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn ISurfaceImageSourceFactory<
        R,
        F: FnOnce(&ISurfaceImageSourceFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            SurfaceImageSource,
            ISurfaceImageSourceFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SurfaceImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, ISurfaceImageSource>();
}
unsafe impl windows_core::Interface for SurfaceImageSource {
    type Vtable = <ISurfaceImageSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISurfaceImageSource as windows_core::Interface>::IID;
}
impl core::ops::Deref for SurfaceImageSource {
    type Target = ISurfaceImageSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for SurfaceImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.SurfaceImageSource";
}
unsafe impl Send for SurfaceImageSource {}
unsafe impl Sync for SurfaceImageSource {}
