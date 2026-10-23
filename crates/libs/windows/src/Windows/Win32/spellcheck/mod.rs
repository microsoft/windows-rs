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
            (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub unsafe fn get_Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Heading(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Heading)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn get_Labels(&self) -> windows_core::Result<super::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Labels)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOptionDescription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub get_Heading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub get_Labels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    get_Labels: usize,
}
#[cfg(feature = "objidlbase")]
pub trait IOptionDescription_Impl: windows_core::IUnknownImpl {
    fn get_Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn get_Heading(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn get_Description(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn get_Labels(&self) -> windows_core::Result<super::IEnumString>;
}
#[cfg(feature = "objidlbase")]
impl IOptionDescription_Vtbl {
    pub const fn new<Identity: IOptionDescription_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Id<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::get_Id(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Heading<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::get_Heading(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::get_Description(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Labels<Identity: IOptionDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOptionDescription_Impl::get_Labels(this) {
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
            get_Id: get_Id::<Identity, OFFSET>,
            get_Heading: get_Heading::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            get_Labels: get_Labels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOptionDescription as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IOptionDescription {}
windows_core::imp::define_interface!(ISpellChecker, ISpellChecker_Vtbl, 0xb6fd0b71_e2bc_4653_8d05_f197e412770b);
windows_core::imp::interface_hierarchy!(ISpellChecker, windows_core::IUnknown);
impl ISpellChecker {
    pub unsafe fn get_LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LanguageTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Check<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Check)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Suggest<P0>(&self, word: P0) -> windows_core::Result<super::IEnumString>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Suggest)(windows_core::Interface::as_raw(self), word.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetOptionValue<P0>(&self, optionid: P0) -> windows_core::Result<super::BYTE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionValue)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn get_OptionIds(&self) -> windows_core::Result<super::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OptionIds)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LocalizedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn add_SpellCheckerChanged<P0>(&self, handler: P0) -> windows_core::Result<super::DWORD>
    where
        P0: windows_core::Param<ISpellCheckerChangedEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).add_SpellCheckerChanged)(windows_core::Interface::as_raw(self), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn remove_SpellCheckerChanged(&self, eventcookie: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).remove_SpellCheckerChanged)(windows_core::Interface::as_raw(self), eventcookie) }
    }
    pub unsafe fn GetOptionDescription<P0>(&self, optionid: P0) -> windows_core::Result<IOptionDescription>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptionDescription)(windows_core::Interface::as_raw(self), optionid.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ComprehensiveCheck<P0>(&self, text: P0) -> windows_core::Result<IEnumSpellingError>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComprehensiveCheck)(windows_core::Interface::as_raw(self), text.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellChecker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_LanguageTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Check: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub Suggest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Suggest: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Ignore: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AutoCorrect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetOptionValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetOptionValue: usize,
    #[cfg(feature = "objidlbase")]
    pub get_OptionIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    get_OptionIds: usize,
    pub get_Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub get_LocalizedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub add_SpellCheckerChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    add_SpellCheckerChanged: usize,
    #[cfg(feature = "shared")]
    pub remove_SpellCheckerChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    remove_SpellCheckerChanged: usize,
    pub GetOptionDescription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComprehensiveCheck: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
pub trait ISpellChecker_Impl: windows_core::IUnknownImpl {
    fn get_LanguageTag(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Check(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
    fn Suggest(&self, word: &windows_core::PCWSTR) -> windows_core::Result<super::IEnumString>;
    fn Add(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Ignore(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AutoCorrect(&self, from: &windows_core::PCWSTR, to: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetOptionValue(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<super::BYTE>;
    fn get_OptionIds(&self) -> windows_core::Result<super::IEnumString>;
    fn get_Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn get_LocalizedName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn add_SpellCheckerChanged(&self, handler: windows_core::Ref<ISpellCheckerChangedEventHandler>) -> windows_core::Result<super::DWORD>;
    fn remove_SpellCheckerChanged(&self, eventcookie: super::DWORD) -> windows_core::Result<()>;
    fn GetOptionDescription(&self, optionid: &windows_core::PCWSTR) -> windows_core::Result<IOptionDescription>;
    fn ComprehensiveCheck(&self, text: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpellingError>;
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
impl ISpellChecker_Vtbl {
    pub const fn new<Identity: ISpellChecker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_LanguageTag<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::get_LanguageTag(this) {
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
        unsafe extern "system" fn GetOptionValue<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optionid: windows_core::PCWSTR, value: *mut super::BYTE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_OptionIds<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::get_OptionIds(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Id<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::get_Id(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LocalizedName<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellChecker_Impl::get_LocalizedName(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn add_SpellCheckerChanged<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, eventcookie: *mut super::DWORD) -> windows_core::HRESULT {
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
        unsafe extern "system" fn remove_SpellCheckerChanged<Identity: ISpellChecker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: super::DWORD) -> windows_core::HRESULT {
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
            get_LanguageTag: get_LanguageTag::<Identity, OFFSET>,
            Check: Check::<Identity, OFFSET>,
            Suggest: Suggest::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Ignore: Ignore::<Identity, OFFSET>,
            AutoCorrect: AutoCorrect::<Identity, OFFSET>,
            GetOptionValue: GetOptionValue::<Identity, OFFSET>,
            get_OptionIds: get_OptionIds::<Identity, OFFSET>,
            get_Id: get_Id::<Identity, OFFSET>,
            get_LocalizedName: get_LocalizedName::<Identity, OFFSET>,
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
#[cfg(all(feature = "objidlbase", feature = "shared"))]
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
#[cfg(all(feature = "objidlbase", feature = "shared"))]
pub trait ISpellChecker2_Impl: ISpellChecker_Impl {
    fn Remove(&self, word: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidlbase", feature = "shared"))]
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
#[cfg(all(feature = "objidlbase", feature = "shared"))]
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
    #[cfg(feature = "objidlbase")]
    pub unsafe fn get_SupportedLanguages(&self) -> windows_core::Result<super::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedLanguages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(self).CreateSpellChecker)(windows_core::Interface::as_raw(self), languagetag.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellCheckerFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub get_SupportedLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    get_SupportedLanguages: usize,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CreateSpellChecker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait ISpellCheckerFactory_Impl: windows_core::IUnknownImpl {
    fn get_SupportedLanguages(&self) -> windows_core::Result<super::IEnumString>;
    fn IsSupported(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn CreateSpellChecker(&self, languagetag: &windows_core::PCWSTR) -> windows_core::Result<ISpellChecker>;
}
#[cfg(feature = "objidlbase")]
impl ISpellCheckerFactory_Vtbl {
    pub const fn new<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_SupportedLanguages<Identity: ISpellCheckerFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellCheckerFactory_Impl::get_SupportedLanguages(this) {
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
            get_SupportedLanguages: get_SupportedLanguages::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
            CreateSpellChecker: CreateSpellChecker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellCheckerFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for ISpellCheckerFactory {}
windows_core::imp::define_interface!(ISpellingError, ISpellingError_Vtbl, 0xb7c82d61_fbe8_4b47_9b27_6c0d2e0de0a3);
windows_core::imp::interface_hierarchy!(ISpellingError, windows_core::IUnknown);
impl ISpellingError {
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartIndex(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Length(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CorrectiveAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Replacement(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Replacement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpellingError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub get_StartIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartIndex: usize,
    #[cfg(feature = "shared")]
    pub get_Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Length: usize,
    pub get_CorrectiveAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CORRECTIVE_ACTION) -> windows_core::HRESULT,
    pub get_Replacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait ISpellingError_Impl: windows_core::IUnknownImpl {
    fn get_StartIndex(&self) -> windows_core::Result<super::ULONG>;
    fn get_Length(&self) -> windows_core::Result<super::ULONG>;
    fn get_CorrectiveAction(&self) -> windows_core::Result<CORRECTIVE_ACTION>;
    fn get_Replacement(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "shared")]
impl ISpellingError_Vtbl {
    pub const fn new<Identity: ISpellingError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_StartIndex<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::get_StartIndex(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Length<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::get_Length(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CorrectiveAction<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut CORRECTIVE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::get_CorrectiveAction(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Replacement<Identity: ISpellingError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpellingError_Impl::get_Replacement(this) {
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
            get_StartIndex: get_StartIndex::<Identity, OFFSET>,
            get_Length: get_Length::<Identity, OFFSET>,
            get_CorrectiveAction: get_CorrectiveAction::<Identity, OFFSET>,
            get_Replacement: get_Replacement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpellingError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
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
pub const MIN_SPELLING_NTDDI: i32 = 100794368;
pub const SpellCheckerFactory: windows_core::GUID = windows_core::GUID::from_u128(0x7ab36653_1796_484b_bdfa_e74f1db7c1dc);
pub type WORDLIST_TYPE = i32;
pub const WORDLIST_TYPE_ADD: WORDLIST_TYPE = 1;
pub const WORDLIST_TYPE_AUTOCORRECT: WORDLIST_TYPE = 3;
pub const WORDLIST_TYPE_EXCLUDE: WORDLIST_TYPE = 2;
pub const WORDLIST_TYPE_IGNORE: WORDLIST_TYPE = 0;
