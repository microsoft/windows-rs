pub const SPALTERNATESCLSID: windows_sys::core::PCWSTR = windows_sys::core::w!("AlternatesCLSID");
pub type SPCFGNOTIFY = i32;
pub const SPCFGN_ACTIVATE: SPCFGNOTIFY = 3;
pub const SPCFGN_ADD: SPCFGNOTIFY = 0;
pub const SPCFGN_DEACTIVATE: SPCFGNOTIFY = 4;
pub const SPCFGN_INVALIDATE: SPCFGNOTIFY = 2;
pub const SPCFGN_REMOVE: SPCFGNOTIFY = 1;
pub type SPGRAMMARHANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPPARSEINFO {
    pub cbSize: u32,
    pub hRule: SPRULEHANDLE,
    pub ullAudioStreamPosition: u64,
    pub ulAudioSize: u32,
    pub cTransitions: u32,
    pub pPath: *mut SPPATHENTRY,
    pub SREngineID: windows_sys::core::GUID,
    pub ulSREnginePrivateDataSize: u32,
    pub pSREnginePrivateData: *const u8,
    pub fHypothesis: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPPATHENTRY {
    pub hTransition: SPTRANSITIONID,
    pub elem: super::SPPHRASEELEMENT,
}
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPPHRASEALT {
    pub pPhrase: *mut core::ffi::c_void,
    pub ulStartElementInParent: u32,
    pub cElementsInParent: u32,
    pub cElementsInAlternate: u32,
    pub pvAltExtra: *mut core::ffi::c_void,
    pub cbAltExtra: u32,
}
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPPHRASEALTREQUEST {
    pub ulStartElement: u32,
    pub cElements: u32,
    pub ulRequestAltCount: u32,
    pub pvResultExtra: *mut core::ffi::c_void,
    pub cbResultExtra: u32,
    pub pPhrase: *mut core::ffi::c_void,
    pub pRecoContext: *mut core::ffi::c_void,
}
pub type SPPHRASEPROPERTYHANDLE = *mut core::ffi::c_void;
pub type SPPHRASERULEHANDLE = *mut core::ffi::c_void;
pub type SPPROPSRC = i32;
pub const SPPROPSRC_RECO_CTX: SPPROPSRC = 1;
pub const SPPROPSRC_RECO_GRAMMAR: SPPROPSRC = 2;
pub const SPPROPSRC_RECO_INST: SPPROPSRC = 0;
pub type SPRECOCONTEXTHANDLE = *mut core::ffi::c_void;
pub const SPRECOEXTENSION: windows_sys::core::PCWSTR = windows_sys::core::w!("RecoExtension");
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPRECORESULTINFO {
    pub cbSize: u32,
    pub eResultType: SPRESULTTYPE,
    pub fHypothesis: windows_sys::core::BOOL,
    pub fProprietaryAutoPause: windows_sys::core::BOOL,
    pub ullStreamPosStart: u64,
    pub ullStreamPosEnd: u64,
    pub hGrammar: SPGRAMMARHANDLE,
    pub ulSizeEngineData: u32,
    pub pvEngineData: *mut core::ffi::c_void,
    pub pPhrase: *mut core::ffi::c_void,
    pub aPhraseAlts: *mut SPPHRASEALT,
    pub ulNumAlts: u32,
}
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPRECORESULTINFOEX {
    pub Base: SPRECORESULTINFO,
    pub ullStreamTimeStart: u64,
    pub ullStreamTimeEnd: u64,
}
pub type SPRESULTTYPE = i32;
pub const SPRIO_NONE: SPRULEINFOOPT = 0;
pub const SPRT_CFG: SPRESULTTYPE = 0;
pub const SPRT_EMULATED: SPRESULTTYPE = 8;
pub const SPRT_EXTENDABLE_PARSE: SPRESULTTYPE = 16;
pub const SPRT_FALSE_RECOGNITION: SPRESULTTYPE = 4;
pub const SPRT_PROPRIETARY: SPRESULTTYPE = 2;
pub const SPRT_SLM: SPRESULTTYPE = 1;
pub const SPRT_TYPE_MASK: SPRESULTTYPE = 3;
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPRULEENTRY {
    pub hRule: SPRULEHANDLE,
    pub hInitialState: super::SPSTATEHANDLE,
    pub Attributes: u32,
    pub pvClientRuleContext: *mut core::ffi::c_void,
    pub pvClientGrammarContext: *mut core::ffi::c_void,
}
pub type SPRULEHANDLE = *mut core::ffi::c_void;
pub type SPRULEINFOOPT = i32;
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPSTATEINFO {
    pub cAllocatedEntries: u32,
    pub pTransitions: *mut SPTRANSITIONENTRY,
    pub cEpsilons: u32,
    pub cRules: u32,
    pub cWords: u32,
    pub cSpecialTransitions: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SPTMTHREADINFO {
    pub lPoolSize: i32,
    pub lPriority: i32,
    pub ulConcurrencyLimit: u32,
    pub ulMaxQuickAllocThreads: u32,
}
pub const SPTRANSDICTATION: SPTRANSITIONTYPE = 5;
pub const SPTRANSEPSILON: SPTRANSITIONTYPE = 0;
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy)]
pub struct SPTRANSITIONENTRY {
    pub ID: SPTRANSITIONID,
    pub hNextState: super::SPSTATEHANDLE,
    pub Type: u8,
    pub RequiredConfidence: i8,
    pub Anonymous: SPTRANSITIONENTRY_0,
    pub Weight: f32,
    pub Anonymous2: SPTRANSITIONENTRY_1,
}
#[cfg(feature = "sapi")]
impl Default for SPTRANSITIONENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPTRANSITIONENTRY_0 {
    pub fHasProperty: u32,
}
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy)]
pub union SPTRANSITIONENTRY_1 {
    pub Anonymous: SPTRANSITIONENTRY_1_0,
    pub Anonymous2: SPTRANSITIONENTRY_1_1,
    pub Anonymous3: SPTRANSITIONENTRY_1_2,
}
#[cfg(feature = "sapi")]
impl Default for SPTRANSITIONENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPTRANSITIONENTRY_1_0 {
    pub hRuleInitialState: super::SPSTATEHANDLE,
    pub hRule: SPRULEHANDLE,
    pub pvClientRuleContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPTRANSITIONENTRY_1_1 {
    pub hWord: SPWORDHANDLE,
    pub pvClientWordContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPTRANSITIONENTRY_1_2 {
    pub pvGrammarCookie: *mut core::ffi::c_void,
}
pub type SPTRANSITIONID = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct SPTRANSITIONPROPERTY {
    pub pszName: windows_sys::core::PCWSTR,
    pub ulId: u32,
    pub pszValue: windows_sys::core::PCWSTR,
    pub vValue: super::VARIANT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for SPTRANSITIONPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SPTRANSITIONTYPE = i32;
pub const SPTRANSRULE: SPTRANSITIONTYPE = 2;
pub const SPTRANSTEXTBUF: SPTRANSITIONTYPE = 3;
pub const SPTRANSWILDCARD: SPTRANSITIONTYPE = 4;
pub const SPTRANSWORD: SPTRANSITIONTYPE = 1;
pub type SPVESACTIONS = i32;
pub const SPVES_ABORT: SPVESACTIONS = 1;
pub const SPVES_CONTINUE: SPVESACTIONS = 0;
pub const SPVES_RATE: SPVESACTIONS = 4;
pub const SPVES_SKIP: SPVESACTIONS = 2;
pub const SPVES_VOLUME: SPVESACTIONS = 8;
pub type SPVSKIPTYPE = i32;
pub const SPVST_SENTENCE: SPVSKIPTYPE = 1;
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPVTEXTFRAG {
    pub pNext: *mut Self,
    pub State: super::SPVSTATE,
    pub pTextStart: windows_sys::core::PCWSTR,
    pub ulTextLen: u32,
    pub ulTextSrcOffset: u32,
}
pub const SPWIO_NONE: SPWORDINFOOPT = 0;
pub const SPWIO_WANT_TEXT: SPWORDINFOOPT = 1;
#[repr(C)]
#[cfg(feature = "sapi")]
#[derive(Clone, Copy, Default)]
pub struct SPWORDENTRY {
    pub hWord: SPWORDHANDLE,
    pub LangID: u16,
    pub pszDisplayText: *mut u16,
    pub pszLexicalForm: *mut u16,
    pub aPhoneId: *mut super::SPPHONEID,
    pub pvClientContext: *mut core::ffi::c_void,
}
pub type SPWORDHANDLE = *mut core::ffi::c_void;
pub type SPWORDINFOOPT = i32;
pub const SR_LOCALIZED_DESCRIPTION: windows_sys::core::PCWSTR = windows_sys::core::w!("Description");
pub const SpDataKey: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd9f6ee60_58c9_458b_88e1_2f908fd7f87c);
pub const SpGramCompBackend: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xda93e903_c843_11d2_a084_00c04f8ef9b5);
pub const SpGrammarCompiler: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb1e29d59_a675_11d2_8302_00c04f8ee6c0);
pub const SpITNProcessor: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x12d73610_a1c9_11d3_bc90_00c04f72df9f);
pub const SpObjectTokenEnum: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3918d75f_0acb_41f2_b733_92aa15bcecf6);
pub const SpPhraseBuilder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x777b6bbd_2ff2_11d3_88fe_00c04f8ef9b5);
pub const SpW3CGrammarCompiler: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd2c13906_51ef_454e_bc67_a52475ff074c);
