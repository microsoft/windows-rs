#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IAutomationPeerOverrides_Impl: Sized {
    fn GetPatternCore(&mut self, patterninterface: PatternInterface) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAcceleratorKeyCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAccessKeyCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAutomationControlTypeCore(&mut self) -> ::windows::core::Result<AutomationControlType>;
    fn GetAutomationIdCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBoundingRectangleCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn GetChildrenCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeer>>;
    fn GetClassNameCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetClickablePointCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn GetHelpTextCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemStatusCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemTypeCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLabeledByCore(&mut self) -> ::windows::core::Result<AutomationPeer>;
    fn GetLocalizedControlTypeCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNameCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOrientationCore(&mut self) -> ::windows::core::Result<AutomationOrientation>;
    fn HasKeyboardFocusCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsContentElementCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsControlElementCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsEnabledCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsKeyboardFocusableCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsOffscreenCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsPasswordCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsRequiredForFormCore(&mut self) -> ::windows::core::Result<bool>;
    fn SetFocusCore(&mut self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn GetPeerFromPointCore(&mut self, point: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<AutomationPeer>;
    fn GetLiveSettingCore(&mut self) -> ::windows::core::Result<AutomationLiveSetting>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IAutomationPeerOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides_Vtbl {
        unsafe extern "system" fn GetPatternCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterninterface: PatternInterface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatternCore(patterninterface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcceleratorKeyCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcceleratorKeyCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessKeyCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessKeyCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationControlTypeCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationControlType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomationControlTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationIdCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomationIdCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangleCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingRectangleCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildrenCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassNameCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassNameCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePointCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClickablePointCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpTextCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpTextCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemStatusCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemStatusCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemTypeCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLabeledByCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLabeledByCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedControlTypeCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalizedControlTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientationCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOrientationCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKeyboardFocusCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasKeyboardFocusCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContentElementCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContentElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlElementCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsControlElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsKeyboardFocusableCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsKeyboardFocusableCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOffscreenCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOffscreenCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPasswordCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPasswordCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequiredForFormCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequiredForFormCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusCore().into()
        }
        unsafe extern "system" fn GetPeerFromPointCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPeerFromPointCore(&*(&point as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiveSettingCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLiveSettingCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides, BASE_OFFSET>(),
            GetPatternCore: GetPatternCore::<Impl, IMPL_OFFSET>,
            GetAcceleratorKeyCore: GetAcceleratorKeyCore::<Impl, IMPL_OFFSET>,
            GetAccessKeyCore: GetAccessKeyCore::<Impl, IMPL_OFFSET>,
            GetAutomationControlTypeCore: GetAutomationControlTypeCore::<Impl, IMPL_OFFSET>,
            GetAutomationIdCore: GetAutomationIdCore::<Impl, IMPL_OFFSET>,
            GetBoundingRectangleCore: GetBoundingRectangleCore::<Impl, IMPL_OFFSET>,
            GetChildrenCore: GetChildrenCore::<Impl, IMPL_OFFSET>,
            GetClassNameCore: GetClassNameCore::<Impl, IMPL_OFFSET>,
            GetClickablePointCore: GetClickablePointCore::<Impl, IMPL_OFFSET>,
            GetHelpTextCore: GetHelpTextCore::<Impl, IMPL_OFFSET>,
            GetItemStatusCore: GetItemStatusCore::<Impl, IMPL_OFFSET>,
            GetItemTypeCore: GetItemTypeCore::<Impl, IMPL_OFFSET>,
            GetLabeledByCore: GetLabeledByCore::<Impl, IMPL_OFFSET>,
            GetLocalizedControlTypeCore: GetLocalizedControlTypeCore::<Impl, IMPL_OFFSET>,
            GetNameCore: GetNameCore::<Impl, IMPL_OFFSET>,
            GetOrientationCore: GetOrientationCore::<Impl, IMPL_OFFSET>,
            HasKeyboardFocusCore: HasKeyboardFocusCore::<Impl, IMPL_OFFSET>,
            IsContentElementCore: IsContentElementCore::<Impl, IMPL_OFFSET>,
            IsControlElementCore: IsControlElementCore::<Impl, IMPL_OFFSET>,
            IsEnabledCore: IsEnabledCore::<Impl, IMPL_OFFSET>,
            IsKeyboardFocusableCore: IsKeyboardFocusableCore::<Impl, IMPL_OFFSET>,
            IsOffscreenCore: IsOffscreenCore::<Impl, IMPL_OFFSET>,
            IsPasswordCore: IsPasswordCore::<Impl, IMPL_OFFSET>,
            IsRequiredForFormCore: IsRequiredForFormCore::<Impl, IMPL_OFFSET>,
            SetFocusCore: SetFocusCore::<Impl, IMPL_OFFSET>,
            GetPeerFromPointCore: GetPeerFromPointCore::<Impl, IMPL_OFFSET>,
            GetLiveSettingCore: GetLiveSettingCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IAutomationPeerOverrides2_Impl: Sized {
    fn ShowContextMenuCore(&mut self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn GetControlledPeersCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AutomationPeer>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides2";
}
#[cfg(feature = "Foundation_Collections")]
impl IAutomationPeerOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides2_Vtbl {
        unsafe extern "system" fn ShowContextMenuCore<Impl: IAutomationPeerOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenuCore().into()
        }
        unsafe extern "system" fn GetControlledPeersCore<Impl: IAutomationPeerOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControlledPeersCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides2, BASE_OFFSET>(),
            ShowContextMenuCore: ShowContextMenuCore::<Impl, IMPL_OFFSET>,
            GetControlledPeersCore: GetControlledPeersCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
pub trait IAutomationPeerOverrides3_Impl: Sized {
    fn NavigateCore(&mut self, direction: AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetElementFromPointCore(&mut self, pointinwindowcoordinates: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetFocusedElementCore(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAnnotationsCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeerAnnotation>>;
    fn GetPositionInSetCore(&mut self) -> ::windows::core::Result<i32>;
    fn GetSizeOfSetCore(&mut self) -> ::windows::core::Result<i32>;
    fn GetLevelCore(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IAutomationPeerOverrides3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides3_Vtbl {
        unsafe extern "system" fn NavigateCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigateCore(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementFromPointCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointinwindowcoordinates: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElementFromPointCore(&*(&pointinwindowcoordinates as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElementCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocusedElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationsCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationsCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPositionInSetCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPositionInSetCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeOfSetCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeOfSetCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevelCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevelCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides3, BASE_OFFSET>(),
            NavigateCore: NavigateCore::<Impl, IMPL_OFFSET>,
            GetElementFromPointCore: GetElementFromPointCore::<Impl, IMPL_OFFSET>,
            GetFocusedElementCore: GetFocusedElementCore::<Impl, IMPL_OFFSET>,
            GetAnnotationsCore: GetAnnotationsCore::<Impl, IMPL_OFFSET>,
            GetPositionInSetCore: GetPositionInSetCore::<Impl, IMPL_OFFSET>,
            GetSizeOfSetCore: GetSizeOfSetCore::<Impl, IMPL_OFFSET>,
            GetLevelCore: GetLevelCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides3 as ::windows::core::Interface>::IID
    }
}
pub trait IAutomationPeerOverrides4_Impl: Sized {
    fn GetLandmarkTypeCore(&mut self) -> ::windows::core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkTypeCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAutomationPeerOverrides4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides4";
}
impl IAutomationPeerOverrides4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides4_Vtbl {
        unsafe extern "system" fn GetLandmarkTypeCore<Impl: IAutomationPeerOverrides4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLandmarkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLandmarkTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedLandmarkTypeCore<Impl: IAutomationPeerOverrides4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalizedLandmarkTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides4, BASE_OFFSET>(),
            GetLandmarkTypeCore: GetLandmarkTypeCore::<Impl, IMPL_OFFSET>,
            GetLocalizedLandmarkTypeCore: GetLocalizedLandmarkTypeCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IAutomationPeerOverrides5_Impl: Sized {
    fn IsPeripheralCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsDataValidForFormCore(&mut self) -> ::windows::core::Result<bool>;
    fn GetFullDescriptionCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDescribedByCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsToCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsFromCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides5";
}
#[cfg(feature = "Foundation_Collections")]
impl IAutomationPeerOverrides5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides5_Vtbl {
        unsafe extern "system" fn IsPeripheralCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPeripheralCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataValidForFormCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataValidForFormCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullDescriptionCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullDescriptionCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescribedByCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescribedByCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsToCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlowsToCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsFromCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlowsFromCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides5, BASE_OFFSET>(),
            IsPeripheralCore: IsPeripheralCore::<Impl, IMPL_OFFSET>,
            IsDataValidForFormCore: IsDataValidForFormCore::<Impl, IMPL_OFFSET>,
            GetFullDescriptionCore: GetFullDescriptionCore::<Impl, IMPL_OFFSET>,
            GetDescribedByCore: GetDescribedByCore::<Impl, IMPL_OFFSET>,
            GetFlowsToCore: GetFlowsToCore::<Impl, IMPL_OFFSET>,
            GetFlowsFromCore: GetFlowsFromCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides5 as ::windows::core::Interface>::IID
    }
}
pub trait IAutomationPeerOverrides6_Impl: Sized {
    fn GetCultureCore(&mut self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IAutomationPeerOverrides6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides6";
}
impl IAutomationPeerOverrides6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides6_Vtbl {
        unsafe extern "system" fn GetCultureCore<Impl: IAutomationPeerOverrides6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCultureCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides6, BASE_OFFSET>(),
            GetCultureCore: GetCultureCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides6 as ::windows::core::Interface>::IID
    }
}
pub trait IAutomationPeerOverrides8_Impl: Sized {
    fn GetHeadingLevelCore(&mut self) -> ::windows::core::Result<AutomationHeadingLevel>;
}
impl ::windows::core::RuntimeName for IAutomationPeerOverrides8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides8";
}
impl IAutomationPeerOverrides8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides8_Vtbl {
        unsafe extern "system" fn GetHeadingLevelCore<Impl: IAutomationPeerOverrides8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationHeadingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeadingLevelCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides8, BASE_OFFSET>(),
            GetHeadingLevelCore: GetHeadingLevelCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides8 as ::windows::core::Interface>::IID
    }
}
pub trait IAutomationPeerOverrides9_Impl: Sized {
    fn IsDialogCore(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IAutomationPeerOverrides9 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides9";
}
impl IAutomationPeerOverrides9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides9_Vtbl {
        unsafe extern "system" fn IsDialogCore<Impl: IAutomationPeerOverrides9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDialogCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides9, BASE_OFFSET>(),
            IsDialogCore: IsDialogCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides9 as ::windows::core::Interface>::IID
    }
}
pub trait IItemsControlAutomationPeerOverrides2_Impl: Sized {
    fn OnCreateItemAutomationPeer(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
impl ::windows::core::RuntimeName for IItemsControlAutomationPeerOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeerOverrides2";
}
impl IItemsControlAutomationPeerOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeerOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlAutomationPeerOverrides2_Vtbl {
        unsafe extern "system" fn OnCreateItemAutomationPeer<Impl: IItemsControlAutomationPeerOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCreateItemAutomationPeer(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsControlAutomationPeerOverrides2, BASE_OFFSET>(),
            OnCreateItemAutomationPeer: OnCreateItemAutomationPeer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsControlAutomationPeerOverrides2 as ::windows::core::Interface>::IID
    }
}
