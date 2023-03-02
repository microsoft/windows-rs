#[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"implement\"`*"]
pub trait ICompositionInteractionSource_Impl: Sized {}
impl ::windows::core::RuntimeName for ICompositionInteractionSource {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.ICompositionInteractionSource";
}
impl ICompositionInteractionSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICompositionInteractionSource_Impl, const OFFSET: isize>() -> ICompositionInteractionSource_Vtbl {
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ICompositionInteractionSource, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionInteractionSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Composition_Interactions\"`, `\"implement\"`*"]
pub trait IInteractionTrackerOwner_Impl: Sized {
    fn CustomAnimationStateEntered(&self, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerCustomAnimationStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn IdleStateEntered(&self, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerIdleStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InertiaStateEntered(&self, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerInertiaStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn InteractingStateEntered(&self, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerInteractingStateEnteredArgs>) -> ::windows::core::Result<()>;
    fn RequestIgnored(&self, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerRequestIgnoredArgs>) -> ::windows::core::Result<()>;
    fn ValuesChanged(&self, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerValuesChangedArgs>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInteractionTrackerOwner {
    const NAME: &'static str = "Windows.UI.Composition.Interactions.IInteractionTrackerOwner";
}
impl IInteractionTrackerOwner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>() -> IInteractionTrackerOwner_Vtbl {
        unsafe extern "system" fn CustomAnimationStateEntered<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CustomAnimationStateEntered(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn IdleStateEntered<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IdleStateEntered(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn InertiaStateEntered<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InertiaStateEntered(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn InteractingStateEntered<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InteractingStateEntered(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn RequestIgnored<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestIgnored(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&args)).into()
        }
        unsafe extern "system" fn ValuesChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ValuesChanged(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&args)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IInteractionTrackerOwner, OFFSET>(),
            CustomAnimationStateEntered: CustomAnimationStateEntered::<Identity, Impl, OFFSET>,
            IdleStateEntered: IdleStateEntered::<Identity, Impl, OFFSET>,
            InertiaStateEntered: InertiaStateEntered::<Identity, Impl, OFFSET>,
            InteractingStateEntered: InteractingStateEntered::<Identity, Impl, OFFSET>,
            RequestIgnored: RequestIgnored::<Identity, Impl, OFFSET>,
            ValuesChanged: ValuesChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractionTrackerOwner as ::windows::core::ComInterface>::IID
    }
}
