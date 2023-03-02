#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDeviceModelPlugIn_Impl: Sized {
    fn Initialize(&self, bstrxml: &::windows::core::BSTR, cnummodels: u32, imodelposition: u32) -> ::windows::core::Result<()>;
    fn GetNumChannels(&self) -> ::windows::core::Result<u32>;
    fn DeviceToColorimetricColors(&self, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows::core::Result<()>;
    fn ColorimetricToDeviceColors(&self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF) -> ::windows::core::Result<f32>;
    fn ColorimetricToDeviceColorsWithBlack(&self, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation) -> ::windows::core::Result<f32>;
    fn SetTransformDeviceModelInfo(&self, imodelposition: u32, pidevicemodelother: ::core::option::Option<&IDeviceModelPlugIn>) -> ::windows::core::Result<()>;
    fn GetPrimarySamples(&self, pprimarycolor: *mut PrimaryXYZColors) -> ::windows::core::Result<()>;
    fn GetGamutBoundaryMeshSize(&self, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows::core::Result<()>;
    fn GetGamutBoundaryMesh(&self, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows::core::Result<()>;
    fn GetNeutralAxisSize(&self) -> ::windows::core::Result<u32>;
    fn GetNeutralAxis(&self, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDeviceModelPlugIn {}
#[cfg(feature = "Win32_Foundation")]
impl IDeviceModelPlugIn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>() -> IDeviceModelPlugIn_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows::core::BSTR>, cnummodels: u32, imodelposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&bstrxml), ::core::mem::transmute_copy(&cnummodels), ::core::mem::transmute_copy(&imodelposition)).into()
        }
        unsafe extern "system" fn GetNumChannels<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumChannels() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumchannels, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceToColorimetricColors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeviceToColorimetricColors(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pdevicevalues), ::core::mem::transmute_copy(&pxyzcolors)).into()
        }
        unsafe extern "system" fn ColorimetricToDeviceColors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pdevicevalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ColorimetricToDeviceColors(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pxyzcolors)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevicevalues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorimetricToDeviceColorsWithBlack<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation, pdevicevalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ColorimetricToDeviceColorsWithBlack(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pxyzcolors), ::core::mem::transmute_copy(&pblackinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevicevalues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformDeviceModelInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imodelposition: u32, pidevicemodelother: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransformDeviceModelInfo(::core::mem::transmute_copy(&imodelposition), ::windows::core::from_raw_borrowed(&pidevicemodelother)).into()
        }
        unsafe extern "system" fn GetPrimarySamples<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprimarycolor: *mut PrimaryXYZColors) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrimarySamples(::core::mem::transmute_copy(&pprimarycolor)).into()
        }
        unsafe extern "system" fn GetGamutBoundaryMeshSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGamutBoundaryMeshSize(::core::mem::transmute_copy(&pnumvertices), ::core::mem::transmute_copy(&pnumtriangles)).into()
        }
        unsafe extern "system" fn GetGamutBoundaryMesh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGamutBoundaryMesh(::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&cvertices), ::core::mem::transmute_copy(&ctriangles), ::core::mem::transmute_copy(&pvertices), ::core::mem::transmute_copy(&ptriangles)).into()
        }
        unsafe extern "system" fn GetNeutralAxisSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNeutralAxisSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccolors, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNeutralAxis<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNeutralAxis(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&pxyzcolors)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDeviceModelPlugIn as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_ColorSystem\"`, `\"implement\"`*"]
pub trait IGamutMapModelPlugIn_Impl: Sized {
    fn Initialize(&self, bstrxml: &::windows::core::BSTR, psrcplugin: ::core::option::Option<&IDeviceModelPlugIn>, pdestplugin: ::core::option::Option<&IDeviceModelPlugIn>, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows::core::Result<()>;
    fn SourceToDestinationAppearanceColors(&self, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGamutMapModelPlugIn {}
impl IGamutMapModelPlugIn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: isize>() -> IGamutMapModelPlugIn_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows::core::BSTR>, psrcplugin: *mut ::core::ffi::c_void, pdestplugin: *mut ::core::ffi::c_void, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&bstrxml), ::windows::core::from_raw_borrowed(&psrcplugin), ::windows::core::from_raw_borrowed(&pdestplugin), ::core::mem::transmute_copy(&psrcgbd), ::core::mem::transmute_copy(&pdestgbd)).into()
        }
        unsafe extern "system" fn SourceToDestinationAppearanceColors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGamutMapModelPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SourceToDestinationAppearanceColors(::core::mem::transmute_copy(&ccolors), ::core::mem::transmute_copy(&pinputcolors), ::core::mem::transmute_copy(&poutputcolors)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SourceToDestinationAppearanceColors: SourceToDestinationAppearanceColors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGamutMapModelPlugIn as ::windows::core::ComInterface>::IID
    }
}
