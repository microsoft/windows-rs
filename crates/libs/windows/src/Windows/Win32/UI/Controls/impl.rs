#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList_Impl: Sized {
    fn Add(&self, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<i32>;
    fn ReplaceIcon(&self, i: i32, hicon: super::WindowsAndMessaging::HICON) -> ::windows::core::Result<i32>;
    fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> ::windows::core::Result<()>;
    fn Replace(&self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()>;
    fn AddMasked(&self, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32) -> ::windows::core::Result<i32>;
    fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::Result<()>;
    fn Remove(&self, i: i32) -> ::windows::core::Result<()>;
    fn GetIcon(&self, i: i32, flags: u32) -> ::windows::core::Result<super::WindowsAndMessaging::HICON>;
    fn GetImageInfo(&self, i: i32) -> ::windows::core::Result<IMAGEINFO>;
    fn Copy(&self, idst: i32, punksrc: &::core::option::Option<::windows::core::IUnknown>, isrc: i32, uflags: u32) -> ::windows::core::Result<()>;
    fn Merge(&self, i1: i32, punk2: &::core::option::Option<::windows::core::IUnknown>, i2: i32, dx: i32, dy: i32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Clone(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetImageRect(&self, i: i32) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> ::windows::core::Result<()>;
    fn SetIconSize(&self, cx: i32, cy: i32) -> ::windows::core::Result<()>;
    fn GetImageCount(&self) -> ::windows::core::Result<i32>;
    fn SetImageCount(&self, unewcount: u32) -> ::windows::core::Result<()>;
    fn SetBkColor(&self, clrbk: u32) -> ::windows::core::Result<u32>;
    fn GetBkColor(&self) -> ::windows::core::Result<u32>;
    fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::Result<()>;
    fn EndDrag(&self) -> ::windows::core::Result<()>;
    fn DragEnter(&self, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows::core::Result<()>;
    fn DragLeave(&self, hwndlock: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn DragMove(&self, x: i32, y: i32) -> ::windows::core::Result<()>;
    fn SetDragCursorImage(&self, punk: &::core::option::Option<::windows::core::IUnknown>, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::Result<()>;
    fn DragShowNolock(&self, fshow: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDragImage(&self, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetItemFlags(&self, i: i32) -> ::windows::core::Result<IMAGE_LIST_ITEM_FLAGS>;
    fn GetOverlayImage(&self, ioverlay: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IImageList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImageList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>() -> IImageList_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&hbmmask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReplaceIcon(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&hicon)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, ioverlay: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOverlayImage(::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&ioverlay)).into()
        }
        unsafe extern "system" fn Replace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Replace(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&hbmmask)).into()
        }
        unsafe extern "system" fn AddMasked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddMasked(::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&crmask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(::core::mem::transmute_copy(&pimldp)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&i)).into()
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIcon(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(picon, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageInfo(::core::mem::transmute_copy(&i)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pimageinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idst: i32, punksrc: *mut ::core::ffi::c_void, isrc: i32, uflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Copy(::core::mem::transmute_copy(&idst), ::core::mem::transmute(&punksrc), ::core::mem::transmute_copy(&isrc), ::core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn Merge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i1: i32, punk2: *mut ::core::ffi::c_void, i2: i32, dx: i32, dy: i32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Merge(::core::mem::transmute_copy(&i1), ::core::mem::transmute(&punk2), ::core::mem::transmute_copy(&i2), ::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clone(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetImageRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageRect(::core::mem::transmute_copy(&i)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIconSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: *mut i32, cy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIconSize(::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)).into()
        }
        unsafe extern "system" fn SetIconSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: i32, cy: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIconSize(::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)).into()
        }
        unsafe extern "system" fn GetImageCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pi: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unewcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetImageCount(::core::mem::transmute_copy(&unewcount)).into()
        }
        unsafe extern "system" fn SetBkColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clrbk: u32, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetBkColor(::core::mem::transmute_copy(&clrbk)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBkColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBkColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDrag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginDrag(::core::mem::transmute_copy(&itrack), ::core::mem::transmute_copy(&dxhotspot), ::core::mem::transmute_copy(&dyhotspot)).into()
        }
        unsafe extern "system" fn EndDrag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndDrag().into()
        }
        unsafe extern "system" fn DragEnter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DragEnter(::core::mem::transmute_copy(&hwndlock), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn DragLeave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DragLeave(::core::mem::transmute_copy(&hwndlock)).into()
        }
        unsafe extern "system" fn DragMove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DragMove(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn SetDragCursorImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDragCursorImage(::core::mem::transmute(&punk), ::core::mem::transmute_copy(&idrag), ::core::mem::transmute_copy(&dxhotspot), ::core::mem::transmute_copy(&dyhotspot)).into()
        }
        unsafe extern "system" fn DragShowNolock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DragShowNolock(::core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn GetDragImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDragImage(::core::mem::transmute_copy(&ppt), ::core::mem::transmute_copy(&ppthotspot), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetItemFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemFlags(::core::mem::transmute_copy(&i)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ioverlay: i32, piindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOverlayImage(::core::mem::transmute_copy(&ioverlay)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            ReplaceIcon: ReplaceIcon::<Identity, Impl, OFFSET>,
            SetOverlayImage: SetOverlayImage::<Identity, Impl, OFFSET>,
            Replace: Replace::<Identity, Impl, OFFSET>,
            AddMasked: AddMasked::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetImageRect: GetImageRect::<Identity, Impl, OFFSET>,
            GetIconSize: GetIconSize::<Identity, Impl, OFFSET>,
            SetIconSize: SetIconSize::<Identity, Impl, OFFSET>,
            GetImageCount: GetImageCount::<Identity, Impl, OFFSET>,
            SetImageCount: SetImageCount::<Identity, Impl, OFFSET>,
            SetBkColor: SetBkColor::<Identity, Impl, OFFSET>,
            GetBkColor: GetBkColor::<Identity, Impl, OFFSET>,
            BeginDrag: BeginDrag::<Identity, Impl, OFFSET>,
            EndDrag: EndDrag::<Identity, Impl, OFFSET>,
            DragEnter: DragEnter::<Identity, Impl, OFFSET>,
            DragLeave: DragLeave::<Identity, Impl, OFFSET>,
            DragMove: DragMove::<Identity, Impl, OFFSET>,
            SetDragCursorImage: SetDragCursorImage::<Identity, Impl, OFFSET>,
            DragShowNolock: DragShowNolock::<Identity, Impl, OFFSET>,
            GetDragImage: GetDragImage::<Identity, Impl, OFFSET>,
            GetItemFlags: GetItemFlags::<Identity, Impl, OFFSET>,
            GetOverlayImage: GetOverlayImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList2_Impl: Sized + IImageList_Impl {
    fn Resize(&self, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows::core::Result<()>;
    fn GetOriginalSize(&self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows::core::Result<()>;
    fn SetOriginalSize(&self, iimage: i32, cx: i32, cy: i32) -> ::windows::core::Result<()>;
    fn SetCallback(&self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetCallback(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ForceImagePresent(&self, iimage: i32, dwflags: u32) -> ::windows::core::Result<()>;
    fn DiscardImages(&self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows::core::Result<()>;
    fn PreloadImages(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::Result<()>;
    fn GetStatistics(&self, pils: *mut IMAGELISTSTATS) -> ::windows::core::Result<()>;
    fn Initialize(&self, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows::core::Result<()>;
    fn Replace2(&self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: &::core::option::Option<::windows::core::IUnknown>, dwflags: u32) -> ::windows::core::Result<()>;
    fn ReplaceFromImageList(&self, i: i32, pil: &::core::option::Option<IImageList>, isrc: i32, punk: &::core::option::Option<::windows::core::IUnknown>, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IImageList2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImageList2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>() -> IImageList2_Vtbl {
        unsafe extern "system" fn Resize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resize(::core::mem::transmute_copy(&cxnewiconsize), ::core::mem::transmute_copy(&cynewiconsize)).into()
        }
        unsafe extern "system" fn GetOriginalSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOriginalSize(::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pcx), ::core::mem::transmute_copy(&pcy)).into()
        }
        unsafe extern "system" fn SetOriginalSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, cx: i32, cy: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOriginalSize(::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)).into()
        }
        unsafe extern "system" fn SetCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCallback(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCallback(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ForceImagePresent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForceImagePresent(::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DiscardImages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DiscardImages(::core::mem::transmute_copy(&ifirstimage), ::core::mem::transmute_copy(&ilastimage), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn PreloadImages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreloadImages(::core::mem::transmute_copy(&pimldp)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pils: *mut IMAGELISTSTATS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatistics(::core::mem::transmute_copy(&pils)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&cinitial), ::core::mem::transmute_copy(&cgrow)).into()
        }
        unsafe extern "system" fn Replace2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Replace2(::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&hbmmask), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ReplaceFromImageList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, i: i32, pil: *mut ::core::ffi::c_void, isrc: i32, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReplaceFromImageList(::core::mem::transmute_copy(&i), ::core::mem::transmute(&pil), ::core::mem::transmute_copy(&isrc), ::core::mem::transmute(&punk), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: IImageList_Vtbl::new::<Identity, Impl, OFFSET>(),
            Resize: Resize::<Identity, Impl, OFFSET>,
            GetOriginalSize: GetOriginalSize::<Identity, Impl, OFFSET>,
            SetOriginalSize: SetOriginalSize::<Identity, Impl, OFFSET>,
            SetCallback: SetCallback::<Identity, Impl, OFFSET>,
            GetCallback: GetCallback::<Identity, Impl, OFFSET>,
            ForceImagePresent: ForceImagePresent::<Identity, Impl, OFFSET>,
            DiscardImages: DiscardImages::<Identity, Impl, OFFSET>,
            PreloadImages: PreloadImages::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Replace2: Replace2::<Identity, Impl, OFFSET>,
            ReplaceFromImageList: ReplaceFromImageList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageList2 as ::windows::core::Interface>::IID || iid == &<IImageList as ::windows::core::Interface>::IID
    }
}
