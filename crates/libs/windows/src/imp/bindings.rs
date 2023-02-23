use super::*;
use std::ffi::c_void;

link!("kernel32.dll" "system" fn CreateEventW(attributes: *const c_void, manual_reset: i32, initial_state: i32, name: *const c_void) -> isize);
link!("kernel32.dll" "system" fn EncodePointer(ptr: *const c_void) -> *mut c_void);
link!("kernel32.dll" "system" fn FormatMessageW(flags: u32, source: *const c_void, code: u32, lang: u32, buffer: core::PWSTR, len: u32, args: *const *const i8) -> u32);
link!("kernel32.dll" "system" fn FreeLibrary(library: isize) -> i32);
link!("kernel32.dll" "system" fn GetLastError() -> u32);
link!("kernel32.dll" "system" fn GetProcAddress(library: isize, name: core::PCSTR) -> *const c_void);
link!("kernel32.dll" "system" fn GetProcessHeap() -> isize);
link!("kernel32.dll" "system" fn HeapAlloc(heap: isize, flags: u32, len: usize) -> *mut c_void);
link!("kernel32.dll" "system" fn HeapFree(heap: isize, flags: u32, ptr: *const c_void) -> i32);
link!("kernel32.dll" "system" fn LoadLibraryA(name: core::PCSTR) -> isize);
link!("kernel32.dll" "system" fn SetEvent(handle: isize) -> i32);
link!("kernel32.dll" "system" fn WaitForSingleObject(handle: isize, milliseconds: u32) -> u32);
link!("kernel32.dll""system" fn CloseHandle(handle: isize) -> i32);
link!("ole32.dll" "system" fn CoCreateGuid(guid: *mut core::GUID) -> core::HRESULT);
link!("ole32.dll" "system" fn CoTaskMemAlloc(len: usize) -> *mut c_void);
link!("ole32.dll" "system" fn CoTaskMemFree(ptr: *const c_void) -> ());
link!("ole32.dll" "system" fn RoGetAgileReference(options: i32, iid: &core::GUID, object: *const c_void, reference: *mut *mut c_void) -> core::HRESULT);
link!("oleaut32.dll" "system" fn GetErrorInfo(reserved: u32, info: *mut *mut c_void) -> core::HRESULT);
link!("oleaut32.dll" "system" fn SetErrorInfo(reserved: u32, info: *const c_void) -> core::HRESULT);
link!("oleaut32.dll" "system" fn SysAllocStringLen(input: *const u16, len: u32) -> *const u16);
link!("oleaut32.dll" "system" fn SysFreeString(bstr: *const u16) -> ());
link!("oleaut32.dll" "system" fn SysStringLen(bstr: *const u16) -> u32);

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
// TODO: need a "windows::imp" for public/implementation details
pub const E_BOUNDS: core::HRESULT = core::HRESULT(-2147483637i32);


// TODO: whatever depends on this needs to move to Windows.Foundation

#[doc(hidden)]
#[repr(transparent)]
#[derive(Clone)]
pub struct IPropertyValueStatics( core::IUnknown);
unsafe impl core::Vtable for IPropertyValueStatics {
    type Vtable = IPropertyValueStatics_Vtbl;
}
unsafe impl core::Interface for IPropertyValueStatics {
    const IID: core::GUID = core::GUID::from_u128(0x629bdbc8_d932_4ff4_96b9_8d96c5c1e858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValueStatics_Vtbl {
    pub base__: core::IInspectable_Vtbl,
    pub CreateEmpty: unsafe extern "system" fn(this: *mut c_void, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateUInt8: unsafe extern "system" fn(this: *mut c_void, value: u8, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateInt16: unsafe extern "system" fn(this: *mut c_void, value: i16, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateUInt16: unsafe extern "system" fn(this: *mut c_void, value: u16, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateInt32: unsafe extern "system" fn(this: *mut c_void, value: i32, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateUInt32: unsafe extern "system" fn(this: *mut c_void, value: u32, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateInt64: unsafe extern "system" fn(this: *mut c_void, value: i64, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateUInt64: unsafe extern "system" fn(this: *mut c_void, value: u64, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateSingle: unsafe extern "system" fn(this: *mut c_void, value: f32, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateDouble: unsafe extern "system" fn(this: *mut c_void, value: f64, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateChar16: unsafe extern "system" fn(this: *mut c_void, value: u16, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateBoolean: unsafe extern "system" fn(this: *mut c_void, value: bool, result__: *mut *mut c_void) -> core::HRESULT,
    pub CreateString: unsafe extern "system" fn(this: *mut c_void, value: *mut c_void, result__: *mut *mut c_void) -> core::HRESULT,
}

#[repr(transparent)]
pub struct IReference<T>( core::IUnknown, std::marker::PhantomData<T>)
where
    T: core::RuntimeType + 'static;
impl<T: core::RuntimeType + 'static> IReference<T> {
    pub fn Value(&self) -> core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).Value)(core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl<T: core::RuntimeType + 'static> std::clone::Clone for IReference<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), std::marker::PhantomData::<T>)
    }
}
impl<T: core::RuntimeType + 'static> std::cmp::PartialEq for IReference<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: core::RuntimeType + 'static> std::cmp::Eq for IReference<T> {}
impl<T: core::RuntimeType + 'static> core::RuntimeType for IReference<T> {
    const SIGNATURE: ConstBuffer = { ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17706-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as core::RuntimeType>::SIGNATURE).push_slice(b")") };
}

unsafe impl<T: core::RuntimeType + 'static> core::Vtable for IReference<T> {
    type Vtable = IReference_Vtbl<T>;
}
unsafe impl<T: core::RuntimeType + 'static> core::Interface for IReference<T> {
    const IID: core::GUID = core::GUID::from_signature(<Self as core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReference_Vtbl<T>
where
    T: core::RuntimeType + 'static,
{
    pub base__: core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut c_void, result__: *mut <T as core::Type<T>>::Abi) -> core::HRESULT,
    pub T: std::marker::PhantomData<T>,
}

#[repr(transparent)]
pub struct IStringable( core::IUnknown);
impl IStringable {
    pub fn ToString(&self) -> core::Result<core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = windows::core::zeroed::<core::HSTRING>();
            (core::Vtable::vtable(this).ToString)(core::Vtable::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl std::clone::Clone for IStringable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl std::cmp::PartialEq for IStringable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::Eq for IStringable {}
impl core::RuntimeType for IStringable {
    const SIGNATURE: ConstBuffer = ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
}

unsafe impl core::Vtable for IStringable {
    type Vtable = IStringable_Vtbl;
}
unsafe impl core::Interface for IStringable {
    const IID: core::GUID = core::GUID::from_u128(0x96369f54_8eb6_48f0_abce_c1b211e627c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringable_Vtbl {
    pub base__: core::IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, result__: *mut std::mem::MaybeUninit<core::HSTRING>) -> core::HRESULT,
}

pub struct PropertyValue;
impl PropertyValue {
    pub fn CreateUInt8(value: u8) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateUInt8)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateInt16(value: i16) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateInt16)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateUInt16(value: u16) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateUInt16)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateInt32(value: i32) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateInt32)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateUInt32(value: u32) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateUInt32)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateInt64(value: i64) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateInt64)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateUInt64(value: u64) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateUInt64)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateSingle(value: f32) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateSingle)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateDouble(value: f64) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateDouble)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateBoolean(value: bool) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateBoolean)(core::Vtable::as_raw(this), value, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateString(value: &core::HSTRING) -> core::Result<core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = std::mem::zeroed();
            (core::Vtable::vtable(this).CreateString)(core::Vtable::as_raw(this), std::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    pub fn IPropertyValueStatics<R, F: FnOnce(&IPropertyValueStatics) -> core::Result<R>>(callback: F) -> core::Result<R> {
        static SHARED: FactoryCache<PropertyValue, IPropertyValueStatics> = FactoryCache::new();
        SHARED.call(callback)
    }
}
impl core::RuntimeName for PropertyValue {
    const NAME: &'static str = "Windows.Foundation.PropertyValue";
}

#[repr(transparent)]
pub struct IAgileObject( core::IUnknown);
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
unsafe impl core::Vtable for IAgileObject {
    type Vtable = IAgileObject_Vtbl;
}
unsafe impl core::Interface for IAgileObject {
    const IID: core::GUID = core::GUID::from_u128(0x94ea2b94_e9cc_49e0_c0ff_ee64ca8f5b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileObject_Vtbl {
    pub base__: core::IUnknown_Vtbl,
}

#[repr(transparent)]
pub struct IErrorInfo( core::IUnknown);
impl IErrorInfo {
    pub unsafe fn GetDescription(&self) -> core::Result<core::BSTR> {
        let mut result__ = core::zeroed::<core::BSTR>();
        (core::Vtable::vtable(self).GetDescription)(core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
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
unsafe impl core::Vtable for IErrorInfo {
    type Vtable = IErrorInfo_Vtbl;
}
unsafe impl core::Interface for IErrorInfo {
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
pub struct IAgileReference(pub(crate)  core::IUnknown);
impl IAgileReference {
    pub unsafe fn Resolve<T>(&self) -> core::Result<T>
    where
        T: core::Interface,
    {
        let mut result__ = std::ptr::null_mut();
        (core::Vtable::vtable(self).Resolve)(core::Vtable::as_raw(self), &<T as core::Interface>::IID, &mut result__).from_abi(result__)
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
unsafe impl core::Vtable for IAgileReference {
    type Vtable = IAgileReference_Vtbl;
}
unsafe impl core::Interface for IAgileReference {
    const IID: core::GUID = core::GUID::from_u128(0xc03f6a43_65a4_9818_987e_e0b810d2a6f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut c_void, riid: *const core::GUID, ppvobjectreference: *mut *mut c_void) -> core::HRESULT,
}

#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo( core::IUnknown);
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
unsafe impl core::Vtable for ILanguageExceptionErrorInfo {
    type Vtable = ILanguageExceptionErrorInfo_Vtbl;
}
unsafe impl core::Interface for ILanguageExceptionErrorInfo {
    const IID: core::GUID = core::GUID::from_u128(0x04a2dbf3_df83_116c_0946_0812abf6e07d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub GetLanguageException: unsafe extern "system" fn(this: *mut c_void, languageexception: *mut *mut c_void) -> core::HRESULT,
}

#[repr(transparent)]
pub struct ILanguageExceptionErrorInfo2( core::IUnknown);
impl ILanguageExceptionErrorInfo2 {
    pub unsafe fn CapturePropagationContext<P0>(&self, languageexception: P0) -> core::Result<()>
    where
        P0: std::convert::Into<core::InParam< core::IUnknown>>,
    {
        (core::Vtable::vtable(self).CapturePropagationContext)(core::Vtable::as_raw(self), languageexception.into().abi()).ok()
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
unsafe impl core::Vtable for ILanguageExceptionErrorInfo2 {
    type Vtable = ILanguageExceptionErrorInfo2_Vtbl;
}
unsafe impl core::Interface for ILanguageExceptionErrorInfo2 {
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
pub struct IRestrictedErrorInfo( core::IUnknown);
impl IRestrictedErrorInfo {
    pub unsafe fn GetErrorDetails(&self, description: &mut core::BSTR, error: &mut core::HRESULT, restricteddescription: &mut core::BSTR, capabilitysid: &mut core::BSTR) -> core::Result<()> {
        (core::Vtable::vtable(self).GetErrorDetails)(core::Vtable::as_raw(self), description as *mut _, error as *mut _, restricteddescription as *mut _, capabilitysid as *mut _).ok()
    }
    pub unsafe fn GetReference(&self) -> core::Result<core::BSTR> {
        let mut result__ = core::zeroed::<core::BSTR>();
        (core::Vtable::vtable(self).GetReference)(core::Vtable::as_raw(self), &mut result__).from_abi(result__)
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
unsafe impl core::Vtable for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_Vtbl;
}
unsafe impl core::Interface for IRestrictedErrorInfo {
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
pub struct IWeakReference( core::IUnknown);
impl IWeakReference {
    pub unsafe fn Resolve<T>(&self) -> core::Result<T>
    where
        T: core::Interface,
    {
        let mut result__ = std::ptr::null_mut();
        (core::Vtable::vtable(self).Resolve)(core::Vtable::as_raw(self), &<T as core::Interface>::IID, &mut result__).from_abi(result__)
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
unsafe impl core::Vtable for IWeakReference {
    type Vtable = IWeakReference_Vtbl;
}
unsafe impl core::Interface for IWeakReference {
    const IID: core::GUID = core::GUID::from_u128(0x00000037_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(this: *mut c_void, riid: *const core::GUID, objectreference: *mut *mut c_void) -> core::HRESULT,
}

#[repr(transparent)]
pub struct IWeakReferenceSource( core::IUnknown);
impl IWeakReferenceSource {
    pub unsafe fn GetWeakReference(&self) -> core::Result<IWeakReference> {
        let mut result__ = std::ptr::null_mut();
        (core::Vtable::vtable(self).GetWeakReference)(core::Vtable::as_raw(self), &mut result__).from_abi(result__)
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
unsafe impl core::Vtable for IWeakReferenceSource {
    type Vtable = IWeakReferenceSource_Vtbl;
}
unsafe impl core::Interface for IWeakReferenceSource {
    const IID: core::GUID = core::GUID::from_u128(0x00000038_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub GetWeakReference: unsafe extern "system" fn(this: *mut c_void, weakreference: *mut *mut c_void) -> core::HRESULT,
}
