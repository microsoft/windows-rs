#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICompositionCapabilitiesInteropFactory(pub ::windows::runtime::IUnknown);
impl ICompositionCapabilitiesInteropFactory {
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `UI_Composition`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::runtime::Result<super::super::super::super::UI::Composition::CompositionCapabilities> {
        let mut result__: <super::super::super::super::UI::Composition::CompositionCapabilities as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), &mut result__).from_abi::<super::super::super::super::UI::Composition::CompositionCapabilities>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionCapabilitiesInteropFactory {
    type Vtable = ICompositionCapabilitiesInteropFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2c9db356_e70d_4642_8298_bc4aa5b4865c);
}
impl ::core::convert::From<ICompositionCapabilitiesInteropFactory> for ::windows::runtime::IUnknown {
    fn from(value: ICompositionCapabilitiesInteropFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositionCapabilitiesInteropFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositionCapabilitiesInteropFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICompositionCapabilitiesInteropFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionCapabilitiesInteropFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICompositionDrawingSurfaceInterop(pub ::windows::runtime::IUnknown);
impl ICompositionDrawingSurfaceInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `Win32_Foundation`*"]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `Win32_Foundation`*"]
    pub unsafe fn Resize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::SIZE>>(&self, sizepixels: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), sizepixels.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `Win32_Foundation`*"]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionDrawingSurfaceInterop {
    type Vtable = ICompositionDrawingSurfaceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfd04e6e3_fe0c_4c3c_ab19_a07601a576ee);
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop> for ::windows::runtime::IUnknown {
    fn from(value: ICompositionDrawingSurfaceInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositionDrawingSurfaceInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICompositionDrawingSurfaceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICompositionDrawingSurfaceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICompositionDrawingSurfaceInterop2(pub ::windows::runtime::IUnknown);
impl ICompositionDrawingSurfaceInterop2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `Win32_Foundation`*"]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `Win32_Foundation`*"]
    pub unsafe fn Resize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::SIZE>>(&self, sizepixels: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), sizepixels.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `Win32_Foundation`*"]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `Win32_Foundation`*"]
    pub unsafe fn CopySurface<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, destinationresource: Param0, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), destinationresource.into_param().abi(), ::core::mem::transmute(destinationoffsetx), ::core::mem::transmute(destinationoffsety), ::core::mem::transmute(sourcerectangle)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionDrawingSurfaceInterop2 {
    type Vtable = ICompositionDrawingSurfaceInterop2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x41e64aae_98c0_4239_8e95_a330dd6aa18b);
}
impl ::core::convert::From<ICompositionDrawingSurfaceInterop2> for ::windows::runtime::IUnknown {
    fn from(value: ICompositionDrawingSurfaceInterop2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositionDrawingSurfaceInterop2> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositionDrawingSurfaceInterop2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionDrawingSurfaceInterop> for ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionDrawingSurfaceInterop> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionDrawingSurfaceInterop> for &ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionDrawingSurfaceInterop> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sizepixels: super::super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, destinationresource: ::windows::runtime::RawPtr, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICompositionGraphicsDeviceInterop(pub ::windows::runtime::IUnknown);
impl ICompositionGraphicsDeviceInterop {
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn GetRenderingDevice(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn SetRenderingDevice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionGraphicsDeviceInterop {
    type Vtable = ICompositionGraphicsDeviceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa116ff71_f8bf_4c8a_9c98_70779a32a9c8);
}
impl ::core::convert::From<ICompositionGraphicsDeviceInterop> for ::windows::runtime::IUnknown {
    fn from(value: ICompositionGraphicsDeviceInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositionGraphicsDeviceInterop> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositionGraphicsDeviceInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICompositionGraphicsDeviceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICompositionGraphicsDeviceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionGraphicsDeviceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICompositorDesktopInterop(pub ::windows::runtime::IUnknown);
impl ICompositorDesktopInterop {
    #[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `UI_Composition_Desktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateDesktopWindowTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, hwndtarget: Param0, istopmost: Param1) -> ::windows::runtime::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget> {
        let mut result__: <super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), hwndtarget.into_param().abi(), istopmost.into_param().abi(), &mut result__).from_abi::<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn EnsureOnThread(&self, threadid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(threadid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICompositorDesktopInterop {
    type Vtable = ICompositorDesktopInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x29e691fa_4567_4dca_b319_d0f207eb6807);
}
impl ::core::convert::From<ICompositorDesktopInterop> for ::windows::runtime::IUnknown {
    fn from(value: ICompositorDesktopInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositorDesktopInterop> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositorDesktopInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICompositorDesktopInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICompositorDesktopInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorDesktopInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threadid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICompositorInterop(pub ::windows::runtime::IUnknown);
impl ICompositorInterop {
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `UI_Composition`, `Win32_Foundation`*"]
    pub unsafe fn CreateCompositionSurfaceForHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>>(&self, swapchain: Param0) -> ::windows::runtime::Result<super::super::super::super::UI::Composition::ICompositionSurface> {
        let mut result__: <super::super::super::super::UI::Composition::ICompositionSurface as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi(), &mut result__).from_abi::<super::super::super::super::UI::Composition::ICompositionSurface>(result__)
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `UI_Composition`*"]
    pub unsafe fn CreateCompositionSurfaceForSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, swapchain: Param0) -> ::windows::runtime::Result<super::super::super::super::UI::Composition::ICompositionSurface> {
        let mut result__: <super::super::super::super::UI::Composition::ICompositionSurface as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), swapchain.into_param().abi(), &mut result__).from_abi::<super::super::super::super::UI::Composition::ICompositionSurface>(result__)
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `UI_Composition`*"]
    pub unsafe fn CreateGraphicsDevice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, renderingdevice: Param0) -> ::windows::runtime::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice> {
        let mut result__: <super::super::super::super::UI::Composition::CompositionGraphicsDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), renderingdevice.into_param().abi(), &mut result__).from_abi::<super::super::super::super::UI::Composition::CompositionGraphicsDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICompositorInterop {
    type Vtable = ICompositorInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x25297d5c_3ad4_4c9c_b5cf_e36a38512330);
}
impl ::core::convert::From<ICompositorInterop> for ::windows::runtime::IUnknown {
    fn from(value: ICompositorInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositorInterop> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositorInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICompositorInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICompositorInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: super::super::super::Foundation::HANDLE, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, renderingdevice: ::windows::runtime::RawPtr, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDesktopWindowTargetInterop(pub ::windows::runtime::IUnknown);
impl IDesktopWindowTargetInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `Win32_Foundation`*"]
    pub unsafe fn Hwnd(&self) -> ::windows::runtime::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDesktopWindowTargetInterop {
    type Vtable = IDesktopWindowTargetInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x35dbf59e_e3f9_45b0_81e7_fe75f4145dc9);
}
impl ::core::convert::From<IDesktopWindowTargetInterop> for ::windows::runtime::IUnknown {
    fn from(value: IDesktopWindowTargetInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDesktopWindowTargetInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IDesktopWindowTargetInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDesktopWindowTargetInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDesktopWindowTargetInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTargetInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ISwapChainInterop(pub ::windows::runtime::IUnknown);
impl ISwapChainInterop {
    #[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, swapchain: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISwapChainInterop {
    type Vtable = ISwapChainInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x26f496a0_7f38_45fb_88f7_faaabe67dd59);
}
impl ::core::convert::From<ISwapChainInterop> for ::windows::runtime::IUnknown {
    fn from(value: ISwapChainInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISwapChainInterop> for ::windows::runtime::IUnknown {
    fn from(value: &ISwapChainInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISwapChainInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISwapChainInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT_Composition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVisualInteractionSourceInterop(pub ::windows::runtime::IUnknown);
impl IVisualInteractionSourceInterop {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_WinRT_Composition`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pointerinfo)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceInterop {
    type Vtable = IVisualInteractionSourceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x11f62cd1_2f9d_42d3_b05f_d6790d9e9f8e);
}
impl ::core::convert::From<IVisualInteractionSourceInterop> for ::windows::runtime::IUnknown {
    fn from(value: IVisualInteractionSourceInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVisualInteractionSourceInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IVisualInteractionSourceInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVisualInteractionSourceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVisualInteractionSourceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
