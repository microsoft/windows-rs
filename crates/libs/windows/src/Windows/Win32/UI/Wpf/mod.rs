::windows_core::imp::com_interface!(IMILBitmapEffect, IMILBitmapEffect_Vtbl, 0x8a6ff321_c944_4a1b_9944_9954af301258);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffect, ::windows_core::IUnknown);
impl IMILBitmapEffect {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<P0>(&self, uiindex: u32, pcontext: P0) -> ::windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutput)(::windows_core::Interface::as_raw(self), uiindex, pcontext.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetParentEffect(&self) -> ::windows_core::Result<IMILBitmapEffectGroup> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetParentEffect)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn SetInputSource<P0>(&self, uiindex: u32, pbitmapsource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Imaging::IWICBitmapSource>,
    {
        (::windows_core::Interface::vtable(self).SetInputSource)(::windows_core::Interface::as_raw(self), uiindex, pbitmapsource.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMILBitmapEffect_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub GetParentEffect: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub SetInputSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    SetInputSource: usize,
}
::windows_core::imp::com_interface!(IMILBitmapEffectConnections, IMILBitmapEffectConnections_Vtbl, 0xc2b5d861_9b1a_4374_89b0_dec4874d6a81);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnections, ::windows_core::IUnknown);
impl IMILBitmapEffectConnections {
    pub unsafe fn GetInputConnector(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInputConnector)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOutputConnector(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputConnector)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMILBitmapEffectConnections_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetInputConnector: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOutputConnector: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectConnectionsInfo, IMILBitmapEffectConnectionsInfo_Vtbl, 0x476b538a_c765_4237_ba4a_d6a880ff0cfc);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnectionsInfo, ::windows_core::IUnknown);
impl IMILBitmapEffectConnectionsInfo {
    pub unsafe fn GetNumberInputs(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNumberInputs)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNumberOutputs(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNumberOutputs)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetInputConnectorInfo(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInputConnectorInfo)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetOutputConnectorInfo(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputConnectorInfo)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMILBitmapEffectConnectionsInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetNumberInputs: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32) -> ::windows_core::HRESULT,
    pub GetNumberOutputs: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32) -> ::windows_core::HRESULT,
    pub GetInputConnectorInfo: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOutputConnectorInfo: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectConnector, IMILBitmapEffectConnector_Vtbl, 0xf59567b3_76c1_4d47_ba1e_79f955e350ef);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnector, ::windows_core::IUnknown, IMILBitmapEffectConnectorInfo);
impl IMILBitmapEffectConnector {
    pub unsafe fn GetIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetIndex)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOptimalFormat)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNumberFormats)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetFormat)(::windows_core::Interface::as_raw(self), ulindex, &mut result__).map(|| result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsConnected)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBitmapEffect)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMILBitmapEffectConnector_Vtbl {
    pub base__: IMILBitmapEffectConnectorInfo_Vtbl,
    pub IsConnected: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    pub GetBitmapEffect: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnectorInfo_Vtbl, 0xf66d2e4b_b46b_42fc_859e_3da0ecdb3c43);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnectorInfo, ::windows_core::IUnknown);
impl IMILBitmapEffectConnectorInfo {
    pub unsafe fn GetIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIndex)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOptimalFormat)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNumberFormats)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFormat)(::windows_core::Interface::as_raw(self), ulindex, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMILBitmapEffectConnectorInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetIndex: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32) -> ::windows_core::HRESULT,
    pub GetOptimalFormat: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetNumberFormats: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32) -> ::windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectEvents, IMILBitmapEffectEvents_Vtbl, 0x2e880dd8_f8ce_457b_8199_d60bb3d7ef98);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectEvents, ::windows_core::IUnknown);
impl IMILBitmapEffectEvents {
    pub unsafe fn PropertyChange<P0, P1>(&self, peffect: P0, bstrpropertyname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffect>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).PropertyChange)(::windows_core::Interface::as_raw(self), peffect.into_param().abi(), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn DirtyRegion<P0>(&self, peffect: P0, prect: *const MilRectD) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffect>,
    {
        (::windows_core::Interface::vtable(self).DirtyRegion)(::windows_core::Interface::as_raw(self), peffect.into_param().abi(), prect).ok()
    }
}
#[repr(C)]
pub struct IMILBitmapEffectEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub PropertyChange: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DirtyRegion: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *const MilRectD) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectFactory, IMILBitmapEffectFactory_Vtbl, 0x33a9df34_a403_4ec7_b07e_bc0682370845);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectFactory, ::windows_core::IUnknown);
impl IMILBitmapEffectFactory {
    pub unsafe fn CreateEffect(&self, pguideffect: *const ::windows_core::GUID) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateEffect)(::windows_core::Interface::as_raw(self), pguideffect, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateContext(&self) -> ::windows_core::Result<IMILBitmapEffectRenderContext> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateContext)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CreateEffectOuter(&self) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateEffectOuter)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMILBitmapEffectFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateEffect: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateContext: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateEffectOuter: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectGroup, IMILBitmapEffectGroup_Vtbl, 0x2f952360_698a_4ac6_81a1_bcfdf08eb8e8);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectGroup, ::windows_core::IUnknown);
impl IMILBitmapEffectGroup {
    pub unsafe fn GetInteriorInputConnector(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInteriorInputConnector)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetInteriorOutputConnector(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInteriorOutputConnector)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Add<P0>(&self, peffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffect>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), peffect.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMILBitmapEffectGroup_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetInteriorInputConnector: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetInteriorOutputConnector: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectGroupImpl, IMILBitmapEffectGroupImpl_Vtbl, 0x78fed518_1cfc_4807_8b85_6b6e51398f62);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectGroupImpl, ::windows_core::IUnknown);
impl IMILBitmapEffectGroupImpl {
    pub unsafe fn Preprocess<P0>(&self, pcontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        (::windows_core::Interface::vtable(self).Preprocess)(::windows_core::Interface::as_raw(self), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn GetNumberChildren(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNumberChildren)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetChildren(&self) -> ::windows_core::Result<IMILBitmapEffects> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetChildren)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMILBitmapEffectGroupImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Preprocess: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNumberChildren: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32) -> ::windows_core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectImpl, IMILBitmapEffectImpl_Vtbl, 0xcc2468f2_9936_47be_b4af_06b5df5dbcbb);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectImpl, ::windows_core::IUnknown);
impl IMILBitmapEffectImpl {
    pub unsafe fn IsInPlaceModificationAllowed<P0>(&self, poutputconnector: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectOutputConnector>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsInPlaceModificationAllowed)(::windows_core::Interface::as_raw(self), poutputconnector.into_param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParentEffect<P0>(&self, pparenteffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectGroup>,
    {
        (::windows_core::Interface::vtable(self).SetParentEffect)(::windows_core::Interface::as_raw(self), pparenteffect.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputSource(&self, uiindex: u32) -> ::windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInputSource)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetInputSourceBounds(&self, uiindex: u32, prect: *mut MilRectD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInputSourceBounds)(::windows_core::Interface::as_raw(self), uiindex, prect).ok()
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputBitmapSource<P0>(&self, uiindex: u32, prendercontext: P0, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInputBitmapSource)(::windows_core::Interface::as_raw(self), uiindex, prendercontext.into_param().abi(), pfmodifyinplace, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutputBitmapSource<P0>(&self, uiindex: u32, prendercontext: P0, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputBitmapSource)(::windows_core::Interface::as_raw(self), uiindex, prendercontext.into_param().abi(), pfmodifyinplace, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Initialize<P0>(&self, pinner: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).Initialize)(::windows_core::Interface::as_raw(self), pinner.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMILBitmapEffectImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsInPlaceModificationAllowed: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    pub SetParentEffect: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputSource: usize,
    pub GetInputSourceBounds: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut MilRectD) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputBitmapSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut ::core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputBitmapSource: usize,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutputBitmapSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut ::core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutputBitmapSource: usize,
    pub Initialize: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectInputConnector, IMILBitmapEffectInputConnector_Vtbl, 0xa9b4ecaa_7a3c_45e7_8573_f4b81b60dd6c);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectInputConnector, ::windows_core::IUnknown, IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnector);
impl IMILBitmapEffectInputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetIndex)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOptimalFormat)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetNumberFormats)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetFormat)(::windows_core::Interface::as_raw(self), ulindex, &mut result__).map(|| result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsConnected)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBitmapEffect)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ConnectTo<P0>(&self, pconnector: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectOutputConnector>,
    {
        (::windows_core::Interface::vtable(self).ConnectTo)(::windows_core::Interface::as_raw(self), pconnector.into_param().abi()).ok()
    }
    pub unsafe fn GetConnection(&self) -> ::windows_core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetConnection)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMILBitmapEffectInputConnector_Vtbl {
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub ConnectTo: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectInteriorInputConnector, IMILBitmapEffectInteriorInputConnector_Vtbl, 0x20287e9e_86a2_4e15_953d_eb1438a5b842);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectInteriorInputConnector, ::windows_core::IUnknown);
impl IMILBitmapEffectInteriorInputConnector {
    pub unsafe fn GetInputConnector(&self) -> ::windows_core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInputConnector)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetInputConnector: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectInteriorOutputConnector, IMILBitmapEffectInteriorOutputConnector_Vtbl, 0x00bbb6dc_acc9_4bfc_b344_8bee383dfefa);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectInteriorOutputConnector, ::windows_core::IUnknown);
impl IMILBitmapEffectInteriorOutputConnector {
    pub unsafe fn GetOutputConnector(&self) -> ::windows_core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputConnector)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOutputConnector: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectOutputConnector, IMILBitmapEffectOutputConnector_Vtbl, 0x92957aad_841b_4866_82ec_8752468b07fd);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectOutputConnector, ::windows_core::IUnknown, IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnector);
impl IMILBitmapEffectOutputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetIndex)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOptimalFormat)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetNumberFormats)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetFormat)(::windows_core::Interface::as_raw(self), ulindex, &mut result__).map(|| result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsConnected)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBitmapEffect)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetNumberConnections(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNumberConnections)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetConnection(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetConnection)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IMILBitmapEffectOutputConnector_Vtbl {
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub GetNumberConnections: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32) -> ::windows_core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectOutputConnectorImpl, IMILBitmapEffectOutputConnectorImpl_Vtbl, 0x21fae777_8b39_4bfa_9f2d_f3941ed36913);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectOutputConnectorImpl, ::windows_core::IUnknown);
impl IMILBitmapEffectOutputConnectorImpl {
    pub unsafe fn AddBackLink<P0>(&self, pconnection: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectInputConnector>,
    {
        (::windows_core::Interface::vtable(self).AddBackLink)(::windows_core::Interface::as_raw(self), pconnection.into_param().abi()).ok()
    }
    pub unsafe fn RemoveBackLink<P0>(&self, pconnection: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectInputConnector>,
    {
        (::windows_core::Interface::vtable(self).RemoveBackLink)(::windows_core::Interface::as_raw(self), pconnection.into_param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddBackLink: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveBackLink: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectPrimitive, IMILBitmapEffectPrimitive_Vtbl, 0x67e31025_3091_4dfc_98d6_dd494551461d);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectPrimitive, ::windows_core::IUnknown);
impl IMILBitmapEffectPrimitive {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<P0>(&self, uiindex: u32, pcontext: P0, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutput)(::windows_core::Interface::as_raw(self), uiindex, pcontext.into_param().abi(), pfmodifyinplace, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn TransformPoint<P0, P1>(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: P0, pcontext: P1, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        (::windows_core::Interface::vtable(self).TransformPoint)(::windows_core::Interface::as_raw(self), uiindex, p, fforwardtransform.into_param().abi(), pcontext.into_param().abi(), pfpointtransformed).ok()
    }
    pub unsafe fn TransformRect<P0, P1>(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: P0, pcontext: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        (::windows_core::Interface::vtable(self).TransformRect)(::windows_core::Interface::as_raw(self), uiindex, p, fforwardtransform.into_param().abi(), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn HasAffineTransform(&self, uiindex: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HasAffineTransform)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).map(|| result__)
    }
    pub unsafe fn HasInverseTransform(&self, uiindex: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HasInverseTransform)(::windows_core::Interface::as_raw(self), uiindex, &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub unsafe fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAffineMatrix)(::windows_core::Interface::as_raw(self), uiindex, pmatrix).ok()
    }
}
#[repr(C)]
pub struct IMILBitmapEffectPrimitive_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut ::core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub TransformPoint: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut MilPoint2D, super::super::Foundation::VARIANT_BOOL, *mut ::core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    pub TransformRect: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut MilRectD, super::super::Foundation::VARIANT_BOOL, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub HasAffineTransform: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    pub HasInverseTransform: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub GetAffineMatrix: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dwm"))]
    GetAffineMatrix: usize,
}
::windows_core::imp::com_interface!(IMILBitmapEffectPrimitiveImpl, IMILBitmapEffectPrimitiveImpl_Vtbl, 0xce41e00b_efa6_44e7_b007_dd042e3ae126);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectPrimitiveImpl, ::windows_core::IUnknown);
impl IMILBitmapEffectPrimitiveImpl {
    pub unsafe fn IsDirty(&self, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
        (::windows_core::Interface::vtable(self).IsDirty)(::windows_core::Interface::as_raw(self), uioutputindex, pfdirty)
    }
    pub unsafe fn IsVolatile(&self, uioutputindex: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsVolatile)(::windows_core::Interface::as_raw(self), uioutputindex, &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMILBitmapEffectPrimitiveImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    pub IsVolatile: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectRenderContext, IMILBitmapEffectRenderContext_Vtbl, 0x12a2ec7e_2d33_44b2_b334_1abb7846e390);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectRenderContext, ::windows_core::IUnknown);
impl IMILBitmapEffectRenderContext {
    pub unsafe fn SetOutputPixelFormat(&self, format: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutputPixelFormat)(::windows_core::Interface::as_raw(self), format).ok()
    }
    pub unsafe fn GetOutputPixelFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputPixelFormat)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUseSoftwareRenderer<P0>(&self, fsoftware: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseSoftwareRenderer)(::windows_core::Interface::as_raw(self), fsoftware.into_param().abi()).ok()
    }
    pub unsafe fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInitialTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetFinalTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFinalTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOutputDPI)(::windows_core::Interface::as_raw(self), dbldpix, dbldpiy).ok()
    }
    pub unsafe fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOutputDPI)(::windows_core::Interface::as_raw(self), pdbldpix, pdbldpiy).ok()
    }
    pub unsafe fn SetRegionOfInterest(&self, prect: *const MilRectD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRegionOfInterest)(::windows_core::Interface::as_raw(self), prect).ok()
    }
}
#[repr(C)]
pub struct IMILBitmapEffectRenderContext_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetOutputPixelFormat: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetOutputPixelFormat: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub SetUseSoftwareRenderer: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    pub SetInitialTransform: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const MILMatrixF) -> ::windows_core::HRESULT,
    pub GetFinalTransform: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MILMatrixF) -> ::windows_core::HRESULT,
    pub SetOutputDPI: unsafe extern "system" fn(*mut ::core::ffi::c_void, f64, f64) -> ::windows_core::HRESULT,
    pub GetOutputDPI: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut f64, *mut f64) -> ::windows_core::HRESULT,
    pub SetRegionOfInterest: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const MilRectD) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffectRenderContextImpl, IMILBitmapEffectRenderContextImpl_Vtbl, 0x4d25accb_797d_4fd2_b128_dffeff84fcc3);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectRenderContextImpl, ::windows_core::IUnknown);
impl IMILBitmapEffectRenderContextImpl {
    pub unsafe fn GetUseSoftwareRenderer(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUseSoftwareRenderer)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateTransform)(::windows_core::Interface::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetOutputBounds(&self, prect: *mut MilRectD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOutputBounds)(::windows_core::Interface::as_raw(self), prect).ok()
    }
    pub unsafe fn UpdateOutputBounds(&self, prect: *const MilRectD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UpdateOutputBounds)(::windows_core::Interface::as_raw(self), prect).ok()
    }
}
#[repr(C)]
pub struct IMILBitmapEffectRenderContextImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetUseSoftwareRenderer: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MILMatrixF) -> ::windows_core::HRESULT,
    pub UpdateTransform: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const MILMatrixF) -> ::windows_core::HRESULT,
    pub GetOutputBounds: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut MilRectD) -> ::windows_core::HRESULT,
    pub UpdateOutputBounds: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const MilRectD) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMILBitmapEffects, IMILBitmapEffects_Vtbl, 0x51ac3dce_67c5_448b_9180_ad3eabddd5dd);
::windows_core::imp::interface_hierarchy!(IMILBitmapEffects, ::windows_core::IUnknown);
impl IMILBitmapEffects {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<IMILBitmapEffectGroup> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Parent)(::windows_core::Interface::as_raw(self), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Item(&self, uindex: u32) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Item)(::windows_core::Interface::as_raw(self), uindex, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IMILBitmapEffects_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u32) -> ::windows_core::HRESULT,
}
pub const CLSID_MILBitmapEffectBevel: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd361dbe_6c9b_4de0_8290_f6400c2737ed);
pub const CLSID_MILBitmapEffectBlur: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa924df87_225d_4373_8f5b_b90ec85ae3de);
pub const CLSID_MILBitmapEffectDropShadow: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x459a3fbe_d8ac_4692_874b_7a265715aa16);
pub const CLSID_MILBitmapEffectEmboss: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd299846_824f_47ec_a007_12aa767f2816);
pub const CLSID_MILBitmapEffectGroup: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac9c1a9a_7e18_4f64_ac7e_47cf7f051e95);
pub const CLSID_MILBitmapEffectOuterGlow: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2161bdd_7eb6_4725_9c0b_8a2a1b4f0667);
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[repr(C)]
pub struct MILMatrixF {
    pub _11: f64,
    pub _12: f64,
    pub _13: f64,
    pub _14: f64,
    pub _21: f64,
    pub _22: f64,
    pub _23: f64,
    pub _24: f64,
    pub _31: f64,
    pub _32: f64,
    pub _33: f64,
    pub _34: f64,
    pub _41: f64,
    pub _42: f64,
    pub _43: f64,
    pub _44: f64,
}
impl ::core::marker::Copy for MILMatrixF {}
impl ::core::clone::Clone for MILMatrixF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MILMatrixF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MILMatrixF").field("_11", &self._11).field("_12", &self._12).field("_13", &self._13).field("_14", &self._14).field("_21", &self._21).field("_22", &self._22).field("_23", &self._23).field("_24", &self._24).field("_31", &self._31).field("_32", &self._32).field("_33", &self._33).field("_34", &self._34).field("_41", &self._41).field("_42", &self._42).field("_43", &self._43).field("_44", &self._44).finish()
    }
}
impl ::windows_core::TypeKind for MILMatrixF {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MILMatrixF {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._14 == other._14 && self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._24 == other._24 && self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._34 == other._34 && self._41 == other._41 && self._42 == other._42 && self._43 == other._43 && self._44 == other._44
    }
}
impl ::core::cmp::Eq for MILMatrixF {}
impl ::core::default::Default for MILMatrixF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
impl ::core::marker::Copy for MilPoint2D {}
impl ::core::clone::Clone for MilPoint2D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MilPoint2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MilPoint2D").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::windows_core::TypeKind for MilPoint2D {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MilPoint2D {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for MilPoint2D {}
impl ::core::default::Default for MilPoint2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl ::core::marker::Copy for MilRectD {}
impl ::core::clone::Clone for MilRectD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MilRectD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MilRectD").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::windows_core::TypeKind for MilRectD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MilRectD {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for MilRectD {}
impl ::core::default::Default for MilRectD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
