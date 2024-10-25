pub const KeyBasicInformation: KEY_INFORMATION_CLASS = 0i32;
pub const KeyCachedInformation: KEY_INFORMATION_CLASS = 4i32;
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = 2i32;
pub const KeyFlagsInformation: KEY_INFORMATION_CLASS = 5i32;
pub const KeyFullInformation: KEY_INFORMATION_CLASS = 2i32;
pub const KeyHandleTagsInformation: KEY_INFORMATION_CLASS = 7i32;
pub const KeyLayerInformation: KEY_INFORMATION_CLASS = 9i32;
pub const KeyNameInformation: KEY_INFORMATION_CLASS = 3i32;
pub const KeyNodeInformation: KEY_INFORMATION_CLASS = 1i32;
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = 4i32;
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = 5i32;
pub const KeySetLayerInformation: KEY_SET_INFORMATION_CLASS = 6i32;
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = 3i32;
pub const KeyTrustInformation: KEY_INFORMATION_CLASS = 8i32;
pub const KeyValueBasicInformation: KEY_VALUE_INFORMATION_CLASS = 0i32;
pub const KeyValueFullInformation: KEY_VALUE_INFORMATION_CLASS = 1i32;
pub const KeyValueFullInformationAlign64: KEY_VALUE_INFORMATION_CLASS = 3i32;
pub const KeyValueLayerInformation: KEY_VALUE_INFORMATION_CLASS = 5i32;
pub const KeyValuePartialInformation: KEY_VALUE_INFORMATION_CLASS = 2i32;
pub const KeyValuePartialInformationAlign64: KEY_VALUE_INFORMATION_CLASS = 4i32;
pub const KeyVirtualizationInformation: KEY_INFORMATION_CLASS = 6i32;
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = 1i32;
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = 0i32;
pub const MaxKeyInfoClass: KEY_INFORMATION_CLASS = 10i32;
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = 7i32;
pub const MaxKeyValueInfoClass: KEY_VALUE_INFORMATION_CLASS = 6i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KEY_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for KEY_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KEY_SET_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for KEY_SET_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KEY_VALUE_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for KEY_VALUE_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
impl Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for KEY_VALUE_ENTRY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_ENUMERATE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub Index: u32,
    pub KeyInformationClass: KEY_INFORMATION_CLASS,
    pub KeyInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_ENUMERATE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REG_ENUMERATE_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_ENUMERATE_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub Index: u32,
    pub KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
    pub KeyValueInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_ENUMERATE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REG_ENUMERATE_VALUE_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_QUERY_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub KeyInformationClass: KEY_INFORMATION_CLASS,
    pub KeyInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_QUERY_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REG_QUERY_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueEntries: *mut KEY_VALUE_ENTRY,
    pub EntryCount: u32,
    pub ValueBuffer: *mut core::ffi::c_void,
    pub BufferLength: *mut u32,
    pub RequiredBufferLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_QUERY_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
    pub KeyValueInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_QUERY_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REG_QUERY_VALUE_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_SET_INFORMATION_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
    pub KeySetInformation: *mut core::ffi::c_void,
    pub KeySetInformationLength: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_SET_INFORMATION_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REG_SET_INFORMATION_KEY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
