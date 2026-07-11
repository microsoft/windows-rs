#[inline]
pub unsafe fn ImageList_CoCreateInstance<P1, T>(rclsid: *const windows_core::GUID, punkouter: P1) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("comctl32.dll" "system" fn ImageList_CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { ImageList_CoCreateInstance(rclsid, punkouter.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
windows_core::imp::define_interface!(IImageList, IImageList_Vtbl, 0x46eb5926_582e_4017_9fdf_e8998daa0950);
windows_core::imp::interface_hierarchy!(IImageList, windows_core::IUnknown);
impl IImageList {
    #[cfg(feature = "windef")]
    pub unsafe fn Add(&self, hbmimage: super::windef::HBITMAP, hbmmask: Option<super::windef::HBITMAP>) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ReplaceIcon(&self, i: i32, hicon: super::windef::HICON) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReplaceIcon)(windows_core::Interface::as_raw(self), i, hicon, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOverlayImage)(windows_core::Interface::as_raw(self), iimage, ioverlay) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Replace(&self, i: i32, hbmimage: super::windef::HBITMAP, hbmmask: Option<super::windef::HBITMAP>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Replace)(windows_core::Interface::as_raw(self), i, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddMasked(&self, hbmimage: super::windef::HBITMAP, crmask: super::windef::COLORREF) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddMasked)(windows_core::Interface::as_raw(self), hbmimage, crmask, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "commctrl", feature = "windef"))]
    pub unsafe fn Draw(&self, pimldp: *const super::commctrl::IMAGELISTDRAWPARAMS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), pimldp) }
    }
    pub unsafe fn Remove(&self, i: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), i) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetIcon(&self, i: i32, flags: u32) -> windows_core::Result<super::windef::HICON> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIcon)(windows_core::Interface::as_raw(self), i, flags, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "commctrl", feature = "windef"))]
    pub unsafe fn GetImageInfo(&self, i: i32, pimageinfo: *mut super::commctrl::IMAGEINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetImageInfo)(windows_core::Interface::as_raw(self), i, pimageinfo as _) }
    }
    pub unsafe fn Copy<P1>(&self, idst: i32, punksrc: P1, isrc: i32, uflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), idst, punksrc.param().abi(), isrc, uflags) }
    }
    pub unsafe fn Merge<P1, T>(&self, i1: i32, punk2: P1, i2: i32, dx: i32, dy: i32) -> windows_core::Result<T>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Merge)(windows_core::Interface::as_raw(self), i1, punk2.param().abi(), i2, dx, dy, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Clone<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetImageRect(&self, i: i32) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageRect)(windows_core::Interface::as_raw(self), i, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIconSize)(windows_core::Interface::as_raw(self), cx as _, cy as _) }
    }
    pub unsafe fn SetIconSize(&self, cx: i32, cy: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIconSize)(windows_core::Interface::as_raw(self), cx, cy) }
    }
    pub unsafe fn GetImageCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetImageCount(&self, unewcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetImageCount)(windows_core::Interface::as_raw(self), unewcount) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetBkColor(&self, clrbk: super::windef::COLORREF) -> windows_core::Result<super::windef::COLORREF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetBkColor)(windows_core::Interface::as_raw(self), clrbk, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetBkColor(&self) -> windows_core::Result<super::windef::COLORREF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBkColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginDrag)(windows_core::Interface::as_raw(self), itrack, dxhotspot, dyhotspot) }
    }
    pub unsafe fn EndDrag(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndDrag)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn DragEnter(&self, hwndlock: Option<super::windef::HWND>, x: i32, y: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DragEnter)(windows_core::Interface::as_raw(self), hwndlock.unwrap_or(core::mem::zeroed()) as _, x, y) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn DragLeave(&self, hwndlock: Option<super::windef::HWND>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DragLeave)(windows_core::Interface::as_raw(self), hwndlock.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn DragMove(&self, x: i32, y: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DragMove)(windows_core::Interface::as_raw(self), x, y) }
    }
    pub unsafe fn SetDragCursorImage<P0>(&self, punk: P0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDragCursorImage)(windows_core::Interface::as_raw(self), punk.param().abi(), idrag, dxhotspot, dyhotspot) }
    }
    pub unsafe fn DragShowNolock(&self, fshow: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DragShowNolock)(windows_core::Interface::as_raw(self), fshow.into()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetDragImage<T>(&self, ppt: Option<*mut super::windef::POINT>, ppthotspot: Option<*mut super::windef::POINT>) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDragImage)(windows_core::Interface::as_raw(self), ppt.unwrap_or(core::mem::zeroed()) as _, ppthotspot.unwrap_or(core::mem::zeroed()) as _, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetItemFlags(&self, i: i32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemFlags)(windows_core::Interface::as_raw(self), i, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOverlayImage(&self, ioverlay: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverlayImage)(windows_core::Interface::as_raw(self), ioverlay, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HBITMAP, super::windef::HBITMAP, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Add: usize,
    #[cfg(feature = "windef")]
    pub ReplaceIcon: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::windef::HICON, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ReplaceIcon: usize,
    pub SetOverlayImage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Replace: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::windef::HBITMAP, super::windef::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Replace: usize,
    #[cfg(feature = "windef")]
    pub AddMasked: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HBITMAP, super::windef::COLORREF, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddMasked: usize,
    #[cfg(all(feature = "commctrl", feature = "windef"))]
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::commctrl::IMAGELISTDRAWPARAMS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "commctrl", feature = "windef")))]
    Draw: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut super::windef::HICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetIcon: usize,
    #[cfg(all(feature = "commctrl", feature = "windef"))]
    pub GetImageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::commctrl::IMAGEINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "commctrl", feature = "windef")))]
    GetImageInfo: usize,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub Merge: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, i32, i32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetImageRect: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetImageRect: usize,
    pub GetIconSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetIconSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetImageCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetImageCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetBkColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::COLORREF, *mut super::windef::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetBkColor: usize,
    #[cfg(feature = "windef")]
    pub GetBkColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetBkColor: usize,
    pub BeginDrag: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub EndDrag: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub DragEnter: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    DragEnter: usize,
    #[cfg(feature = "windef")]
    pub DragLeave: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    DragLeave: usize,
    pub DragMove: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub SetDragCursorImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub DragShowNolock: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetDragImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::POINT, *mut super::windef::POINT, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetDragImage: usize,
    pub GetItemFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32) -> windows_core::HRESULT,
    pub GetOverlayImage: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "commctrl", feature = "windef"))]
pub trait IImageList_Impl: windows_core::IUnknownImpl {
    fn Add(&self, hbmimage: super::windef::HBITMAP, hbmmask: super::windef::HBITMAP) -> windows_core::Result<i32>;
    fn ReplaceIcon(&self, i: i32, hicon: super::windef::HICON) -> windows_core::Result<i32>;
    fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> windows_core::Result<()>;
    fn Replace(&self, i: i32, hbmimage: super::windef::HBITMAP, hbmmask: super::windef::HBITMAP) -> windows_core::Result<()>;
    fn AddMasked(&self, hbmimage: super::windef::HBITMAP, crmask: super::windef::COLORREF) -> windows_core::Result<i32>;
    fn Draw(&self, pimldp: *const super::commctrl::IMAGELISTDRAWPARAMS) -> windows_core::Result<()>;
    fn Remove(&self, i: i32) -> windows_core::Result<()>;
    fn GetIcon(&self, i: i32, flags: u32) -> windows_core::Result<super::windef::HICON>;
    fn GetImageInfo(&self, i: i32, pimageinfo: *mut super::commctrl::IMAGEINFO) -> windows_core::Result<()>;
    fn Copy(&self, idst: i32, punksrc: windows_core::Ref<windows_core::IUnknown>, isrc: i32, uflags: u32) -> windows_core::Result<()>;
    fn Merge(&self, i1: i32, punk2: windows_core::Ref<windows_core::IUnknown>, i2: i32, dx: i32, dy: i32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Clone(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetImageRect(&self, i: i32) -> windows_core::Result<super::windef::RECT>;
    fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> windows_core::Result<()>;
    fn SetIconSize(&self, cx: i32, cy: i32) -> windows_core::Result<()>;
    fn GetImageCount(&self) -> windows_core::Result<i32>;
    fn SetImageCount(&self, unewcount: u32) -> windows_core::Result<()>;
    fn SetBkColor(&self, clrbk: super::windef::COLORREF) -> windows_core::Result<super::windef::COLORREF>;
    fn GetBkColor(&self) -> windows_core::Result<super::windef::COLORREF>;
    fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::Result<()>;
    fn EndDrag(&self) -> windows_core::Result<()>;
    fn DragEnter(&self, hwndlock: super::windef::HWND, x: i32, y: i32) -> windows_core::Result<()>;
    fn DragLeave(&self, hwndlock: super::windef::HWND) -> windows_core::Result<()>;
    fn DragMove(&self, x: i32, y: i32) -> windows_core::Result<()>;
    fn SetDragCursorImage(&self, punk: windows_core::Ref<windows_core::IUnknown>, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::Result<()>;
    fn DragShowNolock(&self, fshow: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetDragImage(&self, ppt: *mut super::windef::POINT, ppthotspot: *mut super::windef::POINT, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetItemFlags(&self, i: i32) -> windows_core::Result<u32>;
    fn GetOverlayImage(&self, ioverlay: i32) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "commctrl", feature = "windef"))]
impl IImageList_Vtbl {
    pub const fn new<Identity: IImageList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmimage: super::windef::HBITMAP, hbmmask: super::windef::HBITMAP, pi: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::Add(this, core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&hbmmask)) {
                    Ok(ok__) => {
                        pi.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReplaceIcon<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, hicon: super::windef::HICON, pi: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::ReplaceIcon(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&hicon)) {
                    Ok(ok__) => {
                        pi.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOverlayImage<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, ioverlay: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::SetOverlayImage(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&ioverlay)).into()
            }
        }
        unsafe extern "system" fn Replace<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, hbmimage: super::windef::HBITMAP, hbmmask: super::windef::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Replace(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&hbmmask)).into()
            }
        }
        unsafe extern "system" fn AddMasked<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmimage: super::windef::HBITMAP, crmask: super::windef::COLORREF, pi: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::AddMasked(this, core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&crmask)) {
                    Ok(ok__) => {
                        pi.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Draw<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimldp: *const super::commctrl::IMAGELISTDRAWPARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Draw(this, core::mem::transmute_copy(&pimldp)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Remove(this, core::mem::transmute_copy(&i)).into()
            }
        }
        unsafe extern "system" fn GetIcon<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, flags: u32, picon: *mut super::windef::HICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetIcon(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        picon.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetImageInfo<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, pimageinfo: *mut super::commctrl::IMAGEINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::GetImageInfo(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&pimageinfo)).into()
            }
        }
        unsafe extern "system" fn Copy<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idst: i32, punksrc: *mut core::ffi::c_void, isrc: i32, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Copy(this, core::mem::transmute_copy(&idst), core::mem::transmute_copy(&punksrc), core::mem::transmute_copy(&isrc), core::mem::transmute_copy(&uflags)).into()
            }
        }
        unsafe extern "system" fn Merge<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i1: i32, punk2: *mut core::ffi::c_void, i2: i32, dx: i32, dy: i32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Merge(this, core::mem::transmute_copy(&i1), core::mem::transmute_copy(&punk2), core::mem::transmute_copy(&i2), core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::Clone(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetImageRect<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, prc: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetImageRect(this, core::mem::transmute_copy(&i)) {
                    Ok(ok__) => {
                        prc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIconSize<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: *mut i32, cy: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::GetIconSize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
            }
        }
        unsafe extern "system" fn SetIconSize<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: i32, cy: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::SetIconSize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
            }
        }
        unsafe extern "system" fn GetImageCount<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pi: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetImageCount(this) {
                    Ok(ok__) => {
                        pi.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetImageCount<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unewcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::SetImageCount(this, core::mem::transmute_copy(&unewcount)).into()
            }
        }
        unsafe extern "system" fn SetBkColor<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clrbk: super::windef::COLORREF, pclr: *mut super::windef::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::SetBkColor(this, core::mem::transmute_copy(&clrbk)) {
                    Ok(ok__) => {
                        pclr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBkColor<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclr: *mut super::windef::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetBkColor(this) {
                    Ok(ok__) => {
                        pclr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginDrag<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::BeginDrag(this, core::mem::transmute_copy(&itrack), core::mem::transmute_copy(&dxhotspot), core::mem::transmute_copy(&dyhotspot)).into()
            }
        }
        unsafe extern "system" fn EndDrag<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::EndDrag(this).into()
            }
        }
        unsafe extern "system" fn DragEnter<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndlock: super::windef::HWND, x: i32, y: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::DragEnter(this, core::mem::transmute_copy(&hwndlock), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn DragLeave<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndlock: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::DragLeave(this, core::mem::transmute_copy(&hwndlock)).into()
            }
        }
        unsafe extern "system" fn DragMove<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::DragMove(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn SetDragCursorImage<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::SetDragCursorImage(this, core::mem::transmute_copy(&punk), core::mem::transmute_copy(&idrag), core::mem::transmute_copy(&dxhotspot), core::mem::transmute_copy(&dyhotspot)).into()
            }
        }
        unsafe extern "system" fn DragShowNolock<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::DragShowNolock(this, core::mem::transmute_copy(&fshow)).into()
            }
        }
        unsafe extern "system" fn GetDragImage<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppt: *mut super::windef::POINT, ppthotspot: *mut super::windef::POINT, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList_Impl::GetDragImage(this, core::mem::transmute_copy(&ppt), core::mem::transmute_copy(&ppthotspot), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetItemFlags<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, dwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetItemFlags(this, core::mem::transmute_copy(&i)) {
                    Ok(ok__) => {
                        dwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOverlayImage<Identity: IImageList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ioverlay: i32, piindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IImageList_Impl::GetOverlayImage(this, core::mem::transmute_copy(&ioverlay)) {
                    Ok(ok__) => {
                        piindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            ReplaceIcon: ReplaceIcon::<Identity, OFFSET>,
            SetOverlayImage: SetOverlayImage::<Identity, OFFSET>,
            Replace: Replace::<Identity, OFFSET>,
            AddMasked: AddMasked::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetImageRect: GetImageRect::<Identity, OFFSET>,
            GetIconSize: GetIconSize::<Identity, OFFSET>,
            SetIconSize: SetIconSize::<Identity, OFFSET>,
            GetImageCount: GetImageCount::<Identity, OFFSET>,
            SetImageCount: SetImageCount::<Identity, OFFSET>,
            SetBkColor: SetBkColor::<Identity, OFFSET>,
            GetBkColor: GetBkColor::<Identity, OFFSET>,
            BeginDrag: BeginDrag::<Identity, OFFSET>,
            EndDrag: EndDrag::<Identity, OFFSET>,
            DragEnter: DragEnter::<Identity, OFFSET>,
            DragLeave: DragLeave::<Identity, OFFSET>,
            DragMove: DragMove::<Identity, OFFSET>,
            SetDragCursorImage: SetDragCursorImage::<Identity, OFFSET>,
            DragShowNolock: DragShowNolock::<Identity, OFFSET>,
            GetDragImage: GetDragImage::<Identity, OFFSET>,
            GetItemFlags: GetItemFlags::<Identity, OFFSET>,
            GetOverlayImage: GetOverlayImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "commctrl", feature = "windef"))]
impl windows_core::RuntimeName for IImageList {}
windows_core::imp::define_interface!(IImageList2, IImageList2_Vtbl, 0x192b9d83_50fc_457b_90a0_2b82a8b5dae1);
impl core::ops::Deref for IImageList2 {
    type Target = IImageList;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageList2, windows_core::IUnknown, IImageList);
impl IImageList2 {
    pub unsafe fn Resize(&self, cxnewiconsize: i32, cynewiconsize: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), cxnewiconsize, cynewiconsize) }
    }
    pub unsafe fn GetOriginalSize(&self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOriginalSize)(windows_core::Interface::as_raw(self), iimage, dwflags, pcx as _, pcy as _) }
    }
    pub unsafe fn SetOriginalSize(&self, iimage: i32, cx: i32, cy: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOriginalSize)(windows_core::Interface::as_raw(self), iimage, cx, cy) }
    }
    pub unsafe fn SetCallback<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCallback)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn GetCallback<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCallback)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn ForceImagePresent(&self, iimage: i32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ForceImagePresent)(windows_core::Interface::as_raw(self), iimage, dwflags) }
    }
    pub unsafe fn DiscardImages(&self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DiscardImages)(windows_core::Interface::as_raw(self), ifirstimage, ilastimage, dwflags) }
    }
    #[cfg(all(feature = "commctrl", feature = "windef"))]
    pub unsafe fn PreloadImages(&self, pimldp: *const super::commctrl::IMAGELISTDRAWPARAMS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PreloadImages)(windows_core::Interface::as_raw(self), pimldp) }
    }
    pub unsafe fn GetStatistics(&self, pils: *mut IMAGELISTSTATS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), pils as _) }
    }
    pub unsafe fn Initialize(&self, cx: i32, cy: i32, flags: u32, cinitial: i32, cgrow: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), cx, cy, flags, cinitial, cgrow) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Replace2<P3>(&self, i: i32, hbmimage: super::windef::HBITMAP, hbmmask: Option<super::windef::HBITMAP>, punk: P3, dwflags: u32) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Replace2)(windows_core::Interface::as_raw(self), i, hbmimage, hbmmask.unwrap_or(core::mem::zeroed()) as _, punk.param().abi(), dwflags) }
    }
    pub unsafe fn ReplaceFromImageList<P1, P3>(&self, i: i32, pil: P1, isrc: i32, punk: P3, dwflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IImageList>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReplaceFromImageList)(windows_core::Interface::as_raw(self), i, pil.param().abi(), isrc, punk.param().abi(), dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList2_Vtbl {
    pub base__: IImageList_Vtbl,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetOriginalSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub SetOriginalSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, i32) -> windows_core::HRESULT,
    pub SetCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ForceImagePresent: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32) -> windows_core::HRESULT,
    pub DiscardImages: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "commctrl", feature = "windef"))]
    pub PreloadImages: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::commctrl::IMAGELISTDRAWPARAMS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "commctrl", feature = "windef")))]
    PreloadImages: usize,
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAGELISTSTATS) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, u32, i32, i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub Replace2: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::windef::HBITMAP, super::windef::HBITMAP, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Replace2: usize,
    pub ReplaceFromImageList: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "commctrl", feature = "windef"))]
pub trait IImageList2_Impl: IImageList_Impl {
    fn Resize(&self, cxnewiconsize: i32, cynewiconsize: i32) -> windows_core::Result<()>;
    fn GetOriginalSize(&self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> windows_core::Result<()>;
    fn SetOriginalSize(&self, iimage: i32, cx: i32, cy: i32) -> windows_core::Result<()>;
    fn SetCallback(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCallback(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ForceImagePresent(&self, iimage: i32, dwflags: u32) -> windows_core::Result<()>;
    fn DiscardImages(&self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> windows_core::Result<()>;
    fn PreloadImages(&self, pimldp: *const super::commctrl::IMAGELISTDRAWPARAMS) -> windows_core::Result<()>;
    fn GetStatistics(&self, pils: *mut IMAGELISTSTATS) -> windows_core::Result<()>;
    fn Initialize(&self, cx: i32, cy: i32, flags: u32, cinitial: i32, cgrow: i32) -> windows_core::Result<()>;
    fn Replace2(&self, i: i32, hbmimage: super::windef::HBITMAP, hbmmask: super::windef::HBITMAP, punk: windows_core::Ref<windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<()>;
    fn ReplaceFromImageList(&self, i: i32, pil: windows_core::Ref<IImageList>, isrc: i32, punk: windows_core::Ref<windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "commctrl", feature = "windef"))]
impl IImageList2_Vtbl {
    pub const fn new<Identity: IImageList2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resize<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cxnewiconsize: i32, cynewiconsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::Resize(this, core::mem::transmute_copy(&cxnewiconsize), core::mem::transmute_copy(&cynewiconsize)).into()
            }
        }
        unsafe extern "system" fn GetOriginalSize<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::GetOriginalSize(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pcx), core::mem::transmute_copy(&pcy)).into()
            }
        }
        unsafe extern "system" fn SetOriginalSize<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, cx: i32, cy: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::SetOriginalSize(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
            }
        }
        unsafe extern "system" fn SetCallback<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::SetCallback(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn GetCallback<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::GetCallback(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn ForceImagePresent<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::ForceImagePresent(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn DiscardImages<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::DiscardImages(this, core::mem::transmute_copy(&ifirstimage), core::mem::transmute_copy(&ilastimage), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn PreloadImages<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimldp: *const super::commctrl::IMAGELISTDRAWPARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::PreloadImages(this, core::mem::transmute_copy(&pimldp)).into()
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pils: *mut IMAGELISTSTATS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::GetStatistics(this, core::mem::transmute_copy(&pils)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: i32, cy: i32, flags: u32, cinitial: i32, cgrow: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::Initialize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&cinitial), core::mem::transmute_copy(&cgrow)).into()
            }
        }
        unsafe extern "system" fn Replace2<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, hbmimage: super::windef::HBITMAP, hbmmask: super::windef::HBITMAP, punk: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::Replace2(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&hbmmask), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn ReplaceFromImageList<Identity: IImageList2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, pil: *mut core::ffi::c_void, isrc: i32, punk: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageList2_Impl::ReplaceFromImageList(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&pil), core::mem::transmute_copy(&isrc), core::mem::transmute_copy(&punk), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: IImageList_Vtbl::new::<Identity, OFFSET>(),
            Resize: Resize::<Identity, OFFSET>,
            GetOriginalSize: GetOriginalSize::<Identity, OFFSET>,
            SetOriginalSize: SetOriginalSize::<Identity, OFFSET>,
            SetCallback: SetCallback::<Identity, OFFSET>,
            GetCallback: GetCallback::<Identity, OFFSET>,
            ForceImagePresent: ForceImagePresent::<Identity, OFFSET>,
            DiscardImages: DiscardImages::<Identity, OFFSET>,
            PreloadImages: PreloadImages::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            Replace2: Replace2::<Identity, OFFSET>,
            ReplaceFromImageList: ReplaceFromImageList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageList2 as windows_core::Interface>::IID || iid == &<IImageList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "commctrl", feature = "windef"))]
impl windows_core::RuntimeName for IImageList2 {}
pub const ILDI_PURGE: u32 = 1;
pub const ILDI_QUERYACCESS: u32 = 8;
pub const ILDI_RESETACCESS: u32 = 4;
pub const ILDI_STANDBY: u32 = 2;
pub const ILDRF_IMAGELOWQUALITY: u32 = 1;
pub const ILDRF_OVERLAYLOWQUALITY: u32 = 16;
pub const ILFIP_ALWAYS: u32 = 0;
pub const ILFIP_FROMSTANDBY: u32 = 1;
pub const ILGOS_ALWAYS: u32 = 0;
pub const ILGOS_FROMSTANDBY: u32 = 1;
pub const ILIF_ALPHA: u32 = 1;
pub const ILIF_LOWQUALITY: u32 = 2;
pub const ILR_DEFAULT: u32 = 0;
pub const ILR_HORIZONTAL_CENTER: u32 = 1;
pub const ILR_HORIZONTAL_LEFT: u32 = 0;
pub const ILR_HORIZONTAL_RIGHT: u32 = 2;
pub const ILR_SCALE_ASPECTRATIO: u32 = 256;
pub const ILR_SCALE_CLIP: u32 = 0;
pub const ILR_VERTICAL_BOTTOM: u32 = 32;
pub const ILR_VERTICAL_CENTER: u32 = 16;
pub const ILR_VERTICAL_TOP: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGELISTSTATS {
    pub cbSize: u32,
    pub cAlloc: i32,
    pub cUsed: i32,
    pub cStandby: i32,
}
pub const ImageList: windows_core::GUID = windows_core::GUID::from_u128(0x7c476ba2_02b1_48f4_8048_b24619ddc058);
