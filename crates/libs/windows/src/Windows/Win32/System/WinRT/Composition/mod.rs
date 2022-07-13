#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`*"]
#[repr(transparent)]
pub struct ICompositionCapabilitiesInteropFactory(::windows::core::IUnknown);
impl ICompositionCapabilitiesInteropFactory {
    #[doc = "*Required features: `\"UI_Composition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    pub unsafe fn GetForWindow<'a, P0>(&self, hwnd: P0) -> ::windows::core::Result<super::super::super::super::UI::Composition::CompositionCapabilities>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), hwnd.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::super::UI::Composition::CompositionCapabilities>(result__)
    }
}
impl ::core::convert::From<ICompositionCapabilitiesInteropFactory> for ::windows::core::IUnknown {
    fn from(value: ICompositionCapabilitiesInteropFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICompositionCapabilitiesInteropFactory> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICompositionCapabilitiesInteropFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionCapabilitiesInteropFactory> for ::windows::core::IUnknown {
    fn from(value: &ICompositionCapabilitiesInteropFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICompositionCapabilitiesInteropFactory> for ::windows::core::IInspectable {
    fn from(value: ICompositionCapabilitiesInteropFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICompositionCapabilitiesInteropFactory> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ICompositionCapabilitiesInteropFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionCapabilitiesInteropFactory> for ::windows::core::IInspectable {
    fn from(value: &ICompositionCapabilitiesInteropFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICompositionCapabilitiesInteropFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionCapabilitiesInteropFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionCapabilitiesInteropFactory {}
impl ::core::fmt::Debug for ICompositionCapabilitiesInteropFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionCapabilitiesInteropFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICompositionCapabilitiesInteropFactory {
    type Vtable = ICompositionCapabilitiesInteropFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c9db356_e70d_4642_8298_bc4aa5b4865c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionCapabilitiesInteropFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`*"]
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop(::windows::core::IUnknown);
impl ICompositionDrawingSurfaceInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw<T>(&self, updaterect: *const super::super::super::Foundation::RECT, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).BeginDraw)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(updaterect), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _, ::core::mem::transmute(updateoffset)).and_some(result__)
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Resize(&self, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(sizepixels)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Scroll)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), offsetx, offsety).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResumeDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspendDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop> for ::windows::core::IUnknown {
    fn from(value: ICompositionDrawingSurfaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICompositionDrawingSurfaceInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICompositionDrawingSurfaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop> for ::windows::core::IUnknown {
    fn from(value: &ICompositionDrawingSurfaceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICompositionDrawingSurfaceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionDrawingSurfaceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionDrawingSurfaceInterop {}
impl ::core::fmt::Debug for ICompositionDrawingSurfaceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionDrawingSurfaceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICompositionDrawingSurfaceInterop {
    type Vtable = ICompositionDrawingSurfaceInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd04e6e3_fe0c_4c3c_ab19_a07601a576ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Resize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scroll: usize,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`*"]
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop2(::windows::core::IUnknown);
impl ICompositionDrawingSurfaceInterop2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw<T>(&self, updaterect: *const super::super::super::Foundation::RECT, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).base__.BeginDraw)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(updaterect), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _, ::core::mem::transmute(updateoffset)).and_some(result__)
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Resize(&self, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Resize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(sizepixels)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Scroll)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), offsetx, offsety).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResumeDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SuspendDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopySurface<'a, P0>(&self, destinationresource: P0, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).CopySurface)(::windows::core::Interface::as_raw(self), destinationresource.into().abi(), destinationoffsetx, destinationoffsety, ::core::mem::transmute(sourcerectangle)).ok()
    }
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop2> for ::windows::core::IUnknown {
    fn from(value: ICompositionDrawingSurfaceInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICompositionDrawingSurfaceInterop2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICompositionDrawingSurfaceInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop2> for ::windows::core::IUnknown {
    fn from(value: &ICompositionDrawingSurfaceInterop2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop2> for ICompositionDrawingSurfaceInterop {
    fn from(value: ICompositionDrawingSurfaceInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICompositionDrawingSurfaceInterop2> for &'a ICompositionDrawingSurfaceInterop {
    fn from(value: &'a ICompositionDrawingSurfaceInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop2> for ICompositionDrawingSurfaceInterop {
    fn from(value: &ICompositionDrawingSurfaceInterop2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICompositionDrawingSurfaceInterop2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionDrawingSurfaceInterop2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionDrawingSurfaceInterop2 {}
impl ::core::fmt::Debug for ICompositionDrawingSurfaceInterop2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionDrawingSurfaceInterop2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICompositionDrawingSurfaceInterop2 {
    type Vtable = ICompositionDrawingSurfaceInterop2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41e64aae_98c0_4239_8e95_a330dd6aa18b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop2_Vtbl {
    pub base__: ICompositionDrawingSurfaceInterop_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CopySurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationresource: *mut ::core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CopySurface: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`*"]
#[repr(transparent)]
pub struct ICompositionGraphicsDeviceInterop(::windows::core::IUnknown);
impl ICompositionGraphicsDeviceInterop {
    pub unsafe fn GetRenderingDevice(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRenderingDevice)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetRenderingDevice<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetRenderingDevice)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
}
impl ::core::convert::From<ICompositionGraphicsDeviceInterop> for ::windows::core::IUnknown {
    fn from(value: ICompositionGraphicsDeviceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICompositionGraphicsDeviceInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICompositionGraphicsDeviceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionGraphicsDeviceInterop> for ::windows::core::IUnknown {
    fn from(value: &ICompositionGraphicsDeviceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICompositionGraphicsDeviceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionGraphicsDeviceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionGraphicsDeviceInterop {}
impl ::core::fmt::Debug for ICompositionGraphicsDeviceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionGraphicsDeviceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICompositionGraphicsDeviceInterop {
    type Vtable = ICompositionGraphicsDeviceInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa116ff71_f8bf_4c8a_9c98_70779a32a9c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionGraphicsDeviceInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetRenderingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRenderingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`*"]
#[repr(transparent)]
pub struct ICompositorDesktopInterop(::windows::core::IUnknown);
impl ICompositorDesktopInterop {
    #[doc = "*Required features: `\"UI_Composition_Desktop\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
    pub unsafe fn CreateDesktopWindowTarget<'a, P0, P1>(&self, hwndtarget: P0, istopmost: P1) -> ::windows::core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateDesktopWindowTarget)(::windows::core::Interface::as_raw(self), hwndtarget.into(), istopmost.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>(result__)
    }
    pub unsafe fn EnsureOnThread(&self, threadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnsureOnThread)(::windows::core::Interface::as_raw(self), threadid).ok()
    }
}
impl ::core::convert::From<ICompositorDesktopInterop> for ::windows::core::IUnknown {
    fn from(value: ICompositorDesktopInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICompositorDesktopInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICompositorDesktopInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositorDesktopInterop> for ::windows::core::IUnknown {
    fn from(value: &ICompositorDesktopInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICompositorDesktopInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositorDesktopInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositorDesktopInterop {}
impl ::core::fmt::Debug for ICompositorDesktopInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositorDesktopInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICompositorDesktopInterop {
    type Vtable = ICompositorDesktopInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29e691fa_4567_4dca_b319_d0f207eb6807);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorDesktopInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
    pub CreateDesktopWindowTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation")))]
    CreateDesktopWindowTarget: usize,
    pub EnsureOnThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`*"]
#[repr(transparent)]
pub struct ICompositorInterop(::windows::core::IUnknown);
impl ICompositorInterop {
    #[doc = "*Required features: `\"UI_Composition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    pub unsafe fn CreateCompositionSurfaceForHandle<'a, P0>(&self, swapchain: P0) -> ::windows::core::Result<super::super::super::super::UI::Composition::ICompositionSurface>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateCompositionSurfaceForHandle)(::windows::core::Interface::as_raw(self), swapchain.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::super::UI::Composition::ICompositionSurface>(result__)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForSwapChain<'a, P0>(&self, swapchain: P0) -> ::windows::core::Result<super::super::super::super::UI::Composition::ICompositionSurface>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateCompositionSurfaceForSwapChain)(::windows::core::Interface::as_raw(self), swapchain.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::super::UI::Composition::ICompositionSurface>(result__)
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateGraphicsDevice<'a, P0>(&self, renderingdevice: P0) -> ::windows::core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateGraphicsDevice)(::windows::core::Interface::as_raw(self), renderingdevice.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::super::UI::Composition::CompositionGraphicsDevice>(result__)
    }
}
impl ::core::convert::From<ICompositorInterop> for ::windows::core::IUnknown {
    fn from(value: ICompositorInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICompositorInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICompositorInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositorInterop> for ::windows::core::IUnknown {
    fn from(value: &ICompositorInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICompositorInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositorInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositorInterop {}
impl ::core::fmt::Debug for ICompositorInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositorInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICompositorInterop {
    type Vtable = ICompositorInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25297d5c_3ad4_4c9c_b5cf_e36a38512330);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    pub CreateCompositionSurfaceForHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))]
    CreateCompositionSurfaceForHandle: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionSurfaceForSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionSurfaceForSwapChain: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateGraphicsDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateGraphicsDevice: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`*"]
#[repr(transparent)]
pub struct IDesktopWindowTargetInterop(::windows::core::IUnknown);
impl IDesktopWindowTargetInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Hwnd(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Hwnd)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
impl ::core::convert::From<IDesktopWindowTargetInterop> for ::windows::core::IUnknown {
    fn from(value: IDesktopWindowTargetInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDesktopWindowTargetInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDesktopWindowTargetInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowTargetInterop> for ::windows::core::IUnknown {
    fn from(value: &IDesktopWindowTargetInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDesktopWindowTargetInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDesktopWindowTargetInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDesktopWindowTargetInterop {}
impl ::core::fmt::Debug for IDesktopWindowTargetInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDesktopWindowTargetInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDesktopWindowTargetInterop {
    type Vtable = IDesktopWindowTargetInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35dbf59e_e3f9_45b0_81e7_fe75f4145dc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTargetInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Hwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Hwnd: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`*"]
#[repr(transparent)]
pub struct ISwapChainInterop(::windows::core::IUnknown);
impl ISwapChainInterop {
    pub unsafe fn SetSwapChain<'a, P0>(&self, swapchain: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetSwapChain)(::windows::core::Interface::as_raw(self), swapchain.into().abi()).ok()
    }
}
impl ::core::convert::From<ISwapChainInterop> for ::windows::core::IUnknown {
    fn from(value: ISwapChainInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISwapChainInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISwapChainInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainInterop> for ::windows::core::IUnknown {
    fn from(value: &ISwapChainInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISwapChainInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISwapChainInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISwapChainInterop {}
impl ::core::fmt::Debug for ISwapChainInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISwapChainInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISwapChainInterop {
    type Vtable = ISwapChainInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26f496a0_7f38_45fb_88f7_faaabe67dd59);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Composition\"`*"]
#[repr(transparent)]
pub struct IVisualInteractionSourceInterop(::windows::core::IUnknown);
impl IVisualInteractionSourceInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Input_Pointer\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TryRedirectForManipulation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pointerinfo)).ok()
    }
}
impl ::core::convert::From<IVisualInteractionSourceInterop> for ::windows::core::IUnknown {
    fn from(value: IVisualInteractionSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualInteractionSourceInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVisualInteractionSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualInteractionSourceInterop> for ::windows::core::IUnknown {
    fn from(value: &IVisualInteractionSourceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVisualInteractionSourceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVisualInteractionSourceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVisualInteractionSourceInterop {}
impl ::core::fmt::Debug for IVisualInteractionSourceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVisualInteractionSourceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVisualInteractionSourceInterop {
    type Vtable = IVisualInteractionSourceInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11f62cd1_2f9d_42d3_b05f_d6790d9e9f8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceInterop_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub TryRedirectForManipulation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging")))]
    TryRedirectForManipulation: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
