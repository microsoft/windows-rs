#[doc = "*Required features: `\"Win32_Storage_IndexServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IFilter_Impl: Sized {
    fn Init(&self, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32;
    fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> i32;
    fn GetText(&self, pcwcbuffer: *mut u32, awcbuffer: ::windows::core::PWSTR) -> i32;
    fn GetValue(&self, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32;
    fn BindRegion(&self, origpos: &FILTERREGION, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IFilter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFilter_Impl, const OFFSET: isize>() -> IFilter_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&cattributes), ::core::mem::transmute_copy(&aattributes), ::core::mem::transmute_copy(&pflags))
        }
        unsafe extern "system" fn GetChunk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstat: *mut STAT_CHUNK) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChunk(::core::mem::transmute_copy(&pstat))
        }
        unsafe extern "system" fn GetText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwcbuffer: *mut u32, awcbuffer: ::windows::core::PWSTR) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(::core::mem::transmute_copy(&pcwcbuffer), ::core::mem::transmute_copy(&awcbuffer))
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetValue(::core::mem::transmute_copy(&pppropvalue))
        }
        unsafe extern "system" fn BindRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origpos: FILTERREGION, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindRegion(::core::mem::transmute(&origpos), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetChunk: GetChunk::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            BindRegion: BindRegion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_IndexServer\"`, `\"implement\"`*"]
pub trait IPhraseSink_Impl: Sized {
    fn PutSmallPhrase(&self, pwcnoun: &::windows::core::PCWSTR, cwcnoun: u32, pwcmodifier: &::windows::core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::core::Result<()>;
    fn PutPhrase(&self, pwcphrase: &::windows::core::PCWSTR, cwcphrase: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPhraseSink {}
impl IPhraseSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhraseSink_Impl, const OFFSET: isize>() -> IPhraseSink_Vtbl {
        unsafe extern "system" fn PutSmallPhrase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhraseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcnoun: ::windows::core::PCWSTR, cwcnoun: u32, pwcmodifier: ::windows::core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutSmallPhrase(::core::mem::transmute(&pwcnoun), ::core::mem::transmute_copy(&cwcnoun), ::core::mem::transmute(&pwcmodifier), ::core::mem::transmute_copy(&cwcmodifier), ::core::mem::transmute_copy(&ulattachmenttype)).into()
        }
        unsafe extern "system" fn PutPhrase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPhraseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcphrase: ::windows::core::PCWSTR, cwcphrase: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutPhrase(::core::mem::transmute(&pwcphrase), ::core::mem::transmute_copy(&cwcphrase)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutSmallPhrase: PutSmallPhrase::<Identity, Impl, OFFSET>,
            PutPhrase: PutPhrase::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhraseSink as ::windows::core::ComInterface>::IID
    }
}
