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
impl ::windows::core::RuntimeName for IDeviceModelPlugIn {
    const NAME: &'static str = "Windows.Win32.UI.ColorSystem.IDeviceModelPlugIn";
}
impl IDeviceModelPlugInVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceModelPlugInImpl, const OFFSET: isize>() -> IDeviceModelPlugInVtbl {
        unsafe extern "system" fn Initialize<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, cnummodels: u32, imodelposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&bstrxml as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), cnummodels, imodelposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumChannels<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumChannels(::core::mem::transmute_copy(&pnumchannels)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceToColorimetricColors<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pdevicevalues: *const f32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceToColorimetricColors(ccolors, cchannels, pdevicevalues, ::core::mem::transmute_copy(&pxyzcolors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorimetricToDeviceColors<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pdevicevalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorimetricToDeviceColors(ccolors, cchannels, &*(&pxyzcolors as *const <XYZColorF as ::windows::core::Abi>::Abi as *const <XYZColorF as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdevicevalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorimetricToDeviceColorsWithBlack<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, cchannels: u32, pxyzcolors: *const XYZColorF, pblackinformation: *const BlackInformation, pdevicevalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorimetricToDeviceColorsWithBlack(ccolors, cchannels, &*(&pxyzcolors as *const <XYZColorF as ::windows::core::Abi>::Abi as *const <XYZColorF as ::windows::core::DefaultType>::DefaultType), &*(&pblackinformation as *const <BlackInformation as ::windows::core::Abi>::Abi as *const <BlackInformation as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdevicevalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformDeviceModelInfo<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, imodelposition: u32, pidevicemodelother: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransformDeviceModelInfo(imodelposition, &*(&pidevicemodelother as *const <IDeviceModelPlugIn as ::windows::core::Abi>::Abi as *const <IDeviceModelPlugIn as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrimarySamples<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprimarycolor: *mut PrimaryXYZColors) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrimarySamples(::core::mem::transmute_copy(&pprimarycolor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamutBoundaryMeshSize<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumvertices: *mut u32, pnumtriangles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamutBoundaryMeshSize(::core::mem::transmute_copy(&pnumvertices), ::core::mem::transmute_copy(&pnumtriangles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGamutBoundaryMesh<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchannels: u32, cvertices: u32, ctriangles: u32, pvertices: *mut f32, ptriangles: *mut GamutShellTriangle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGamutBoundaryMesh(cchannels, cvertices, ctriangles, ::core::mem::transmute_copy(&pvertices), ::core::mem::transmute_copy(&ptriangles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNeutralAxisSize<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccolors: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNeutralAxisSize(::core::mem::transmute_copy(&pccolors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNeutralAxis<Impl: IDeviceModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, pxyzcolors: *mut XYZColorF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNeutralAxis(ccolors, ::core::mem::transmute_copy(&pxyzcolors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDeviceModelPlugIn>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            GetNumChannels::<Impl, OFFSET>,
            DeviceToColorimetricColors::<Impl, OFFSET>,
            ColorimetricToDeviceColors::<Impl, OFFSET>,
            ColorimetricToDeviceColorsWithBlack::<Impl, OFFSET>,
            SetTransformDeviceModelInfo::<Impl, OFFSET>,
            GetPrimarySamples::<Impl, OFFSET>,
            GetGamutBoundaryMeshSize::<Impl, OFFSET>,
            GetGamutBoundaryMesh::<Impl, OFFSET>,
            GetNeutralAxisSize::<Impl, OFFSET>,
            GetNeutralAxis::<Impl, OFFSET>,
        )
    }
}
pub trait IGamutMapModelPlugInImpl: Sized {
    fn Initialize();
    fn SourceToDestinationAppearanceColors();
}
impl ::windows::core::RuntimeName for IGamutMapModelPlugIn {
    const NAME: &'static str = "Windows.Win32.UI.ColorSystem.IGamutMapModelPlugIn";
}
impl IGamutMapModelPlugInVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGamutMapModelPlugInImpl, const OFFSET: isize>() -> IGamutMapModelPlugInVtbl {
        unsafe extern "system" fn Initialize<Impl: IGamutMapModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psrcplugin: ::windows::core::RawPtr, pdestplugin: ::windows::core::RawPtr, psrcgbd: *const GamutBoundaryDescription, pdestgbd: *const GamutBoundaryDescription) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&bstrxml as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psrcplugin as *const <IDeviceModelPlugIn as ::windows::core::Abi>::Abi as *const <IDeviceModelPlugIn as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestplugin as *const <IDeviceModelPlugIn as ::windows::core::Abi>::Abi as *const <IDeviceModelPlugIn as ::windows::core::DefaultType>::DefaultType),
                &*(&psrcgbd as *const <GamutBoundaryDescription as ::windows::core::Abi>::Abi as *const <GamutBoundaryDescription as ::windows::core::DefaultType>::DefaultType),
                &*(&pdestgbd as *const <GamutBoundaryDescription as ::windows::core::Abi>::Abi as *const <GamutBoundaryDescription as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourceToDestinationAppearanceColors<Impl: IGamutMapModelPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccolors: u32, pinputcolors: *const JChColorF, poutputcolors: *mut JChColorF) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceToDestinationAppearanceColors(ccolors, &*(&pinputcolors as *const <JChColorF as ::windows::core::Abi>::Abi as *const <JChColorF as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&poutputcolors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGamutMapModelPlugIn>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, SourceToDestinationAppearanceColors::<Impl, OFFSET>)
    }
}
