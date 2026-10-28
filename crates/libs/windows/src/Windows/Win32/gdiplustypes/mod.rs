pub const Aborted: Status = 9;
pub const AccessDenied: Status = 12;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CharacterRange {
    pub First: super::INT,
    pub Length: super::INT,
}
pub type DrawImageAbort = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[cfg(all(feature = "gdiplusenums", feature = "shared"))]
pub type EnumerateMetafileProc = Option<unsafe extern "system" fn(param0: super::EmfPlusRecordType, param1: super::UINT, param2: super::UINT, param3: *const super::BYTE, param4: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub const FileNotFound: Status = 10;
pub const FontFamilyNotFound: Status = 14;
pub const FontStyleNotFound: Status = 15;
windows_core::imp::define_interface!(GdiplusAbort, GdiplusAbort_Vtbl);
impl GdiplusAbort {
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct GdiplusAbort_Vtbl {
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait GdiplusAbort_Impl {
    fn Abort(&self) -> windows_core::Result<()>;
}
impl GdiplusAbort_Vtbl {
    pub const fn new<Identity: GdiplusAbort_Impl>() -> Self {
        unsafe extern "system" fn Abort<Identity: GdiplusAbort_Impl>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                GdiplusAbort_Impl::Abort(this).into()
            }
        }
        Self { Abort: Abort::<Identity> }
    }
}
struct GdiplusAbort_ImplVtbl<T: GdiplusAbort_Impl>(core::marker::PhantomData<T>);
impl<T: GdiplusAbort_Impl> GdiplusAbort_ImplVtbl<T> {
    const VTABLE: GdiplusAbort_Vtbl = GdiplusAbort_Vtbl::new::<T>();
}
impl GdiplusAbort {
    pub fn new<'a, T: GdiplusAbort_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &GdiplusAbort_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
pub const GdiplusNotInitialized: Status = 18;
pub const GenericError: Status = 1;
pub type GetThumbnailImageAbort = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type ImageAbort = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub const InsufficientBuffer: Status = 5;
pub const InvalidParameter: Status = 2;
pub const NotImplemented: Status = 6;
pub const NotTrueTypeFont: Status = 16;
pub const ObjectBusy: Status = 4;
pub const Ok: Status = 0;
pub const OutOfMemory: Status = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PathData(pub u8);
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct Point {
    pub X: super::INT,
    pub Y: super::INT,
}
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct PointF {
    pub X: REAL,
    pub Y: REAL,
}
pub const ProfileNotFound: Status = 21;
pub const PropertyNotFound: Status = 19;
pub const PropertyNotSupported: Status = 20;
pub type REAL = f32;
pub const REAL_EPSILON: f32 = 0.00000011920929;
pub const REAL_MAX: f32 = 340282350000000000000000000000000000000.0;
pub const REAL_MIN: f32 = 0.000000000000000000000000000000000000011754944;
pub const REAL_TOLERANCE: f32 = 0.00000000000000000000000000000000000117549435;
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub X: super::INT,
    pub Y: super::INT,
    pub Width: super::INT,
    pub Height: super::INT,
}
#[repr(C, packed(8))]
#[derive(Clone, Copy, Default)]
pub struct RectF {
    pub X: REAL,
    pub Y: REAL,
    pub Width: REAL,
    pub Height: REAL,
}
pub type Status = i32;
pub const UnknownImageFormat: Status = 13;
pub const UnsupportedGdiplusVersion: Status = 17;
pub const ValueOverflow: Status = 11;
pub const Win32Error: Status = 7;
pub const WrongState: Status = 8;
