use super::*;
use std::ffi::c_void;

windows_targets::link!("kernel32.dll" "system" fn CreateEventW(attributes: *const c_void, manual_reset: i32, initial_state: i32, name: *const c_void) -> isize);
windows_targets::link!("kernel32.dll" "system" fn EncodePointer(ptr: *const c_void) -> *mut c_void);
windows_targets::link!("kernel32.dll" "system" fn FormatMessageW(flags: u32, source: *const c_void, code: u32, lang: u32, buffer: core::PWSTR, len: u32, args: *const *const i8) -> u32);
windows_targets::link!("kernel32.dll" "system" fn FreeLibrary(library: isize) -> i32);
windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> u32);
windows_targets::link!("kernel32.dll" "system" fn GetProcAddress(library: isize, name: core::PCSTR) -> *const c_void);
windows_targets::link!("kernel32.dll" "system" fn GetProcessHeap() -> isize);
windows_targets::link!("kernel32.dll" "system" fn HeapAlloc(heap: isize, flags: u32, len: usize) -> *mut c_void);
windows_targets::link!("kernel32.dll" "system" fn HeapFree(heap: isize, flags: u32, ptr: *const c_void) -> i32);
windows_targets::link!("kernel32.dll" "system" fn LoadLibraryA(name: core::PCSTR) -> isize);
windows_targets::link!("kernel32.dll" "system" fn SetEvent(handle: isize) -> i32);
windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObject(handle: isize, milliseconds: u32) -> u32);
windows_targets::link!("kernel32.dll""system" fn CloseHandle(handle: isize) -> i32);
windows_targets::link!("ole32.dll" "system" fn CoCreateGuid(guid: *mut core::GUID) -> core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoTaskMemAlloc(len: usize) -> *mut c_void);
windows_targets::link!("ole32.dll" "system" fn CoTaskMemFree(ptr: *const c_void) -> ());
windows_targets::link!("ole32.dll" "system" fn RoGetAgileReference(options: i32, iid: &core::GUID, object: *const c_void, reference: *mut *mut c_void) -> core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn GetErrorInfo(reserved: u32, info: *mut *mut c_void) -> core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn SetErrorInfo(reserved: u32, info: *const c_void) -> core::HRESULT);
windows_targets::link!("oleaut32.dll" "system" fn SysAllocStringLen(input: *const u16, len: u32) -> *const u16);
windows_targets::link!("oleaut32.dll" "system" fn SysFreeString(bstr: *const u16) -> ());
windows_targets::link!("oleaut32.dll" "system" fn SysStringLen(bstr: *const u16) -> u32);

pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 256;
pub const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 4096;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 512;
pub const AGILEREFERENCE_DEFAULT: i32 = 0;

pub const ERROR_NO_UNICODE_TRANSLATION: u32 = 1113u32;
pub const CLASS_E_CLASSNOTAVAILABLE: core::HRESULT = core::HRESULT(-2147221231i32);
pub const CO_E_NOTINITIALIZED: core::HRESULT = core::HRESULT(-2147221008i32);
pub const E_NOINTERFACE: core::HRESULT = core::HRESULT(-2147467262i32);
pub const E_OUTOFMEMORY: core::HRESULT = core::HRESULT(-2147024882i32);
pub const RPC_E_DISCONNECTED: core::HRESULT = core::HRESULT(-2147417848i32);
pub const JSCRIPT_E_CANTEXECUTE: core::HRESULT = core::HRESULT(-1996357631i32);
pub const S_OK: core::HRESULT = core::HRESULT(0i32);
pub const E_BOUNDS: core::HRESULT = core::HRESULT(-2147483637i32);

#[repr(transparent)]
pub struct IAgileObject(core::IUnknown);
impl std::clone::Clone for IAgileObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IAgileObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IAgileObject {}
unsafe impl core::Interface for IAgileObject {
    type Vtable = IAgileObject_Vtbl;
}
unsafe impl core::ComInterface for IAgileObject {
    const IID: core::GUID = core::GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base__: core::IUnknown_Vtbl,
}

#[repr(transparent)]
pub struct IErrorInfo(core::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetDescription(&self) -> core::Result<core::BSTR> {
        let mut result__ = core::zeroed::<core::BSTR>();
        (core::Interface::vtable(self).GetDescription)(core::Interface::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}

impl std::clone::Clone for IErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IErrorInfo {}
unsafe impl core::Interface for IErrorInfo {
    type Vtable = IErrorInfo_Vtbl;
}
unsafe impl core::ComInterface for IErrorInfo {
    const IID: core::GUID = core::GUID::from_u128(0x1cf2b120_547d_101b_8e65_08002b2bd119);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorInfo_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(this: *mut c_void, pguid: *mut core::GUID) -> core::HRESULT,
    pub GetSource: unsafe extern "system" fn(this: *mut c_void, pbstrsource: *mut core::BSTR) -> core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut c_void, pbstrdescription: *mut core::BSTR) -> core::HRESULT,
    pub GetHelpFile: unsafe extern "system" fn(this: *mut c_void, pbstrhelpfile: *mut core::BSTR) -> core::HRESULT,
    pub GetHelpContext: unsafe extern "system" fn(this: *mut c_void, pdwhelpcontext: *mut u32) -> core::HRESULT,
}

#[repr(transparent)]
pub struct IAgileReference(pub(crate) core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> core::Result<T>
    where
        T: core::ComInterface,
    {
        let mut result__ = std::ptr::null_mut();
        (core::Interface::vtable(self).Resolve)(core::Interface::as_raw(self), &<T as core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}

impl std::clone::Clone for IAgileReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IAgileReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IAgileReference {}
unsafe impl core::Interface for IAgileReference {
    type Vtable = IAgileReference_Vtbl;
}
unsafe impl core::ComInterface for IAgileReference {
    const IID: core::GUID = core::GUID::from_u128(0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut c_void, riid: *const core::GUID, ppvobjectreference: *mut *mut c_void) -> core::HRESULT,
}

#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo(core::IUnknown);
impl std::clone::Clone for ILanguageExceptionErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for ILanguageExceptionErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for ILanguageExceptionErrorInfo {}
unsafe impl core::Interface for ILanguageExceptionErrorInfo {
    type Vtable = ILanguageExceptionErrorInfo_Vtbl;
}
unsafe impl core::ComInterface for ILanguageExceptionErrorInfo {
    const IID: core::GUID = core::GUID::from_u128(0x04a2dbf3_df83_116c_0946_0812abf6e07d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub GetLanguageException: unsafe extern "system" fn(this: *mut c_void, languageexception: *mut *mut c_void) -> core::HRESULT,
}

#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo2(core::IUnknown);
impl ILanguageExceptionErrorInfo2 {
    pub unsafe fn CapturePropagationContext<P0>(&self, languageexception: P0) -> core::Result<()>
    where
        P0: core::IntoParam<core::IUnknown>,
    {
        (core::Interface::vtable(self).CapturePropagationContext)(core::Interface::as_raw(self), languageexception.into_param().abi()).ok()
    }
}
impl std::clone::Clone for ILanguageExceptionErrorInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for ILanguageExceptionErrorInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for ILanguageExceptionErrorInfo2 {}
unsafe impl core::Interface for ILanguageExceptionErrorInfo2 {
    type Vtable = ILanguageExceptionErrorInfo2_Vtbl;
}
unsafe impl core::ComInterface for ILanguageExceptionErrorInfo2 {
    const IID: core::GUID = core::GUID::from_u128(0x5746e5c4_5b97_424c_b620_2822915734dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo2_Vtbl {
    pub base__: ILanguageExceptionErrorInfo_Vtbl,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(this: *mut c_void, previouslanguageexceptionerrorinfo: *mut *mut c_void) -> core::HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(this: *mut c_void, languageexception: *mut c_void) -> core::HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(this: *mut c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut c_void) -> core::HRESULT,
}

#[repr(transparent)]
pub struct IRestrictedErrorInfo(core::IUnknown);
impl IRestrictedErrorInfo {
    pub unsafe fn GetErrorDetails(&self, description: &mut core::BSTR, error: &mut core::HRESULT, restricteddescription: &mut core::BSTR, capabilitysid: &mut core::BSTR) -> core::Result<()> {
        (core::Interface::vtable(self).GetErrorDetails)(core::Interface::as_raw(self), description as *mut _, error as *mut _, restricteddescription as *mut _, capabilitysid as *mut _).ok()
    }
    pub unsafe fn GetReference(&self) -> core::Result<core::BSTR> {
        let mut result__ = core::zeroed::<core::BSTR>();
        (core::Interface::vtable(self).GetReference)(core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
impl std::clone::Clone for IRestrictedErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IRestrictedErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IRestrictedErrorInfo {}
unsafe impl std::marker::Send for IRestrictedErrorInfo {}
unsafe impl std::marker::Sync for IRestrictedErrorInfo {}
unsafe impl core::Interface for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_Vtbl;
}
unsafe impl core::ComInterface for IRestrictedErrorInfo {
    const IID: core::GUID = core::GUID::from_u128(0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub GetErrorDetails: unsafe extern "system" fn(this: *mut c_void, description: *mut core::BSTR, error: *mut core::HRESULT, restricteddescription: *mut core::BSTR, capabilitysid: *mut core::BSTR) -> core::HRESULT,
    pub GetReference: unsafe extern "system" fn(this: *mut c_void, reference: *mut std::mem::MaybeUninit<core::BSTR>) -> core::HRESULT,
}

#[repr(transparent)]
pub struct IWeakReference(core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> core::Result<T>
    where
        T: core::ComInterface,
    {
        let mut result__ = std::ptr::null_mut();
        (core::Interface::vtable(self).Resolve)(core::Interface::as_raw(self), &<T as core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
impl std::clone::Clone for IWeakReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IWeakReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IWeakReference {}
unsafe impl core::Interface for IWeakReference {
    type Vtable = IWeakReference_Vtbl;
}
unsafe impl core::ComInterface for IWeakReference {
    const IID: core::GUID = core::GUID::from_u128(0x00000037_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut c_void, riid: *const core::GUID, objectreference: *mut *mut c_void) -> core::HRESULT,
}

#[repr(transparent)]
pub struct IWeakReferenceSource(core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> core::Result<IWeakReference> {
        let mut result__ = std::ptr::null_mut();
        (core::Interface::vtable(self).GetWeakReference)(core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
impl std::clone::Clone for IWeakReferenceSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IWeakReferenceSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IWeakReferenceSource {}
unsafe impl core::Interface for IWeakReferenceSource {
    type Vtable = IWeakReferenceSource_Vtbl;
}
unsafe impl core::ComInterface for IWeakReferenceSource {
    const IID: core::GUID = core::GUID::from_u128(0x00000038_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub GetWeakReference: unsafe extern "system" fn(this: *mut c_void, weakreference: *mut *mut c_void) -> core::HRESULT,
}
