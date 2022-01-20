#[cfg(feature = "Win32_Foundation")]
pub trait IDeviceModelPlugIn_Impl: Sized {
    fn Initialize(&mut self, bstrxml: &super::super::Foundation::BSTR, cnummodels: u32, imodelposition: u32) -> ::windows::core::Result<()>;
    fn GetNumChannels(&mut self) -> ::windows::core::Result<u32>;
    fn DeviceToColorimetricColors(&mut self, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows::core::Result<()>;
    fn ColorimetricToDeviceColors(&mut self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF) -> ::windows::core::Result<f32>;
    fn ColorimetricToDeviceColorsWithBlack(&mut self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation) -> ::windows::core::Result<f32>;
    fn SetTransformDeviceModelInfo(&mut self, imodelposition: u32, pidevicemodelother: &::core::option::Option<IDeviceModelPlugIn>) -> ::windows::core::Result<()>;
    fn GetPrimarySamples(&mut self) -> ::windows::core::Result<PrimaryXYZColors>;
    fn GetGamutBoundaryMeshSize(&mut self, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows::core::Result<()>;
    fn GetGamutBoundaryMesh(&mut self, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows::core::Result<()>;
    fn GetNeutralAxisSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetNeutralAxis(&mut self, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDeviceModelPlugIn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>() -> IDeviceModelPlugIn_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cnummodels: u32, imodelposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&bstrxml), ::core::mem::transmute_copy(&cnummodels), ::core::mem::transmute_copy(&imodelposition)).into()
        }
        unsafe extern "system" fn GetNumChannels<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumChannels() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumchannels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceToColorimetricColors<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceToColorimetricColors(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pdevicevalues), ::core::mem::transmute_copy(&pxyzcolors)).into()
        }
        unsafe extern "system" fn ColorimetricToDeviceColors<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pdevicevalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ColorimetricToDeviceColors(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pxyzcolors)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdevicevalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorimetricToDeviceColorsWithBlack<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation, pdevicevalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ColorimetricToDeviceColorsWithBlack(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pxyzcolors), ::core::mem::transmute_copy(&pblackinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdevicevalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformDeviceModelInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imodelposition: u32, pidevicemodelother: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransformDeviceModelInfo(::core::mem::transmute_copy(&imodelposition), ::core::mem::transmute(&pidevicemodelother)).into()
        }
        unsafe extern "system" fn GetPrimarySamples<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprimarycolor: *mut PrimaryXYZColors) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPrimarySamples() {
                ::core::result::Result::Ok(ok__) => {
                    *pprimarycolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamutBoundaryMeshSize<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGamutBoundaryMeshSize(::core::mem::transmute_copy(&pnumvertices), ::core::mem::transmute_copy(&pnumtriangles)).into()
        }
        unsafe extern "system" fn GetGamutBoundaryMesh<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGamutBoundaryMesh(::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&cvertices), ::core::mem::transmute_copy(&ctriangles), ::core::mem::transmute_copy(&pvertices), ::core::mem::transmute_copy(&ptriangles)).into()
        }
        unsafe extern "system" fn GetNeutralAxisSize<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNeutralAxisSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pccolors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNeutralAxis<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNeutralAxis(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&pxyzcolors)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetNumChannels: GetNumChannels::<Identity, Impl, OFFSET>,
            DeviceToColorimetricColors: DeviceToColorimetricColors::<Identity, Impl, OFFSET>,
            ColorimetricToDeviceColors: ColorimetricToDeviceColors::<Identity, Impl, OFFSET>,
            ColorimetricToDeviceColorsWithBlack: ColorimetricToDeviceColorsWithBlack::<Identity, Impl, OFFSET>,
            SetTransformDeviceModelInfo: SetTransformDeviceModelInfo::<Identity, Impl, OFFSET>,
            GetPrimarySamples: GetPrimarySamples::<Identity, Impl, OFFSET>,
            GetGamutBoundaryMeshSize: GetGamutBoundaryMeshSize::<Identity, Impl, OFFSET>,
            GetGamutBoundaryMesh: GetGamutBoundaryMesh::<Identity, Impl, OFFSET>,
            GetNeutralAxisSize: GetNeutralAxisSize::<Identity, Impl, OFFSET>,
            GetNeutralAxis: GetNeutralAxis::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceModelPlugIn as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGamutMapModelPlugIn_Impl: Sized {
    fn Initialize(&mut self, bstrxml: &super::super::Foundation::BSTR, psrcplugin: &::core::option::Option<IDeviceModelPlugIn>, pdestplugin: &::core::option::Option<IDeviceModelPlugIn>, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows::core::Result<()>;
    fn SourceToDestinationAppearanceColors(&mut self, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGamutMapModelPlugIn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: isize>() -> IGamutMapModelPlugIn_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psrcplugin: ::windows::core::RawPtr, pdestplugin: ::windows::core::RawPtr, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&bstrxml), ::core::mem::transmute(&psrcplugin), ::core::mem::transmute(&pdestplugin), ::core::mem::transmute_copy(&psrcgbd), ::core::mem::transmute_copy(&pdestgbd)).into()
        }
        unsafe extern "system" fn SourceToDestinationAppearanceColors<Identity: ::windows::core::IUnknownImpl, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SourceToDestinationAppearanceColors(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&pinputcolors), ::core::mem::transmute_copy(&poutputcolors)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SourceToDestinationAppearanceColors: SourceToDestinationAppearanceColors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGamutMapModelPlugIn as ::windows::core::Interface>::IID
    }
}
