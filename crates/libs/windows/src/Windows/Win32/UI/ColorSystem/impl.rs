#[cfg(feature = "Win32_Foundation")]
pub trait IDeviceModelPlugInImpl: Sized {
    fn Initialize();
    fn GetNumChannels();
    fn DeviceToColorimetricColors();
    fn ColorimetricToDeviceColors();
    fn ColorimetricToDeviceColorsWithBlack();
    fn SetTransformDeviceModelInfo();
    fn GetPrimarySamples();
    fn GetGamutBoundaryMeshSize();
    fn GetGamutBoundaryMesh();
    fn GetNeutralAxisSize();
    fn GetNeutralAxis();
}
#[cfg(feature = "Win32_Foundation")]
impl IDeviceModelPlugInVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugInImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceModelPlugInVtbl {
        unsafe extern "system" fn Initialize<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cnummodels: u32, imodelposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumChannels<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceToColorimetricColors<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ColorimetricToDeviceColors<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pdevicevalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ColorimetricToDeviceColorsWithBlack<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation, pdevicevalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransformDeviceModelInfo<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imodelposition: u32, pidevicemodelother: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrimarySamples<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprimarycolor: *mut PrimaryXYZColors) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGamutBoundaryMeshSize<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGamutBoundaryMesh<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNeutralAxisSize<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNeutralAxis<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetNumChannels: GetNumChannels::<Impl, IMPL_OFFSET>,
            DeviceToColorimetricColors: DeviceToColorimetricColors::<Impl, IMPL_OFFSET>,
            ColorimetricToDeviceColors: ColorimetricToDeviceColors::<Impl, IMPL_OFFSET>,
            ColorimetricToDeviceColorsWithBlack: ColorimetricToDeviceColorsWithBlack::<Impl, IMPL_OFFSET>,
            SetTransformDeviceModelInfo: SetTransformDeviceModelInfo::<Impl, IMPL_OFFSET>,
            GetPrimarySamples: GetPrimarySamples::<Impl, IMPL_OFFSET>,
            GetGamutBoundaryMeshSize: GetGamutBoundaryMeshSize::<Impl, IMPL_OFFSET>,
            GetGamutBoundaryMesh: GetGamutBoundaryMesh::<Impl, IMPL_OFFSET>,
            GetNeutralAxisSize: GetNeutralAxisSize::<Impl, IMPL_OFFSET>,
            GetNeutralAxis: GetNeutralAxis::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceModelPlugIn as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGamutMapModelPlugInImpl: Sized {
    fn Initialize();
    fn SourceToDestinationAppearanceColors();
}
#[cfg(feature = "Win32_Foundation")]
impl IGamutMapModelPlugInVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamutMapModelPlugInImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGamutMapModelPlugInVtbl {
        unsafe extern "system" fn Initialize<Impl: IGamutMapModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psrcplugin: ::windows::core::RawPtr, pdestplugin: ::windows::core::RawPtr, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SourceToDestinationAppearanceColors<Impl: IGamutMapModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            SourceToDestinationAppearanceColors: SourceToDestinationAppearanceColors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGamutMapModelPlugIn as ::windows::core::Interface>::IID
    }
}
