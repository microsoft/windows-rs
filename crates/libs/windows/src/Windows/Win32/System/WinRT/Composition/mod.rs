#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
#[repr(transparent)]
pub struct ICompositionCapabilitiesInteropFactory(::windows::core::IUnknown);
impl ICompositionCapabilitiesInteropFactory {
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'UI_Composition', 'Win32_Foundation'*"]
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<super::super::super::super::UI::Composition::CompositionCapabilities> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Composition::CompositionCapabilities>(result__)
    }
}
impl ::core::convert::From<ICompositionCapabilitiesInteropFactory> for ::windows::core::IInspectable {
    fn from(value: ICompositionCapabilitiesInteropFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionCapabilitiesInteropFactory> for ::windows::core::IInspectable {
    fn from(value: &ICompositionCapabilitiesInteropFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICompositionCapabilitiesInteropFactory> for ::windows::core::IUnknown {
    fn from(value: ICompositionCapabilitiesInteropFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionCapabilitiesInteropFactory> for ::windows::core::IUnknown {
    fn from(value: &ICompositionCapabilitiesInteropFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ICompositionCapabilitiesInteropFactory {
    type Vtable = ICompositionCapabilitiesInteropFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c9db356_e70d_4642_8298_bc4aa5b4865c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionCapabilitiesInteropFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop(::windows::core::IUnknown);
impl ICompositionDrawingSurfaceInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Resize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::SIZE>>(&self, sizepixels: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), sizepixels.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop> for ::windows::core::IUnknown {
    fn from(value: ICompositionDrawingSurfaceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop> for ::windows::core::IUnknown {
    fn from(value: &ICompositionDrawingSurfaceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICompositionDrawingSurfaceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICompositionDrawingSurfaceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ICompositionDrawingSurfaceInterop {
    type Vtable = ICompositionDrawingSurfaceInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd04e6e3_fe0c_4c3c_ab19_a07601a576ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop2(::windows::core::IUnknown);
impl ICompositionDrawingSurfaceInterop2 {
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Resize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::SIZE>>(&self, sizepixels: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), sizepixels.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopySurface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, destinationresource: Param0, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), destinationresource.into_param().abi(), ::core::mem::transmute(destinationoffsetx), ::core::mem::transmute(destinationoffsety), ::core::mem::transmute(sourcerectangle)).ok()
    }
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop2> for ICompositionDrawingSurfaceInterop {
    fn from(value: ICompositionDrawingSurfaceInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop2> for ICompositionDrawingSurfaceInterop {
    fn from(value: &ICompositionDrawingSurfaceInterop2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICompositionDrawingSurfaceInterop> for ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICompositionDrawingSurfaceInterop> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICompositionDrawingSurfaceInterop> for &ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::core::Param<'a, ICompositionDrawingSurfaceInterop> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop2> for ::windows::core::IUnknown {
    fn from(value: ICompositionDrawingSurfaceInterop2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop2> for ::windows::core::IUnknown {
    fn from(value: &ICompositionDrawingSurfaceInterop2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ICompositionDrawingSurfaceInterop2 {
    type Vtable = ICompositionDrawingSurfaceInterop2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41e64aae_98c0_4239_8e95_a330dd6aa18b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationresource: *mut ::core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
#[repr(transparent)]
pub struct ICompositionGraphicsDeviceInterop(::windows::core::IUnknown);
impl ICompositionGraphicsDeviceInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn GetRenderingDevice(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn SetRenderingDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, value: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ICompositionGraphicsDeviceInterop> for ::windows::core::IUnknown {
    fn from(value: ICompositionGraphicsDeviceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionGraphicsDeviceInterop> for ::windows::core::IUnknown {
    fn from(value: &ICompositionGraphicsDeviceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICompositionGraphicsDeviceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICompositionGraphicsDeviceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ICompositionGraphicsDeviceInterop {
    type Vtable = ICompositionGraphicsDeviceInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa116ff71_f8bf_4c8a_9c98_70779a32a9c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionGraphicsDeviceInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
#[repr(transparent)]
pub struct ICompositorDesktopInterop(::windows::core::IUnknown);
impl ICompositorDesktopInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'UI_Composition_Desktop', 'Win32_Foundation'*"]
    #[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
    pub unsafe fn CreateDesktopWindowTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, hwndtarget: Param0, istopmost: Param1) -> ::windows::core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hwndtarget.into_param().abi(), istopmost.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn EnsureOnThread(&self, threadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(threadid)).ok()
    }
}
impl ::core::convert::From<ICompositorDesktopInterop> for ::windows::core::IUnknown {
    fn from(value: ICompositorDesktopInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositorDesktopInterop> for ::windows::core::IUnknown {
    fn from(value: &ICompositorDesktopInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICompositorDesktopInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICompositorDesktopInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ICompositorDesktopInterop {
    type Vtable = ICompositorDesktopInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29e691fa_4567_4dca_b319_d0f207eb6807);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorDesktopInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
#[repr(transparent)]
pub struct ICompositorInterop(::windows::core::IUnknown);
impl ICompositorInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'UI_Composition', 'Win32_Foundation'*"]
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    pub unsafe fn CreateCompositionSurfaceForHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, swapchain: Param0) -> ::windows::core::Result<super::super::super::super::UI::Composition::ICompositionSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Composition::ICompositionSurface>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'UI_Composition'*"]
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateCompositionSurfaceForSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, swapchain: Param0) -> ::windows::core::Result<super::super::super::super::UI::Composition::ICompositionSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), swapchain.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Composition::ICompositionSurface>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'UI_Composition'*"]
    #[cfg(feature = "UI_Composition")]
    pub unsafe fn CreateGraphicsDevice<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows::core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), renderingdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::UI::Composition::CompositionGraphicsDevice>(result__)
    }
}
impl ::core::convert::From<ICompositorInterop> for ::windows::core::IUnknown {
    fn from(value: ICompositorInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositorInterop> for ::windows::core::IUnknown {
    fn from(value: &ICompositorInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICompositorInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICompositorInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ICompositorInterop {
    type Vtable = ICompositorInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25297d5c_3ad4_4c9c_b5cf_e36a38512330);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
#[repr(transparent)]
pub struct IDesktopWindowTargetInterop(::windows::core::IUnknown);
impl IDesktopWindowTargetInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Hwnd(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: super::super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
impl ::core::convert::From<IDesktopWindowTargetInterop> for ::windows::core::IUnknown {
    fn from(value: IDesktopWindowTargetInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDesktopWindowTargetInterop> for ::windows::core::IUnknown {
    fn from(value: &IDesktopWindowTargetInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDesktopWindowTargetInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDesktopWindowTargetInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IDesktopWindowTargetInterop {
    type Vtable = IDesktopWindowTargetInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x35dbf59e_e3f9_45b0_81e7_fe75f4145dc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTargetInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
#[repr(transparent)]
pub struct ISwapChainInterop(::windows::core::IUnknown);
impl ISwapChainInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, swapchain: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
impl ::core::convert::From<ISwapChainInterop> for ::windows::core::IUnknown {
    fn from(value: ISwapChainInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISwapChainInterop> for ::windows::core::IUnknown {
    fn from(value: &ISwapChainInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISwapChainInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ISwapChainInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for ISwapChainInterop {
    type Vtable = ISwapChainInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26f496a0_7f38_45fb_88f7_faaabe67dd59);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainInteropVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
#[doc = "*Required features: 'Win32_System_WinRT_Composition'*"]
#[repr(transparent)]
pub struct IVisualInteractionSourceInterop(::windows::core::IUnknown);
impl IVisualInteractionSourceInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Composition', 'Win32_Foundation', 'Win32_UI_Input_Pointer', 'Win32_UI_WindowsAndMessaging'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerinfo)).ok()
    }
}
impl ::core::convert::From<IVisualInteractionSourceInterop> for ::windows::core::IUnknown {
    fn from(value: IVisualInteractionSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualInteractionSourceInterop> for ::windows::core::IUnknown {
    fn from(value: &IVisualInteractionSourceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVisualInteractionSourceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVisualInteractionSourceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IVisualInteractionSourceInterop {
    type Vtable = IVisualInteractionSourceInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11f62cd1_2f9d_42d3_b05f_d6790d9e9f8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
