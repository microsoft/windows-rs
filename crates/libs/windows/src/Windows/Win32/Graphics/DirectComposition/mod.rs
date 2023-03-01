#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionAttachMouseDragToHwnd<P0, P1, P2>(visual: P0, hwnd: P1, enable: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IDCompositionVisual>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionAttachMouseDragToHwnd ( visual : * mut::core::ffi::c_void , hwnd : super::super::Foundation:: HWND , enable : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    DCompositionAttachMouseDragToHwnd(visual.into_param().abi(), hwnd.into_param().abi(), enable.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionAttachMouseWheelToHwnd<P0, P1, P2>(visual: P0, hwnd: P1, enable: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IDCompositionVisual>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionAttachMouseWheelToHwnd ( visual : * mut::core::ffi::c_void , hwnd : super::super::Foundation:: HWND , enable : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    DCompositionAttachMouseWheelToHwnd(visual.into_param().abi(), hwnd.into_param().abi(), enable.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionBoostCompositorClock<P0>(enable: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionBoostCompositorClock ( enable : super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    DCompositionBoostCompositorClock(enable.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn DCompositionCreateDevice<P0, T>(dxgidevice: P0) -> ::windows::core::Result<T>
where
    P0: ::windows::core::IntoParam<super::Dxgi::IDXGIDevice>,
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionCreateDevice ( dxgidevice : * mut::core::ffi::c_void , iid : *const :: windows::core::GUID , dcompositiondevice : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    DCompositionCreateDevice(dxgidevice.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[inline]
pub unsafe fn DCompositionCreateDevice2<P0, T>(renderingdevice: P0) -> ::windows::core::Result<T>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionCreateDevice2 ( renderingdevice : * mut::core::ffi::c_void , iid : *const :: windows::core::GUID , dcompositiondevice : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    DCompositionCreateDevice2(renderingdevice.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[inline]
pub unsafe fn DCompositionCreateDevice3<P0, T>(renderingdevice: P0) -> ::windows::core::Result<T>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionCreateDevice3 ( renderingdevice : * mut::core::ffi::c_void , iid : *const :: windows::core::GUID , dcompositiondevice : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    DCompositionCreateDevice3(renderingdevice.into_param().abi(), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionCreateSurfaceHandle ( desiredaccess : u32 , securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES , surfacehandle : *mut super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HANDLE>();
    DCompositionCreateSurfaceHandle(desiredaccess, ::core::mem::transmute(securityattributes.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[inline]
pub unsafe fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE) -> ::windows::core::Result<u64> {
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionGetFrameId ( frameidtype : COMPOSITION_FRAME_ID_TYPE , frameid : *mut u64 ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    DCompositionGetFrameId(frameidtype, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: ::core::option::Option<*mut COMPOSITION_TARGET_ID>, actualtargetidcount: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionGetStatistics ( frameid : u64 , framestats : *mut COMPOSITION_FRAME_STATS , targetidcount : u32 , targetids : *mut COMPOSITION_TARGET_ID , actualtargetidcount : *mut u32 ) -> :: windows::core::HRESULT );
    DCompositionGetStatistics(frameid, framestats, targetidcount, ::core::mem::transmute(targetids.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(actualtargetidcount.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID, targetstats: *mut COMPOSITION_TARGET_STATS) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionGetTargetStatistics ( frameid : u64 , targetid : *const COMPOSITION_TARGET_ID , targetstats : *mut COMPOSITION_TARGET_STATS ) -> :: windows::core::HRESULT );
    DCompositionGetTargetStatistics(frameid, targetid, targetstats).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionWaitForCompositorClock(handles: ::core::option::Option<&[super::super::Foundation::HANDLE]>, timeoutinms: u32) -> u32 {
    ::windows::imp::link ! ( "dcomp.dll""system" fn DCompositionWaitForCompositorClock ( count : u32 , handles : *const super::super::Foundation:: HANDLE , timeoutinms : u32 ) -> u32 );
    DCompositionWaitForCompositorClock(handles.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(handles.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), timeoutinms)
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionAffineTransform2DEffect(::windows::core::IUnknown);
impl IDCompositionAffineTransform2DEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInterpolationMode)(::windows::core::Interface::as_raw(self), interpolationmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBorderMode)(::windows::core::Interface::as_raw(self), bordermode).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransformMatrix(&self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformMatrix)(::windows::core::Interface::as_raw(self), transformmatrix).ok()
    }
    pub unsafe fn SetTransformMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetTransformMatrixElement)(::windows::core::Interface::as_raw(self), row, column, animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformMatrixElement2)(::windows::core::Interface::as_raw(self), row, column, value).ok()
    }
    pub unsafe fn SetSharpness<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetSharpness)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetSharpness2(&self, sharpness: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSharpness2)(::windows::core::Interface::as_raw(self), sharpness).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionAffineTransform2DEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionAffineTransform2DEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionAffineTransform2DEffect {}
impl ::core::fmt::Debug for IDCompositionAffineTransform2DEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionAffineTransform2DEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionAffineTransform2DEffect {
    type Vtable = IDCompositionAffineTransform2DEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionAffineTransform2DEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionAffineTransform2DEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b74b9e8_cdd6_492f_bbbc_5ed32157026d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAffineTransform2DEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetInterpolationMode: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
    pub SetTransformMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTransformMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSharpness2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionAnimation(::windows::core::IUnknown);
impl IDCompositionAnimation {
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetAbsoluteBeginTime(&self, begintime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAbsoluteBeginTime)(::windows::core::Interface::as_raw(self), begintime).ok()
    }
    pub unsafe fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddCubic)(::windows::core::Interface::as_raw(self), beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient).ok()
    }
    pub unsafe fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSinusoidal)(::windows::core::Interface::as_raw(self), beginoffset, bias, amplitude, frequency, phase).ok()
    }
    pub unsafe fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRepeat)(::windows::core::Interface::as_raw(self), beginoffset, durationtorepeat).ok()
    }
    pub unsafe fn End(&self, endoffset: f64, endvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End)(::windows::core::Interface::as_raw(self), endoffset, endvalue).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionAnimation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionAnimation {}
impl ::core::fmt::Debug for IDCompositionAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionAnimation {
    type Vtable = IDCompositionAnimation_Vtbl;
}
impl ::core::clone::Clone for IDCompositionAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionAnimation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAnimation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAbsoluteBeginTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows::core::HRESULT,
    pub AddCubic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT,
    pub AddRepeat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionArithmeticCompositeEffect(::windows::core::IUnknown);
impl IDCompositionArithmeticCompositeEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetCoefficients(&self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficients)(::windows::core::Interface::as_raw(self), coefficients).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<P0>(&self, clampoutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetClampOutput)(::windows::core::Interface::as_raw(self), clampoutput.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient1<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCoefficient1)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient12(&self, coeffcient1: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient12)(::windows::core::Interface::as_raw(self), coeffcient1).ok()
    }
    pub unsafe fn SetCoefficient2<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCoefficient2)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient22(&self, coefficient2: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient22)(::windows::core::Interface::as_raw(self), coefficient2).ok()
    }
    pub unsafe fn SetCoefficient3<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCoefficient3)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient32(&self, coefficient3: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient32)(::windows::core::Interface::as_raw(self), coefficient3).ok()
    }
    pub unsafe fn SetCoefficient4<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCoefficient4)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCoefficient42(&self, coefficient4: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient42)(::windows::core::Interface::as_raw(self), coefficient4).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionArithmeticCompositeEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionArithmeticCompositeEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionArithmeticCompositeEffect {}
impl ::core::fmt::Debug for IDCompositionArithmeticCompositeEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionArithmeticCompositeEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionArithmeticCompositeEffect {
    type Vtable = IDCompositionArithmeticCompositeEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionArithmeticCompositeEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionArithmeticCompositeEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b67dfa8_e3dd_4e61_b640_46c2f3d739dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionArithmeticCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCoefficients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCoefficients: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
    pub SetCoefficient1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCoefficient12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows::core::HRESULT,
    pub SetCoefficient2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCoefficient22: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows::core::HRESULT,
    pub SetCoefficient3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCoefficient32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows::core::HRESULT,
    pub SetCoefficient4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCoefficient42: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionBlendEffect(::windows::core::IUnknown);
impl IDCompositionBlendEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionBlendEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionBlendEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionBlendEffect {}
impl ::core::fmt::Debug for IDCompositionBlendEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionBlendEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionBlendEffect {
    type Vtable = IDCompositionBlendEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionBlendEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionBlendEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ecdc0a_578a_4a11_9c14_0cb90517f9c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBlendEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionBrightnessEffect(::windows::core::IUnknown);
impl IDCompositionBrightnessEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetWhitePoint(&self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePoint)(::windows::core::Interface::as_raw(self), whitepoint).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBlackPoint(&self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlackPoint)(::windows::core::Interface::as_raw(self), blackpoint).ok()
    }
    pub unsafe fn SetWhitePointX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetWhitePointX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetWhitePointX2(&self, whitepointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePointX2)(::windows::core::Interface::as_raw(self), whitepointx).ok()
    }
    pub unsafe fn SetWhitePointY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetWhitePointY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetWhitePointY2(&self, whitepointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePointY2)(::windows::core::Interface::as_raw(self), whitepointy).ok()
    }
    pub unsafe fn SetBlackPointX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBlackPointX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlackPointX2(&self, blackpointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlackPointX2)(::windows::core::Interface::as_raw(self), blackpointx).ok()
    }
    pub unsafe fn SetBlackPointY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBlackPointY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlackPointY2(&self, blackpointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlackPointY2)(::windows::core::Interface::as_raw(self), blackpointy).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionBrightnessEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionBrightnessEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionBrightnessEffect {}
impl ::core::fmt::Debug for IDCompositionBrightnessEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionBrightnessEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionBrightnessEffect {
    type Vtable = IDCompositionBrightnessEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionBrightnessEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionBrightnessEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6027496e_cb3a_49ab_934f_d798da4f7da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBrightnessEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetWhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetWhitePoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBlackPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBlackPoint: usize,
    pub SetWhitePointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWhitePointX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows::core::HRESULT,
    pub SetWhitePointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWhitePointY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows::core::HRESULT,
    pub SetBlackPointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBlackPointX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows::core::HRESULT,
    pub SetBlackPointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBlackPointY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionClip(::windows::core::IUnknown);
impl IDCompositionClip {}
::windows::imp::interface_hierarchy!(IDCompositionClip, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionClip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionClip {}
impl ::core::fmt::Debug for IDCompositionClip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionClip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionClip {
    type Vtable = IDCompositionClip_Vtbl;
}
impl ::core::clone::Clone for IDCompositionClip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionClip {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64ac3703_9d3f_45ec_a109_7cac0e7a13a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionClip_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionColorMatrixEffect(::windows::core::IUnknown);
impl IDCompositionColorMatrixEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrix)(::windows::core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetMatrixElement)(::windows::core::Interface::as_raw(self), row, column, animation.into_param().abi()).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixElement2)(::windows::core::Interface::as_raw(self), row, column, value).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<P0>(&self, clamp: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetClampOutput)(::windows::core::Interface::as_raw(self), clamp.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionColorMatrixEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionColorMatrixEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionColorMatrixEffect {}
impl ::core::fmt::Debug for IDCompositionColorMatrixEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionColorMatrixEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionColorMatrixEffect {
    type Vtable = IDCompositionColorMatrixEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionColorMatrixEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionColorMatrixEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1170a22_3ce2_4966_90d4_55408bfc84c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionColorMatrixEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetAlphaMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionCompositeEffect(::windows::core::IUnknown);
impl IDCompositionCompositeEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionCompositeEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionCompositeEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionCompositeEffect {}
impl ::core::fmt::Debug for IDCompositionCompositeEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionCompositeEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionCompositeEffect {
    type Vtable = IDCompositionCompositeEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionCompositeEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionCompositeEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x576616c0_a231_494d_a38d_00fd5ec4db46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDelegatedInkTrail(::windows::core::IUnknown);
impl IDCompositionDelegatedInkTrail {
    pub unsafe fn AddTrailPoints(&self, inkpoints: &[DCompositionInkTrailPoint]) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddTrailPoints)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(inkpoints.as_ptr()), inkpoints.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddTrailPointsWithPrediction(&self, inkpoints: &[DCompositionInkTrailPoint], predictedinkpoints: &[DCompositionInkTrailPoint]) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddTrailPointsWithPrediction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(inkpoints.as_ptr()), inkpoints.len() as _, ::core::mem::transmute(predictedinkpoints.as_ptr()), predictedinkpoints.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveTrailPoints(&self, generationid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveTrailPoints)(::windows::core::Interface::as_raw(self), generationid).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartNewTrail)(::windows::core::Interface::as_raw(self), color).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionDelegatedInkTrail, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionDelegatedInkTrail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDelegatedInkTrail {}
impl ::core::fmt::Debug for IDCompositionDelegatedInkTrail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDelegatedInkTrail").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDelegatedInkTrail {
    type Vtable = IDCompositionDelegatedInkTrail_Vtbl;
}
impl ::core::clone::Clone for IDCompositionDelegatedInkTrail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionDelegatedInkTrail {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2448e9b_547d_4057_8cf5_8144ede1c2da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDelegatedInkTrail_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddTrailPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT,
    pub AddTrailPointsWithPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveTrailPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub StartNewTrail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    StartNewTrail: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDesktopDevice(::windows::core::IUnknown);
impl IDCompositionDesktopDevice {
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.WaitForCommitCompletion)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), statistics).ok()
    }
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionVisual2>();
        (::windows::core::Interface::vtable(self).base__.CreateVisual)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSurfaceFactory<P0>(&self, renderingdevice: P0) -> ::windows::core::Result<IDCompositionSurfaceFactory>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSurfaceFactory>();
        (::windows::core::Interface::vtable(self).base__.CreateSurfaceFactory)(::windows::core::Interface::as_raw(self), renderingdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSurface>();
        (::windows::core::Interface::vtable(self).base__.CreateSurface)(::windows::core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionVirtualSurface>();
        (::windows::core::Interface::vtable(self).base__.CreateVirtualSurface)(::windows::core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTranslateTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateTranslateTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionScaleTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateScaleTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRotateTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateRotateTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSkewTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateSkewTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionMatrixTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateMatrixTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateTransformGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transforms.as_ptr()), transforms.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTranslateTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateTranslateTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionScaleTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateScaleTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRotateTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateRotateTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionMatrixTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateMatrixTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateTransform3DGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transforms3d.as_ptr()), transforms3d.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionEffectGroup>();
        (::windows::core::Interface::vtable(self).base__.CreateEffectGroup)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRectangleClip>();
        (::windows::core::Interface::vtable(self).base__.CreateRectangleClip)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionAnimation>();
        (::windows::core::Interface::vtable(self).base__.CreateAnimation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTargetForHwnd<P0, P1>(&self, hwnd: P0, topmost: P1) -> ::windows::core::Result<IDCompositionTarget>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTarget>();
        (::windows::core::Interface::vtable(self).CreateTargetForHwnd)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), topmost.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHandle<P0>(&self, handle: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateSurfaceFromHandle)(::windows::core::Interface::as_raw(self), handle.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHwnd<P0>(&self, hwnd: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateSurfaceFromHwnd)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDCompositionDesktopDevice, ::windows::core::IUnknown, IDCompositionDevice2);
impl ::core::cmp::PartialEq for IDCompositionDesktopDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDesktopDevice {}
impl ::core::fmt::Debug for IDCompositionDesktopDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDesktopDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDesktopDevice {
    type Vtable = IDCompositionDesktopDevice_Vtbl;
}
impl ::core::clone::Clone for IDCompositionDesktopDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionDesktopDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f4633fe_1e08_4cb8_8c75_ce24333f5602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDesktopDevice_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetForHwnd: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHwnd: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDevice(::windows::core::IUnknown);
impl IDCompositionDevice {
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForCommitCompletion)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameStatistics)(::windows::core::Interface::as_raw(self), statistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTargetForHwnd<P0, P1>(&self, hwnd: P0, topmost: P1) -> ::windows::core::Result<IDCompositionTarget>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTarget>();
        (::windows::core::Interface::vtable(self).CreateTargetForHwnd)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), topmost.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionVisual>();
        (::windows::core::Interface::vtable(self).CreateVisual)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSurface>();
        (::windows::core::Interface::vtable(self).CreateSurface)(::windows::core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionVirtualSurface>();
        (::windows::core::Interface::vtable(self).CreateVirtualSurface)(::windows::core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHandle<P0>(&self, handle: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateSurfaceFromHandle)(::windows::core::Interface::as_raw(self), handle.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHwnd<P0>(&self, hwnd: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateSurfaceFromHwnd)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTranslateTransform>();
        (::windows::core::Interface::vtable(self).CreateTranslateTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionScaleTransform>();
        (::windows::core::Interface::vtable(self).CreateScaleTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRotateTransform>();
        (::windows::core::Interface::vtable(self).CreateRotateTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSkewTransform>();
        (::windows::core::Interface::vtable(self).CreateSkewTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionMatrixTransform>();
        (::windows::core::Interface::vtable(self).CreateMatrixTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTransform>();
        (::windows::core::Interface::vtable(self).CreateTransformGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transforms.as_ptr()), transforms.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTranslateTransform3D>();
        (::windows::core::Interface::vtable(self).CreateTranslateTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionScaleTransform3D>();
        (::windows::core::Interface::vtable(self).CreateScaleTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRotateTransform3D>();
        (::windows::core::Interface::vtable(self).CreateRotateTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionMatrixTransform3D>();
        (::windows::core::Interface::vtable(self).CreateMatrixTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTransform3D>();
        (::windows::core::Interface::vtable(self).CreateTransform3DGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transforms3d.as_ptr()), transforms3d.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionEffectGroup>();
        (::windows::core::Interface::vtable(self).CreateEffectGroup)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRectangleClip>();
        (::windows::core::Interface::vtable(self).CreateRectangleClip)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionAnimation>();
        (::windows::core::Interface::vtable(self).CreateAnimation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CheckDeviceState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDCompositionDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDevice {}
impl ::core::fmt::Debug for IDCompositionDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDevice {
    type Vtable = IDCompositionDevice_Vtbl;
}
impl ::core::clone::Clone for IDCompositionDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc37ea93a_e7aa_450d_b16f_9746cb0407f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetForHwnd: usize,
    pub CreateVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHwnd: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skewtransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms: *const *mut ::core::ffi::c_void, elements: u32, transformgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms3d: *const *mut ::core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckDeviceState: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDevice2(::windows::core::IUnknown);
impl IDCompositionDevice2 {
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForCommitCompletion)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameStatistics)(::windows::core::Interface::as_raw(self), statistics).ok()
    }
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionVisual2>();
        (::windows::core::Interface::vtable(self).CreateVisual)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSurfaceFactory<P0>(&self, renderingdevice: P0) -> ::windows::core::Result<IDCompositionSurfaceFactory>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSurfaceFactory>();
        (::windows::core::Interface::vtable(self).CreateSurfaceFactory)(::windows::core::Interface::as_raw(self), renderingdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSurface>();
        (::windows::core::Interface::vtable(self).CreateSurface)(::windows::core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionVirtualSurface>();
        (::windows::core::Interface::vtable(self).CreateVirtualSurface)(::windows::core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTranslateTransform>();
        (::windows::core::Interface::vtable(self).CreateTranslateTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionScaleTransform>();
        (::windows::core::Interface::vtable(self).CreateScaleTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRotateTransform>();
        (::windows::core::Interface::vtable(self).CreateRotateTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSkewTransform>();
        (::windows::core::Interface::vtable(self).CreateSkewTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionMatrixTransform>();
        (::windows::core::Interface::vtable(self).CreateMatrixTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTransform>();
        (::windows::core::Interface::vtable(self).CreateTransformGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transforms.as_ptr()), transforms.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTranslateTransform3D>();
        (::windows::core::Interface::vtable(self).CreateTranslateTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionScaleTransform3D>();
        (::windows::core::Interface::vtable(self).CreateScaleTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRotateTransform3D>();
        (::windows::core::Interface::vtable(self).CreateRotateTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionMatrixTransform3D>();
        (::windows::core::Interface::vtable(self).CreateMatrixTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTransform3D>();
        (::windows::core::Interface::vtable(self).CreateTransform3DGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transforms3d.as_ptr()), transforms3d.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionEffectGroup>();
        (::windows::core::Interface::vtable(self).CreateEffectGroup)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRectangleClip>();
        (::windows::core::Interface::vtable(self).CreateRectangleClip)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionAnimation>();
        (::windows::core::Interface::vtable(self).CreateAnimation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDCompositionDevice2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDevice2 {}
impl ::core::fmt::Debug for IDCompositionDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDevice2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDevice2 {
    type Vtable = IDCompositionDevice2_Vtbl;
}
impl ::core::clone::Clone for IDCompositionDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionDevice2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75f6468d_1b8e_447c_9bc6_75fea80b5b25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    pub CreateVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSurfaceFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skewtransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms: *const *mut ::core::ffi::c_void, elements: u32, transformgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms3d: *const *mut ::core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDevice3(::windows::core::IUnknown);
impl IDCompositionDevice3 {
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.WaitForCommitCompletion)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFrameStatistics)(::windows::core::Interface::as_raw(self), statistics).ok()
    }
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionVisual2>();
        (::windows::core::Interface::vtable(self).base__.CreateVisual)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSurfaceFactory<P0>(&self, renderingdevice: P0) -> ::windows::core::Result<IDCompositionSurfaceFactory>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSurfaceFactory>();
        (::windows::core::Interface::vtable(self).base__.CreateSurfaceFactory)(::windows::core::Interface::as_raw(self), renderingdevice.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSurface>();
        (::windows::core::Interface::vtable(self).base__.CreateSurface)(::windows::core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionVirtualSurface>();
        (::windows::core::Interface::vtable(self).base__.CreateVirtualSurface)(::windows::core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTranslateTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateTranslateTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionScaleTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateScaleTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRotateTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateRotateTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSkewTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateSkewTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionMatrixTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateMatrixTransform)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateTransformGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transforms.as_ptr()), transforms.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTranslateTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateTranslateTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionScaleTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateScaleTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRotateTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateRotateTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionMatrixTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateMatrixTransform3D)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTransform3D>();
        (::windows::core::Interface::vtable(self).base__.CreateTransform3DGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(transforms3d.as_ptr()), transforms3d.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionEffectGroup>();
        (::windows::core::Interface::vtable(self).base__.CreateEffectGroup)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionRectangleClip>();
        (::windows::core::Interface::vtable(self).base__.CreateRectangleClip)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionAnimation>();
        (::windows::core::Interface::vtable(self).base__.CreateAnimation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGaussianBlurEffect(&self) -> ::windows::core::Result<IDCompositionGaussianBlurEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionGaussianBlurEffect>();
        (::windows::core::Interface::vtable(self).CreateGaussianBlurEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBrightnessEffect(&self) -> ::windows::core::Result<IDCompositionBrightnessEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionBrightnessEffect>();
        (::windows::core::Interface::vtable(self).CreateBrightnessEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateColorMatrixEffect(&self) -> ::windows::core::Result<IDCompositionColorMatrixEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionColorMatrixEffect>();
        (::windows::core::Interface::vtable(self).CreateColorMatrixEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateShadowEffect(&self) -> ::windows::core::Result<IDCompositionShadowEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionShadowEffect>();
        (::windows::core::Interface::vtable(self).CreateShadowEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateHueRotationEffect(&self) -> ::windows::core::Result<IDCompositionHueRotationEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionHueRotationEffect>();
        (::windows::core::Interface::vtable(self).CreateHueRotationEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSaturationEffect(&self) -> ::windows::core::Result<IDCompositionSaturationEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSaturationEffect>();
        (::windows::core::Interface::vtable(self).CreateSaturationEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTurbulenceEffect(&self) -> ::windows::core::Result<IDCompositionTurbulenceEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTurbulenceEffect>();
        (::windows::core::Interface::vtable(self).CreateTurbulenceEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransferEffect(&self) -> ::windows::core::Result<IDCompositionLinearTransferEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionLinearTransferEffect>();
        (::windows::core::Interface::vtable(self).CreateLinearTransferEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTableTransferEffect(&self) -> ::windows::core::Result<IDCompositionTableTransferEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionTableTransferEffect>();
        (::windows::core::Interface::vtable(self).CreateTableTransferEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateCompositeEffect(&self) -> ::windows::core::Result<IDCompositionCompositeEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionCompositeEffect>();
        (::windows::core::Interface::vtable(self).CreateCompositeEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBlendEffect(&self) -> ::windows::core::Result<IDCompositionBlendEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionBlendEffect>();
        (::windows::core::Interface::vtable(self).CreateBlendEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateArithmeticCompositeEffect(&self) -> ::windows::core::Result<IDCompositionArithmeticCompositeEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionArithmeticCompositeEffect>();
        (::windows::core::Interface::vtable(self).CreateArithmeticCompositeEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateAffineTransform2DEffect(&self) -> ::windows::core::Result<IDCompositionAffineTransform2DEffect> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionAffineTransform2DEffect>();
        (::windows::core::Interface::vtable(self).CreateAffineTransform2DEffect)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDCompositionDevice3, ::windows::core::IUnknown, IDCompositionDevice2);
impl ::core::cmp::PartialEq for IDCompositionDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDevice3 {}
impl ::core::fmt::Debug for IDCompositionDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDevice3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDevice3 {
    type Vtable = IDCompositionDevice3_Vtbl;
}
impl ::core::clone::Clone for IDCompositionDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionDevice3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0987cb06_f916_48bf_8d35_ce7641781bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice3_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    pub CreateGaussianBlurEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBrightnessEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brightnesseffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateColorMatrixEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateShadowEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shadoweffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateHueRotationEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, huerotationeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSaturationEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, saturationeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTurbulenceEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLinearTransferEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTableTransferEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCompositeEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositeeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBlendEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blendeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateArithmeticCompositeEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateAffineTransform2DEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDeviceDebug(::windows::core::IUnknown);
impl IDCompositionDeviceDebug {
    pub unsafe fn EnableDebugCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableDebugCounters)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableDebugCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableDebugCounters)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionDeviceDebug, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionDeviceDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDeviceDebug {}
impl ::core::fmt::Debug for IDCompositionDeviceDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDeviceDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDeviceDebug {
    type Vtable = IDCompositionDeviceDebug_Vtbl;
}
impl ::core::clone::Clone for IDCompositionDeviceDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionDeviceDebug {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1a3c64a_224f_4a81_9773_4f03a89d3c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDeviceDebug_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnableDebugCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableDebugCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionEffect(::windows::core::IUnknown);
impl IDCompositionEffect {}
::windows::imp::interface_hierarchy!(IDCompositionEffect, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionEffect {}
impl ::core::fmt::Debug for IDCompositionEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionEffect {
    type Vtable = IDCompositionEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec81b08f_bfcb_4e8d_b193_a915587999e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffect_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionEffectGroup(::windows::core::IUnknown);
impl IDCompositionEffectGroup {
    pub unsafe fn SetOpacity<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOpacity)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacity2)(::windows::core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn SetTransform3D<P0>(&self, transform3d: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionTransform3D>,
    {
        (::windows::core::Interface::vtable(self).SetTransform3D)(::windows::core::Interface::as_raw(self), transform3d.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionEffectGroup, ::windows::core::IUnknown, IDCompositionEffect);
impl ::core::cmp::PartialEq for IDCompositionEffectGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionEffectGroup {}
impl ::core::fmt::Debug for IDCompositionEffectGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionEffectGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionEffectGroup {
    type Vtable = IDCompositionEffectGroup_Vtbl;
}
impl ::core::clone::Clone for IDCompositionEffectGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionEffectGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7929a74_e6b2_4bd6_8b95_4040119ca34d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffectGroup_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub SetTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform3d: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionFilterEffect(::windows::core::IUnknown);
impl IDCompositionFilterEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionFilterEffect, ::windows::core::IUnknown, IDCompositionEffect);
impl ::core::cmp::PartialEq for IDCompositionFilterEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionFilterEffect {}
impl ::core::fmt::Debug for IDCompositionFilterEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionFilterEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionFilterEffect {
    type Vtable = IDCompositionFilterEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionFilterEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionFilterEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30c421d5_8cb2_4e9f_b133_37be270d4ac2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionFilterEffect_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionGaussianBlurEffect(::windows::core::IUnknown);
impl IDCompositionGaussianBlurEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    pub unsafe fn SetStandardDeviation<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetStandardDeviation)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStandardDeviation2)(::windows::core::Interface::as_raw(self), amount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBorderMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionGaussianBlurEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionGaussianBlurEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionGaussianBlurEffect {}
impl ::core::fmt::Debug for IDCompositionGaussianBlurEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionGaussianBlurEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionGaussianBlurEffect {
    type Vtable = IDCompositionGaussianBlurEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionGaussianBlurEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionGaussianBlurEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d4d0b7_1bd4_454e_8894_2bfa68443033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionGaussianBlurEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionHueRotationEffect(::windows::core::IUnknown);
impl IDCompositionHueRotationEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    pub unsafe fn SetAngle<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAngle)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngle2(&self, amountdegrees: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngle2)(::windows::core::Interface::as_raw(self), amountdegrees).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionHueRotationEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionHueRotationEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionHueRotationEffect {}
impl ::core::fmt::Debug for IDCompositionHueRotationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionHueRotationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionHueRotationEffect {
    type Vtable = IDCompositionHueRotationEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionHueRotationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionHueRotationEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6db9f920_0770_4781_b0c6_381912f9d167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionHueRotationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionInkTrailDevice(::windows::core::IUnknown);
impl IDCompositionInkTrailDevice {
    pub unsafe fn CreateDelegatedInkTrail(&self) -> ::windows::core::Result<IDCompositionDelegatedInkTrail> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionDelegatedInkTrail>();
        (::windows::core::Interface::vtable(self).CreateDelegatedInkTrail)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDelegatedInkTrailForSwapChain<P0>(&self, swapchain: P0) -> ::windows::core::Result<IDCompositionDelegatedInkTrail>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<IDCompositionDelegatedInkTrail>();
        (::windows::core::Interface::vtable(self).CreateDelegatedInkTrailForSwapChain)(::windows::core::Interface::as_raw(self), swapchain.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDCompositionInkTrailDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionInkTrailDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionInkTrailDevice {}
impl ::core::fmt::Debug for IDCompositionInkTrailDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionInkTrailDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionInkTrailDevice {
    type Vtable = IDCompositionInkTrailDevice_Vtbl;
}
impl ::core::clone::Clone for IDCompositionInkTrailDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionInkTrailDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0c7cec_cdeb_4d4a_b91c_721bf22f4e6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionInkTrailDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateDelegatedInkTrail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inktrail: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDelegatedInkTrailForSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionLinearTransferEffect(::windows::core::IUnknown);
impl IDCompositionLinearTransferEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    pub unsafe fn SetRedYIntercept<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetRedYIntercept)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRedYIntercept2(&self, redyintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedYIntercept2)(::windows::core::Interface::as_raw(self), redyintercept).ok()
    }
    pub unsafe fn SetRedSlope<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetRedSlope)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRedSlope2(&self, redslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedSlope2)(::windows::core::Interface::as_raw(self), redslope).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRedDisable<P0>(&self, reddisable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetRedDisable)(::windows::core::Interface::as_raw(self), reddisable.into_param().abi()).ok()
    }
    pub unsafe fn SetGreenYIntercept<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetGreenYIntercept)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetGreenYIntercept2(&self, greenyintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenYIntercept2)(::windows::core::Interface::as_raw(self), greenyintercept).ok()
    }
    pub unsafe fn SetGreenSlope<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetGreenSlope)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetGreenSlope2(&self, greenslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenSlope2)(::windows::core::Interface::as_raw(self), greenslope).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGreenDisable<P0>(&self, greendisable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetGreenDisable)(::windows::core::Interface::as_raw(self), greendisable.into_param().abi()).ok()
    }
    pub unsafe fn SetBlueYIntercept<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBlueYIntercept)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlueYIntercept2(&self, blueyintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueYIntercept2)(::windows::core::Interface::as_raw(self), blueyintercept).ok()
    }
    pub unsafe fn SetBlueSlope<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBlueSlope)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlueSlope2(&self, blueslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueSlope2)(::windows::core::Interface::as_raw(self), blueslope).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlueDisable<P0>(&self, bluedisable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBlueDisable)(::windows::core::Interface::as_raw(self), bluedisable.into_param().abi()).ok()
    }
    pub unsafe fn SetAlphaYIntercept<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAlphaYIntercept)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaYIntercept2)(::windows::core::Interface::as_raw(self), alphayintercept).ok()
    }
    pub unsafe fn SetAlphaSlope<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAlphaSlope)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAlphaSlope2(&self, alphaslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaSlope2)(::windows::core::Interface::as_raw(self), alphaslope).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlphaDisable<P0>(&self, alphadisable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAlphaDisable)(::windows::core::Interface::as_raw(self), alphadisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<P0>(&self, clampoutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetClampOutput)(::windows::core::Interface::as_raw(self), clampoutput.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionLinearTransferEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionLinearTransferEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionLinearTransferEffect {}
impl ::core::fmt::Debug for IDCompositionLinearTransferEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionLinearTransferEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionLinearTransferEffect {
    type Vtable = IDCompositionLinearTransferEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionLinearTransferEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionLinearTransferEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4305ee5b_c4a0_4c88_9385_67124e017683);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionLinearTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRedYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows::core::HRESULT,
    pub SetRedSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRedSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRedDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRedDisable: usize,
    pub SetGreenYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGreenYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows::core::HRESULT,
    pub SetGreenSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGreenSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGreenDisable: usize,
    pub SetBlueYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBlueYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows::core::HRESULT,
    pub SetBlueSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBlueSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlueDisable: usize,
    pub SetAlphaYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAlphaYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows::core::HRESULT,
    pub SetAlphaSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAlphaSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphaDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionMatrixTransform(::windows::core::IUnknown);
impl IDCompositionMatrixTransform {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrix)(::windows::core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetMatrixElement)(::windows::core::Interface::as_raw(self), row, column, animation.into_param().abi()).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixElement2)(::windows::core::Interface::as_raw(self), row, column, value).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionMatrixTransform, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl ::core::cmp::PartialEq for IDCompositionMatrixTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionMatrixTransform {}
impl ::core::fmt::Debug for IDCompositionMatrixTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionMatrixTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionMatrixTransform {
    type Vtable = IDCompositionMatrixTransform_Vtbl;
}
impl ::core::clone::Clone for IDCompositionMatrixTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionMatrixTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16cdff07_c503_419c_83f2_0965c7af1fa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionMatrixTransform3D(::windows::core::IUnknown);
impl IDCompositionMatrixTransform3D {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrix)(::windows::core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetMatrixElement)(::windows::core::Interface::as_raw(self), row, column, animation.into_param().abi()).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixElement2)(::windows::core::Interface::as_raw(self), row, column, value).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionMatrixTransform3D, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl ::core::cmp::PartialEq for IDCompositionMatrixTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionMatrixTransform3D {}
impl ::core::fmt::Debug for IDCompositionMatrixTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionMatrixTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionMatrixTransform3D {
    type Vtable = IDCompositionMatrixTransform3D_Vtbl;
}
impl ::core::clone::Clone for IDCompositionMatrixTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionMatrixTransform3D {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b3363f0_643b_41b7_b6e0_ccf22d34467c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionRectangleClip(::windows::core::IUnknown);
impl IDCompositionRectangleClip {
    pub unsafe fn SetLeft<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetLeft)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetLeft2(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLeft2)(::windows::core::Interface::as_raw(self), left).ok()
    }
    pub unsafe fn SetTop<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetTop)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTop2(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTop2)(::windows::core::Interface::as_raw(self), top).ok()
    }
    pub unsafe fn SetRight<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetRight)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRight2(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRight2)(::windows::core::Interface::as_raw(self), right).ok()
    }
    pub unsafe fn SetBottom<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBottom)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottom2(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottom2)(::windows::core::Interface::as_raw(self), bottom).ok()
    }
    pub unsafe fn SetTopLeftRadiusX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetTopLeftRadiusX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTopLeftRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopLeftRadiusX2)(::windows::core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetTopLeftRadiusY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetTopLeftRadiusY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTopLeftRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopLeftRadiusY2)(::windows::core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetTopRightRadiusX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetTopRightRadiusX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTopRightRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopRightRadiusX2)(::windows::core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetTopRightRadiusY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetTopRightRadiusY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetTopRightRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopRightRadiusY2)(::windows::core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetBottomLeftRadiusX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBottomLeftRadiusX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottomLeftRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomLeftRadiusX2)(::windows::core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetBottomLeftRadiusY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBottomLeftRadiusY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottomLeftRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomLeftRadiusY2)(::windows::core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetBottomRightRadiusX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBottomRightRadiusX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottomRightRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomRightRadiusX2)(::windows::core::Interface::as_raw(self), radius).ok()
    }
    pub unsafe fn SetBottomRightRadiusY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBottomRightRadiusY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBottomRightRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomRightRadiusY2)(::windows::core::Interface::as_raw(self), radius).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionRectangleClip, ::windows::core::IUnknown, IDCompositionClip);
impl ::core::cmp::PartialEq for IDCompositionRectangleClip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionRectangleClip {}
impl ::core::fmt::Debug for IDCompositionRectangleClip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionRectangleClip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionRectangleClip {
    type Vtable = IDCompositionRectangleClip_Vtbl;
}
impl ::core::clone::Clone for IDCompositionRectangleClip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionRectangleClip {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9842ad7d_d9cf_4908_aed7_48b51da5e7c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRectangleClip_Vtbl {
    pub base__: IDCompositionClip_Vtbl,
    pub SetLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLeft2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTop2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRight2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBottom2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT,
    pub SetTopLeftRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTopLeftRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetTopLeftRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTopLeftRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetTopRightRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTopRightRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetTopRightRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTopRightRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetBottomLeftRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBottomLeftRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetBottomLeftRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBottomLeftRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetBottomRightRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBottomRightRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetBottomRightRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBottomRightRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionRotateTransform(::windows::core::IUnknown);
impl IDCompositionRotateTransform {
    pub unsafe fn SetAngle<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAngle)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngle2)(::windows::core::Interface::as_raw(self), angle).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::windows::core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::windows::core::Interface::as_raw(self), centery).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionRotateTransform, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl ::core::cmp::PartialEq for IDCompositionRotateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionRotateTransform {}
impl ::core::fmt::Debug for IDCompositionRotateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionRotateTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionRotateTransform {
    type Vtable = IDCompositionRotateTransform_Vtbl;
}
impl ::core::clone::Clone for IDCompositionRotateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionRotateTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x641ed83c_ae96_46c5_90dc_32774cc5c6d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionRotateTransform3D(::windows::core::IUnknown);
impl IDCompositionRotateTransform3D {
    pub unsafe fn SetAngle<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAngle)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngle2)(::windows::core::Interface::as_raw(self), angle).ok()
    }
    pub unsafe fn SetAxisX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAxisX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAxisX2(&self, axisx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAxisX2)(::windows::core::Interface::as_raw(self), axisx).ok()
    }
    pub unsafe fn SetAxisY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAxisY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAxisY2(&self, axisy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAxisY2)(::windows::core::Interface::as_raw(self), axisy).ok()
    }
    pub unsafe fn SetAxisZ<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAxisZ)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAxisZ2(&self, axisz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAxisZ2)(::windows::core::Interface::as_raw(self), axisz).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::windows::core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::windows::core::Interface::as_raw(self), centery).ok()
    }
    pub unsafe fn SetCenterZ<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterZ)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterZ2)(::windows::core::Interface::as_raw(self), centerz).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionRotateTransform3D, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl ::core::cmp::PartialEq for IDCompositionRotateTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionRotateTransform3D {}
impl ::core::fmt::Debug for IDCompositionRotateTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionRotateTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionRotateTransform3D {
    type Vtable = IDCompositionRotateTransform3D_Vtbl;
}
impl ::core::clone::Clone for IDCompositionRotateTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionRotateTransform3D {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f5b23f_d429_4a91_b55a_d2f45fd75b18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT,
    pub SetAxisX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAxisX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows::core::HRESULT,
    pub SetAxisY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAxisY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows::core::HRESULT,
    pub SetAxisZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAxisZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionSaturationEffect(::windows::core::IUnknown);
impl IDCompositionSaturationEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    pub unsafe fn SetSaturation<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetSaturation)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetSaturation2(&self, ratio: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSaturation2)(::windows::core::Interface::as_raw(self), ratio).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionSaturationEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionSaturationEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSaturationEffect {}
impl ::core::fmt::Debug for IDCompositionSaturationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSaturationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSaturationEffect {
    type Vtable = IDCompositionSaturationEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionSaturationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionSaturationEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa08debda_3258_4fa4_9f16_9174d3fe93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSaturationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSaturation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionScaleTransform(::windows::core::IUnknown);
impl IDCompositionScaleTransform {
    pub unsafe fn SetScaleX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetScaleX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleX2)(::windows::core::Interface::as_raw(self), scalex).ok()
    }
    pub unsafe fn SetScaleY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetScaleY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleY2)(::windows::core::Interface::as_raw(self), scaley).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::windows::core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::windows::core::Interface::as_raw(self), centery).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionScaleTransform, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl ::core::cmp::PartialEq for IDCompositionScaleTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionScaleTransform {}
impl ::core::fmt::Debug for IDCompositionScaleTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionScaleTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionScaleTransform {
    type Vtable = IDCompositionScaleTransform_Vtbl;
}
impl ::core::clone::Clone for IDCompositionScaleTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionScaleTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71fde914_40ef_45ef_bd51_68b037c339f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionScaleTransform3D(::windows::core::IUnknown);
impl IDCompositionScaleTransform3D {
    pub unsafe fn SetScaleX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetScaleX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleX2)(::windows::core::Interface::as_raw(self), scalex).ok()
    }
    pub unsafe fn SetScaleY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetScaleY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleY2)(::windows::core::Interface::as_raw(self), scaley).ok()
    }
    pub unsafe fn SetScaleZ<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetScaleZ)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetScaleZ2(&self, scalez: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleZ2)(::windows::core::Interface::as_raw(self), scalez).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::windows::core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::windows::core::Interface::as_raw(self), centery).ok()
    }
    pub unsafe fn SetCenterZ<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterZ)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterZ2)(::windows::core::Interface::as_raw(self), centerz).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionScaleTransform3D, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl ::core::cmp::PartialEq for IDCompositionScaleTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionScaleTransform3D {}
impl ::core::fmt::Debug for IDCompositionScaleTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionScaleTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionScaleTransform3D {
    type Vtable = IDCompositionScaleTransform3D_Vtbl;
}
impl ::core::clone::Clone for IDCompositionScaleTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionScaleTransform3D {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a9e9ead_364b_4b15_a7c4_a1997f78b389);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT,
    pub SetScaleZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScaleZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionShadowEffect(::windows::core::IUnknown);
impl IDCompositionShadowEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    pub unsafe fn SetStandardDeviation<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetStandardDeviation)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStandardDeviation2)(::windows::core::Interface::as_raw(self), amount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetColor(&self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColor)(::windows::core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn SetRed<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetRed)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRed2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRed2)(::windows::core::Interface::as_raw(self), amount).ok()
    }
    pub unsafe fn SetGreen<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetGreen)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetGreen2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreen2)(::windows::core::Interface::as_raw(self), amount).ok()
    }
    pub unsafe fn SetBlue<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBlue)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlue2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlue2)(::windows::core::Interface::as_raw(self), amount).ok()
    }
    pub unsafe fn SetAlpha<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAlpha)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAlpha2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlpha2)(::windows::core::Interface::as_raw(self), amount).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionShadowEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionShadowEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionShadowEffect {}
impl ::core::fmt::Debug for IDCompositionShadowEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionShadowEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionShadowEffect {
    type Vtable = IDCompositionShadowEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionShadowEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionShadowEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ad18ac0_cfd2_4c2f_bb62_96e54fdb6879);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionShadowEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    pub SetRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRed2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    pub SetGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGreen2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    pub SetBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBlue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    pub SetAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAlpha2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionSkewTransform(::windows::core::IUnknown);
impl IDCompositionSkewTransform {
    pub unsafe fn SetAngleX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAngleX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngleX2(&self, anglex: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngleX2)(::windows::core::Interface::as_raw(self), anglex).ok()
    }
    pub unsafe fn SetAngleY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAngleY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAngleY2(&self, angley: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngleY2)(::windows::core::Interface::as_raw(self), angley).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::windows::core::Interface::as_raw(self), centerx).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetCenterY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::windows::core::Interface::as_raw(self), centery).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionSkewTransform, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl ::core::cmp::PartialEq for IDCompositionSkewTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSkewTransform {}
impl ::core::fmt::Debug for IDCompositionSkewTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSkewTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSkewTransform {
    type Vtable = IDCompositionSkewTransform_Vtbl;
}
impl ::core::clone::Clone for IDCompositionSkewTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionSkewTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe57aa735_dcdb_4c72_9c61_0591f58889ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSkewTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetAngleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAngleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows::core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAngleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionSurface(::windows::core::IUnknown);
impl IDCompositionSurface {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw<T>(&self, updaterect: ::core::option::Option<*const super::super::Foundation::RECT>, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).BeginDraw)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(updaterect.unwrap_or(::std::ptr::null())), &<T as ::windows::core::ComInterface>::IID, &mut result__, updateoffset).from_abi(result__)
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspendDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResumeDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: ::core::option::Option<*const super::super::Foundation::RECT>, cliprect: ::core::option::Option<*const super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Scroll)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scrollrect.unwrap_or(::std::ptr::null())), ::core::mem::transmute(cliprect.unwrap_or(::std::ptr::null())), offsetx, offsety).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionSurface, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSurface {}
impl ::core::fmt::Debug for IDCompositionSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSurface {
    type Vtable = IDCompositionSurface_Vtbl;
}
impl ::core::clone::Clone for IDCompositionSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionSurface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb8a4953_2c99_4f5a_96f5_4819027fa3ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurface_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scroll: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionSurfaceFactory(::windows::core::IUnknown);
impl IDCompositionSurfaceFactory {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionSurface>();
        (::windows::core::Interface::vtable(self).CreateSurface)(::windows::core::Interface::as_raw(self), width, height, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::windows::core::zeroed::<IDCompositionVirtualSurface>();
        (::windows::core::Interface::vtable(self).CreateVirtualSurface)(::windows::core::Interface::as_raw(self), initialwidth, initialheight, pixelformat, alphamode, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDCompositionSurfaceFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionSurfaceFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSurfaceFactory {}
impl ::core::fmt::Debug for IDCompositionSurfaceFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSurfaceFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSurfaceFactory {
    type Vtable = IDCompositionSurfaceFactory_Vtbl;
}
impl ::core::clone::Clone for IDCompositionSurfaceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionSurfaceFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe334bc12_3937_4e02_85eb_fcf4eb30d2c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurfaceFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTableTransferEffect(::windows::core::IUnknown);
impl IDCompositionTableTransferEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    pub unsafe fn SetRedTable(&self, tablevalues: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedTable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(tablevalues.as_ptr()), tablevalues.len() as _).ok()
    }
    pub unsafe fn SetGreenTable(&self, tablevalues: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenTable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(tablevalues.as_ptr()), tablevalues.len() as _).ok()
    }
    pub unsafe fn SetBlueTable(&self, tablevalues: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueTable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(tablevalues.as_ptr()), tablevalues.len() as _).ok()
    }
    pub unsafe fn SetAlphaTable(&self, tablevalues: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaTable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(tablevalues.as_ptr()), tablevalues.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRedDisable<P0>(&self, reddisable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetRedDisable)(::windows::core::Interface::as_raw(self), reddisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGreenDisable<P0>(&self, greendisable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetGreenDisable)(::windows::core::Interface::as_raw(self), greendisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlueDisable<P0>(&self, bluedisable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBlueDisable)(::windows::core::Interface::as_raw(self), bluedisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlphaDisable<P0>(&self, alphadisable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAlphaDisable)(::windows::core::Interface::as_raw(self), alphadisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<P0>(&self, clampoutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetClampOutput)(::windows::core::Interface::as_raw(self), clampoutput.into_param().abi()).ok()
    }
    pub unsafe fn SetRedTableValue<P0>(&self, index: u32, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetRedTableValue)(::windows::core::Interface::as_raw(self), index, animation.into_param().abi()).ok()
    }
    pub unsafe fn SetRedTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedTableValue2)(::windows::core::Interface::as_raw(self), index, value).ok()
    }
    pub unsafe fn SetGreenTableValue<P0>(&self, index: u32, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetGreenTableValue)(::windows::core::Interface::as_raw(self), index, animation.into_param().abi()).ok()
    }
    pub unsafe fn SetGreenTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenTableValue2)(::windows::core::Interface::as_raw(self), index, value).ok()
    }
    pub unsafe fn SetBlueTableValue<P0>(&self, index: u32, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetBlueTableValue)(::windows::core::Interface::as_raw(self), index, animation.into_param().abi()).ok()
    }
    pub unsafe fn SetBlueTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueTableValue2)(::windows::core::Interface::as_raw(self), index, value).ok()
    }
    pub unsafe fn SetAlphaTableValue<P0>(&self, index: u32, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetAlphaTableValue)(::windows::core::Interface::as_raw(self), index, animation.into_param().abi()).ok()
    }
    pub unsafe fn SetAlphaTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaTableValue2)(::windows::core::Interface::as_raw(self), index, value).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionTableTransferEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionTableTransferEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTableTransferEffect {}
impl ::core::fmt::Debug for IDCompositionTableTransferEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTableTransferEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTableTransferEffect {
    type Vtable = IDCompositionTableTransferEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionTableTransferEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionTableTransferEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e82e2_69c5_4eb4_a5f5_a7033f5132cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTableTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    pub SetGreenTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    pub SetBlueTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    pub SetAlphaTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRedDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRedDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGreenDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlueDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphaDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
    pub SetRedTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRedTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT,
    pub SetGreenTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGreenTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT,
    pub SetBlueTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBlueTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT,
    pub SetAlphaTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAlphaTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTarget(::windows::core::IUnknown);
impl IDCompositionTarget {
    pub unsafe fn SetRoot<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).SetRoot)(::windows::core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionTarget, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTarget {}
impl ::core::fmt::Debug for IDCompositionTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTarget {
    type Vtable = IDCompositionTarget_Vtbl;
}
impl ::core::clone::Clone for IDCompositionTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeacdd04c_117e_4e17_88f4_d1b12b0e3d89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTransform(::windows::core::IUnknown);
impl IDCompositionTransform {}
::windows::imp::interface_hierarchy!(IDCompositionTransform, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl ::core::cmp::PartialEq for IDCompositionTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTransform {}
impl ::core::fmt::Debug for IDCompositionTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTransform {
    type Vtable = IDCompositionTransform_Vtbl;
}
impl ::core::clone::Clone for IDCompositionTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd55faa7_37e0_4c20_95d2_9be45bc33f55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTransform3D(::windows::core::IUnknown);
impl IDCompositionTransform3D {}
::windows::imp::interface_hierarchy!(IDCompositionTransform3D, ::windows::core::IUnknown, IDCompositionEffect);
impl ::core::cmp::PartialEq for IDCompositionTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTransform3D {}
impl ::core::fmt::Debug for IDCompositionTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTransform3D {
    type Vtable = IDCompositionTransform3D_Vtbl;
}
impl ::core::clone::Clone for IDCompositionTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionTransform3D {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71185722_246b_41f2_aad1_0443f7f4bfc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform3D_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTranslateTransform(::windows::core::IUnknown);
impl IDCompositionTranslateTransform {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOffsetX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetX2)(::windows::core::Interface::as_raw(self), offsetx).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOffsetY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetY2)(::windows::core::Interface::as_raw(self), offsety).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionTranslateTransform, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
impl ::core::cmp::PartialEq for IDCompositionTranslateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTranslateTransform {}
impl ::core::fmt::Debug for IDCompositionTranslateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTranslateTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTranslateTransform {
    type Vtable = IDCompositionTranslateTransform_Vtbl;
}
impl ::core::clone::Clone for IDCompositionTranslateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionTranslateTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06791122_c6f0_417d_8323_269e987f5954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTranslateTransform3D(::windows::core::IUnknown);
impl IDCompositionTranslateTransform3D {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOffsetX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetX2)(::windows::core::Interface::as_raw(self), offsetx).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOffsetY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetY2)(::windows::core::Interface::as_raw(self), offsety).ok()
    }
    pub unsafe fn SetOffsetZ<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOffsetZ)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetZ2)(::windows::core::Interface::as_raw(self), offsetz).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionTranslateTransform3D, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
impl ::core::cmp::PartialEq for IDCompositionTranslateTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTranslateTransform3D {}
impl ::core::fmt::Debug for IDCompositionTranslateTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTranslateTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTranslateTransform3D {
    type Vtable = IDCompositionTranslateTransform3D_Vtbl;
}
impl ::core::clone::Clone for IDCompositionTranslateTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionTranslateTransform3D {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91636d4b_9ba1_4532_aaf7_e3344994d788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTurbulenceEffect(::windows::core::IUnknown);
impl IDCompositionTurbulenceEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetInput)(::windows::core::Interface::as_raw(self), index, input.into_param().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetOffset(&self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffset)(::windows::core::Interface::as_raw(self), offset).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBaseFrequency(&self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBaseFrequency)(::windows::core::Interface::as_raw(self), frequency).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSize(&self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSize)(::windows::core::Interface::as_raw(self), size).ok()
    }
    pub unsafe fn SetNumOctaves(&self, numoctaves: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNumOctaves)(::windows::core::Interface::as_raw(self), numoctaves).ok()
    }
    pub unsafe fn SetSeed(&self, seed: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSeed)(::windows::core::Interface::as_raw(self), seed).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNoise)(::windows::core::Interface::as_raw(self), noise).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStitchable<P0>(&self, stitchable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetStitchable)(::windows::core::Interface::as_raw(self), stitchable.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionTurbulenceEffect, ::windows::core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
impl ::core::cmp::PartialEq for IDCompositionTurbulenceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTurbulenceEffect {}
impl ::core::fmt::Debug for IDCompositionTurbulenceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTurbulenceEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTurbulenceEffect {
    type Vtable = IDCompositionTurbulenceEffect_Vtbl;
}
impl ::core::clone::Clone for IDCompositionTurbulenceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionTurbulenceEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6a55bda_c09c_49f3_9193_a41922c89715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTurbulenceEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetOffset: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBaseFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBaseFrequency: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSize: usize,
    pub SetNumOctaves: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows::core::HRESULT,
    pub SetSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetNoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetNoise: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStitchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStitchable: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVirtualSurface(::windows::core::IUnknown);
impl IDCompositionVirtualSurface {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw<T>(&self, updaterect: ::core::option::Option<*const super::super::Foundation::RECT>, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.BeginDraw)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(updaterect.unwrap_or(::std::ptr::null())), &<T as ::windows::core::ComInterface>::IID, &mut result__, updateoffset).from_abi(result__)
    }
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SuspendDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResumeDraw)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: ::core::option::Option<*const super::super::Foundation::RECT>, cliprect: ::core::option::Option<*const super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Scroll)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scrollrect.unwrap_or(::std::ptr::null())), ::core::mem::transmute(cliprect.unwrap_or(::std::ptr::null())), offsetx, offsety).ok()
    }
    pub unsafe fn Resize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resize)(::windows::core::Interface::as_raw(self), width, height).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Trim(&self, rectangles: ::core::option::Option<&[super::super::Foundation::RECT]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Trim)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rectangles.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), rectangles.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionVirtualSurface, ::windows::core::IUnknown, IDCompositionSurface);
impl ::core::cmp::PartialEq for IDCompositionVirtualSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVirtualSurface {}
impl ::core::fmt::Debug for IDCompositionVirtualSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVirtualSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVirtualSurface {
    type Vtable = IDCompositionVirtualSurface_Vtbl;
}
impl ::core::clone::Clone for IDCompositionVirtualSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionVirtualSurface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae471c51_5f53_4a24_8d3e_d0c39c30b3f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVirtualSurface_Vtbl {
    pub base__: IDCompositionSurface_Vtbl,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Trim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Trim: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVisual(::windows::core::IUnknown);
impl IDCompositionVisual {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOffsetX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetX2)(::windows::core::Interface::as_raw(self), offsetx).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOffsetY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetY2)(::windows::core::Interface::as_raw(self), offsety).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionTransform>,
    {
        (::windows::core::Interface::vtable(self).SetTransform)(::windows::core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransform2)(::windows::core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).SetTransformParent)(::windows::core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionEffect>,
    {
        (::windows::core::Interface::vtable(self).SetEffect)(::windows::core::Interface::as_raw(self), effect.into_param().abi()).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBitmapInterpolationMode)(::windows::core::Interface::as_raw(self), interpolationmode).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBorderMode)(::windows::core::Interface::as_raw(self), bordermode).ok()
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionClip>,
    {
        (::windows::core::Interface::vtable(self).SetClip)(::windows::core::Interface::as_raw(self), clip.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClip2)(::windows::core::Interface::as_raw(self), rect).ok()
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetContent)(::windows::core::Interface::as_raw(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<P0, P1, P2>(&self, visual: P0, insertabove: P1, referencevisual: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).AddVisual)(::windows::core::Interface::as_raw(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).RemoveVisual)(::windows::core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllVisuals)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCompositeMode)(::windows::core::Interface::as_raw(self), compositemode).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionVisual, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDCompositionVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisual {}
impl ::core::fmt::Debug for IDCompositionVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisual").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisual {
    type Vtable = IDCompositionVisual_Vtbl;
}
impl ::core::clone::Clone for IDCompositionVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionVisual {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d93059d_097b_4651_9a60_f0f25116e2f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform2: usize,
    pub SetTransformParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT,
    pub SetClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetClip2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetClip2: usize,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void, insertabove: super::super::Foundation::BOOL, referencevisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddVisual: usize,
    pub RemoveVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAllVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVisual2(::windows::core::IUnknown);
impl IDCompositionVisual2 {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).base__.SetOffsetX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetOffsetX2)(::windows::core::Interface::as_raw(self), offsetx).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).base__.SetOffsetY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetOffsetY2)(::windows::core::Interface::as_raw(self), offsety).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionTransform>,
    {
        (::windows::core::Interface::vtable(self).base__.SetTransform)(::windows::core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetTransform2)(::windows::core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).base__.SetTransformParent)(::windows::core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionEffect>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEffect)(::windows::core::Interface::as_raw(self), effect.into_param().abi()).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBitmapInterpolationMode)(::windows::core::Interface::as_raw(self), interpolationmode).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBorderMode)(::windows::core::Interface::as_raw(self), bordermode).ok()
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionClip>,
    {
        (::windows::core::Interface::vtable(self).base__.SetClip)(::windows::core::Interface::as_raw(self), clip.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetClip2)(::windows::core::Interface::as_raw(self), rect).ok()
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetContent)(::windows::core::Interface::as_raw(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<P0, P1, P2>(&self, visual: P0, insertabove: P1, referencevisual: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).base__.AddVisual)(::windows::core::Interface::as_raw(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).base__.RemoveVisual)(::windows::core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveAllVisuals)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCompositeMode)(::windows::core::Interface::as_raw(self), compositemode).ok()
    }
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacityMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackFaceVisibility)(::windows::core::Interface::as_raw(self), visibility).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionVisual2, ::windows::core::IUnknown, IDCompositionVisual);
impl ::core::cmp::PartialEq for IDCompositionVisual2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisual2 {}
impl ::core::fmt::Debug for IDCompositionVisual2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisual2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisual2 {
    type Vtable = IDCompositionVisual2_Vtbl;
}
impl ::core::clone::Clone for IDCompositionVisual2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionVisual2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8de1639_4331_4b26_bc5f_6a321d347a85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual2_Vtbl {
    pub base__: IDCompositionVisual_Vtbl,
    pub SetOpacityMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT,
    pub SetBackFaceVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVisual3(::windows::core::IUnknown);
impl IDCompositionVisual3 {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetOffsetX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetOffsetX2)(::windows::core::Interface::as_raw(self), offsetx).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetOffsetY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetOffsetY2)(::windows::core::Interface::as_raw(self), offsety).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionTransform>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetTransform)(::windows::core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetTransform2)(::windows::core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetTransformParent)(::windows::core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionEffect>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetEffect)(::windows::core::Interface::as_raw(self), effect.into_param().abi()).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetBitmapInterpolationMode)(::windows::core::Interface::as_raw(self), interpolationmode).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetBorderMode)(::windows::core::Interface::as_raw(self), bordermode).ok()
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionClip>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetClip)(::windows::core::Interface::as_raw(self), clip.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetClip2)(::windows::core::Interface::as_raw(self), rect).ok()
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetContent)(::windows::core::Interface::as_raw(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<P0, P1, P2>(&self, visual: P0, insertabove: P1, referencevisual: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.AddVisual)(::windows::core::Interface::as_raw(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemoveVisual)(::windows::core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.RemoveAllVisuals)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetCompositeMode)(::windows::core::Interface::as_raw(self), compositemode).ok()
    }
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetOpacityMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetBackFaceVisibility)(::windows::core::Interface::as_raw(self), visibility).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnableHeatMap)(::windows::core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn DisableHeatMap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DisableHeatMap)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EnableRedrawRegions)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DisableRedrawRegions)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDepthMode(&self, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDepthMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetOffsetZ<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOffsetZ)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetZ2)(::windows::core::Interface::as_raw(self), offsetz).ok()
    }
    pub unsafe fn SetOpacity<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).SetOpacity)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacity2)(::windows::core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn SetTransform3<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionTransform3D>,
    {
        (::windows::core::Interface::vtable(self).SetTransform3)(::windows::core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetTransform4(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransform4)(::windows::core::Interface::as_raw(self), matrix).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVisible<P0>(&self, visible: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetVisible)(::windows::core::Interface::as_raw(self), visible.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionVisual3, ::windows::core::IUnknown, IDCompositionVisual, IDCompositionVisual2, IDCompositionVisualDebug);
impl ::core::cmp::PartialEq for IDCompositionVisual3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisual3 {}
impl ::core::fmt::Debug for IDCompositionVisual3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisual3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisual3 {
    type Vtable = IDCompositionVisual3_Vtbl;
}
impl ::core::clone::Clone for IDCompositionVisual3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionVisual3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2775f462_b6c1_4015_b0be_b3e7d6a4976d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual3_Vtbl {
    pub base__: IDCompositionVisualDebug_Vtbl,
    pub SetDepthMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub SetTransform3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetTransform4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetTransform4: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVisible: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVisualDebug(::windows::core::IUnknown);
impl IDCompositionVisualDebug {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetOffsetX)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetOffsetX2)(::windows::core::Interface::as_raw(self), offsetx).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionAnimation>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetOffsetY)(::windows::core::Interface::as_raw(self), animation.into_param().abi()).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetOffsetY2)(::windows::core::Interface::as_raw(self), offsety).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionTransform>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetTransform)(::windows::core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetTransform2)(::windows::core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetTransformParent)(::windows::core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionEffect>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetEffect)(::windows::core::Interface::as_raw(self), effect.into_param().abi()).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetBitmapInterpolationMode)(::windows::core::Interface::as_raw(self), interpolationmode).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetBorderMode)(::windows::core::Interface::as_raw(self), bordermode).ok()
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionClip>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetClip)(::windows::core::Interface::as_raw(self), clip.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetClip2)(::windows::core::Interface::as_raw(self), rect).ok()
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetContent)(::windows::core::Interface::as_raw(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<P0, P1, P2>(&self, visual: P0, insertabove: P1, referencevisual: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddVisual)(::windows::core::Interface::as_raw(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDCompositionVisual>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveVisual)(::windows::core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveAllVisuals)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetCompositeMode)(::windows::core::Interface::as_raw(self), compositemode).ok()
    }
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetOpacityMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBackFaceVisibility)(::windows::core::Interface::as_raw(self), visibility).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableHeatMap)(::windows::core::Interface::as_raw(self), color).ok()
    }
    pub unsafe fn DisableHeatMap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableHeatMap)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableRedrawRegions)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableRedrawRegions)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDCompositionVisualDebug, ::windows::core::IUnknown, IDCompositionVisual, IDCompositionVisual2);
impl ::core::cmp::PartialEq for IDCompositionVisualDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisualDebug {}
impl ::core::fmt::Debug for IDCompositionVisualDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisualDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisualDebug {
    type Vtable = IDCompositionVisualDebug_Vtbl;
}
impl ::core::clone::Clone for IDCompositionVisualDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDCompositionVisualDebug {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfed2b808_5eb4_43a0_aea3_35f65280f91b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisualDebug_Vtbl {
    pub base__: IDCompositionVisual2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub EnableHeatMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    EnableHeatMap: usize,
    pub DisableHeatMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnableRedrawRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableRedrawRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPOSITION_FRAME_ID_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(2i32);
impl ::core::marker::Copy for COMPOSITION_FRAME_ID_TYPE {}
impl ::core::clone::Clone for COMPOSITION_FRAME_ID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPOSITION_FRAME_ID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COMPOSITION_FRAME_ID_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COMPOSITION_FRAME_ID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPOSITION_FRAME_ID_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BACKFACE_VISIBILITY {}
impl ::core::clone::Clone for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DCOMPOSITION_BACKFACE_VISIBILITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BACKFACE_VISIBILITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BITMAP_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_BORDER_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BORDER_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BORDER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BORDER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DCOMPOSITION_BORDER_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_BORDER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BORDER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_COMPOSITE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_COMPOSITE_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_COMPOSITE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_COMPOSITE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DCOMPOSITION_COMPOSITE_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_COMPOSITE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_COMPOSITE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_DEPTH_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_DEPTH_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_DEPTH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_DEPTH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DCOMPOSITION_DEPTH_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_DEPTH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_DEPTH_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_OPACITY_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_OPACITY_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_OPACITY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_OPACITY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DCOMPOSITION_OPACITY_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_OPACITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_OPACITY_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
impl ::core::marker::Copy for COMPOSITION_FRAME_STATS {}
impl ::core::clone::Clone for COMPOSITION_FRAME_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_FRAME_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_FRAME_STATS").field("startTime", &self.startTime).field("targetTime", &self.targetTime).field("framePeriod", &self.framePeriod).finish()
    }
}
impl ::windows::core::TypeKind for COMPOSITION_FRAME_STATS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COMPOSITION_FRAME_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.startTime == other.startTime && self.targetTime == other.targetTime && self.framePeriod == other.framePeriod
    }
}
impl ::core::cmp::Eq for COMPOSITION_FRAME_STATS {}
impl ::core::default::Default for COMPOSITION_FRAME_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
impl ::core::marker::Copy for COMPOSITION_STATS {}
impl ::core::clone::Clone for COMPOSITION_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_STATS").field("presentCount", &self.presentCount).field("refreshCount", &self.refreshCount).field("virtualRefreshCount", &self.virtualRefreshCount).field("time", &self.time).finish()
    }
}
impl ::windows::core::TypeKind for COMPOSITION_STATS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COMPOSITION_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.presentCount == other.presentCount && self.refreshCount == other.refreshCount && self.virtualRefreshCount == other.virtualRefreshCount && self.time == other.time
    }
}
impl ::core::cmp::Eq for COMPOSITION_STATS {}
impl ::core::default::Default for COMPOSITION_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: super::super::Foundation::LUID,
    pub renderAdapterLuid: super::super::Foundation::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPOSITION_TARGET_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPOSITION_TARGET_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_TARGET_ID").field("displayAdapterLuid", &self.displayAdapterLuid).field("renderAdapterLuid", &self.renderAdapterLuid).field("vidPnSourceId", &self.vidPnSourceId).field("vidPnTargetId", &self.vidPnTargetId).field("uniqueId", &self.uniqueId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for COMPOSITION_TARGET_ID {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_ID {
    fn eq(&self, other: &Self) -> bool {
        self.displayAdapterLuid == other.displayAdapterLuid && self.renderAdapterLuid == other.renderAdapterLuid && self.vidPnSourceId == other.vidPnSourceId && self.vidPnTargetId == other.vidPnTargetId && self.uniqueId == other.uniqueId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPOSITION_TARGET_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
impl ::core::marker::Copy for COMPOSITION_TARGET_STATS {}
impl ::core::clone::Clone for COMPOSITION_TARGET_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_TARGET_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_TARGET_STATS").field("outstandingPresents", &self.outstandingPresents).field("presentTime", &self.presentTime).field("vblankDuration", &self.vblankDuration).field("presentedStats", &self.presentedStats).field("completedStats", &self.completedStats).finish()
    }
}
impl ::windows::core::TypeKind for COMPOSITION_TARGET_STATS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.outstandingPresents == other.outstandingPresents && self.presentTime == other.presentTime && self.vblankDuration == other.vblankDuration && self.presentedStats == other.presentedStats && self.completedStats == other.completedStats
    }
}
impl ::core::cmp::Eq for COMPOSITION_TARGET_STATS {}
impl ::core::default::Default for COMPOSITION_TARGET_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DCOMPOSITION_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DCOMPOSITION_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCOMPOSITION_FRAME_STATISTICS").field("lastFrameTime", &self.lastFrameTime).field("currentCompositionRate", &self.currentCompositionRate).field("currentTime", &self.currentTime).field("timeFrequency", &self.timeFrequency).field("nextEstimatedFrameTime", &self.nextEstimatedFrameTime).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::TypeKind for DCOMPOSITION_FRAME_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DCOMPOSITION_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.lastFrameTime == other.lastFrameTime && self.currentCompositionRate == other.currentCompositionRate && self.currentTime == other.currentTime && self.timeFrequency == other.timeFrequency && self.nextEstimatedFrameTime == other.nextEstimatedFrameTime
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DCOMPOSITION_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl ::core::marker::Copy for DCompositionInkTrailPoint {}
impl ::core::clone::Clone for DCompositionInkTrailPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCompositionInkTrailPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCompositionInkTrailPoint").field("x", &self.x).field("y", &self.y).field("radius", &self.radius).finish()
    }
}
impl ::windows::core::TypeKind for DCompositionInkTrailPoint {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DCompositionInkTrailPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.radius == other.radius
    }
}
impl ::core::cmp::Eq for DCompositionInkTrailPoint {}
impl ::core::default::Default for DCompositionInkTrailPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
