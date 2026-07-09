pub type CORRECTIVE_ACTION = i32;
pub const CORRECTIVE_ACTION_DELETE: CORRECTIVE_ACTION = 3;
pub const CORRECTIVE_ACTION_GET_SUGGESTIONS: CORRECTIVE_ACTION = 1;
pub const CORRECTIVE_ACTION_NONE: CORRECTIVE_ACTION = 0;
pub const CORRECTIVE_ACTION_REPLACE: CORRECTIVE_ACTION = 2;
windows_core::imp::define_interface!(IEnumSpellingError, IEnumSpellingError_Vtbl, 0x803e3bd4_2828_4410_8290_418d1d73c762);
windows_core::imp::interface_hierarchy!(IEnumSpellingError, windows_core::IUnknown);
impl IEnumSpellingError {
    pub unsafe fn Next(&self) -> windows_core::Result<ISpellingError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSpellingError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumSpellingError_Impl: windows_core::IUnknownImpl {
    fn Next(&self) -> windows_core::Result<ISpellingError>;
}
impl IEnumSpellingError_Vtbl {
    pub const fn new<Identity: IEnumSpellingError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSpellingError_Impl::Next(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Next: Next::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSpellingError as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumSpellingError {}
windows_core::imp::define_interface!(IOptionDescription, IOptionDescription_Vtbl, 0x432e5f85_35cf_4606_a801_6f70277e1d7a);
windows_core::imp::interface_hierarchy!(IOptionDescription, windows_core::IUnknown);
impl IOptionDescription {
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Heading(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Heading)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn Labels(&self) -> windows_core::Result<super::objidlbase::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Labels)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOptionDescription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Heading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub Labels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    Labels: usize,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait IOptionDescription_Impl: windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Heading(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Labels(&self) -> windows_core::Result<super::objidlbase::IEnumString>;
}
#[cfg(feature = "Win32_objidlbase")]
impl IOptionDescription_Vtbl {
    pub const fn new<Identity: IOptionDescription_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::Id(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Heading<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::Heading(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::Description(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Labels<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::Labels(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Heading: Heading::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Labels: Labels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOptionDescription as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for IOptionDescription {}
windows_core::imp::define_interface!(ISpellChecker, ISpellChecker_Vtbl, 0xb6fd0b71_e2bc_4653_8d05_f197e412770b);
windows_core::imp::interface_hierarchy!(ISpellChecker, windows_core::IUnknown);
impl ISpellChecker {
    pub unsafe fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LanguageTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Check<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Check)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn Suggest<P0>(&self, word: P0) -> windows_core::Result<super::objidlbase::IEnumString>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Suggest)(windows_core::Interface::as_raw(self), word.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Add<P0>(&self, word: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), word.param().abi()) }
    }
    pub unsafe fn Ignore<P0>(&self, word: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Ignore)(windows_core::Interface::as_raw(self), word.param().abi()) }
    }
    pub unsafe fn AutoCorrect<P0, P1>(&self, from: P0, to: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AutoCorrect)(windows_core::Interface::as_raw(self), from.param().abi(), to.param().abi()) }
    }
    pub unsafe fn GetOptionValue<P0>(&self, optionid: P0) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionValue)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn OptionIds(&self) -> windows_core::Result<super::objidlbase::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OptionIds)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LocalizedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn add_SpellCheckerChanged<P0>(&self, handler: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ISpellCheckerChangedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_SpellCheckerChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn remove_SpellCheckerChanged(&self, eventcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).remove_SpellCheckerChanged)(windows_core::Interface::as_raw(self), eventcookie) }
    }
    pub unsafe fn GetOptionDescription<P0>(&self, optionid: P0) -> windows_core::Result<IOptionDescription>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionDescription)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ComprehensiveCheck<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComprehensiveCheck)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellChecker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LanguageTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Check: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub Suggest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    Suggest: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Ignore: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AutoCorrect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetOptionValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub OptionIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    OptionIds: usize,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub LocalizedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub add_SpellCheckerChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub remove_SpellCheckerChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOptionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComprehensiveCheck: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait ISpellChecker_Impl: windows_core::IUnknownImpl {
    fn LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Check(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
    fn Suggest(&self, word: &windows_core::PCWSTR) -> windows_core::Result<super::objidlbase::IEnumString>;
    fn Add(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Ignore(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AutoCorrect(&self, from: &windows_core::PCWSTR, to: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetOptionValue(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<u8>;
    fn OptionIds(&self) -> windows_core::Result<super::objidlbase::IEnumString>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn add_SpellCheckerChanged(&self, handler: windows_core::Ref<ISpellCheckerChangedEventHandler>) -> windows_core::Result<u32>;
    fn remove_SpellCheckerChanged(&self, eventcookie: u32) -> windows_core::Result<()>;
    fn GetOptionDescription(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<IOptionDescription>;
    fn ComprehensiveCheck(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
}
#[cfg(feature = "Win32_objidlbase")]
impl ISpellChecker_Vtbl {
    pub const fn new<Identity: ISpellChecker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LanguageTag<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::LanguageTag(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Check<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::Check(this, core::mem::transmute(&text)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Suggest<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::Suggest(this, core::mem::transmute(&word)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker_Impl::Add(this, core::mem::transmute(&word)).into()
            }
        }
        unsafe extern "system" fn Ignore<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker_Impl::Ignore(this, core::mem::transmute(&word)).into()
            }
        }
        unsafe extern "system" fn AutoCorrect<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, from: windows_core::PCWSTR, to: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker_Impl::AutoCorrect(this, core::mem::transmute(&from), core::mem::transmute(&to)).into()
            }
        }
        unsafe extern "system" fn GetOptionValue<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::GetOptionValue(this, core::mem::transmute(&optionid)) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OptionIds<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::OptionIds(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::Id(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LocalizedName<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::LocalizedName(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn add_SpellCheckerChanged<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, eventcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::add_SpellCheckerChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        eventcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn remove_SpellCheckerChanged<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker_Impl::remove_SpellCheckerChanged(this, core::mem::transmute_copy(&eventcookie)).into()
            }
        }
        unsafe extern "system" fn GetOptionDescription<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::GetOptionDescription(this, core::mem::transmute(&optionid)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ComprehensiveCheck<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::ComprehensiveCheck(this, core::mem::transmute(&text)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LanguageTag: LanguageTag::<Identity, OFFSET>,
            Check: Check::<Identity, OFFSET>,
            Suggest: Suggest::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Ignore: Ignore::<Identity, OFFSET>,
            AutoCorrect: AutoCorrect::<Identity, OFFSET>,
            GetOptionValue: GetOptionValue::<Identity, OFFSET>,
            OptionIds: OptionIds::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            LocalizedName: LocalizedName::<Identity, OFFSET>,
            add_SpellCheckerChanged: add_SpellCheckerChanged::<Identity, OFFSET>,
            remove_SpellCheckerChanged: remove_SpellCheckerChanged::<Identity, OFFSET>,
            GetOptionDescription: GetOptionDescription::<Identity, OFFSET>,
            ComprehensiveCheck: ComprehensiveCheck::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellChecker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for ISpellChecker {}
windows_core::imp::define_interface!(ISpellChecker2, ISpellChecker2_Vtbl, 0xe7ed1c71_87f7_4378_a840_c9200dacee47);
impl core::ops::Deref for ISpellChecker2 {
    type Target = ISpellChecker;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpellChecker2, windows_core::IUnknown, ISpellChecker);
impl ISpellChecker2 {
    pub unsafe fn Remove<P0>(&self, word: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), word.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellChecker2_Vtbl {
    pub base__: ISpellChecker_Vtbl,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait ISpellChecker2_Impl: ISpellChecker_Impl {
    fn Remove(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_objidlbase")]
impl ISpellChecker2_Vtbl {
    pub const fn new<Identity: ISpellChecker2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Remove<Identity: ISpellChecker2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellChecker2_Impl::Remove(this, core::mem::transmute(&word)).into()
            }
        }
        Self { base__: ISpellChecker_Vtbl::new::<Identity, OFFSET>(), Remove: Remove::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellChecker2 as windows_core::Interface>::IID || iid == &<ISpellChecker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for ISpellChecker2 {}
windows_core::imp::define_interface!(ISpellCheckerChangedEventHandler, ISpellCheckerChangedEventHandler_Vtbl, 0x0b83a5b0_792f_4eab_9799_acf52c5ed08a);
windows_core::imp::interface_hierarchy!(ISpellCheckerChangedEventHandler, windows_core::IUnknown);
impl ISpellCheckerChangedEventHandler {
    pub unsafe fn Invoke<P0>(&self, sender: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISpellChecker>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellCheckerChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpellCheckerChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self, sender: windows_core::Ref<ISpellChecker>) -> windows_core::Result<()>;
}
impl ISpellCheckerChangedEventHandler_Vtbl {
    pub const fn new<Identity: ISpellCheckerChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: ISpellCheckerChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpellCheckerChangedEventHandler_Impl::Invoke(this, core::mem::transmute_copy(&sender)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckerChangedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpellCheckerChangedEventHandler {}
windows_core::imp::define_interface!(ISpellCheckerFactory, ISpellCheckerFactory_Vtbl, 0x8e018a9d_2415_4677_bf08_794ea61f94bb);
windows_core::imp::interface_hierarchy!(ISpellCheckerFactory, windows_core::IUnknown);
impl ISpellCheckerFactory {
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn SupportedLanguages(&self) -> windows_core::Result<super::objidlbase::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedLanguages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsSupported<P0>(&self, languagetag: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSupported)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateSpellChecker<P0>(&self, languagetag: P0) -> windows_core::Result<ISpellChecker>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSpellChecker)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellCheckerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_objidlbase")]
    pub SupportedLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    SupportedLanguages: usize,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateSpellChecker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_objidlbase")]
pub trait ISpellCheckerFactory_Impl: windows_core::IUnknownImpl {
    fn SupportedLanguages(&self) -> windows_core::Result<super::objidlbase::IEnumString>;
    fn IsSupported(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn CreateSpellChecker(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<ISpellChecker>;
}
#[cfg(feature = "Win32_objidlbase")]
impl ISpellCheckerFactory_Vtbl {
    pub const fn new<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SupportedLanguages<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::SupportedLanguages(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSupported<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::IsSupported(this, core::mem::transmute(&languagetag)) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSpellChecker<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languagetag: windows_core::PCWSTR, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::CreateSpellChecker(this, core::mem::transmute(&languagetag)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SupportedLanguages: SupportedLanguages::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
            CreateSpellChecker: CreateSpellChecker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckerFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl windows_core::RuntimeName for ISpellCheckerFactory {}
windows_core::imp::define_interface!(ISpellingError, ISpellingError_Vtbl, 0xb7c82d61_fbe8_4b47_9b27_6c0d2e0de0a3);
windows_core::imp::interface_hierarchy!(ISpellingError, windows_core::IUnknown);
impl ISpellingError {
    pub unsafe fn StartIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Length(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrectiveAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Replacement(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Replacement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellingError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CorrectiveAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CORRECTIVE_ACTION) -> windows_core::HRESULT,
    pub Replacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait ISpellingError_Impl: windows_core::IUnknownImpl {
    fn StartIndex(&self) -> windows_core::Result<u32>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION>;
    fn Replacement(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl ISpellingError_Vtbl {
    pub const fn new<Identity: ISpellingError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartIndex<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::StartIndex(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Length<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::Length(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorrectiveAction<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut CORRECTIVE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::CorrectiveAction(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Replacement<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::Replacement(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartIndex: StartIndex::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            CorrectiveAction: CorrectiveAction::<Identity, OFFSET>,
            Replacement: Replacement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellingError as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpellingError {}
windows_core::imp::define_interface!(IUserDictionariesRegistrar, IUserDictionariesRegistrar_Vtbl, 0xaa176b85_0e12_4844_8e1a_eef1da77f586);
windows_core::imp::interface_hierarchy!(IUserDictionariesRegistrar, windows_core::IUnknown);
impl IUserDictionariesRegistrar {
    pub unsafe fn RegisterUserDictionary<P0, P1>(&self, dictionarypath: P0, languagetag: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterUserDictionary)(windows_core::Interface::as_raw(self), dictionarypath.param().abi(), languagetag.param().abi()) }
    }
    pub unsafe fn UnregisterUserDictionary<P0, P1>(&self, dictionarypath: P0, languagetag: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterUserDictionary)(windows_core::Interface::as_raw(self), dictionarypath.param().abi(), languagetag.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDictionariesRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterUserDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterUserDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IUserDictionariesRegistrar_Impl: windows_core::IUnknownImpl {
    fn RegisterUserDictionary(&self, dictionarypath: &windows_core::PCWSTR, languagetag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterUserDictionary(&self, dictionarypath: &windows_core::PCWSTR, languagetag: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IUserDictionariesRegistrar_Vtbl {
    pub const fn new<Identity: IUserDictionariesRegistrar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterUserDictionary<Identity: IUserDictionariesRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarypath: windows_core::PCWSTR, languagetag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserDictionariesRegistrar_Impl::RegisterUserDictionary(this, core::mem::transmute(&dictionarypath), core::mem::transmute(&languagetag)).into()
            }
        }
        unsafe extern "system" fn UnregisterUserDictionary<Identity: IUserDictionariesRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionarypath: windows_core::PCWSTR, languagetag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUserDictionariesRegistrar_Impl::UnregisterUserDictionary(this, core::mem::transmute(&dictionarypath), core::mem::transmute(&languagetag)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterUserDictionary: RegisterUserDictionary::<Identity, OFFSET>,
            UnregisterUserDictionary: UnregisterUserDictionary::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUserDictionariesRegistrar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUserDictionariesRegistrar {}
pub const MIN_SPELLING_NTDDI: u32 = 100794368;
pub const SpellCheckerFactory: windows_core::GUID = windows_core::GUID::from_u128(0x7ab36653_1796_484b_bdfa_e74f1db7c1dc);
pub type WORDLIST_TYPE = i32;
pub const WORDLIST_TYPE_ADD: WORDLIST_TYPE = 1;
pub const WORDLIST_TYPE_AUTOCORRECT: WORDLIST_TYPE = 3;
pub const WORDLIST_TYPE_EXCLUDE: WORDLIST_TYPE = 2;
pub const WORDLIST_TYPE_IGNORE: WORDLIST_TYPE = 0;
