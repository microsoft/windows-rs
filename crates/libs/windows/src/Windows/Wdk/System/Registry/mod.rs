#[doc = "*Required features: `\"Wdk_System_Registry\"`, `\"Wdk_Foundation\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn NtNotifyChangeMultipleKeys<P0, P1, P2, P3>(masterkeyhandle: P0, subordinateobjects: ::core::option::Option<&[super::super::Foundation::OBJECT_ATTRIBUTES]>, event: P1, apcroutine: super::super::super::Win32::System::IO::PIO_APC_ROUTINE, apccontext: ::core::option::Option<*const ::core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, completionfilter: u32, watchtree: P2, buffer: ::core::option::Option<*mut ::core::ffi::c_void>, buffersize: u32, asynchronous: P3) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
    P2: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
    P3: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn NtNotifyChangeMultipleKeys(masterkeyhandle : super::super::super::Win32::Foundation:: HANDLE, count : u32, subordinateobjects : *const super::super::Foundation:: OBJECT_ATTRIBUTES, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const ::core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, completionfilter : u32, watchtree : super::super::super::Win32::Foundation:: BOOLEAN, buffer : *mut ::core::ffi::c_void, buffersize : u32, asynchronous : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtNotifyChangeMultipleKeys(
        masterkeyhandle.into_param().abi(),
        subordinateobjects.as_deref().map_or(0, |slice| slice.len() as _),
        ::core::mem::transmute(subordinateobjects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        event.into_param().abi(),
        apcroutine,
        ::core::mem::transmute(apccontext.unwrap_or(::std::ptr::null())),
        iostatusblock,
        completionfilter,
        watchtree.into_param().abi(),
        ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())),
        buffersize,
        asynchronous.into_param().abi(),
    )
    .ok()
}
#[doc = "*Required features: `\"Wdk_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryMultipleValueKey<P0>(keyhandle: P0, valueentries: &mut [KEY_VALUE_ENTRY], valuebuffer: *mut ::core::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn NtQueryMultipleValueKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, valueentries : *mut KEY_VALUE_ENTRY, entrycount : u32, valuebuffer : *mut ::core::ffi::c_void, bufferlength : *mut u32, requiredbufferlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtQueryMultipleValueKey(keyhandle.into_param().abi(), ::core::mem::transmute(valueentries.as_ptr()), valueentries.len() as _, valuebuffer, bufferlength, ::core::mem::transmute(requiredbufferlength.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Wdk_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtRenameKey<P0>(keyhandle: P0, newname: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn NtRenameKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, newname : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtRenameKey(keyhandle.into_param().abi(), newname).ok()
}
#[doc = "*Required features: `\"Wdk_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtSetInformationKey<P0>(keyhandle: P0, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const ::core::ffi::c_void, keysetinformationlength: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn NtSetInformationKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keysetinformationclass : KEY_SET_INFORMATION_CLASS, keysetinformation : *const ::core::ffi::c_void, keysetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtSetInformationKey(keyhandle.into_param().abi(), keysetinformationclass, keysetinformation, keysetinformationlength).ok()
}
#[doc = "*Required features: `\"Wdk_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ZwSetInformationKey<P0>(keyhandle: P0, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const ::core::ffi::c_void, keysetinformationlength: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationKey(keyhandle : super::super::super::Win32::Foundation:: HANDLE, keysetinformationclass : KEY_SET_INFORMATION_CLASS, keysetinformation : *const ::core::ffi::c_void, keysetinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    ZwSetInformationKey(keyhandle.into_param().abi(), keysetinformationclass, keysetinformation, keysetinformationlength).ok()
}
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
pub const KeySetLayerInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(7i32);
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KEY_SET_INFORMATION_CLASS(pub i32);
impl ::core::marker::Copy for KEY_SET_INFORMATION_CLASS {}
impl ::core::clone::Clone for KEY_SET_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KEY_SET_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for KEY_SET_INFORMATION_CLASS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KEY_SET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEY_SET_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for KEY_VALUE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for KEY_VALUE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEY_VALUE_ENTRY").field("ValueName", &self.ValueName).field("DataLength", &self.DataLength).field("DataOffset", &self.DataOffset).field("Type", &self.Type).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for KEY_VALUE_ENTRY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for KEY_VALUE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ValueName == other.ValueName && self.DataLength == other.DataLength && self.DataOffset == other.DataOffset && self.Type == other.Type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_System_Registry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    pub Object: *mut ::core::ffi::c_void,
    pub ValueEntries: *mut KEY_VALUE_ENTRY,
    pub EntryCount: u32,
    pub ValueBuffer: *mut ::core::ffi::c_void,
    pub BufferLength: *mut u32,
    pub RequiredBufferLength: *mut u32,
    pub CallContext: *mut ::core::ffi::c_void,
    pub ObjectContext: *mut ::core::ffi::c_void,
    pub Reserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION").field("Object", &self.Object).field("ValueEntries", &self.ValueEntries).field("EntryCount", &self.EntryCount).field("ValueBuffer", &self.ValueBuffer).field("BufferLength", &self.BufferLength).field("RequiredBufferLength", &self.RequiredBufferLength).field("CallContext", &self.CallContext).field("ObjectContext", &self.ObjectContext).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Object == other.Object && self.ValueEntries == other.ValueEntries && self.EntryCount == other.EntryCount && self.ValueBuffer == other.ValueBuffer && self.BufferLength == other.BufferLength && self.RequiredBufferLength == other.RequiredBufferLength && self.CallContext == other.CallContext && self.ObjectContext == other.ObjectContext && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Wdk_System_Registry\"`*"]
pub struct REG_SET_INFORMATION_KEY_INFORMATION {
    pub Object: *mut ::core::ffi::c_void,
    pub KeySetInformationClass: KEY_SET_INFORMATION_CLASS,
    pub KeySetInformation: *mut ::core::ffi::c_void,
    pub KeySetInformationLength: u32,
    pub CallContext: *mut ::core::ffi::c_void,
    pub ObjectContext: *mut ::core::ffi::c_void,
    pub Reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for REG_SET_INFORMATION_KEY_INFORMATION {}
impl ::core::clone::Clone for REG_SET_INFORMATION_KEY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REG_SET_INFORMATION_KEY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REG_SET_INFORMATION_KEY_INFORMATION").field("Object", &self.Object).field("KeySetInformationClass", &self.KeySetInformationClass).field("KeySetInformation", &self.KeySetInformation).field("KeySetInformationLength", &self.KeySetInformationLength).field("CallContext", &self.CallContext).field("ObjectContext", &self.ObjectContext).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows_core::TypeKind for REG_SET_INFORMATION_KEY_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for REG_SET_INFORMATION_KEY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Object == other.Object && self.KeySetInformationClass == other.KeySetInformationClass && self.KeySetInformation == other.KeySetInformation && self.KeySetInformationLength == other.KeySetInformationLength && self.CallContext == other.CallContext && self.ObjectContext == other.ObjectContext && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for REG_SET_INFORMATION_KEY_INFORMATION {}
impl ::core::default::Default for REG_SET_INFORMATION_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
