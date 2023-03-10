#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IEnumVdsObject(::windows::core::IUnknown);
impl IEnumVdsObject {
    pub unsafe fn Next(&self, celt: u32, ppobjectarray: *mut ::core::option::Option<::windows::core::IUnknown>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(ppobjectarray), pcfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumVdsObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumVdsObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumVdsObject {}
impl ::core::fmt::Debug for IEnumVdsObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumVdsObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumVdsObject {
    type Vtable = IEnumVdsObject_Vtbl;
}
impl ::core::clone::Clone for IEnumVdsObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumVdsObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x118610b7_8d94_4030_b5b8_500889788e4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumVdsObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, ppobjectarray: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsAdmin(::windows::core::IUnknown);
impl IVdsAdmin {
    pub unsafe fn RegisterProvider<P0, P1, P2>(&self, providerid: ::windows::core::GUID, providerclsid: ::windows::core::GUID, pwszname: P0, r#type: VDS_PROVIDER_TYPE, pwszmachinename: P1, pwszversion: P2, guidversionid: ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RegisterProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(providerid), ::core::mem::transmute(providerclsid), pwszname.into_param().abi(), r#type, pwszmachinename.into_param().abi(), pwszversion.into_param().abi(), ::core::mem::transmute(guidversionid)).ok()
    }
    pub unsafe fn UnregisterProvider(&self, providerid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(providerid)).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsAdmin, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsAdmin {}
impl ::core::fmt::Debug for IVdsAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsAdmin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsAdmin {
    type Vtable = IVdsAdmin_Vtbl;
}
impl ::core::clone::Clone for IVdsAdmin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsAdmin {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd188e97d_85aa_4d33_abc6_26299a10ffc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAdmin_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, providerclsid: ::windows::core::GUID, pwszname: ::windows::core::PCWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: ::windows::core::PCWSTR, pwszversion: ::windows::core::PCWSTR, guidversionid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UnregisterProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsAdvancedDisk(::windows::core::IUnknown);
impl IVdsAdvancedDisk {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartitionProperties(&self, ulloffset: u64, ppartitionprop: *mut VDS_PARTITION_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPartitionProperties)(::windows::core::Interface::as_raw(self), ulloffset, ppartitionprop).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryPartitions(&self, pppartitionproparray: *mut *mut VDS_PARTITION_PROP, plnumberofpartitions: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryPartitions)(::windows::core::Interface::as_raw(self), pppartitionproparray, plnumberofpartitions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePartition(&self, ulloffset: u64, ullsize: u64, para: *const CREATE_PARTITION_PARAMETERS) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).CreatePartition)(::windows::core::Interface::as_raw(self), ulloffset, ullsize, para, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeletePartition<P0, P1>(&self, ulloffset: u64, bforce: P0, bforceprotected: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).DeletePartition)(::windows::core::Interface::as_raw(self), ulloffset, bforce.into_param().abi(), bforceprotected.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeAttributes(&self, ulloffset: u64, para: *const CHANGE_ATTRIBUTES_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangeAttributes)(::windows::core::Interface::as_raw(self), ulloffset, para).ok()
    }
    pub unsafe fn AssignDriveLetter(&self, ulloffset: u64, wcletter: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssignDriveLetter)(::windows::core::Interface::as_raw(self), ulloffset, wcletter).ok()
    }
    pub unsafe fn DeleteDriveLetter(&self, ulloffset: u64, wcletter: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteDriveLetter)(::windows::core::Interface::as_raw(self), ulloffset, wcletter).ok()
    }
    pub unsafe fn GetDriveLetter(&self, ulloffset: u64, pwcletter: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDriveLetter)(::windows::core::Interface::as_raw(self), ulloffset, ::core::mem::transmute(pwcletter)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatPartition<P0, P1, P2, P3>(&self, ulloffset: u64, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: P0, dwunitallocationsize: u32, bforce: P1, bquickformat: P2, benablecompression: P3) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).FormatPartition)(::windows::core::Interface::as_raw(self), ulloffset, r#type, pwszlabel.into_param().abi(), dwunitallocationsize, bforce.into_param().abi(), bquickformat.into_param().abi(), benablecompression.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Clean<P0, P1, P2>(&self, bforce: P0, bforceoem: P1, bfullclean: P2) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Clean)(::windows::core::Interface::as_raw(self), bforce.into_param().abi(), bforceoem.into_param().abi(), bfullclean.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsAdvancedDisk, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsAdvancedDisk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsAdvancedDisk {}
impl ::core::fmt::Debug for IVdsAdvancedDisk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsAdvancedDisk").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsAdvancedDisk {
    type Vtable = IVdsAdvancedDisk_Vtbl;
}
impl ::core::clone::Clone for IVdsAdvancedDisk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsAdvancedDisk {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e6f6b40_977c_4069_bddd_ac710059f8c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAdvancedDisk_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPartitionProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, ppartitionprop: *mut VDS_PARTITION_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPartitionProperties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryPartitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppartitionproparray: *mut *mut VDS_PARTITION_PROP, plnumberofpartitions: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryPartitions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePartition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, ullsize: u64, para: *const CREATE_PARTITION_PARAMETERS, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePartition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeletePartition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, bforce: super::super::Foundation::BOOL, bforceprotected: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeletePartition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangeAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, para: *const CHANGE_ATTRIBUTES_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangeAttributes: usize,
    pub AssignDriveLetter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, wcletter: u16) -> ::windows::core::HRESULT,
    pub DeleteDriveLetter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, wcletter: u16) -> ::windows::core::HRESULT,
    pub GetDriveLetter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, pwcletter: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatPartition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: ::windows::core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatPartition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Clean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bforce: super::super::Foundation::BOOL, bforceoem: super::super::Foundation::BOOL, bfullclean: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Clean: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsAdvancedDisk2(::windows::core::IUnknown);
impl IVdsAdvancedDisk2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangePartitionType<P0>(&self, ulloffset: u64, bforce: P0, para: *const CHANGE_PARTITION_TYPE_PARAMETERS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).ChangePartitionType)(::windows::core::Interface::as_raw(self), ulloffset, bforce.into_param().abi(), para).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsAdvancedDisk2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsAdvancedDisk2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsAdvancedDisk2 {}
impl ::core::fmt::Debug for IVdsAdvancedDisk2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsAdvancedDisk2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsAdvancedDisk2 {
    type Vtable = IVdsAdvancedDisk2_Vtbl;
}
impl ::core::clone::Clone for IVdsAdvancedDisk2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsAdvancedDisk2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9723f420_9355_42de_ab66_e31bb15beeac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAdvancedDisk2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangePartitionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, bforce: super::super::Foundation::BOOL, para: *const CHANGE_PARTITION_TYPE_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangePartitionType: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsAdvancedDisk3(::windows::core::IUnknown);
impl IVdsAdvancedDisk3 {
    pub unsafe fn GetProperties(&self, padvdiskprop: *mut VDS_ADVANCEDDISK_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), padvdiskprop).ok()
    }
    pub unsafe fn GetUniqueId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetUniqueId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsAdvancedDisk3, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsAdvancedDisk3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsAdvancedDisk3 {}
impl ::core::fmt::Debug for IVdsAdvancedDisk3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsAdvancedDisk3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsAdvancedDisk3 {
    type Vtable = IVdsAdvancedDisk3_Vtbl;
}
impl ::core::clone::Clone for IVdsAdvancedDisk3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsAdvancedDisk3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3858c0d5_0f35_4bf5_9714_69874963bc36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAdvancedDisk3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padvdiskprop: *mut VDS_ADVANCEDDISK_PROP) -> ::windows::core::HRESULT,
    pub GetUniqueId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsAdviseSink(::windows::core::IUnknown);
impl IVdsAdviseSink {
    pub unsafe fn OnNotify(&self, pnotificationarray: &[VDS_NOTIFICATION]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnNotify)(::windows::core::Interface::as_raw(self), pnotificationarray.len() as _, ::core::mem::transmute(pnotificationarray.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsAdviseSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsAdviseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsAdviseSink {}
impl ::core::fmt::Debug for IVdsAdviseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsAdviseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsAdviseSink {
    type Vtable = IVdsAdviseSink_Vtbl;
}
impl ::core::clone::Clone for IVdsAdviseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsAdviseSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8326cd1d_cf59_4936_b786_5efc08798e25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAdviseSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsAsync(::windows::core::IUnknown);
impl IVdsAsync {
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Wait(&self, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Wait)(::windows::core::Interface::as_raw(self), phrresult, pasyncout).ok()
    }
    pub unsafe fn QueryStatus(&self, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryStatus)(::windows::core::Interface::as_raw(self), phrresult, pulpercentcompleted).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsAsync, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsAsync {}
impl ::core::fmt::Debug for IVdsAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsAsync {
    type Vtable = IVdsAsync_Vtbl;
}
impl ::core::clone::Clone for IVdsAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsAsync {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5d23b6d_5a55_4492_9889_397a3c2d2dbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAsync_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::HRESULT,
    pub QueryStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsController(::windows::core::IUnknown);
impl IVdsController {
    pub unsafe fn GetProperties(&self, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pcontrollerprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__ = ::windows::core::zeroed::<IVdsSubSystem>();
        (::windows::core::Interface::vtable(self).GetSubSystem)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPortProperties(&self, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPortProperties)(::windows::core::Interface::as_raw(self), sportnumber, pportprop).ok()
    }
    pub unsafe fn FlushCache(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlushCache)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn InvalidateCache(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InvalidateCache)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn QueryAssociatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryAssociatedLuns)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStatus(&self, status: VDS_CONTROLLER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), status).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsController, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsController {}
impl ::core::fmt::Debug for IVdsController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsController {
    type Vtable = IVdsController_Vtbl;
}
impl ::core::clone::Clone for IVdsController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb53d96e_dffb_474a_a078_790d1e2bc082);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsController_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows::core::HRESULT,
    pub GetSubSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPortProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT,
    pub FlushCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InvalidateCache: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryAssociatedLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_CONTROLLER_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsControllerControllerPort(::windows::core::IUnknown);
impl IVdsControllerControllerPort {
    pub unsafe fn QueryControllerPorts(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryControllerPorts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsControllerControllerPort, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsControllerControllerPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsControllerControllerPort {}
impl ::core::fmt::Debug for IVdsControllerControllerPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsControllerControllerPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsControllerControllerPort {
    type Vtable = IVdsControllerControllerPort_Vtbl;
}
impl ::core::clone::Clone for IVdsControllerControllerPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsControllerControllerPort {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca5d735f_6bae_42c0_b30e_f2666045ce71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsControllerControllerPort_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryControllerPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsControllerPort(::windows::core::IUnknown);
impl IVdsControllerPort {
    pub unsafe fn GetProperties(&self, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pportprop).ok()
    }
    pub unsafe fn GetController(&self) -> ::windows::core::Result<IVdsController> {
        let mut result__ = ::windows::core::zeroed::<IVdsController>();
        (::windows::core::Interface::vtable(self).GetController)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryAssociatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryAssociatedLuns)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_PORT_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), status).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsControllerPort, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsControllerPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsControllerPort {}
impl ::core::fmt::Debug for IVdsControllerPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsControllerPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsControllerPort {
    type Vtable = IVdsControllerPort_Vtbl;
}
impl ::core::clone::Clone for IVdsControllerPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsControllerPort {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18691d0d_4e7f_43e8_92e4_cf44beeed11c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsControllerPort_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT,
    pub GetController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontroller: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryAssociatedLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_PORT_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsCreatePartitionEx(::windows::core::IUnknown);
impl IVdsCreatePartitionEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePartitionEx(&self, ulloffset: u64, ullsize: u64, ulalign: u32, para: *const CREATE_PARTITION_PARAMETERS) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).CreatePartitionEx)(::windows::core::Interface::as_raw(self), ulloffset, ullsize, ulalign, para, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsCreatePartitionEx, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsCreatePartitionEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsCreatePartitionEx {}
impl ::core::fmt::Debug for IVdsCreatePartitionEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsCreatePartitionEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsCreatePartitionEx {
    type Vtable = IVdsCreatePartitionEx_Vtbl;
}
impl ::core::clone::Clone for IVdsCreatePartitionEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsCreatePartitionEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9882f547_cfc3_420b_9750_00dfbec50662);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsCreatePartitionEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePartitionEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, ullsize: u64, ulalign: u32, para: *const CREATE_PARTITION_PARAMETERS, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePartitionEx: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsDisk(::windows::core::IUnknown);
impl IVdsDisk {
    pub unsafe fn GetProperties(&self, pdiskproperties: *mut VDS_DISK_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pdiskproperties).ok()
    }
    pub unsafe fn GetPack(&self) -> ::windows::core::Result<IVdsPack> {
        let mut result__ = ::windows::core::zeroed::<IVdsPack>();
        (::windows::core::Interface::vtable(self).GetPack)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdentificationData(&self, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIdentificationData)(::windows::core::Interface::as_raw(self), pluninfo).ok()
    }
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryExtents)(::windows::core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    pub unsafe fn ConvertStyle(&self, newstyle: VDS_PARTITION_STYLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConvertStyle)(::windows::core::Interface::as_raw(self), newstyle).ok()
    }
    pub unsafe fn SetFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlags)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearFlags)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsDisk, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsDisk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDisk {}
impl ::core::fmt::Debug for IVdsDisk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDisk").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDisk {
    type Vtable = IVdsDisk_Vtbl;
}
impl ::core::clone::Clone for IVdsDisk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsDisk {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07e5c822_f00c_47a1_8fce_b244da56fd06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDisk_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdiskproperties: *mut VDS_DISK_PROP) -> ::windows::core::HRESULT,
    pub GetPack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppack: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdentificationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdentificationData: usize,
    pub QueryExtents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT,
    pub ConvertStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstyle: VDS_PARTITION_STYLE) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub ClearFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsDisk2(::windows::core::IUnknown);
impl IVdsDisk2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSANMode<P0>(&self, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetSANMode)(::windows::core::Interface::as_raw(self), benable.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsDisk2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsDisk2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDisk2 {}
impl ::core::fmt::Debug for IVdsDisk2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDisk2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDisk2 {
    type Vtable = IVdsDisk2_Vtbl;
}
impl ::core::clone::Clone for IVdsDisk2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsDisk2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40f73c8b_687d_4a13_8d96_3d7f2e683936);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDisk2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSANMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSANMode: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsDisk3(::windows::core::IUnknown);
impl IVdsDisk3 {
    pub unsafe fn GetProperties2(&self, pdiskproperties: *mut VDS_DISK_PROP2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties2)(::windows::core::Interface::as_raw(self), pdiskproperties).ok()
    }
    pub unsafe fn QueryFreeExtents(&self, ulalign: u32, ppfreeextentarray: *mut *mut VDS_DISK_FREE_EXTENT, plnumberoffreeextents: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryFreeExtents)(::windows::core::Interface::as_raw(self), ulalign, ppfreeextentarray, plnumberoffreeextents).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsDisk3, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsDisk3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDisk3 {}
impl ::core::fmt::Debug for IVdsDisk3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDisk3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDisk3 {
    type Vtable = IVdsDisk3_Vtbl;
}
impl ::core::clone::Clone for IVdsDisk3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsDisk3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f4b2f5d_ec15_4357_992f_473ef10975b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDisk3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdiskproperties: *mut VDS_DISK_PROP2) -> ::windows::core::HRESULT,
    pub QueryFreeExtents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulalign: u32, ppfreeextentarray: *mut *mut VDS_DISK_FREE_EXTENT, plnumberoffreeextents: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsDiskOnline(::windows::core::IUnknown);
impl IVdsDiskOnline {
    pub unsafe fn Online(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Online)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Offline(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Offline)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsDiskOnline, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsDiskOnline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDiskOnline {}
impl ::core::fmt::Debug for IVdsDiskOnline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDiskOnline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDiskOnline {
    type Vtable = IVdsDiskOnline_Vtbl;
}
impl ::core::clone::Clone for IVdsDiskOnline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsDiskOnline {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90681b1d_6a7f_48e8_9061_31b7aa125322);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDiskOnline_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Online: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Offline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsDiskPartitionMF(::windows::core::IUnknown);
impl IVdsDiskPartitionMF {
    pub unsafe fn GetPartitionFileSystemProperties(&self, ulloffset: u64, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPartitionFileSystemProperties)(::windows::core::Interface::as_raw(self), ulloffset, pfilesystemprop).ok()
    }
    pub unsafe fn GetPartitionFileSystemTypeName(&self, ulloffset: u64) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetPartitionFileSystemTypeName)(::windows::core::Interface::as_raw(self), ulloffset, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryPartitionFileSystemFormatSupport(&self, ulloffset: u64, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryPartitionFileSystemFormatSupport)(::windows::core::Interface::as_raw(self), ulloffset, ppfilesystemsupportprops, plnumberoffilesystems).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatPartitionEx<P0, P1, P2, P3, P4>(&self, ulloffset: u64, pwszfilesystemtypename: P0, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: P1, bforce: P2, bquickformat: P3, benablecompression: P4) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).FormatPartitionEx)(::windows::core::Interface::as_raw(self), ulloffset, pwszfilesystemtypename.into_param().abi(), usfilesystemrevision, uldesiredunitallocationsize, pwszlabel.into_param().abi(), bforce.into_param().abi(), bquickformat.into_param().abi(), benablecompression.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsDiskPartitionMF, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsDiskPartitionMF {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDiskPartitionMF {}
impl ::core::fmt::Debug for IVdsDiskPartitionMF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDiskPartitionMF").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDiskPartitionMF {
    type Vtable = IVdsDiskPartitionMF_Vtbl;
}
impl ::core::clone::Clone for IVdsDiskPartitionMF {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsDiskPartitionMF {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x538684e0_ba3d_4bc0_aca9_164aff85c2a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDiskPartitionMF_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPartitionFileSystemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> ::windows::core::HRESULT,
    pub GetPartitionFileSystemTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, ppwszfilesystemtypename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub QueryPartitionFileSystemFormatSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatPartitionEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, pwszfilesystemtypename: ::windows::core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: ::windows::core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatPartitionEx: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsDiskPartitionMF2(::windows::core::IUnknown);
impl IVdsDiskPartitionMF2 {
    pub unsafe fn FormatPartitionEx2<P0, P1>(&self, ulloffset: u64, pwszfilesystemtypename: P0, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: P1, options: u32) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).FormatPartitionEx2)(::windows::core::Interface::as_raw(self), ulloffset, pwszfilesystemtypename.into_param().abi(), usfilesystemrevision, uldesiredunitallocationsize, pwszlabel.into_param().abi(), options, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsDiskPartitionMF2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsDiskPartitionMF2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDiskPartitionMF2 {}
impl ::core::fmt::Debug for IVdsDiskPartitionMF2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDiskPartitionMF2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDiskPartitionMF2 {
    type Vtable = IVdsDiskPartitionMF2_Vtbl;
}
impl ::core::clone::Clone for IVdsDiskPartitionMF2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsDiskPartitionMF2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbe50ca_f2d2_4bf4_ace1_96896b729625);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDiskPartitionMF2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FormatPartitionEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffset: u64, pwszfilesystemtypename: ::windows::core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: ::windows::core::PCWSTR, options: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsDrive(::windows::core::IUnknown);
impl IVdsDrive {
    pub unsafe fn GetProperties(&self, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pdriveprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__ = ::windows::core::zeroed::<IVdsSubSystem>();
        (::windows::core::Interface::vtable(self).GetSubSystem)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryExtents)(::windows::core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    pub unsafe fn SetFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlags)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearFlags)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_DRIVE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), status).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsDrive, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsDrive {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDrive {}
impl ::core::fmt::Debug for IVdsDrive {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDrive").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDrive {
    type Vtable = IVdsDrive_Vtbl;
}
impl ::core::clone::Clone for IVdsDrive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsDrive {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff24efa4_aade_4b6b_898b_eaa6a20887c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDrive_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows::core::HRESULT,
    pub GetSubSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryExtents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryExtents: usize,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub ClearFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_DRIVE_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsDrive2(::windows::core::IUnknown);
impl IVdsDrive2 {
    pub unsafe fn GetProperties2(&self, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties2)(::windows::core::Interface::as_raw(self), pdriveprop2).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsDrive2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsDrive2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsDrive2 {}
impl ::core::fmt::Debug for IVdsDrive2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsDrive2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsDrive2 {
    type Vtable = IVdsDrive2_Vtbl;
}
impl ::core::clone::Clone for IVdsDrive2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsDrive2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60b5a730_addf_4436_8ca7_5769e2d1ffa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDrive2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsHbaPort(::windows::core::IUnknown);
impl IVdsHbaPort {
    pub unsafe fn GetProperties(&self, phbaportprop: *mut VDS_HBAPORT_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), phbaportprop).ok()
    }
    pub unsafe fn SetAllPathStatuses(&self, status: VDS_PATH_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllPathStatuses)(::windows::core::Interface::as_raw(self), status).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsHbaPort, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsHbaPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHbaPort {}
impl ::core::fmt::Debug for IVdsHbaPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHbaPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHbaPort {
    type Vtable = IVdsHbaPort_Vtbl;
}
impl ::core::clone::Clone for IVdsHbaPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsHbaPort {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2abd757f_2851_4997_9a13_47d2a885d6ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHbaPort_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phbaportprop: *mut VDS_HBAPORT_PROP) -> ::windows::core::HRESULT,
    pub SetAllPathStatuses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_PATH_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsHwProvider(::windows::core::IUnknown);
impl IVdsHwProvider {
    pub unsafe fn QuerySubSystems(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QuerySubSystems)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reenumerate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reenumerate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsHwProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsHwProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProvider {}
impl ::core::fmt::Debug for IVdsHwProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProvider {
    type Vtable = IVdsHwProvider_Vtbl;
}
impl ::core::clone::Clone for IVdsHwProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsHwProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd99bdaae_b13a_4178_9fdb_e27f16b4603e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QuerySubSystems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reenumerate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsHwProviderPrivate(::windows::core::IUnknown);
impl IVdsHwProviderPrivate {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryIfCreatedLun<P0>(&self, pwszdevicepath: P0, pvdsluninformation: *const VDS_LUN_INFORMATION) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).QueryIfCreatedLun)(::windows::core::Interface::as_raw(self), pwszdevicepath.into_param().abi(), pvdsluninformation, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsHwProviderPrivate, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsHwProviderPrivate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderPrivate {}
impl ::core::fmt::Debug for IVdsHwProviderPrivate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderPrivate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderPrivate {
    type Vtable = IVdsHwProviderPrivate_Vtbl;
}
impl ::core::clone::Clone for IVdsHwProviderPrivate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsHwProviderPrivate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98f17bf3_9f33_4f12_8714_8b4075092c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderPrivate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryIfCreatedLun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdevicepath: ::windows::core::PCWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryIfCreatedLun: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsHwProviderPrivateMpio(::windows::core::IUnknown);
impl IVdsHwProviderPrivateMpio {
    pub unsafe fn SetAllPathStatusesFromHbaPort(&self, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllPathStatusesFromHbaPort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(hbaportprop), status).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsHwProviderPrivateMpio, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsHwProviderPrivateMpio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderPrivateMpio {}
impl ::core::fmt::Debug for IVdsHwProviderPrivateMpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderPrivateMpio").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderPrivateMpio {
    type Vtable = IVdsHwProviderPrivateMpio_Vtbl;
}
impl ::core::clone::Clone for IVdsHwProviderPrivateMpio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsHwProviderPrivateMpio {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x310a7715_ac2b_4c6f_9827_3d742f351676);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderPrivateMpio_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetAllPathStatusesFromHbaPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsHwProviderStoragePools(::windows::core::IUnknown);
impl IVdsHwProviderStoragePools {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryStoragePools(&self, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: ::core::option::Option<*const VDS_POOL_ATTRIBUTES>) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryStoragePools)(::windows::core::Interface::as_raw(self), ulflags, ullremainingfreespace, ::core::mem::transmute(ppoolattributes.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLunInStoragePool<P0>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows::core::GUID, pwszunmaskinglist: P0, phints2: ::core::option::Option<*const VDS_HINTS2>) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).CreateLunInStoragePool)(::windows::core::Interface::as_raw(self), r#type, ullsizeinbytes, ::core::mem::transmute(storagepoolid), pwszunmaskinglist.into_param().abi(), ::core::mem::transmute(phints2.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMaxLunCreateSizeInStoragePool(&self, r#type: VDS_LUN_TYPE, storagepoolid: ::windows::core::GUID, phints2: ::core::option::Option<*const VDS_HINTS2>) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).QueryMaxLunCreateSizeInStoragePool)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(storagepoolid), ::core::mem::transmute(phints2.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsHwProviderStoragePools, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsHwProviderStoragePools {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderStoragePools {}
impl ::core::fmt::Debug for IVdsHwProviderStoragePools {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderStoragePools").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderStoragePools {
    type Vtable = IVdsHwProviderStoragePools_Vtbl;
}
impl ::core::clone::Clone for IVdsHwProviderStoragePools {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsHwProviderStoragePools {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5b5937a_f188_4c79_b86c_11c920ad11b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderStoragePools_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryStoragePools: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryStoragePools: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateLunInStoragePool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows::core::GUID, pwszunmaskinglist: ::windows::core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateLunInStoragePool: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryMaxLunCreateSizeInStoragePool: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, storagepoolid: ::windows::core::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryMaxLunCreateSizeInStoragePool: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsHwProviderType(::windows::core::IUnknown);
impl IVdsHwProviderType {
    pub unsafe fn GetProviderType(&self) -> ::windows::core::Result<VDS_HWPROVIDER_TYPE> {
        let mut result__ = ::windows::core::zeroed::<VDS_HWPROVIDER_TYPE>();
        (::windows::core::Interface::vtable(self).GetProviderType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsHwProviderType, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsHwProviderType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderType {}
impl ::core::fmt::Debug for IVdsHwProviderType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderType {
    type Vtable = IVdsHwProviderType_Vtbl;
}
impl ::core::clone::Clone for IVdsHwProviderType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsHwProviderType {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e0f5166_542d_4fc6_947a_012174240b7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderType_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProviderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsHwProviderType2(::windows::core::IUnknown);
impl IVdsHwProviderType2 {
    pub unsafe fn GetProviderType2(&self) -> ::windows::core::Result<VDS_HWPROVIDER_TYPE> {
        let mut result__ = ::windows::core::zeroed::<VDS_HWPROVIDER_TYPE>();
        (::windows::core::Interface::vtable(self).GetProviderType2)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsHwProviderType2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsHwProviderType2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsHwProviderType2 {}
impl ::core::fmt::Debug for IVdsHwProviderType2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsHwProviderType2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsHwProviderType2 {
    type Vtable = IVdsHwProviderType2_Vtbl;
}
impl ::core::clone::Clone for IVdsHwProviderType2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsHwProviderType2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8190236f_c4d0_4e81_8011_d69512fcc984);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderType2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProviderType2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsIscsiInitiatorAdapter(::windows::core::IUnknown);
impl IVdsIscsiInitiatorAdapter {
    pub unsafe fn GetProperties(&self, pinitiatoradapterprop: *mut VDS_ISCSI_INITIATOR_ADAPTER_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pinitiatoradapterprop).ok()
    }
    pub unsafe fn QueryInitiatorPortals(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryInitiatorPortals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoginToTarget<P0, P1>(&self, logintype: VDS_ISCSI_LOGIN_TYPE, targetid: ::windows::core::GUID, targetportalid: ::windows::core::GUID, initiatorportalid: ::windows::core::GUID, ulloginflags: u32, bheaderdigest: P0, bdatadigest: P1, authtype: VDS_ISCSI_AUTH_TYPE) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).LoginToTarget)(::windows::core::Interface::as_raw(self), logintype, ::core::mem::transmute(targetid), ::core::mem::transmute(targetportalid), ::core::mem::transmute(initiatorportalid), ulloginflags, bheaderdigest.into_param().abi(), bdatadigest.into_param().abi(), authtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn LogoutFromTarget(&self, targetid: ::windows::core::GUID) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).LogoutFromTarget)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(targetid), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsIscsiInitiatorAdapter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsIscsiInitiatorAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsIscsiInitiatorAdapter {}
impl ::core::fmt::Debug for IVdsIscsiInitiatorAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsIscsiInitiatorAdapter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsIscsiInitiatorAdapter {
    type Vtable = IVdsIscsiInitiatorAdapter_Vtbl;
}
impl ::core::clone::Clone for IVdsIscsiInitiatorAdapter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsIscsiInitiatorAdapter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb07fedd4_1682_4440_9189_a39b55194dc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiInitiatorAdapter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiatoradapterprop: *mut VDS_ISCSI_INITIATOR_ADAPTER_PROP) -> ::windows::core::HRESULT,
    pub QueryInitiatorPortals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoginToTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logintype: VDS_ISCSI_LOGIN_TYPE, targetid: ::windows::core::GUID, targetportalid: ::windows::core::GUID, initiatorportalid: ::windows::core::GUID, ulloginflags: u32, bheaderdigest: super::super::Foundation::BOOL, bdatadigest: super::super::Foundation::BOOL, authtype: VDS_ISCSI_AUTH_TYPE, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoginToTarget: usize,
    pub LogoutFromTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::windows::core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsIscsiInitiatorPortal(::windows::core::IUnknown);
impl IVdsIscsiInitiatorPortal {
    pub unsafe fn GetProperties(&self, pinitiatorportalprop: *mut VDS_ISCSI_INITIATOR_PORTAL_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pinitiatorportalprop).ok()
    }
    pub unsafe fn GetInitiatorAdapter(&self) -> ::windows::core::Result<IVdsIscsiInitiatorAdapter> {
        let mut result__ = ::windows::core::zeroed::<IVdsIscsiInitiatorAdapter>();
        (::windows::core::Interface::vtable(self).GetInitiatorAdapter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIpsecTunnelAddress)(::windows::core::Interface::as_raw(self), ptunneladdress, pdestinationaddress).ok()
    }
    pub unsafe fn GetIpsecSecurity(&self, targetportalid: ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetIpsecSecurity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(targetportalid), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIpsecSecurity(&self, targetportalid: ::windows::core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIpsecSecurity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(targetportalid), ullsecurityflags, pipseckey).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsIscsiInitiatorPortal, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsIscsiInitiatorPortal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsIscsiInitiatorPortal {}
impl ::core::fmt::Debug for IVdsIscsiInitiatorPortal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsIscsiInitiatorPortal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsIscsiInitiatorPortal {
    type Vtable = IVdsIscsiInitiatorPortal_Vtbl;
}
impl ::core::clone::Clone for IVdsIscsiInitiatorPortal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsIscsiInitiatorPortal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38a0a9ab_7cc8_4693_ac07_1f28bd03c3da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiInitiatorPortal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiatorportalprop: *mut VDS_ISCSI_INITIATOR_PORTAL_PROP) -> ::windows::core::HRESULT,
    pub GetInitiatorAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinitiatoradapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIpsecTunnelAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::HRESULT,
    pub GetIpsecSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetportalid: ::windows::core::GUID, pullsecurityflags: *mut u64) -> ::windows::core::HRESULT,
    pub SetIpsecSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetportalid: ::windows::core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsIscsiPortal(::windows::core::IUnknown);
impl IVdsIscsiPortal {
    pub unsafe fn GetProperties(&self, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pportalprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__ = ::windows::core::zeroed::<IVdsSubSystem>();
        (::windows::core::Interface::vtable(self).GetSubSystem)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryAssociatedPortalGroups(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryAssociatedPortalGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStatus(&self, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIpsecTunnelAddress)(::windows::core::Interface::as_raw(self), ptunneladdress, pdestinationaddress).ok()
    }
    pub unsafe fn GetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).GetIpsecSecurity)(::windows::core::Interface::as_raw(self), pinitiatorportaladdress, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: ::core::option::Option<*const VDS_ISCSI_IPSEC_KEY>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIpsecSecurity)(::windows::core::Interface::as_raw(self), pinitiatorportaladdress, ullsecurityflags, ::core::mem::transmute(pipseckey.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsIscsiPortal, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsIscsiPortal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsIscsiPortal {}
impl ::core::fmt::Debug for IVdsIscsiPortal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsIscsiPortal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsIscsiPortal {
    type Vtable = IVdsIscsiPortal_Vtbl;
}
impl ::core::clone::Clone for IVdsIscsiPortal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsIscsiPortal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fa1499d_ec85_4a8a_a47b_ff69201fcd34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiPortal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows::core::HRESULT,
    pub GetSubSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryAssociatedPortalGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::HRESULT,
    pub SetIpsecTunnelAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::HRESULT,
    pub GetIpsecSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> ::windows::core::HRESULT,
    pub SetIpsecSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsIscsiPortalGroup(::windows::core::IUnknown);
impl IVdsIscsiPortalGroup {
    pub unsafe fn GetProperties(&self, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pportalgroupprop).ok()
    }
    pub unsafe fn GetTarget(&self) -> ::windows::core::Result<IVdsIscsiTarget> {
        let mut result__ = ::windows::core::zeroed::<IVdsIscsiTarget>();
        (::windows::core::Interface::vtable(self).GetTarget)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryAssociatedPortals(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryAssociatedPortals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddPortal(&self, portalid: ::windows::core::GUID) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).AddPortal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(portalid), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemovePortal(&self, portalid: ::windows::core::GUID) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).RemovePortal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(portalid), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsIscsiPortalGroup, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsIscsiPortalGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsIscsiPortalGroup {}
impl ::core::fmt::Debug for IVdsIscsiPortalGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsIscsiPortalGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsIscsiPortalGroup {
    type Vtable = IVdsIscsiPortalGroup_Vtbl;
}
impl ::core::clone::Clone for IVdsIscsiPortalGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsIscsiPortalGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfef5f89d_a3dd_4b36_bf28_e7dde045c593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiPortalGroup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows::core::HRESULT,
    pub GetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryAssociatedPortals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddPortal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemovePortal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsIscsiPortalLocal(::windows::core::IUnknown);
impl IVdsIscsiPortalLocal {
    pub unsafe fn SetIpsecSecurityLocal(&self, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIpsecSecurityLocal)(::windows::core::Interface::as_raw(self), ullsecurityflags, pipseckey).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsIscsiPortalLocal, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsIscsiPortalLocal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsIscsiPortalLocal {}
impl ::core::fmt::Debug for IVdsIscsiPortalLocal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsIscsiPortalLocal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsIscsiPortalLocal {
    type Vtable = IVdsIscsiPortalLocal_Vtbl;
}
impl ::core::clone::Clone for IVdsIscsiPortalLocal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsIscsiPortalLocal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad837c28_52c1_421d_bf04_fae7da665396);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiPortalLocal_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetIpsecSecurityLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsIscsiTarget(::windows::core::IUnknown);
impl IVdsIscsiTarget {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), ptargetprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__ = ::windows::core::zeroed::<IVdsSubSystem>();
        (::windows::core::Interface::vtable(self).GetSubSystem)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryPortalGroups(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryPortalGroups)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryAssociatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryAssociatedLuns)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePortalGroup(&self) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).CreatePortalGroup)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFriendlyName<P0>(&self, pwszfriendlyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetFriendlyName)(::windows::core::Interface::as_raw(self), pwszfriendlyname.into_param().abi()).ok()
    }
    pub unsafe fn SetSharedSecret<P0>(&self, ptargetsharedsecret: ::core::option::Option<*const VDS_ISCSI_SHARED_SECRET>, pwszinitiatorname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetSharedSecret)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptargetsharedsecret.unwrap_or(::std::ptr::null())), pwszinitiatorname.into_param().abi()).ok()
    }
    pub unsafe fn RememberInitiatorSharedSecret<P0>(&self, pwszinitiatorname: P0, pinitiatorsharedsecret: ::core::option::Option<*const VDS_ISCSI_SHARED_SECRET>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RememberInitiatorSharedSecret)(::windows::core::Interface::as_raw(self), pwszinitiatorname.into_param().abi(), ::core::mem::transmute(pinitiatorsharedsecret.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetConnectedInitiators(&self, pppwszinitiatorlist: *mut *mut ::windows::core::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConnectedInitiators)(::windows::core::Interface::as_raw(self), pppwszinitiatorlist, plnumberofinitiators).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsIscsiTarget, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsIscsiTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsIscsiTarget {}
impl ::core::fmt::Debug for IVdsIscsiTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsIscsiTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsIscsiTarget {
    type Vtable = IVdsIscsiTarget_Vtbl;
}
impl ::core::clone::Clone for IVdsIscsiTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsIscsiTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa8f5055_83e5_4bcc_aa73_19851a36a849);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetProperties: usize,
    pub GetSubSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryPortalGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryAssociatedLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePortalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetSharedSecret: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub RememberInitiatorSharedSecret: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinitiatorname: ::windows::core::PCWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::HRESULT,
    pub GetConnectedInitiators: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppwszinitiatorlist: *mut *mut ::windows::core::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsLun(::windows::core::IUnknown);
impl IVdsLun {
    pub unsafe fn GetProperties(&self, plunprop: *mut VDS_LUN_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), plunprop).ok()
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::core::Result<IVdsSubSystem> {
        let mut result__ = ::windows::core::zeroed::<IVdsSubSystem>();
        (::windows::core::Interface::vtable(self).GetSubSystem)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdentificationData(&self, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIdentificationData)(::windows::core::Interface::as_raw(self), pluninfo).ok()
    }
    pub unsafe fn QueryActiveControllers(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryActiveControllers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Extend(&self, ullnumberofbytestoadd: u64, pdriveidarray: ::core::option::Option<&[::windows::core::GUID]>) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Extend)(::windows::core::Interface::as_raw(self), ullnumberofbytestoadd, ::core::mem::transmute(pdriveidarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriveidarray.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
    }
    pub unsafe fn Shrink(&self, ullnumberofbytestoremove: u64) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Shrink)(::windows::core::Interface::as_raw(self), ullnumberofbytestoremove, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryPlexes(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryPlexes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddPlex(&self, lunid: ::windows::core::GUID) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).AddPlex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lunid), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemovePlex(&self, plexid: ::windows::core::GUID) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).RemovePlex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plexid), &mut result__).from_abi(result__)
    }
    pub unsafe fn Recover(&self) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Recover)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMask<P0>(&self, pwszunmaskinglist: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetMask)(::windows::core::Interface::as_raw(self), pwszunmaskinglist.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AssociateControllers(&self, pactivecontrolleridarray: ::core::option::Option<&[::windows::core::GUID]>, pinactivecontrolleridarray: ::core::option::Option<&[::windows::core::GUID]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssociateControllers)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pactivecontrolleridarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pactivecontrolleridarray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pinactivecontrolleridarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pinactivecontrolleridarray.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHints(&self, phints: *mut VDS_HINTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryHints)(::windows::core::Interface::as_raw(self), phints).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyHints)(::windows::core::Interface::as_raw(self), phints).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_LUN_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn QueryMaxLunExtendSize(&self, pdriveidarray: ::core::option::Option<&[::windows::core::GUID]>) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).QueryMaxLunExtendSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdriveidarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriveidarray.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsLun, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsLun {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLun {}
impl ::core::fmt::Debug for IVdsLun {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLun").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLun {
    type Vtable = IVdsLun_Vtbl;
}
impl ::core::clone::Clone for IVdsLun {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsLun {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3540a9c7_e60f_4111_a840_8bba6c2c83d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLun_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plunprop: *mut VDS_LUN_PROP) -> ::windows::core::HRESULT,
    pub GetSubSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubsystem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdentificationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdentificationData: usize,
    pub QueryActiveControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Extend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Shrink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryPlexes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddPlex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lunid: ::windows::core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemovePlex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plexid: ::windows::core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Recover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszunmaskinglist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AssociateControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactivecontrolleridarray: *const ::windows::core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::core::GUID, lnumberofinactivecontrollers: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryHints: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyHints: usize,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_LUN_STATUS) -> ::windows::core::HRESULT,
    pub QueryMaxLunExtendSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsLun2(::windows::core::IUnknown);
impl IVdsLun2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHints2(&self, phints2: *mut VDS_HINTS2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryHints2)(::windows::core::Interface::as_raw(self), phints2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyHints2(&self, phints2: *const VDS_HINTS2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyHints2)(::windows::core::Interface::as_raw(self), phints2).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsLun2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsLun2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLun2 {}
impl ::core::fmt::Debug for IVdsLun2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLun2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLun2 {
    type Vtable = IVdsLun2_Vtbl;
}
impl ::core::clone::Clone for IVdsLun2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsLun2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5b3a735_9efb_499a_8071_4394d9ee6fcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLun2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryHints2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints2: *mut VDS_HINTS2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryHints2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyHints2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints2: *const VDS_HINTS2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyHints2: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsLunControllerPorts(::windows::core::IUnknown);
impl IVdsLunControllerPorts {
    pub unsafe fn AssociateControllerPorts(&self, pactivecontrollerportidarray: ::core::option::Option<&[::windows::core::GUID]>, pinactivecontrollerportidarray: ::core::option::Option<&[::windows::core::GUID]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssociateControllerPorts)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pactivecontrollerportidarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pactivecontrollerportidarray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pinactivecontrollerportidarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pinactivecontrollerportidarray.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn QueryActiveControllerPorts(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryActiveControllerPorts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsLunControllerPorts, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsLunControllerPorts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunControllerPorts {}
impl ::core::fmt::Debug for IVdsLunControllerPorts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunControllerPorts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunControllerPorts {
    type Vtable = IVdsLunControllerPorts_Vtbl;
}
impl ::core::clone::Clone for IVdsLunControllerPorts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsLunControllerPorts {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x451fe266_da6d_406a_bb60_82e534f85aeb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunControllerPorts_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AssociateControllerPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::core::HRESULT,
    pub QueryActiveControllerPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsLunIscsi(::windows::core::IUnknown);
impl IVdsLunIscsi {
    pub unsafe fn AssociateTargets(&self, ptargetidarray: ::core::option::Option<&[::windows::core::GUID]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AssociateTargets)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptargetidarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ptargetidarray.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn QueryAssociatedTargets(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryAssociatedTargets)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsLunIscsi, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsLunIscsi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunIscsi {}
impl ::core::fmt::Debug for IVdsLunIscsi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunIscsi").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunIscsi {
    type Vtable = IVdsLunIscsi_Vtbl;
}
impl ::core::clone::Clone for IVdsLunIscsi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsLunIscsi {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d7c1e64_b59b_45ae_b86a_2c2cc6a42067);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunIscsi_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AssociateTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptargetidarray: *const ::windows::core::GUID, lnumberoftargets: i32) -> ::windows::core::HRESULT,
    pub QueryAssociatedTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsLunMpio(::windows::core::IUnknown);
impl IVdsLunMpio {
    pub unsafe fn GetPathInfo(&self, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPathInfo)(::windows::core::Interface::as_raw(self), pppaths, plnumberofpaths).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLoadBalancePolicy(&self, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLoadBalancePolicy)(::windows::core::Interface::as_raw(self), ppolicy, pppaths, plnumberofpaths).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLoadBalancePolicy(&self, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: ::core::option::Option<&[VDS_PATH_POLICY]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLoadBalancePolicy)(::windows::core::Interface::as_raw(self), policy, ::core::mem::transmute(ppaths.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppaths.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn GetSupportedLbPolicies(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSupportedLbPolicies)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsLunMpio, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsLunMpio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunMpio {}
impl ::core::fmt::Debug for IVdsLunMpio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunMpio").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunMpio {
    type Vtable = IVdsLunMpio_Vtbl;
}
impl ::core::clone::Clone for IVdsLunMpio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsLunMpio {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c5fbae3_333a_48a1_a982_33c15788cde3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunMpio_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPathInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLoadBalancePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLoadBalancePolicy: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLoadBalancePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLoadBalancePolicy: usize,
    pub GetSupportedLbPolicies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullbflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsLunNaming(::windows::core::IUnknown);
impl IVdsLunNaming {
    pub unsafe fn SetFriendlyName<P0>(&self, pwszfriendlyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetFriendlyName)(::windows::core::Interface::as_raw(self), pwszfriendlyname.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsLunNaming, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsLunNaming {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunNaming {}
impl ::core::fmt::Debug for IVdsLunNaming {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunNaming").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunNaming {
    type Vtable = IVdsLunNaming_Vtbl;
}
impl ::core::clone::Clone for IVdsLunNaming {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsLunNaming {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x907504cb_6b4e_4d88_a34d_17ba661fbb06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunNaming_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsLunNumber(::windows::core::IUnknown);
impl IVdsLunNumber {
    pub unsafe fn GetLunNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLunNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsLunNumber, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsLunNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunNumber {}
impl ::core::fmt::Debug for IVdsLunNumber {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunNumber").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunNumber {
    type Vtable = IVdsLunNumber_Vtbl;
}
impl ::core::clone::Clone for IVdsLunNumber {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsLunNumber {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3f95e46_54b3_41f9_b678_0f1871443a08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunNumber_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLunNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullunnumber: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsLunPlex(::windows::core::IUnknown);
impl IVdsLunPlex {
    pub unsafe fn GetProperties(&self, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pplexprop).ok()
    }
    pub unsafe fn GetLun(&self) -> ::windows::core::Result<IVdsLun> {
        let mut result__ = ::windows::core::zeroed::<IVdsLun>();
        (::windows::core::Interface::vtable(self).GetLun)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryExtents)(::windows::core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHints(&self, phints: *mut VDS_HINTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryHints)(::windows::core::Interface::as_raw(self), phints).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ApplyHints)(::windows::core::Interface::as_raw(self), phints).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsLunPlex, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsLunPlex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsLunPlex {}
impl ::core::fmt::Debug for IVdsLunPlex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsLunPlex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsLunPlex {
    type Vtable = IVdsLunPlex_Vtbl;
}
impl ::core::clone::Clone for IVdsLunPlex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsLunPlex {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ee1a790_5d2e_4abb_8c99_c481e8be2138);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunPlex_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows::core::HRESULT,
    pub GetLun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplun: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryExtents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryExtents: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryHints: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ApplyHints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ApplyHints: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsMaintenance(::windows::core::IUnknown);
impl IVdsMaintenance {
    pub unsafe fn StartMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartMaintenance)(::windows::core::Interface::as_raw(self), operation).ok()
    }
    pub unsafe fn StopMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopMaintenance)(::windows::core::Interface::as_raw(self), operation).ok()
    }
    pub unsafe fn PulseMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PulseMaintenance)(::windows::core::Interface::as_raw(self), operation, ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsMaintenance, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsMaintenance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsMaintenance {}
impl ::core::fmt::Debug for IVdsMaintenance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsMaintenance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsMaintenance {
    type Vtable = IVdsMaintenance_Vtbl;
}
impl ::core::clone::Clone for IVdsMaintenance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsMaintenance {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaebeef3_8523_47ed_a2b9_05cecce2a1ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsMaintenance_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartMaintenance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT,
    pub StopMaintenance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT,
    pub PulseMaintenance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsOpenVDisk(::windows::core::IUnknown);
impl IVdsOpenVDisk {
    #[doc = "*Required features: `\"Win32_Storage_Vhd\"`*"]
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Attach<P0>(&self, pstringsecuritydescriptor: P0, flags: super::Vhd::ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, timeoutinms: u32) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Attach)(::windows::core::Interface::as_raw(self), pstringsecuritydescriptor.into_param().abi(), flags, providerspecificflags, timeoutinms, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Vhd\"`*"]
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Detach(&self, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Detach)(::windows::core::Interface::as_raw(self), flags, providerspecificflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Vhd\"`*"]
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn DetachAndDelete(&self, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DetachAndDelete)(::windows::core::Interface::as_raw(self), flags, providerspecificflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Vhd\"`*"]
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Compact(&self, flags: super::Vhd::COMPACT_VIRTUAL_DISK_FLAG, reserved: u32) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Compact)(::windows::core::Interface::as_raw(self), flags, reserved, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Vhd\"`*"]
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Merge(&self, flags: super::Vhd::MERGE_VIRTUAL_DISK_FLAG, mergedepth: u32) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Merge)(::windows::core::Interface::as_raw(self), flags, mergedepth, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Vhd\"`*"]
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Expand(&self, flags: super::Vhd::EXPAND_VIRTUAL_DISK_FLAG, newsize: u64) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Expand)(::windows::core::Interface::as_raw(self), flags, newsize, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsOpenVDisk, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsOpenVDisk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsOpenVDisk {}
impl ::core::fmt::Debug for IVdsOpenVDisk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsOpenVDisk").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsOpenVDisk {
    type Vtable = IVdsOpenVDisk_Vtbl;
}
impl ::core::clone::Clone for IVdsOpenVDisk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsOpenVDisk {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75c8f324_f715_4fe3_a28e_f9011b61a4a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsOpenVDisk_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstringsecuritydescriptor: ::windows::core::PCWSTR, flags: super::Vhd::ATTACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32, timeoutinms: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Attach: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Detach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Detach: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub DetachAndDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: super::Vhd::DETACH_VIRTUAL_DISK_FLAG, providerspecificflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    DetachAndDelete: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Compact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: super::Vhd::COMPACT_VIRTUAL_DISK_FLAG, reserved: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Compact: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Merge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: super::Vhd::MERGE_VIRTUAL_DISK_FLAG, mergedepth: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Merge: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Expand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: super::Vhd::EXPAND_VIRTUAL_DISK_FLAG, newsize: u64, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Expand: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsPack(::windows::core::IUnknown);
impl IVdsPack {
    pub unsafe fn GetProperties(&self, ppackprop: *mut VDS_PACK_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), ppackprop).ok()
    }
    pub unsafe fn GetProvider(&self) -> ::windows::core::Result<IVdsProvider> {
        let mut result__ = ::windows::core::zeroed::<IVdsProvider>();
        (::windows::core::Interface::vtable(self).GetProvider)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryVolumes(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryVolumes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDisks(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryDisks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateVolume(&self, r#type: VDS_VOLUME_TYPE, pinputdiskarray: &[VDS_INPUT_DISK], ulstripesize: u32) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).CreateVolume)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(pinputdiskarray.as_ptr()), pinputdiskarray.len() as _, ulstripesize, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDisk<P0>(&self, diskid: ::windows::core::GUID, partitionstyle: VDS_PARTITION_STYLE, bashotspare: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).AddDisk)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(diskid), partitionstyle, bashotspare.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MigrateDisks<P0, P1>(&self, pdiskarray: *const ::windows::core::GUID, lnumberofdisks: i32, targetpack: ::windows::core::GUID, bforce: P0, bqueryonly: P1, presults: *mut ::windows::core::HRESULT, pbrebootneeded: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).MigrateDisks)(::windows::core::Interface::as_raw(self), pdiskarray, lnumberofdisks, ::core::mem::transmute(targetpack), bforce.into_param().abi(), bqueryonly.into_param().abi(), presults, pbrebootneeded).ok()
    }
    pub unsafe fn ReplaceDisk(&self, olddiskid: ::windows::core::GUID, newdiskid: ::windows::core::GUID) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).ReplaceDisk)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(olddiskid), ::core::mem::transmute(newdiskid), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveMissingDisk(&self, diskid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveMissingDisk)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(diskid)).ok()
    }
    pub unsafe fn Recover(&self) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Recover)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsPack, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsPack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsPack {}
impl ::core::fmt::Debug for IVdsPack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsPack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsPack {
    type Vtable = IVdsPack_Vtbl;
}
impl ::core::clone::Clone for IVdsPack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsPack {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b69d7f5_9d94_4648_91ca_79939ba263bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsPack_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppackprop: *mut VDS_PACK_PROP) -> ::windows::core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryVolumes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryDisks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, diskid: ::windows::core::GUID, partitionstyle: VDS_PARTITION_STYLE, bashotspare: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDisk: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MigrateDisks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdiskarray: *const ::windows::core::GUID, lnumberofdisks: i32, targetpack: ::windows::core::GUID, bforce: super::super::Foundation::BOOL, bqueryonly: super::super::Foundation::BOOL, presults: *mut ::windows::core::HRESULT, pbrebootneeded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MigrateDisks: usize,
    pub ReplaceDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, olddiskid: ::windows::core::GUID, newdiskid: ::windows::core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveMissingDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, diskid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Recover: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsPack2(::windows::core::IUnknown);
impl IVdsPack2 {
    pub unsafe fn CreateVolume2(&self, r#type: VDS_VOLUME_TYPE, pinputdiskarray: &[VDS_INPUT_DISK], ulstripesize: u32, ulalign: u32) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).CreateVolume2)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(pinputdiskarray.as_ptr()), pinputdiskarray.len() as _, ulstripesize, ulalign, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsPack2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsPack2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsPack2 {}
impl ::core::fmt::Debug for IVdsPack2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsPack2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsPack2 {
    type Vtable = IVdsPack2_Vtbl;
}
impl ::core::clone::Clone for IVdsPack2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsPack2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13b50bff_290a_47dd_8558_b7c58db1a71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsPack2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateVolume2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_VOLUME_TYPE, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ulstripesize: u32, ulalign: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsProvider(::windows::core::IUnknown);
impl IVdsProvider {
    pub unsafe fn GetProperties(&self, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pproviderprop).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsProvider {}
impl ::core::fmt::Debug for IVdsProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsProvider {
    type Vtable = IVdsProvider_Vtbl;
}
impl ::core::clone::Clone for IVdsProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10c5e575_7984_4e81_a56b_431f5f92ae42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsProviderPrivate(::windows::core::IUnknown);
impl IVdsProviderPrivate {
    pub unsafe fn GetObject(&self, objectid: ::windows::core::GUID, r#type: VDS_OBJECT_TYPE) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(objectid), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnLoad<P0, P1>(&self, pwszmachinename: P0, pcallbackobject: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnLoad)(::windows::core::Interface::as_raw(self), pwszmachinename.into_param().abi(), pcallbackobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUnload<P0>(&self, bforceunload: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnUnload)(::windows::core::Interface::as_raw(self), bforceunload.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsProviderPrivate, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsProviderPrivate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsProviderPrivate {}
impl ::core::fmt::Debug for IVdsProviderPrivate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsProviderPrivate").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsProviderPrivate {
    type Vtable = IVdsProviderPrivate_Vtbl;
}
impl ::core::clone::Clone for IVdsProviderPrivate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsProviderPrivate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11f3cd41_b7e8_48ff_9472_9dff018aa292);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsProviderPrivate_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::windows::core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmachinename: ::windows::core::PCWSTR, pcallbackobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUnload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUnload: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsProviderSupport(::windows::core::IUnknown);
impl IVdsProviderSupport {
    pub unsafe fn GetVersionSupport(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetVersionSupport)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsProviderSupport, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsProviderSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsProviderSupport {}
impl ::core::fmt::Debug for IVdsProviderSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsProviderSupport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsProviderSupport {
    type Vtable = IVdsProviderSupport_Vtbl;
}
impl ::core::clone::Clone for IVdsProviderSupport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsProviderSupport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1732be13_e8f9_4a03_bfbc_5f616aa66ce1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsProviderSupport_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetVersionSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulversionsupport: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsRemovable(::windows::core::IUnknown);
impl IVdsRemovable {
    pub unsafe fn QueryMedia(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryMedia)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Eject(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Eject)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsRemovable, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsRemovable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsRemovable {}
impl ::core::fmt::Debug for IVdsRemovable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsRemovable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsRemovable {
    type Vtable = IVdsRemovable_Vtbl;
}
impl ::core::clone::Clone for IVdsRemovable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsRemovable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0316560b_5db4_4ed9_bbb5_213436ddc0d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsRemovable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Eject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsService(::windows::core::IUnknown);
impl IVdsService {
    pub unsafe fn IsServiceReady(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsServiceReady)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForServiceReady(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForServiceReady)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<VDS_SERVICE_PROP> {
        let mut result__ = ::windows::core::zeroed::<VDS_SERVICE_PROP>();
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryProviders(&self, masks: u32) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryProviders)(::windows::core::Interface::as_raw(self), masks, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryMaskedDisks(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryMaskedDisks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryUnallocatedDisks(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryUnallocatedDisks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetObject(&self, objectid: ::windows::core::GUID, r#type: VDS_OBJECT_TYPE) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(objectid), r#type, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryDriveLetters(&self, wcfirstletter: u16, pdriveletterproparray: &mut [VDS_DRIVE_LETTER_PROP]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryDriveLetters)(::windows::core::Interface::as_raw(self), wcfirstletter, pdriveletterproparray.len() as _, ::core::mem::transmute(pdriveletterproparray.as_ptr())).ok()
    }
    pub unsafe fn QueryFileSystemTypes(&self, ppfilesystemtypeprops: *mut *mut VDS_FILE_SYSTEM_TYPE_PROP, plnumberoffilesystems: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryFileSystemTypes)(::windows::core::Interface::as_raw(self), ppfilesystemtypeprops, plnumberoffilesystems).ok()
    }
    pub unsafe fn Reenumerate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reenumerate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CleanupObsoleteMountPoints(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CleanupObsoleteMountPoints)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Advise<P0>(&self, psink: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<IVdsAdviseSink>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Advise)(::windows::core::Interface::as_raw(self), psink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn Reboot(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reboot)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlags)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearFlags)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsService, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsService {}
impl ::core::fmt::Debug for IVdsService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsService {
    type Vtable = IVdsService_Vtbl;
}
impl ::core::clone::Clone for IVdsService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0818a8ef_9ba9_40d8_a6f9_e22833cc771e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsService_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub IsServiceReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForServiceReady: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserviceprop: *mut VDS_SERVICE_PROP) -> ::windows::core::HRESULT,
    pub QueryProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masks: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryMaskedDisks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryUnallocatedDisks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::windows::core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryDriveLetters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wcfirstletter: u16, count: u32, pdriveletterproparray: *mut VDS_DRIVE_LETTER_PROP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryDriveLetters: usize,
    pub QueryFileSystemTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfilesystemtypeprops: *mut *mut VDS_FILE_SYSTEM_TYPE_PROP, plnumberoffilesystems: *mut i32) -> ::windows::core::HRESULT,
    pub Reenumerate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CleanupObsoleteMountPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Advise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub Reboot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub ClearFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsServiceHba(::windows::core::IUnknown);
impl IVdsServiceHba {
    pub unsafe fn QueryHbaPorts(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryHbaPorts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsServiceHba, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsServiceHba {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsServiceHba {}
impl ::core::fmt::Debug for IVdsServiceHba {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsServiceHba").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsServiceHba {
    type Vtable = IVdsServiceHba_Vtbl;
}
impl ::core::clone::Clone for IVdsServiceHba {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsServiceHba {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ac13689_3134_47c6_a17c_4669216801be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsServiceHba_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryHbaPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsServiceInitialization(::windows::core::IUnknown);
impl IVdsServiceInitialization {
    pub unsafe fn Initialize<P0>(&self, pwszmachinename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pwszmachinename.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsServiceInitialization, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsServiceInitialization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsServiceInitialization {}
impl ::core::fmt::Debug for IVdsServiceInitialization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsServiceInitialization").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsServiceInitialization {
    type Vtable = IVdsServiceInitialization_Vtbl;
}
impl ::core::clone::Clone for IVdsServiceInitialization {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsServiceInitialization {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4afc3636_db01_4052_80c3_03bbcb8d3c69);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsServiceInitialization_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmachinename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsServiceIscsi(::windows::core::IUnknown);
impl IVdsServiceIscsi {
    pub unsafe fn GetInitiatorName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetInitiatorName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryInitiatorAdapters(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryInitiatorAdapters)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIpsecGroupPresharedKey(&self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIpsecGroupPresharedKey)(::windows::core::Interface::as_raw(self), pipseckey).ok()
    }
    pub unsafe fn SetAllIpsecTunnelAddresses(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllIpsecTunnelAddresses)(::windows::core::Interface::as_raw(self), ptunneladdress, pdestinationaddress).ok()
    }
    pub unsafe fn SetAllIpsecSecurity(&self, targetportalid: ::windows::core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllIpsecSecurity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(targetportalid), ullsecurityflags, pipseckey).ok()
    }
    pub unsafe fn SetInitiatorSharedSecret(&self, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET, targetid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInitiatorSharedSecret)(::windows::core::Interface::as_raw(self), pinitiatorsharedsecret, ::core::mem::transmute(targetid)).ok()
    }
    pub unsafe fn RememberTargetSharedSecret(&self, targetid: ::windows::core::GUID, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RememberTargetSharedSecret)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(targetid), ptargetsharedsecret).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsServiceIscsi, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsServiceIscsi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsServiceIscsi {}
impl ::core::fmt::Debug for IVdsServiceIscsi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsServiceIscsi").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsServiceIscsi {
    type Vtable = IVdsServiceIscsi_Vtbl;
}
impl ::core::clone::Clone for IVdsServiceIscsi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsServiceIscsi {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14fbe036_3ed7_4e10_90e9_a5ff991aff01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsServiceIscsi_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInitiatorName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwsziscsiname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub QueryInitiatorAdapters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIpsecGroupPresharedKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT,
    pub SetAllIpsecTunnelAddresses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::HRESULT,
    pub SetAllIpsecSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetportalid: ::windows::core::GUID, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT,
    pub SetInitiatorSharedSecret: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET, targetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RememberTargetSharedSecret: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetid: ::windows::core::GUID, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsServiceLoader(::windows::core::IUnknown);
impl IVdsServiceLoader {
    pub unsafe fn LoadService<P0>(&self, pwszmachinename: P0) -> ::windows::core::Result<IVdsService>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsService>();
        (::windows::core::Interface::vtable(self).LoadService)(::windows::core::Interface::as_raw(self), pwszmachinename.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsServiceLoader, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsServiceLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsServiceLoader {}
impl ::core::fmt::Debug for IVdsServiceLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsServiceLoader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsServiceLoader {
    type Vtable = IVdsServiceLoader_Vtbl;
}
impl ::core::clone::Clone for IVdsServiceLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsServiceLoader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0393303_90d4_4a97_ab71_e9b671ee2729);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsServiceLoader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LoadService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszmachinename: ::windows::core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsServiceSAN(::windows::core::IUnknown);
impl IVdsServiceSAN {
    pub unsafe fn GetSANPolicy(&self) -> ::windows::core::Result<VDS_SAN_POLICY> {
        let mut result__ = ::windows::core::zeroed::<VDS_SAN_POLICY>();
        (::windows::core::Interface::vtable(self).GetSANPolicy)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSANPolicy(&self, sanpolicy: VDS_SAN_POLICY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSANPolicy)(::windows::core::Interface::as_raw(self), sanpolicy).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsServiceSAN, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsServiceSAN {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsServiceSAN {}
impl ::core::fmt::Debug for IVdsServiceSAN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsServiceSAN").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsServiceSAN {
    type Vtable = IVdsServiceSAN_Vtbl;
}
impl ::core::clone::Clone for IVdsServiceSAN {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsServiceSAN {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc5d23e8_a88b_41a5_8de0_2d2f73c5a630);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsServiceSAN_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSANPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psanpolicy: *mut VDS_SAN_POLICY) -> ::windows::core::HRESULT,
    pub SetSANPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sanpolicy: VDS_SAN_POLICY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsServiceSw(::windows::core::IUnknown);
impl IVdsServiceSw {
    pub unsafe fn GetDiskObject<P0>(&self, pwszdeviceid: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetDiskObject)(::windows::core::Interface::as_raw(self), pwszdeviceid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsServiceSw, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsServiceSw {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsServiceSw {}
impl ::core::fmt::Debug for IVdsServiceSw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsServiceSw").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsServiceSw {
    type Vtable = IVdsServiceSw_Vtbl;
}
impl ::core::clone::Clone for IVdsServiceSw {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsServiceSw {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15fc031c_0652_4306_b2c3_f558b8f837e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsServiceSw_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDiskObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdeviceid: ::windows::core::PCWSTR, ppdiskunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsServiceUninstallDisk(::windows::core::IUnknown);
impl IVdsServiceUninstallDisk {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDiskIdFromLunInfo(&self, pluninfo: *const VDS_LUN_INFORMATION) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetDiskIdFromLunInfo)(::windows::core::Interface::as_raw(self), pluninfo, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UninstallDisks<P0>(&self, pdiskidarray: *const ::windows::core::GUID, ulcount: u32, bforce: P0, pbreboot: *mut u8, presults: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).UninstallDisks)(::windows::core::Interface::as_raw(self), pdiskidarray, ulcount, bforce.into_param().abi(), pbreboot, presults).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsServiceUninstallDisk, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsServiceUninstallDisk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsServiceUninstallDisk {}
impl ::core::fmt::Debug for IVdsServiceUninstallDisk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsServiceUninstallDisk").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsServiceUninstallDisk {
    type Vtable = IVdsServiceUninstallDisk_Vtbl;
}
impl ::core::clone::Clone for IVdsServiceUninstallDisk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsServiceUninstallDisk {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b22da8_f903_4be7_b492_c09d875ac9da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsServiceUninstallDisk_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDiskIdFromLunInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pluninfo: *const VDS_LUN_INFORMATION, pdiskid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDiskIdFromLunInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UninstallDisks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdiskidarray: *const ::windows::core::GUID, ulcount: u32, bforce: super::super::Foundation::BOOLEAN, pbreboot: *mut u8, presults: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UninstallDisks: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsStoragePool(::windows::core::IUnknown);
impl IVdsStoragePool {
    pub unsafe fn GetProvider(&self) -> ::windows::core::Result<IVdsProvider> {
        let mut result__ = ::windows::core::zeroed::<IVdsProvider>();
        (::windows::core::Interface::vtable(self).GetProvider)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProperties(&self, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pstoragepoolprop).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes(&self, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes)(::windows::core::Interface::as_raw(self), pstoragepoolattributes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryDriveExtents(&self, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryDriveExtents)(::windows::core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    pub unsafe fn QueryAllocatedLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryAllocatedLuns)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryAllocatedStoragePools(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryAllocatedStoragePools)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsStoragePool, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsStoragePool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsStoragePool {}
impl ::core::fmt::Debug for IVdsStoragePool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsStoragePool").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsStoragePool {
    type Vtable = IVdsStoragePool_Vtbl;
}
impl ::core::clone::Clone for IVdsStoragePool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsStoragePool {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x932ca8cf_0eb3_4ba8_9620_22665d7f8450);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsStoragePool_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryDriveExtents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryDriveExtents: usize,
    pub QueryAllocatedLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryAllocatedStoragePools: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsSubSystem(::windows::core::IUnknown);
impl IVdsSubSystem {
    pub unsafe fn GetProperties(&self, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), psubsystemprop).ok()
    }
    pub unsafe fn GetProvider(&self) -> ::windows::core::Result<IVdsProvider> {
        let mut result__ = ::windows::core::zeroed::<IVdsProvider>();
        (::windows::core::Interface::vtable(self).GetProvider)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryControllers(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryControllers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryLuns(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryLuns)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDrives(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryDrives)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDrive(&self, sbusnumber: i16, sslotnumber: i16) -> ::windows::core::Result<IVdsDrive> {
        let mut result__ = ::windows::core::zeroed::<IVdsDrive>();
        (::windows::core::Interface::vtable(self).GetDrive)(::windows::core::Interface::as_raw(self), sbusnumber, sslotnumber, &mut result__).from_abi(result__)
    }
    pub unsafe fn Reenumerate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reenumerate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetControllerStatus(&self, ponlinecontrolleridarray: &[::windows::core::GUID], pofflinecontrolleridarray: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetControllerStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ponlinecontrolleridarray.as_ptr()), ponlinecontrolleridarray.len() as _, ::core::mem::transmute(pofflinecontrolleridarray.as_ptr()), pofflinecontrolleridarray.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLun<P0>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: ::core::option::Option<&[::windows::core::GUID]>, pwszunmaskinglist: P0, phints: ::core::option::Option<*const VDS_HINTS>) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).CreateLun)(::windows::core::Interface::as_raw(self), r#type, ullsizeinbytes, ::core::mem::transmute(pdriveidarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriveidarray.as_deref().map_or(0, |slice| slice.len() as _), pwszunmaskinglist.into_param().abi(), ::core::mem::transmute(phints.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReplaceDrive(&self, drivetobereplaced: ::windows::core::GUID, replacementdrive: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReplaceDrive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(drivetobereplaced), ::core::mem::transmute(replacementdrive)).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), status).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMaxLunCreateSize(&self, r#type: VDS_LUN_TYPE, pdriveidarray: ::core::option::Option<&[::windows::core::GUID]>, phints: ::core::option::Option<*const VDS_HINTS>) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).QueryMaxLunCreateSize)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(pdriveidarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriveidarray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(phints.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsSubSystem, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsSubSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystem {}
impl ::core::fmt::Debug for IVdsSubSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystem {
    type Vtable = IVdsSubSystem_Vtbl;
}
impl ::core::clone::Clone for IVdsSubSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsSubSystem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fcee2d3_6d90_4f91_80e2_a5c7caaca9d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows::core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryDrives: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDrive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reenumerate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetControllerStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ponlinecontrolleridarray: *const ::windows::core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::core::GUID, lnumberofofflinecontrollers: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateLun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: ::windows::core::PCWSTR, phints: *const VDS_HINTS, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateLun: usize,
    pub ReplaceDrive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drivetobereplaced: ::windows::core::GUID, replacementdrive: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryMaxLunCreateSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryMaxLunCreateSize: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsSubSystem2(::windows::core::IUnknown);
impl IVdsSubSystem2 {
    pub unsafe fn GetProperties2(&self, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties2)(::windows::core::Interface::as_raw(self), psubsystemprop2).ok()
    }
    pub unsafe fn GetDrive2(&self, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32) -> ::windows::core::Result<IVdsDrive> {
        let mut result__ = ::windows::core::zeroed::<IVdsDrive>();
        (::windows::core::Interface::vtable(self).GetDrive2)(::windows::core::Interface::as_raw(self), sbusnumber, sslotnumber, ulenclosurenumber, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLun2<P0>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: ::core::option::Option<&[::windows::core::GUID]>, pwszunmaskinglist: P0, phints2: ::core::option::Option<*const VDS_HINTS2>) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).CreateLun2)(::windows::core::Interface::as_raw(self), r#type, ullsizeinbytes, ::core::mem::transmute(pdriveidarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriveidarray.as_deref().map_or(0, |slice| slice.len() as _), pwszunmaskinglist.into_param().abi(), ::core::mem::transmute(phints2.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMaxLunCreateSize2(&self, r#type: VDS_LUN_TYPE, pdriveidarray: ::core::option::Option<&[::windows::core::GUID]>, phints2: ::core::option::Option<*const VDS_HINTS2>) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).QueryMaxLunCreateSize2)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(pdriveidarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdriveidarray.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(phints2.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsSubSystem2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsSubSystem2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystem2 {}
impl ::core::fmt::Debug for IVdsSubSystem2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystem2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystem2 {
    type Vtable = IVdsSubSystem2_Vtbl;
}
impl ::core::clone::Clone for IVdsSubSystem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsSubSystem2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe666735_7800_4a77_9d9c_40f85b87e292);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystem2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows::core::HRESULT,
    pub GetDrive2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateLun2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: ::windows::core::PCWSTR, phints2: *const VDS_HINTS2, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateLun2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryMaxLunCreateSize2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryMaxLunCreateSize2: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsSubSystemImportTarget(::windows::core::IUnknown);
impl IVdsSubSystemImportTarget {
    pub unsafe fn GetImportTarget(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetImportTarget)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetImportTarget<P0>(&self, pwsziscsiname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetImportTarget)(::windows::core::Interface::as_raw(self), pwsziscsiname.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsSubSystemImportTarget, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsSubSystemImportTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystemImportTarget {}
impl ::core::fmt::Debug for IVdsSubSystemImportTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystemImportTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystemImportTarget {
    type Vtable = IVdsSubSystemImportTarget_Vtbl;
}
impl ::core::clone::Clone for IVdsSubSystemImportTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsSubSystemImportTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83bfb87f_43fb_4903_baa6_127f01029eec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemImportTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetImportTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwsziscsiname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetImportTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsziscsiname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsSubSystemInterconnect(::windows::core::IUnknown);
impl IVdsSubSystemInterconnect {
    pub unsafe fn GetSupportedInterconnects(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSupportedInterconnects)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsSubSystemInterconnect, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsSubSystemInterconnect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystemInterconnect {}
impl ::core::fmt::Debug for IVdsSubSystemInterconnect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystemInterconnect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystemInterconnect {
    type Vtable = IVdsSubSystemInterconnect_Vtbl;
}
impl ::core::clone::Clone for IVdsSubSystemInterconnect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsSubSystemInterconnect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6fa560_c141_477b_83ba_0b6c38f7febf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemInterconnect_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSupportedInterconnects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulsupportedinterconnectsflag: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsSubSystemIscsi(::windows::core::IUnknown);
impl IVdsSubSystemIscsi {
    pub unsafe fn QueryTargets(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryTargets)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryPortals(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryPortals)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateTarget<P0, P1>(&self, pwsziscsiname: P0, pwszfriendlyname: P1) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).CreateTarget)(::windows::core::Interface::as_raw(self), pwsziscsiname.into_param().abi(), pwszfriendlyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIpsecGroupPresharedKey(&self, pipseckey: ::core::option::Option<*const VDS_ISCSI_IPSEC_KEY>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIpsecGroupPresharedKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pipseckey.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsSubSystemIscsi, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsSubSystemIscsi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystemIscsi {}
impl ::core::fmt::Debug for IVdsSubSystemIscsi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystemIscsi").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystemIscsi {
    type Vtable = IVdsSubSystemIscsi_Vtbl;
}
impl ::core::clone::Clone for IVdsSubSystemIscsi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsSubSystemIscsi {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0027346f_40d0_4b45_8cec_5906dc0380c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemIscsi_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryPortals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsziscsiname: ::windows::core::PCWSTR, pwszfriendlyname: ::windows::core::PCWSTR, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIpsecGroupPresharedKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsSubSystemNaming(::windows::core::IUnknown);
impl IVdsSubSystemNaming {
    pub unsafe fn SetFriendlyName<P0>(&self, pwszfriendlyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetFriendlyName)(::windows::core::Interface::as_raw(self), pwszfriendlyname.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsSubSystemNaming, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsSubSystemNaming {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSubSystemNaming {}
impl ::core::fmt::Debug for IVdsSubSystemNaming {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSubSystemNaming").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSubSystemNaming {
    type Vtable = IVdsSubSystemNaming_Vtbl;
}
impl ::core::clone::Clone for IVdsSubSystemNaming {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsSubSystemNaming {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d70faa3_9cd4_4900_aa20_6981b6aafc75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemNaming_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsSwProvider(::windows::core::IUnknown);
impl IVdsSwProvider {
    pub unsafe fn QueryPacks(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryPacks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePack(&self) -> ::windows::core::Result<IVdsPack> {
        let mut result__ = ::windows::core::zeroed::<IVdsPack>();
        (::windows::core::Interface::vtable(self).CreatePack)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsSwProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsSwProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsSwProvider {}
impl ::core::fmt::Debug for IVdsSwProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsSwProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsSwProvider {
    type Vtable = IVdsSwProvider_Vtbl;
}
impl ::core::clone::Clone for IVdsSwProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsSwProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9aa58360_ce33_4f92_b658_ed24b14425b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSwProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryPacks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppack: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVDisk(::windows::core::IUnknown);
impl IVdsVDisk {
    #[doc = "*Required features: `\"Win32_Storage_Vhd\"`*"]
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn Open(&self, accessmask: super::Vhd::VIRTUAL_DISK_ACCESS_MASK, flags: super::Vhd::OPEN_VIRTUAL_DISK_FLAG, readwritedepth: u32) -> ::windows::core::Result<IVdsOpenVDisk> {
        let mut result__ = ::windows::core::zeroed::<IVdsOpenVDisk>();
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), accessmask, flags, readwritedepth, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Vhd\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
    pub unsafe fn GetProperties(&self, pdiskproperties: *mut VDS_VDISK_PROPERTIES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pdiskproperties).ok()
    }
    pub unsafe fn GetHostVolume(&self) -> ::windows::core::Result<IVdsVolume> {
        let mut result__ = ::windows::core::zeroed::<IVdsVolume>();
        (::windows::core::Interface::vtable(self).GetHostVolume)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDeviceName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsVDisk, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVDisk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVDisk {}
impl ::core::fmt::Debug for IVdsVDisk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVDisk").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVDisk {
    type Vtable = IVdsVDisk_Vtbl;
}
impl ::core::clone::Clone for IVdsVDisk {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVDisk {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e062b84_e5e6_4b4b_8a25_67b81e8f13e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVDisk_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmask: super::Vhd::VIRTUAL_DISK_ACCESS_MASK, flags: super::Vhd::OPEN_VIRTUAL_DISK_FLAG, readwritedepth: u32, ppopenvdisk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    Open: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdiskproperties: *mut VDS_VDISK_PROPERTIES) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd")))]
    GetProperties: usize,
    pub GetHostVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvolume: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdevicename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVdProvider(::windows::core::IUnknown);
impl IVdsVdProvider {
    pub unsafe fn QueryVDisks(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryVDisks)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Vhd\"`*"]
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn CreateVDisk<P0, P1>(&self, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: P0, pstringsecuritydescriptor: P1, flags: super::Vhd::CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, reserved: u32, pcreatediskparameters: *const VDS_CREATE_VDISK_PARAMETERS, ppasync: ::core::option::Option<*mut ::core::option::Option<IVdsAsync>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CreateVDisk)(::windows::core::Interface::as_raw(self), virtualdevicetype, ppath.into_param().abi(), pstringsecuritydescriptor.into_param().abi(), flags, providerspecificflags, reserved, pcreatediskparameters, ::core::mem::transmute(ppasync.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Vhd\"`*"]
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub unsafe fn AddVDisk<P0>(&self, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: P0, ppvdisk: ::core::option::Option<*mut ::core::option::Option<IVdsVDisk>>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddVDisk)(::windows::core::Interface::as_raw(self), virtualdevicetype, ppath.into_param().abi(), ::core::mem::transmute(ppvdisk.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDiskFromVDisk<P0>(&self, pvdisk: P0) -> ::windows::core::Result<IVdsDisk>
    where
        P0: ::windows::core::IntoParam<IVdsVDisk>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsDisk>();
        (::windows::core::Interface::vtable(self).GetDiskFromVDisk)(::windows::core::Interface::as_raw(self), pvdisk.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVDiskFromDisk<P0>(&self, pdisk: P0) -> ::windows::core::Result<IVdsVDisk>
    where
        P0: ::windows::core::IntoParam<IVdsDisk>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsVDisk>();
        (::windows::core::Interface::vtable(self).GetVDiskFromDisk)(::windows::core::Interface::as_raw(self), pdisk.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsVdProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVdProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVdProvider {}
impl ::core::fmt::Debug for IVdsVdProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVdProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVdProvider {
    type Vtable = IVdsVdProvider_Vtbl;
}
impl ::core::clone::Clone for IVdsVdProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVdProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb481498c_8354_45f9_84a0_0bdd2832a91f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVdProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryVDisks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub CreateVDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: ::windows::core::PCWSTR, pstringsecuritydescriptor: ::windows::core::PCWSTR, flags: super::Vhd::CREATE_VIRTUAL_DISK_FLAG, providerspecificflags: u32, reserved: u32, pcreatediskparameters: *const VDS_CREATE_VDISK_PARAMETERS, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    CreateVDisk: usize,
    #[cfg(feature = "Win32_Storage_Vhd")]
    pub AddVDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, virtualdevicetype: *const super::Vhd::VIRTUAL_STORAGE_TYPE, ppath: ::windows::core::PCWSTR, ppvdisk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Vhd"))]
    AddVDisk: usize,
    pub GetDiskFromVDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvdisk: *mut ::core::ffi::c_void, ppdisk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVDiskFromDisk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisk: *mut ::core::ffi::c_void, ppvdisk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVolume(::windows::core::IUnknown);
impl IVdsVolume {
    pub unsafe fn GetProperties(&self, pvolumeproperties: *mut VDS_VOLUME_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pvolumeproperties).ok()
    }
    pub unsafe fn GetPack(&self) -> ::windows::core::Result<IVdsPack> {
        let mut result__ = ::windows::core::zeroed::<IVdsPack>();
        (::windows::core::Interface::vtable(self).GetPack)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryPlexes(&self) -> ::windows::core::Result<IEnumVdsObject> {
        let mut result__ = ::windows::core::zeroed::<IEnumVdsObject>();
        (::windows::core::Interface::vtable(self).QueryPlexes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Extend(&self, pinputdiskarray: ::core::option::Option<&[VDS_INPUT_DISK]>) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Extend)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinputdiskarray.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pinputdiskarray.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
    }
    pub unsafe fn Shrink(&self, ullnumberofbytestoremove: u64) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Shrink)(::windows::core::Interface::as_raw(self), ullnumberofbytestoremove, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddPlex(&self, volumeid: ::windows::core::GUID) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).AddPlex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(volumeid), &mut result__).from_abi(result__)
    }
    pub unsafe fn BreakPlex(&self, plexid: ::windows::core::GUID) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).BreakPlex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plexid), &mut result__).from_abi(result__)
    }
    pub unsafe fn RemovePlex(&self, plexid: ::windows::core::GUID) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).RemovePlex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plexid), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delete<P0>(&self, bforce: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), bforce.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFlags<P0>(&self, ulflags: u32, brevertonclose: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetFlags)(::windows::core::Interface::as_raw(self), ulflags, brevertonclose.into_param().abi()).ok()
    }
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearFlags)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsVolume, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVolume {}
impl ::core::fmt::Debug for IVdsVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVolume").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVolume {
    type Vtable = IVdsVolume_Vtbl;
}
impl ::core::clone::Clone for IVdsVolume {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVolume {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88306bb2_e71f_478c_86a2_79da200a0f11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVolume_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvolumeproperties: *mut VDS_VOLUME_PROP) -> ::windows::core::HRESULT,
    pub GetPack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppack: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryPlexes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Extend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Shrink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddPlex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumeid: ::windows::core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BreakPlex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plexid: ::windows::core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemovePlex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plexid: ::windows::core::GUID, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bforce: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delete: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32, brevertonclose: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFlags: usize,
    pub ClearFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVolume2(::windows::core::IUnknown);
impl IVdsVolume2 {
    pub unsafe fn GetProperties2(&self, pvolumeproperties: *mut VDS_VOLUME_PROP2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties2)(::windows::core::Interface::as_raw(self), pvolumeproperties).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsVolume2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVolume2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVolume2 {}
impl ::core::fmt::Debug for IVdsVolume2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVolume2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVolume2 {
    type Vtable = IVdsVolume2_Vtbl;
}
impl ::core::clone::Clone for IVdsVolume2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVolume2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72ae6713_dcbb_4a03_b36b_371f6ac6b53d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVolume2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvolumeproperties: *mut VDS_VOLUME_PROP2) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVolumeMF(::windows::core::IUnknown);
impl IVdsVolumeMF {
    pub unsafe fn GetFileSystemProperties(&self, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFileSystemProperties)(::windows::core::Interface::as_raw(self), pfilesystemprop).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Format<P0, P1, P2, P3>(&self, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: P0, dwunitallocationsize: u32, bforce: P1, bquickformat: P2, benablecompression: P3) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Format)(::windows::core::Interface::as_raw(self), r#type, pwszlabel.into_param().abi(), dwunitallocationsize, bforce.into_param().abi(), bquickformat.into_param().abi(), benablecompression.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn AddAccessPath<P0>(&self, pwszpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddAccessPath)(::windows::core::Interface::as_raw(self), pwszpath.into_param().abi()).ok()
    }
    pub unsafe fn QueryAccessPaths(&self, pwszpatharray: *mut *mut ::windows::core::PWSTR, plnumberofaccesspaths: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryAccessPaths)(::windows::core::Interface::as_raw(self), pwszpatharray, plnumberofaccesspaths).ok()
    }
    pub unsafe fn QueryReparsePoints(&self, ppreparsepointprops: *mut *mut VDS_REPARSE_POINT_PROP, plnumberofreparsepointprops: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryReparsePoints)(::windows::core::Interface::as_raw(self), ppreparsepointprops, plnumberofreparsepointprops).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteAccessPath<P0, P1>(&self, pwszpath: P0, bforce: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).DeleteAccessPath)(::windows::core::Interface::as_raw(self), pwszpath.into_param().abi(), bforce.into_param().abi()).ok()
    }
    pub unsafe fn Mount(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Mount)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Dismount<P0, P1>(&self, bforce: P0, bpermanent: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Dismount)(::windows::core::Interface::as_raw(self), bforce.into_param().abi(), bpermanent.into_param().abi()).ok()
    }
    pub unsafe fn SetFileSystemFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFileSystemFlags)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
    pub unsafe fn ClearFileSystemFlags(&self, ulflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearFileSystemFlags)(::windows::core::Interface::as_raw(self), ulflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsVolumeMF, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVolumeMF {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVolumeMF {}
impl ::core::fmt::Debug for IVdsVolumeMF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVolumeMF").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVolumeMF {
    type Vtable = IVdsVolumeMF_Vtbl;
}
impl ::core::clone::Clone for IVdsVolumeMF {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVolumeMF {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee2d5ded_6236_4169_931d_b9778ce03dc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVolumeMF_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFileSystemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilesystemprop: *mut VDS_FILE_SYSTEM_PROP) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: VDS_FILE_SYSTEM_TYPE, pwszlabel: ::windows::core::PCWSTR, dwunitallocationsize: u32, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Format: usize,
    pub AddAccessPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub QueryAccessPaths: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpatharray: *mut *mut ::windows::core::PWSTR, plnumberofaccesspaths: *mut i32) -> ::windows::core::HRESULT,
    pub QueryReparsePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreparsepointprops: *mut *mut VDS_REPARSE_POINT_PROP, plnumberofreparsepointprops: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteAccessPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpath: ::windows::core::PCWSTR, bforce: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteAccessPath: usize,
    pub Mount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Dismount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bforce: super::super::Foundation::BOOL, bpermanent: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Dismount: usize,
    pub SetFileSystemFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
    pub ClearFileSystemFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVolumeMF2(::windows::core::IUnknown);
impl IVdsVolumeMF2 {
    pub unsafe fn GetFileSystemTypeName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetFileSystemTypeName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryFileSystemFormatSupport(&self, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryFileSystemFormatSupport)(::windows::core::Interface::as_raw(self), ppfilesystemsupportprops, plnumberoffilesystems).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FormatEx<P0, P1, P2, P3, P4>(&self, pwszfilesystemtypename: P0, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: P1, bforce: P2, bquickformat: P3, benablecompression: P4) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).FormatEx)(::windows::core::Interface::as_raw(self), pwszfilesystemtypename.into_param().abi(), usfilesystemrevision, uldesiredunitallocationsize, pwszlabel.into_param().abi(), bforce.into_param().abi(), bquickformat.into_param().abi(), benablecompression.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsVolumeMF2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVolumeMF2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVolumeMF2 {}
impl ::core::fmt::Debug for IVdsVolumeMF2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVolumeMF2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVolumeMF2 {
    type Vtable = IVdsVolumeMF2_Vtbl;
}
impl ::core::clone::Clone for IVdsVolumeMF2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVolumeMF2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dbcee9a_6343_4651_b85f_5e75d74d983c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVolumeMF2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFileSystemTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszfilesystemtypename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub QueryFileSystemFormatSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfilesystemsupportprops: *mut *mut VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP, plnumberoffilesystems: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FormatEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilesystemtypename: ::windows::core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: ::windows::core::PCWSTR, bforce: super::super::Foundation::BOOL, bquickformat: super::super::Foundation::BOOL, benablecompression: super::super::Foundation::BOOL, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FormatEx: usize,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVolumeMF3(::windows::core::IUnknown);
impl IVdsVolumeMF3 {
    pub unsafe fn QueryVolumeGuidPathnames(&self, pwszpatharray: *mut *mut ::windows::core::PWSTR, pulnumberofpaths: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryVolumeGuidPathnames)(::windows::core::Interface::as_raw(self), pwszpatharray, pulnumberofpaths).ok()
    }
    pub unsafe fn FormatEx2<P0, P1>(&self, pwszfilesystemtypename: P0, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: P1, options: u32) -> ::windows::core::Result<IVdsAsync>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).FormatEx2)(::windows::core::Interface::as_raw(self), pwszfilesystemtypename.into_param().abi(), usfilesystemrevision, uldesiredunitallocationsize, pwszlabel.into_param().abi(), options, &mut result__).from_abi(result__)
    }
    pub unsafe fn OfflineVolume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OfflineVolume)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsVolumeMF3, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVolumeMF3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVolumeMF3 {}
impl ::core::fmt::Debug for IVdsVolumeMF3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVolumeMF3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVolumeMF3 {
    type Vtable = IVdsVolumeMF3_Vtbl;
}
impl ::core::clone::Clone for IVdsVolumeMF3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVolumeMF3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6788faf9_214e_4b85_ba59_266953616e09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVolumeMF3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryVolumeGuidPathnames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszpatharray: *mut *mut ::windows::core::PWSTR, pulnumberofpaths: *mut u32) -> ::windows::core::HRESULT,
    pub FormatEx2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszfilesystemtypename: ::windows::core::PCWSTR, usfilesystemrevision: u16, uldesiredunitallocationsize: u32, pwszlabel: ::windows::core::PCWSTR, options: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OfflineVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVolumeOnline(::windows::core::IUnknown);
impl IVdsVolumeOnline {
    pub unsafe fn Online(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Online)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IVdsVolumeOnline, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVolumeOnline {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVolumeOnline {}
impl ::core::fmt::Debug for IVdsVolumeOnline {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVolumeOnline").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVolumeOnline {
    type Vtable = IVdsVolumeOnline_Vtbl;
}
impl ::core::clone::Clone for IVdsVolumeOnline {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVolumeOnline {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1be2275a_b315_4f70_9e44_879b3a2a53f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVolumeOnline_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Online: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVolumePlex(::windows::core::IUnknown);
impl IVdsVolumePlex {
    pub unsafe fn GetProperties(&self, pplexproperties: *mut VDS_VOLUME_PLEX_PROP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProperties)(::windows::core::Interface::as_raw(self), pplexproperties).ok()
    }
    pub unsafe fn GetVolume(&self) -> ::windows::core::Result<IVdsVolume> {
        let mut result__ = ::windows::core::zeroed::<IVdsVolume>();
        (::windows::core::Interface::vtable(self).GetVolume)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryExtents)(::windows::core::Interface::as_raw(self), ppextentarray, plnumberofextents).ok()
    }
    pub unsafe fn Repair(&self, pinputdiskarray: &[VDS_INPUT_DISK]) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Repair)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinputdiskarray.as_ptr()), pinputdiskarray.len() as _, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsVolumePlex, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVolumePlex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVolumePlex {}
impl ::core::fmt::Debug for IVdsVolumePlex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVolumePlex").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVolumePlex {
    type Vtable = IVdsVolumePlex_Vtbl;
}
impl ::core::clone::Clone for IVdsVolumePlex {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVolumePlex {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4daa0135_e1d1_40f1_aaa5_3cc1e53221c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVolumePlex_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplexproperties: *mut VDS_VOLUME_PLEX_PROP) -> ::windows::core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvolume: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryExtents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DISK_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT,
    pub Repair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputdiskarray: *const VDS_INPUT_DISK, lnumberofdisks: i32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
pub struct IVdsVolumeShrink(::windows::core::IUnknown);
impl IVdsVolumeShrink {
    pub unsafe fn QueryMaxReclaimableBytes(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).QueryMaxReclaimableBytes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Shrink(&self, ulldesirednumberofreclaimablebytes: u64, ullminnumberofreclaimablebytes: u64) -> ::windows::core::Result<IVdsAsync> {
        let mut result__ = ::windows::core::zeroed::<IVdsAsync>();
        (::windows::core::Interface::vtable(self).Shrink)(::windows::core::Interface::as_raw(self), ulldesirednumberofreclaimablebytes, ullminnumberofreclaimablebytes, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVdsVolumeShrink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVdsVolumeShrink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVdsVolumeShrink {}
impl ::core::fmt::Debug for IVdsVolumeShrink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVdsVolumeShrink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVdsVolumeShrink {
    type Vtable = IVdsVolumeShrink_Vtbl;
}
impl ::core::clone::Clone for IVdsVolumeShrink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVdsVolumeShrink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd68168c9_82a2_4f85_b6e9_74707c49a58f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsVolumeShrink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub QueryMaxReclaimableBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullmaxnumberofreclaimablebytes: *mut u64) -> ::windows::core::HRESULT,
    pub Shrink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulldesirednumberofreclaimablebytes: u64, ullminnumberofreclaimablebytes: u64, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const CLSID_VdsLoader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c38ed61_d565_4728_aeee_c80952f0ecde);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const CLSID_VdsService: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d1933cb_86f6_4a98_8628_01be94c9a575);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const GPT_PARTITION_NAME_LENGTH: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const MAX_FS_ALLOWED_CLUSTER_SIZES_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const MAX_FS_FORMAT_SUPPORT_NAME_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const MAX_FS_NAME_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ATTACH_VIRTUAL_DISK_FLAG_USE_FILE_ACL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ACCESS_DENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212249i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ACTIVE_PARTITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212232i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ADDRESSES_INCOMPLETELY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211517i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_BEYOND_FIRST_CYLINDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211949i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_IS_ZERO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211888i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_NOT_A_POWER_OF_TWO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211889i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_NOT_SECTOR_SIZE_MULTIPLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211948i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALIGN_NOT_ZERO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211947i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212285i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ANOTHER_CALL_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212284i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ASSOCIATED_LUNS_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211509i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ASSOCIATED_PORTALS_EXIST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211508i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ASYNC_OBJECT_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212210i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_BOOT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211898i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_COOKIE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212271i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_LABEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212247i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_PNP_MESSAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212017i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_PROVIDER_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212223i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BAD_REVISION_NUMBER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211880i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BLOCK_CLUSTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210749i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BOOT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211257i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BOOT_PAGEFILE_DRIVE_LETTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210994i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_BOOT_PARTITION_NUMBER_CHANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212234i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CACHE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211946i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANCEL_TOO_LATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212276i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANNOT_CLEAR_VOLUME_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211945i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANNOT_EXTEND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212274i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANNOT_SHRINK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212002i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANT_INVALIDATE_FVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211886i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CANT_QUICK_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212246i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLEAN_WITH_BOOTBACKING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210743i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLEAN_WITH_CRITICAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210990i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLEAN_WITH_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210992i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLEAN_WITH_OEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210991i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLUSTER_COUNT_BEYOND_32BITS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212240i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLUSTER_SIZE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212241i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CLUSTER_SIZE_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212242i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_COMPRESSION_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210984i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CONFIG_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211976i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CORRUPT_EXTENT_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212021i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CORRUPT_NOTIFICATION_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211990i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CORRUPT_PARTITION_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212023i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CORRUPT_VOLUME_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212029i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CRASHDUMP_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211250i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_CRITICAL_PLEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211906i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DELETE_WITH_BOOTBACKING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210745i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DELETE_WITH_CRITICAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210993i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DEVICE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212269i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_BEING_CLEANED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211944i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_CONFIGURATION_CORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211975i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_CONFIGURATION_NOT_IN_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211974i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_CONFIGURATION_UPDATE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211973i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_DYNAMIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211972i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_HAS_BANDS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210748i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_IN_USE_BY_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212212i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_IO_FAILING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211968i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_IS_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211254i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_IS_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211253i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_LAYOUT_PARTITIONS_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211969i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_CONVERTIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211943i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_CONVERTIBLE_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210971i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212268i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_FOUND_IN_PACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211987i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_IMPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212206i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212265i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_LOADED_TO_CACHE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212217i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212031i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211883i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_NOT_ONLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212213i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_PNP_REG_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212203i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_REMOVEABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211942i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISK_REMOVEABLE_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211941i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DISTINCT_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211909i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DMADMIN_CORRUPT_NOTIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212252i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DMADMIN_METHOD_CALL_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212256i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DMADMIN_SERVICE_CONNECTION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212261i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVER_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212027i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVER_INVALID_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212004i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVER_NO_PACK_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212019i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVER_OBJECT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211971i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DRIVE_LETTER_NOT_FREE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211940i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DUPLICATE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211986i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DUP_EMPTY_PACK_GUID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212020i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_DYNAMIC_DISKS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211967i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTEND_FILE_SYSTEM_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212186i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTEND_MULTIPLE_DISKS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211939i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTEND_TOO_MANY_CLUSTERS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210968i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTEND_UNKNOWN_FILESYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210967i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTENT_EXCEEDS_DISK_FREE_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212011i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_EXTENT_SIZE_LESS_THAN_MIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212237i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAILED_TO_OFFLINE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211881i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAILED_TO_ONLINE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211882i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAT32_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210987i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAT_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210986i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FAULT_TOLERANT_DISKS_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211966i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FLAG_ALREADY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211911i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FORMAT_CRITICAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210989i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210985i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FORMAT_WITH_BOOTBACKING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210744i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_FS_NOT_DETERMINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211885i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_GET_SAN_POLICY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211259i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_GPT_ATTRIBUTES_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211965i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_HIBERNATION_FILE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211251i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_IA64_BOOT_MIRRORED_TO_MBR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212198i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_IMPORT_SET_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212207i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INCOMPATIBLE_FILE_SYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212251i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INCOMPATIBLE_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212250i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INCORRECT_BOOT_VOLUME_EXTENT_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211260i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INCORRECT_SYSTEM_VOLUME_EXTENT_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211248i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INITIALIZED_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212287i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INITIALIZE_NOT_CALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212286i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INITIATOR_ADAPTER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211008i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INITIATOR_SPECIFIC_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211513i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212216i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_BLOCK_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211982i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212007i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_DISK_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211994i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_DRIVE_LETTER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211938i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_DRIVE_LETTER_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211937i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_ENUMERATOR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212028i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_EXTENT_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211993i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_FS_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211936i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_FS_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211935i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_IP_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210997i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_ISCSI_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210980i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_ISCSI_TARGET_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211005i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_MEMBER_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211998i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_MEMBER_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211996i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_OBJECT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211934i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_OPERATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212267i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212006i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PARTITION_LAYOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211933i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PARTITION_STYLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211932i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PARTITION_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211931i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210981i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_BLOCK_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211978i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_COUNT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211999i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_GUID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211988i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211997i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PLEX_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211979i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PORT_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211006i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_CLSID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211930i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_ID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211929i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211928i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211927i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_VERSION_GUID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211926i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_PROVIDER_VERSION_STRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211925i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_QUERY_PROVIDER_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211924i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_SECTOR_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211984i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_SERVICE_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211923i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_SHRINK_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211241i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212282i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210747i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_STRIPE_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211995i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_VOLUME_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211922i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_VOLUME_LENGTH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211954i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_INVALID_VOLUME_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211899i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_IO_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212245i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_CHAP_SECRET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210998i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_GET_IKE_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211003i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_GROUP_PRESHARE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210999i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_INITIATOR_NODE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211000i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_LOGIN_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211512i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_LOGOUT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211511i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_LOGOUT_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211504i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_SESSION_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211510i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ISCSI_SET_IKE_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211002i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LAST_VALID_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211985i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LBN_REMAP_ENABLED_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212202i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LDM_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212191i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LEGACY_VOLUME_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212230i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LOG_UPDATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211897i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211239i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211240i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_NOT_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211238i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_NO_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211237i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DISK_READ_ONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210978i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DYNAMIC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210976i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_DYNAMIC_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210975i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211234i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_NOT_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211236i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211235i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_SHRINK_GPT_HEADER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210974i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_LUN_UPDATE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210977i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MAX_USABLE_MBR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212184i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEDIA_WRITE_PROTECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212248i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEMBER_IS_HEALTHY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211964i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEMBER_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211958i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEMBER_REGENERATING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211963i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MEMBER_SIZE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212010i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MIGRATE_OPEN_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212228i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MIRROR_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210973i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MISSING_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212204i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MULTIPLE_DISCOVERY_DOMAINS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211506i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_MULTIPLE_PACKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212001i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NAME_NOT_UNIQUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211519i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NON_CONTIGUOUS_DATA_PARTITIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212229i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NOT_AN_UNALLOCATED_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212264i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NOT_ENOUGH_DRIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212272i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NOT_ENOUGH_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212273i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212288i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_DISCOVERY_DOMAIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211507i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_DISKS_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212258i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_DISK_PATHNAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211505i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_DRIVELETTER_FLAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212201i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_EXTENTS_FOR_PLEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211980i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_EXTENTS_FOR_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212218i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_FREE_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212233i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_HEALTHY_DISKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211977i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_IMPORT_TARGET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211501i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_MAINTENANCE_MODE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210750i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_MEDIA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212270i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_PNP_DISK_ARRIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212016i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_PNP_DISK_REMOVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212014i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_PNP_VOLUME_ARRIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212015i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_PNP_VOLUME_REMOVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212013i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_POOL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210752i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_POOL_CREATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210751i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_SOFTWARE_PROVIDERS_LOADED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212032i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_VALID_LOG_COPIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211894i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_VOLUME_LAYOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212030i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NO_VOLUME_PATHNAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211503i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_NTFS_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210988i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_DELETED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212277i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212259i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212283i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_OUT_OF_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212205i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OBJECT_STATUS_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212239i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OFFLINE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210970i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ONE_EXTENT_PER_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211983i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_ONLINE_PACK_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212188i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OPERATION_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212275i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OPERATION_DENIED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212278i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_OPERATION_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212279i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PACK_NAME_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211962i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PACK_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212208i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PACK_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212220i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PACK_ONLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212000i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PAGEFILE_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211252i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_LDM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211891i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_LIMIT_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212281i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_MSR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211892i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_NON_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211907i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_NOT_CYLINDER_ALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211970i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212280i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_NOT_OEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211921i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_OF_UNKNOWN_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212231i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_PROTECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211920i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PARTITION_STYLE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211919i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PATH_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212266i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_IS_HEALTHY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211961i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_LAST_ACTIVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211960i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211959i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_NOT_LOADED_TO_CACHE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211893i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_REGENERATING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211957i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PLEX_SIZE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211981i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_CACHE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212257i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_CACHE_OUTOFSYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211502i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_EXITING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212012i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212222i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_INITIALIZATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212260i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_INTERNAL_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211918i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_TYPE_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212214i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_VOL_DEVICE_NAME_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212254i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_PROVIDER_VOL_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212253i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_RAID5_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210972i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211900i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REBOOT_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210996i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REFS_FORMAT_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210746i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REPAIR_VOLUMESTATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212192i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REQUIRES_CONTIGUOUS_DISK_SPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212224i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_RETRY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212189i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REVERT_ON_CLOSE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212200i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REVERT_ON_CLOSE_MISMATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212190i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_REVERT_ON_CLOSE_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212199i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SECTOR_SIZE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211229i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SECURITY_INCOMPLETELY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211515i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SET_SAN_POLICY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211258i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SET_TUNNEL_MODE_OUTER_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211004i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_DIRTY_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211878i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_EXTEND_UNALIGNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210496i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211887i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_LUN_NOT_UNMASKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210979i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_OVER_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211242i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_SIZE_LESS_THAN_MIN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211917i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_SIZE_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211916i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_UNKNOWN_FILESYSTEM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210966i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SHRINK_USER_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211879i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SOURCE_IS_TARGET_PACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211992i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SUBSYSTEM_ID_IS_NULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211001i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_SYSTEM_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211247i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_TARGET_PACK_NOT_EMPTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212003i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_TARGET_PORTAL_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211007i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_TARGET_SPECIFIC_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211514i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212193i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNABLE_TO_FIND_BOOT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211261i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNABLE_TO_FIND_SYSTEM_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211249i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNEXPECTED_DISK_LAYOUT_CHANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211955i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNRECOVERABLE_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212263i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_UNRECOVERABLE_PROVIDER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211915i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VDISK_INVALID_OP_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210982i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VDISK_NOT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210983i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VDISK_PATHNAME_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210969i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_ALREADY_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210956i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_ALREADY_COMPACTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210958i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_ALREADY_DETACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210955i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_ALREADY_MERGING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210957i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_ALREADY_EXPANDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210959i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_ALREADY_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210960i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_IS_COMPACTING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210963i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_IS_EXPANDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210964i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_IS_MERGING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210962i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_DISK_NOT_OPEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210965i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_IS_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210961i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_IS_BEING_ATTACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210953i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_IS_BEING_DETACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210952i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VD_NOT_ATTACHED_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210954i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_DISK_COUNT_MAX_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211991i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_EXTEND_FVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211230i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_EXTEND_FVE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211232i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_EXTEND_FVE_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211233i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_EXTEND_FVE_RECOVERY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211231i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_GUID_PATHNAME_NOT_ALLOWED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147210995i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_HAS_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212194i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_HIDDEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211914i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212238i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_INVALID_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212025i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211953i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_MIRRORED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211896i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_A_MIRROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212219i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_FOUND_IN_PACK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211908i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_HEALTHY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212226i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_MOUNTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212209i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_ONLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212227i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_NOT_RETAINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211952i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_ON_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212005i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_PERMANENTLY_DISMOUNTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212195i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_REGENERATING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211904i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_RETAINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211951i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SHRINK_FVE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211243i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SHRINK_FVE_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211245i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SHRINK_FVE_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211246i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SHRINK_FVE_RECOVERY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211244i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SIMPLE_SPANNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211895i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SPANS_DISKS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212225i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_SYNCHRONIZING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147211905i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_TEMPORARILY_DISMOUNTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212196i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_TOO_BIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212243i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_E_VOLUME_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212244i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_ALLOCATEHOTSPARE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_BUSTYPE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_CONSISTENCYCHECKENABLED: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_FASTCRASHRECOVERYREQUIRED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_HARDWARECHECKSUMENABLED: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_ISYANKABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_MEDIASCANENABLED: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_MOSTLYREADS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALREADS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALWRITES: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_READBACKVERIFYENABLED: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_READCACHINGENABLED: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_REMAPENABLED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_USEMIRROREDCACHE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_WRITECACHINGENABLED: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HINT_WRITETHROUGHCACHINGENABLED: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_LETTER_ASSIGN: u32 = 202u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_LETTER_FREE: u32 = 201u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_FILE_SYSTEM_SHRINKING_PROGRESS: u32 = 206u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_MOUNT_POINTS_CHANGE: u32 = 205u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PARTITION_ARRIVE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PARTITION_DEPART: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PARTITION_MODIFY: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_ARRIVE: u32 = 123u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_DEPART: u32 = 124u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_GROUP_ARRIVE: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_GROUP_DEPART: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_GROUP_MODIFY: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORTAL_MODIFY: u32 = 125u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_SERVICE_OUT_OF_SYNC: u32 = 301u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_SUB_SYSTEM_ARRIVE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_SUB_SYSTEM_DEPART: u32 = 102u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_SUB_SYSTEM_MODIFY: u32 = 151u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_TARGET_ARRIVE: u32 = 126u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_TARGET_DEPART: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_TARGET_MODIFY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_VOLUME_ARRIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_VOLUME_DEPART: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_VOLUME_MODIFY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_VOLUME_REBUILDING_PROGRESS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_BDW_WT_HINT: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_DIR_HINT: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_LTNCY_HINT: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_RNDM_HINT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ACCS_SIZE_HINT: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_ALLOW_SPINDOWN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_BUSTYPE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_CUSTOM_ATTRIB: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_DATA_AVL_HINT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_DATA_RDNCY_DEF: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MAX: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MIN: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NO_SINGLE_POF: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS_DEF: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MAX: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MIN: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_PKG_RDNCY_DEF: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MAX: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MIN: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_RAIDTYPE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STOR_COST_HINT: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STOR_EFFCY_HINT: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_DEF: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MAX: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MIN: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_POOL_ATTRIB_THIN_PROVISION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_REBUILD_PRIORITY_MAX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_REBUILD_PRIORITY_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ACCESS_PATH_NOT_DELETED: ::windows::core::HRESULT = ::windows::core::HRESULT(279108i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(272148i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_BOOT_PARTITION_NUMBER_CHANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(271414i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DEFAULT_PLEX_MEMBER_IDS: ::windows::core::HRESULT = ::windows::core::HRESULT(271640i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISK_DISMOUNT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(272393i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISK_IS_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(271624i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISK_MOUNT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(272392i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISK_PARTIALLY_CLEANED: ::windows::core::HRESULT = ::windows::core::HRESULT(271386i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_DISMOUNT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271735i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_EXTEND_FILE_SYSTEM_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271461i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_FS_LOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(271747i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_GPT_BOOT_MIRRORED_TO_MBR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212183i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_IA64_BOOT_MIRRORED_TO_MBR: ::windows::core::HRESULT = ::windows::core::HRESULT(271450i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(271437i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ISCSI_LOGIN_ALREAD_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(272386i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ISCSI_PERSISTENT_LOGIN_MAY_NOT_BE_REMOVED: ::windows::core::HRESULT = ::windows::core::HRESULT(272385i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_ISCSI_SESSION_NOT_FOUND_PERSISTENT_LOGIN_REMOVED: ::windows::core::HRESULT = ::windows::core::HRESULT(272384i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_MBR_BOOT_MIRRORED_TO_GPT: ::windows::core::HRESULT = ::windows::core::HRESULT(271463i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_NAME_TRUNCATED: ::windows::core::HRESULT = ::windows::core::HRESULT(272128i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_NONCONFORMANT_PARTITION_INFO: ::windows::core::HRESULT = ::windows::core::HRESULT(271626i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_NO_NOTIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(271639i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_PLEX_NOT_LOADED_TO_CACHE: ::windows::core::HRESULT = ::windows::core::HRESULT(271755i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_PROPERTIES_INCOMPLETE: ::windows::core::HRESULT = ::windows::core::HRESULT(272149i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_PROVIDER_ERROR_LOADING_CACHE: ::windows::core::HRESULT = ::windows::core::HRESULT(271393i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_REMOUNT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271736i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_RESYNC_NOTIFICATION_TASK_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271738i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_STATUSES_INCOMPLETELY_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(272130i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_SYSTEM_PARTITION: ::windows::core::HRESULT = ::windows::core::HRESULT(271630i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_UNABLE_TO_GET_GPT_ATTRIBUTES: ::windows::core::HRESULT = ::windows::core::HRESULT(271451i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_UPDATE_BOOTFILE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271412i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_VOLUME_COMPRESS_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(271427i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_VSS_FLUSH_AND_HOLD_WRITES: ::windows::core::HRESULT = ::windows::core::HRESULT(271745i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_VSS_RELEASE_WRITES: ::windows::core::HRESULT = ::windows::core::HRESULT(271746i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_S_WINPE_BOOTENTRY: ::windows::core::HRESULT = ::windows::core::HRESULT(271758i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VER_VDS_LUN_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_ASYNC_OUTPUT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_UNKNOWN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATEVOLUME: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_EXTENDVOLUME: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_SHRINKVOLUME: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_ADDVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_BREAKVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REMOVEVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REPAIRVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_RECOVERPACK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REPLACEDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATEPARTITION: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CLEAN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATELUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(50i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_ADDLUNPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(52i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REMOVELUNPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(53i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_EXTENDLUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(54i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_SHRINKLUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(55i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_RECOVERLUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(56i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_LOGINTOTARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(60i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_LOGOUTFROMTARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(61i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATETARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(62i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(63i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_DELETETARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(64i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_ADDPORTAL: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(65i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_REMOVEPORTAL: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(66i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_DELETEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(67i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_FORMAT: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(101i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_CREATE_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(200i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_ATTACH_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(201i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_COMPACT_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(202i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_MERGE_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(203i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ASYNCOUT_EXPAND_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(204i32);
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_TYPE {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_ASYNC_OUTPUT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_CONTROLLER_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_UNKNOWN: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_ONLINE: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_NOT_READY: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_OFFLINE: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_FAILED: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_CS_REMOVED: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(8i32);
impl ::core::marker::Copy for VDS_CONTROLLER_STATUS {}
impl ::core::clone::Clone for VDS_CONTROLLER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_CONTROLLER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_CONTROLLER_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_CONTROLLER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_CONTROLLER_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_DISK_EXTENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_UNKNOWN: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_FREE: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_DATA: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_OEM: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_ESP: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_MSR: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_LDM: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_CLUSTER: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DET_UNUSABLE: VDS_DISK_EXTENT_TYPE = VDS_DISK_EXTENT_TYPE(32767i32);
impl ::core::marker::Copy for VDS_DISK_EXTENT_TYPE {}
impl ::core::clone::Clone for VDS_DISK_EXTENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_DISK_EXTENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_DISK_EXTENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_DISK_EXTENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_DISK_EXTENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_DISK_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_AUDIO_CD: VDS_DISK_FLAG = VDS_DISK_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_HOTSPARE: VDS_DISK_FLAG = VDS_DISK_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_RESERVE_CAPABLE: VDS_DISK_FLAG = VDS_DISK_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_MASKED: VDS_DISK_FLAG = VDS_DISK_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_STYLE_CONVERTIBLE: VDS_DISK_FLAG = VDS_DISK_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_CLUSTERED: VDS_DISK_FLAG = VDS_DISK_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_READ_ONLY: VDS_DISK_FLAG = VDS_DISK_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_SYSTEM_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_BOOT_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(256i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_PAGEFILE_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(512i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_HIBERNATIONFILE_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(1024i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_CRASHDUMP_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(2048i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_HAS_ARC_PATH: VDS_DISK_FLAG = VDS_DISK_FLAG(4096i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_DYNAMIC: VDS_DISK_FLAG = VDS_DISK_FLAG(8192i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_BOOT_FROM_DISK: VDS_DISK_FLAG = VDS_DISK_FLAG(16384i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_CURRENT_READ_ONLY: VDS_DISK_FLAG = VDS_DISK_FLAG(32768i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DF_REFS_NOT_SUPPORTED: VDS_DISK_FLAG = VDS_DISK_FLAG(65536i32);
impl ::core::marker::Copy for VDS_DISK_FLAG {}
impl ::core::clone::Clone for VDS_DISK_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_DISK_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_DISK_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_DISK_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_DISK_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_DISK_OFFLINE_REASON(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonNone: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonPolicy: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonRedundantPath: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonSnapshot: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonCollision: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonResourceExhaustion: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonWriteFailure: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonDIScan: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSDiskOfflineReasonLostDataPersistence: VDS_DISK_OFFLINE_REASON = VDS_DISK_OFFLINE_REASON(8i32);
impl ::core::marker::Copy for VDS_DISK_OFFLINE_REASON {}
impl ::core::clone::Clone for VDS_DISK_OFFLINE_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_DISK_OFFLINE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_DISK_OFFLINE_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_DISK_OFFLINE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_DISK_OFFLINE_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_DISK_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_UNKNOWN: VDS_DISK_STATUS = VDS_DISK_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_ONLINE: VDS_DISK_STATUS = VDS_DISK_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_NOT_READY: VDS_DISK_STATUS = VDS_DISK_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_NO_MEDIA: VDS_DISK_STATUS = VDS_DISK_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_FAILED: VDS_DISK_STATUS = VDS_DISK_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_MISSING: VDS_DISK_STATUS = VDS_DISK_STATUS(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DS_OFFLINE: VDS_DISK_STATUS = VDS_DISK_STATUS(4i32);
impl ::core::marker::Copy for VDS_DISK_STATUS {}
impl ::core::clone::Clone for VDS_DISK_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_DISK_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_DISK_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_DISK_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_DISK_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_DRIVE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_HOTSPARE: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_ASSIGNED: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_UNASSIGNED: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_HOTSPARE_IN_USE: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRF_HOTSPARE_STANDBY: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(16i32);
impl ::core::marker::Copy for VDS_DRIVE_FLAG {}
impl ::core::clone::Clone for VDS_DRIVE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_DRIVE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_DRIVE_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_DRIVE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_DRIVE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_DRIVE_LETTER_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DLF_NON_PERSISTENT: VDS_DRIVE_LETTER_FLAG = VDS_DRIVE_LETTER_FLAG(1i32);
impl ::core::marker::Copy for VDS_DRIVE_LETTER_FLAG {}
impl ::core::clone::Clone for VDS_DRIVE_LETTER_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_DRIVE_LETTER_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_DRIVE_LETTER_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_DRIVE_LETTER_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_DRIVE_LETTER_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_DRIVE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_UNKNOWN: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_ONLINE: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_NOT_READY: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_OFFLINE: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_FAILED: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_DRS_REMOVED: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(8i32);
impl ::core::marker::Copy for VDS_DRIVE_STATUS {}
impl ::core::clone::Clone for VDS_DRIVE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_DRIVE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_DRIVE_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_DRIVE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_DRIVE_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_FILE_SYSTEM_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_FORMAT: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_QUICK_FORMAT: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_COMPRESS: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_SPECIFY_LABEL: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_MOUNT_POINT: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_REMOVABLE_MEDIA: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_SUPPORT_EXTEND: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_512: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(65536i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_1K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(131072i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_2K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(262144i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_4K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(524288i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_8K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(1048576i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_16K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(2097152i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_32K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(4194304i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_64K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(8388608i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_128K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(16777216i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSF_ALLOCATION_UNIT_256K: VDS_FILE_SYSTEM_FLAG = VDS_FILE_SYSTEM_FLAG(33554432i32);
impl ::core::marker::Copy for VDS_FILE_SYSTEM_FLAG {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_FILE_SYSTEM_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_FILE_SYSTEM_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_FILE_SYSTEM_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_FILE_SYSTEM_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSS_DEFAULT: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSS_PREVIOUS_REVISION: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSS_RECOMMENDED: VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG = VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG(4i32);
impl ::core::marker::Copy for VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_FILE_SYSTEM_FORMAT_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_FILE_SYSTEM_PROP_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FPF_COMPRESSED: VDS_FILE_SYSTEM_PROP_FLAG = VDS_FILE_SYSTEM_PROP_FLAG(1i32);
impl ::core::marker::Copy for VDS_FILE_SYSTEM_PROP_FLAG {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_PROP_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_FILE_SYSTEM_PROP_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_FILE_SYSTEM_PROP_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_FILE_SYSTEM_PROP_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_FILE_SYSTEM_PROP_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_FILE_SYSTEM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_UNKNOWN: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_RAW: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_FAT: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_FAT32: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_NTFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_CDFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_UDF: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_EXFAT: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_CSVFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FST_REFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(9i32);
impl ::core::marker::Copy for VDS_FILE_SYSTEM_TYPE {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_FILE_SYSTEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_FILE_SYSTEM_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_FILE_SYSTEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_FILE_SYSTEM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_FORMAT_OPTION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_NONE: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_FORCE: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_QUICK: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_COMPRESSION: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_FSOF_DUPLICATE_METADATA: VDS_FORMAT_OPTION_FLAGS = VDS_FORMAT_OPTION_FLAGS(8i32);
impl ::core::marker::Copy for VDS_FORMAT_OPTION_FLAGS {}
impl ::core::clone::Clone for VDS_FORMAT_OPTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_FORMAT_OPTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_FORMAT_OPTION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_FORMAT_OPTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_FORMAT_OPTION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_HBAPORT_SPEED_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_UNKNOWN: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_1GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_2GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_10GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_4GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HSF_NOT_NEGOTIATED: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(32768i32);
impl ::core::marker::Copy for VDS_HBAPORT_SPEED_FLAG {}
impl ::core::clone::Clone for VDS_HBAPORT_SPEED_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_HBAPORT_SPEED_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_HBAPORT_SPEED_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_HBAPORT_SPEED_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_HBAPORT_SPEED_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_HBAPORT_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_UNKNOWN: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_ONLINE: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_OFFLINE: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_BYPASSED: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_DIAGNOSTICS: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_LINKDOWN: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_ERROR: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPS_LOOPBACK: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(8i32);
impl ::core::marker::Copy for VDS_HBAPORT_STATUS {}
impl ::core::clone::Clone for VDS_HBAPORT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_HBAPORT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_HBAPORT_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_HBAPORT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_HBAPORT_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_HBAPORT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_UNKNOWN: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_OTHER: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_NOTPRESENT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_NPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_NLPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_FLPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_FPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_EPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_GPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_LPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HPT_PTP: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(21i32);
impl ::core::marker::Copy for VDS_HBAPORT_TYPE {}
impl ::core::clone::Clone for VDS_HBAPORT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_HBAPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_HBAPORT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_HBAPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_HBAPORT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_HEALTH(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_UNKNOWN: VDS_HEALTH = VDS_HEALTH(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_HEALTHY: VDS_HEALTH = VDS_HEALTH(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_REBUILDING: VDS_HEALTH = VDS_HEALTH(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_STALE: VDS_HEALTH = VDS_HEALTH(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILING: VDS_HEALTH = VDS_HEALTH(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILING_REDUNDANCY: VDS_HEALTH = VDS_HEALTH(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILED_REDUNDANCY: VDS_HEALTH = VDS_HEALTH(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILED_REDUNDANCY_FAILING: VDS_HEALTH = VDS_HEALTH(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_FAILED: VDS_HEALTH = VDS_HEALTH(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_REPLACED: VDS_HEALTH = VDS_HEALTH(9i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_PENDING_FAILURE: VDS_HEALTH = VDS_HEALTH(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_H_DEGRADED: VDS_HEALTH = VDS_HEALTH(11i32);
impl ::core::marker::Copy for VDS_HEALTH {}
impl ::core::clone::Clone for VDS_HEALTH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_HEALTH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_HEALTH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_HEALTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_HEALTH").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_HWPROVIDER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_UNKNOWN: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_PCI_RAID: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_FIBRE_CHANNEL: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_ISCSI: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_SAS: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_HWT_HYBRID: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(5i32);
impl ::core::marker::Copy for VDS_HWPROVIDER_TYPE {}
impl ::core::clone::Clone for VDS_HWPROVIDER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_HWPROVIDER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_HWPROVIDER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_HWPROVIDER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_HWPROVIDER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_INTERCONNECT_ADDRESS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_UNKNOWN: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_FCFS: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_FCPH: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_FCPH3: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_MAC: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IA_SCSI: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(5i32);
impl ::core::marker::Copy for VDS_INTERCONNECT_ADDRESS_TYPE {}
impl ::core::clone::Clone for VDS_INTERCONNECT_ADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_INTERCONNECT_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_INTERCONNECT_ADDRESS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_INTERCONNECT_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_INTERCONNECT_ADDRESS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_INTERCONNECT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ITF_PCI_RAID: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ITF_FIBRE_CHANNEL: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ITF_ISCSI: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ITF_SAS: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(8i32);
impl ::core::marker::Copy for VDS_INTERCONNECT_FLAG {}
impl ::core::clone::Clone for VDS_INTERCONNECT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_INTERCONNECT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_INTERCONNECT_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_INTERCONNECT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_INTERCONNECT_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_IPADDRESS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPT_TEXT: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPT_IPV4: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPT_IPV6: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPT_EMPTY: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(3i32);
impl ::core::marker::Copy for VDS_IPADDRESS_TYPE {}
impl ::core::clone::Clone for VDS_IPADDRESS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_IPADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_IPADDRESS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_IPADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_IPADDRESS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_ISCSI_AUTH_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IAT_NONE: VDS_ISCSI_AUTH_TYPE = VDS_ISCSI_AUTH_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IAT_CHAP: VDS_ISCSI_AUTH_TYPE = VDS_ISCSI_AUTH_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IAT_MUTUAL_CHAP: VDS_ISCSI_AUTH_TYPE = VDS_ISCSI_AUTH_TYPE(2i32);
impl ::core::marker::Copy for VDS_ISCSI_AUTH_TYPE {}
impl ::core::clone::Clone for VDS_ISCSI_AUTH_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_ISCSI_AUTH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_AUTH_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_ISCSI_AUTH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_AUTH_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_ISCSI_IPSEC_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_VALID: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_IKE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_MAIN_MODE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_AGGRESSIVE_MODE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_PFS_ENABLE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_TRANSPORT_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IIF_TUNNEL_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(64i32);
impl ::core::marker::Copy for VDS_ISCSI_IPSEC_FLAG {}
impl ::core::clone::Clone for VDS_ISCSI_IPSEC_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_ISCSI_IPSEC_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_IPSEC_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_ISCSI_IPSEC_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_IPSEC_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_ISCSI_LOGIN_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILF_REQUIRE_IPSEC: VDS_ISCSI_LOGIN_FLAG = VDS_ISCSI_LOGIN_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILF_MULTIPATH_ENABLED: VDS_ISCSI_LOGIN_FLAG = VDS_ISCSI_LOGIN_FLAG(2i32);
impl ::core::marker::Copy for VDS_ISCSI_LOGIN_FLAG {}
impl ::core::clone::Clone for VDS_ISCSI_LOGIN_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_ISCSI_LOGIN_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_LOGIN_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_ISCSI_LOGIN_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_LOGIN_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_ISCSI_LOGIN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILT_MANUAL: VDS_ISCSI_LOGIN_TYPE = VDS_ISCSI_LOGIN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILT_PERSISTENT: VDS_ISCSI_LOGIN_TYPE = VDS_ISCSI_LOGIN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_ILT_BOOT: VDS_ISCSI_LOGIN_TYPE = VDS_ISCSI_LOGIN_TYPE(2i32);
impl ::core::marker::Copy for VDS_ISCSI_LOGIN_TYPE {}
impl ::core::clone::Clone for VDS_ISCSI_LOGIN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_ISCSI_LOGIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_LOGIN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_ISCSI_LOGIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_LOGIN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_ISCSI_PORTAL_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_UNKNOWN: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_ONLINE: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_NOT_READY: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_OFFLINE: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_IPS_FAILED: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(5i32);
impl ::core::marker::Copy for VDS_ISCSI_PORTAL_STATUS {}
impl ::core::clone::Clone for VDS_ISCSI_PORTAL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_ISCSI_PORTAL_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_PORTAL_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_ISCSI_PORTAL_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_ISCSI_PORTAL_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_LOADBALANCE_POLICY_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_UNKNOWN: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_FAILOVER: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_ROUND_ROBIN: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_ROUND_ROBIN_WITH_SUBSET: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_DYN_LEAST_QUEUE_DEPTH: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_WEIGHTED_PATHS: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_LEAST_BLOCKS: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBP_VENDOR_SPECIFIC: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(7i32);
impl ::core::marker::Copy for VDS_LOADBALANCE_POLICY_ENUM {}
impl ::core::clone::Clone for VDS_LOADBALANCE_POLICY_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_LOADBALANCE_POLICY_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_LOADBALANCE_POLICY_ENUM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_LOADBALANCE_POLICY_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_LOADBALANCE_POLICY_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_LUN_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_LBN_REMAP_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_READ_BACK_VERIFY_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_WRITE_THROUGH_CACHING_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_HARDWARE_CHECKSUM_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_READ_CACHE_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_WRITE_CACHE_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_MEDIA_SCAN_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_CONSISTENCY_CHECK_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LF_SNAPSHOT: VDS_LUN_FLAG = VDS_LUN_FLAG(256i32);
impl ::core::marker::Copy for VDS_LUN_FLAG {}
impl ::core::clone::Clone for VDS_LUN_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_LUN_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_LUN_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_LUN_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_LUN_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_LUN_PLEX_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPF_LBN_REMAP_ENABLED: VDS_LUN_PLEX_FLAG = VDS_LUN_PLEX_FLAG(1i32);
impl ::core::marker::Copy for VDS_LUN_PLEX_FLAG {}
impl ::core::clone::Clone for VDS_LUN_PLEX_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_LUN_PLEX_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_LUN_PLEX_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_LUN_PLEX_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_LUN_PLEX_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_LUN_PLEX_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_UNKNOWN: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_ONLINE: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_NOT_READY: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_OFFLINE: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPS_FAILED: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(5i32);
impl ::core::marker::Copy for VDS_LUN_PLEX_STATUS {}
impl ::core::clone::Clone for VDS_LUN_PLEX_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_LUN_PLEX_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_LUN_PLEX_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_LUN_PLEX_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_LUN_PLEX_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_LUN_PLEX_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_UNKNOWN: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_SIMPLE: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_SPAN: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_STRIPE: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_PARITY: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID2: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID3: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID4: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID5: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID6: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID03: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID05: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID10: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID15: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID30: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID50: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID53: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LPT_RAID60: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(29i32);
impl ::core::marker::Copy for VDS_LUN_PLEX_TYPE {}
impl ::core::clone::Clone for VDS_LUN_PLEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_LUN_PLEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_LUN_PLEX_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_LUN_PLEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_LUN_PLEX_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_LUN_RESERVE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_NONE: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_EXCLUSIVE_RW: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_EXCLUSIVE_RO: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_SHARED_RO: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LRM_SHARED_RW: VDS_LUN_RESERVE_MODE = VDS_LUN_RESERVE_MODE(4i32);
impl ::core::marker::Copy for VDS_LUN_RESERVE_MODE {}
impl ::core::clone::Clone for VDS_LUN_RESERVE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_LUN_RESERVE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_LUN_RESERVE_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_LUN_RESERVE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_LUN_RESERVE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_LUN_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_UNKNOWN: VDS_LUN_STATUS = VDS_LUN_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_ONLINE: VDS_LUN_STATUS = VDS_LUN_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_NOT_READY: VDS_LUN_STATUS = VDS_LUN_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_OFFLINE: VDS_LUN_STATUS = VDS_LUN_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LS_FAILED: VDS_LUN_STATUS = VDS_LUN_STATUS(5i32);
impl ::core::marker::Copy for VDS_LUN_STATUS {}
impl ::core::clone::Clone for VDS_LUN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_LUN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_LUN_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_LUN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_LUN_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_LUN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_UNKNOWN: VDS_LUN_TYPE = VDS_LUN_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_DEFAULT: VDS_LUN_TYPE = VDS_LUN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_FAULT_TOLERANT: VDS_LUN_TYPE = VDS_LUN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_NON_FAULT_TOLERANT: VDS_LUN_TYPE = VDS_LUN_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_SIMPLE: VDS_LUN_TYPE = VDS_LUN_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_SPAN: VDS_LUN_TYPE = VDS_LUN_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_STRIPE: VDS_LUN_TYPE = VDS_LUN_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_MIRROR: VDS_LUN_TYPE = VDS_LUN_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_PARITY: VDS_LUN_TYPE = VDS_LUN_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID2: VDS_LUN_TYPE = VDS_LUN_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID3: VDS_LUN_TYPE = VDS_LUN_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID4: VDS_LUN_TYPE = VDS_LUN_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID5: VDS_LUN_TYPE = VDS_LUN_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID6: VDS_LUN_TYPE = VDS_LUN_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID01: VDS_LUN_TYPE = VDS_LUN_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID03: VDS_LUN_TYPE = VDS_LUN_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID05: VDS_LUN_TYPE = VDS_LUN_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID10: VDS_LUN_TYPE = VDS_LUN_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID15: VDS_LUN_TYPE = VDS_LUN_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID30: VDS_LUN_TYPE = VDS_LUN_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID50: VDS_LUN_TYPE = VDS_LUN_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID51: VDS_LUN_TYPE = VDS_LUN_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID53: VDS_LUN_TYPE = VDS_LUN_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID60: VDS_LUN_TYPE = VDS_LUN_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LT_RAID61: VDS_LUN_TYPE = VDS_LUN_TYPE(30i32);
impl ::core::marker::Copy for VDS_LUN_TYPE {}
impl ::core::clone::Clone for VDS_LUN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_LUN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_LUN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_LUN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_LUN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_MAINTENANCE_OPERATION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const BlinkLight: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const BeepAlarm: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const SpinDown: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const SpinUp: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const Ping: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(5i32);
impl ::core::marker::Copy for VDS_MAINTENANCE_OPERATION {}
impl ::core::clone::Clone for VDS_MAINTENANCE_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_MAINTENANCE_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_MAINTENANCE_OPERATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_MAINTENANCE_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_MAINTENANCE_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_NF_CONTROLLER(pub u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_CONTROLLER_ARRIVE: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(103u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_CONTROLLER_DEPART: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(104u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_CONTROLLER_MODIFY: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(350u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_CONTROLLER_REMOVED: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(351u32);
impl ::core::marker::Copy for VDS_NF_CONTROLLER {}
impl ::core::clone::Clone for VDS_NF_CONTROLLER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_NF_CONTROLLER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_NF_CONTROLLER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_NF_CONTROLLER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_NF_CONTROLLER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_NF_DISK(pub u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DISK_ARRIVE: VDS_NF_DISK = VDS_NF_DISK(8u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DISK_DEPART: VDS_NF_DISK = VDS_NF_DISK(9u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DISK_MODIFY: VDS_NF_DISK = VDS_NF_DISK(10u32);
impl ::core::marker::Copy for VDS_NF_DISK {}
impl ::core::clone::Clone for VDS_NF_DISK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_NF_DISK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_NF_DISK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_NF_DISK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_NF_DISK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_NF_DRIVE(pub u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_ARRIVE: VDS_NF_DRIVE = VDS_NF_DRIVE(105u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_DEPART: VDS_NF_DRIVE = VDS_NF_DRIVE(106u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_MODIFY: VDS_NF_DRIVE = VDS_NF_DRIVE(107u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_DRIVE_REMOVED: VDS_NF_DRIVE = VDS_NF_DRIVE(354u32);
impl ::core::marker::Copy for VDS_NF_DRIVE {}
impl ::core::clone::Clone for VDS_NF_DRIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_NF_DRIVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_NF_DRIVE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_NF_DRIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_NF_DRIVE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_NF_FILE_SYSTEM(pub u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_FILE_SYSTEM_MODIFY: VDS_NF_FILE_SYSTEM = VDS_NF_FILE_SYSTEM(203u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_FILE_SYSTEM_FORMAT_PROGRESS: VDS_NF_FILE_SYSTEM = VDS_NF_FILE_SYSTEM(204u32);
impl ::core::marker::Copy for VDS_NF_FILE_SYSTEM {}
impl ::core::clone::Clone for VDS_NF_FILE_SYSTEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_NF_FILE_SYSTEM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_NF_FILE_SYSTEM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_NF_FILE_SYSTEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_NF_FILE_SYSTEM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_NF_LUN(pub u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_LUN_ARRIVE: VDS_NF_LUN = VDS_NF_LUN(108u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_LUN_DEPART: VDS_NF_LUN = VDS_NF_LUN(109u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_LUN_MODIFY: VDS_NF_LUN = VDS_NF_LUN(110u32);
impl ::core::marker::Copy for VDS_NF_LUN {}
impl ::core::clone::Clone for VDS_NF_LUN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_NF_LUN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_NF_LUN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_NF_LUN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_NF_LUN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_NF_PACK(pub u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PACK_ARRIVE: VDS_NF_PACK = VDS_NF_PACK(1u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PACK_DEPART: VDS_NF_PACK = VDS_NF_PACK(2u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PACK_MODIFY: VDS_NF_PACK = VDS_NF_PACK(3u32);
impl ::core::marker::Copy for VDS_NF_PACK {}
impl ::core::clone::Clone for VDS_NF_PACK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_NF_PACK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_NF_PACK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_NF_PACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_NF_PACK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_NF_PORT(pub u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORT_ARRIVE: VDS_NF_PORT = VDS_NF_PORT(121u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORT_DEPART: VDS_NF_PORT = VDS_NF_PORT(122u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORT_MODIFY: VDS_NF_PORT = VDS_NF_PORT(352u32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NF_PORT_REMOVED: VDS_NF_PORT = VDS_NF_PORT(353u32);
impl ::core::marker::Copy for VDS_NF_PORT {}
impl ::core::clone::Clone for VDS_NF_PORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_NF_PORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_NF_PORT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_NF_PORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_NF_PORT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_NOTIFICATION_TARGET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_UNKNOWN: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PACK: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_VOLUME: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_DISK: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PARTITION: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(60i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_DRIVE_LETTER: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(61i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_FILE_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(62i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_MOUNT_POINT: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(63i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_SUB_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_CONTROLLER: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_DRIVE: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_LUN: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PORT: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PORTAL: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_TARGET: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_PORTAL_GROUP: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_NTT_SERVICE: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(200i32);
impl ::core::marker::Copy for VDS_NOTIFICATION_TARGET_TYPE {}
impl ::core::clone::Clone for VDS_NOTIFICATION_TARGET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_NOTIFICATION_TARGET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_NOTIFICATION_TARGET_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_NOTIFICATION_TARGET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_NOTIFICATION_TARGET_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_UNKNOWN: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PROVIDER: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PACK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_VOLUME: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_VOLUME_PLEX: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_DISK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_SUB_SYSTEM: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_CONTROLLER: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_DRIVE: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_LUN: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_LUN_PLEX: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PORT: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PORTAL: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_TARGET: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_PORTAL_GROUP: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_STORAGE_POOL: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(39i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_HBAPORT: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(90i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_INIT_ADAPTER: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(91i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_INIT_PORTAL: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(92i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_ASYNC: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(100i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_ENUM: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(101i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_VDISK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(200i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_OT_OPEN_VDISK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(201i32);
impl ::core::marker::Copy for VDS_OBJECT_TYPE {}
impl ::core::clone::Clone for VDS_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_OBJECT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_PACK_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_FOREIGN: VDS_PACK_FLAG = VDS_PACK_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_NOQUORUM: VDS_PACK_FLAG = VDS_PACK_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_POLICY: VDS_PACK_FLAG = VDS_PACK_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_CORRUPTED: VDS_PACK_FLAG = VDS_PACK_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PKF_ONLINE_ERROR: VDS_PACK_FLAG = VDS_PACK_FLAG(16i32);
impl ::core::marker::Copy for VDS_PACK_FLAG {}
impl ::core::clone::Clone for VDS_PACK_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_PACK_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_PACK_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_PACK_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_PACK_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_PACK_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PS_UNKNOWN: VDS_PACK_STATUS = VDS_PACK_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PS_ONLINE: VDS_PACK_STATUS = VDS_PACK_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PS_OFFLINE: VDS_PACK_STATUS = VDS_PACK_STATUS(4i32);
impl ::core::marker::Copy for VDS_PACK_STATUS {}
impl ::core::clone::Clone for VDS_PACK_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_PACK_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_PACK_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_PACK_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_PACK_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_PARTITION_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PTF_SYSTEM: VDS_PARTITION_FLAG = VDS_PARTITION_FLAG(1i32);
impl ::core::marker::Copy for VDS_PARTITION_FLAG {}
impl ::core::clone::Clone for VDS_PARTITION_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_PARTITION_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_PARTITION_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_PARTITION_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_PARTITION_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_PARTITION_STYLE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PST_UNKNOWN: VDS_PARTITION_STYLE = VDS_PARTITION_STYLE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PST_MBR: VDS_PARTITION_STYLE = VDS_PARTITION_STYLE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PST_GPT: VDS_PARTITION_STYLE = VDS_PARTITION_STYLE(2i32);
impl ::core::marker::Copy for VDS_PARTITION_STYLE {}
impl ::core::clone::Clone for VDS_PARTITION_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_PARTITION_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_PARTITION_STYLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_PARTITION_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_PARTITION_STYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_PATH_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_MPS_UNKNOWN: VDS_PATH_STATUS = VDS_PATH_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_MPS_ONLINE: VDS_PATH_STATUS = VDS_PATH_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_MPS_FAILED: VDS_PATH_STATUS = VDS_PATH_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_MPS_STANDBY: VDS_PATH_STATUS = VDS_PATH_STATUS(7i32);
impl ::core::marker::Copy for VDS_PATH_STATUS {}
impl ::core::clone::Clone for VDS_PATH_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_PATH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_PATH_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_PATH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_PATH_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_PORT_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_UNKNOWN: VDS_PORT_STATUS = VDS_PORT_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_ONLINE: VDS_PORT_STATUS = VDS_PORT_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_NOT_READY: VDS_PORT_STATUS = VDS_PORT_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_OFFLINE: VDS_PORT_STATUS = VDS_PORT_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_FAILED: VDS_PORT_STATUS = VDS_PORT_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PRS_REMOVED: VDS_PORT_STATUS = VDS_PORT_STATUS(8i32);
impl ::core::marker::Copy for VDS_PORT_STATUS {}
impl ::core::clone::Clone for VDS_PORT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_PORT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_PORT_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_PORT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_PORT_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_PROVIDER_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_DYNAMIC: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_INTERNAL_HARDWARE_PROVIDER: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_ONE_DISK_ONLY_PER_PACK: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_ONE_PACK_ONLINE_ONLY: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_VOLUME_SPACE_MUST_BE_CONTIGUOUS: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_DYNAMIC: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(-2147483648i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_FAULT_TOLERANT: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(1073741824i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_DYNAMIC_1394: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(536870912i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_MIRROR: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PF_SUPPORT_RAID5: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(64i32);
impl ::core::marker::Copy for VDS_PROVIDER_FLAG {}
impl ::core::clone::Clone for VDS_PROVIDER_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_PROVIDER_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_PROVIDER_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_PROVIDER_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_PROVIDER_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_PROVIDER_LBSUPPORT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_FAILOVER: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_ROUND_ROBIN: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_ROUND_ROBIN_WITH_SUBSET: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_DYN_LEAST_QUEUE_DEPTH: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_WEIGHTED_PATHS: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_LEAST_BLOCKS: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_LBF_VENDOR_SPECIFIC: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(64i32);
impl ::core::marker::Copy for VDS_PROVIDER_LBSUPPORT_FLAG {}
impl ::core::clone::Clone for VDS_PROVIDER_LBSUPPORT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_PROVIDER_LBSUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_PROVIDER_LBSUPPORT_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_PROVIDER_LBSUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_PROVIDER_LBSUPPORT_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_PROVIDER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_UNKNOWN: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_SOFTWARE: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_HARDWARE: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_VIRTUALDISK: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PT_MAX: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(4i32);
impl ::core::marker::Copy for VDS_PROVIDER_TYPE {}
impl ::core::clone::Clone for VDS_PROVIDER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_PROVIDER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_PROVIDER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_PROVIDER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_PROVIDER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_QUERY_PROVIDER_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_QUERY_SOFTWARE_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = VDS_QUERY_PROVIDER_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_QUERY_HARDWARE_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = VDS_QUERY_PROVIDER_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_QUERY_VIRTUALDISK_PROVIDERS: VDS_QUERY_PROVIDER_FLAG = VDS_QUERY_PROVIDER_FLAG(4i32);
impl ::core::marker::Copy for VDS_QUERY_PROVIDER_FLAG {}
impl ::core::clone::Clone for VDS_QUERY_PROVIDER_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_QUERY_PROVIDER_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_QUERY_PROVIDER_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_QUERY_PROVIDER_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_QUERY_PROVIDER_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_RAID_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_UNKNOWN: VDS_RAID_TYPE = VDS_RAID_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID0: VDS_RAID_TYPE = VDS_RAID_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID1: VDS_RAID_TYPE = VDS_RAID_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID2: VDS_RAID_TYPE = VDS_RAID_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID3: VDS_RAID_TYPE = VDS_RAID_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID4: VDS_RAID_TYPE = VDS_RAID_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID5: VDS_RAID_TYPE = VDS_RAID_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID6: VDS_RAID_TYPE = VDS_RAID_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID01: VDS_RAID_TYPE = VDS_RAID_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID03: VDS_RAID_TYPE = VDS_RAID_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID05: VDS_RAID_TYPE = VDS_RAID_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID10: VDS_RAID_TYPE = VDS_RAID_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID15: VDS_RAID_TYPE = VDS_RAID_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID30: VDS_RAID_TYPE = VDS_RAID_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID50: VDS_RAID_TYPE = VDS_RAID_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID51: VDS_RAID_TYPE = VDS_RAID_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID53: VDS_RAID_TYPE = VDS_RAID_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID60: VDS_RAID_TYPE = VDS_RAID_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RT_RAID61: VDS_RAID_TYPE = VDS_RAID_TYPE(27i32);
impl ::core::marker::Copy for VDS_RAID_TYPE {}
impl ::core::clone::Clone for VDS_RAID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_RAID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_RAID_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_RAID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_RAID_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_RECOVER_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RA_UNKNOWN: VDS_RECOVER_ACTION = VDS_RECOVER_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RA_REFRESH: VDS_RECOVER_ACTION = VDS_RECOVER_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_RA_RESTART: VDS_RECOVER_ACTION = VDS_RECOVER_ACTION(2i32);
impl ::core::marker::Copy for VDS_RECOVER_ACTION {}
impl ::core::clone::Clone for VDS_RECOVER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_RECOVER_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_RECOVER_ACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_RECOVER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_RECOVER_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_SAN_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_UNKNOWN: VDS_SAN_POLICY = VDS_SAN_POLICY(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_ONLINE: VDS_SAN_POLICY = VDS_SAN_POLICY(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_OFFLINE_SHARED: VDS_SAN_POLICY = VDS_SAN_POLICY(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_OFFLINE: VDS_SAN_POLICY = VDS_SAN_POLICY(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_OFFLINE_INTERNAL: VDS_SAN_POLICY = VDS_SAN_POLICY(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SP_MAX: VDS_SAN_POLICY = VDS_SAN_POLICY(5i32);
impl ::core::marker::Copy for VDS_SAN_POLICY {}
impl ::core::clone::Clone for VDS_SAN_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_SAN_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_SAN_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_SAN_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_SAN_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_SERVICE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_DYNAMIC: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_FAULT_TOLERANT: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_GPT: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_DYNAMIC_1394: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_CLUSTER_SERVICE_CONFIGURED: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_AUTO_MOUNT_OFF: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_OS_UNINSTALL_VALID: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_EFI: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_MIRROR: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(256i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_RAID5: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(512i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SVF_SUPPORT_REFS: VDS_SERVICE_FLAG = VDS_SERVICE_FLAG(1024i32);
impl ::core::marker::Copy for VDS_SERVICE_FLAG {}
impl ::core::clone::Clone for VDS_SERVICE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_SERVICE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_SERVICE_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_SERVICE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_SERVICE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_STORAGE_BUS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeUnknown: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeScsi: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeAtapi: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeAta: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusType1394: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSsa: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeFibre: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeUsb: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeRAID: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeiScsi: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSas: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSata: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSd: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeMmc: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeMax: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeVirtual: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeFileBackedVirtual: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeSpaces: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeNVMe: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeScm: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeUfs: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSBusTypeMaxReserved: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(127i32);
impl ::core::marker::Copy for VDS_STORAGE_BUS_TYPE {}
impl ::core::clone::Clone for VDS_STORAGE_BUS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_STORAGE_BUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_STORAGE_BUS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_STORAGE_BUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_BUS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_STORAGE_IDENTIFIER_CODE_SET(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdCodeSetReserved: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdCodeSetBinary: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdCodeSetAscii: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdCodeSetUtf8: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(3i32);
impl ::core::marker::Copy for VDS_STORAGE_IDENTIFIER_CODE_SET {}
impl ::core::clone::Clone for VDS_STORAGE_IDENTIFIER_CODE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_STORAGE_IDENTIFIER_CODE_SET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_STORAGE_IDENTIFIER_CODE_SET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_STORAGE_IDENTIFIER_CODE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_IDENTIFIER_CODE_SET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_STORAGE_IDENTIFIER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeVendorSpecific: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeVendorId: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeEUI64: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeFCPHName: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypePortRelative: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeTargetPortGroup: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeLogicalUnitGroup: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeMD5LogicalUnitIdentifier: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDSStorageIdTypeScsiNameString: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(8i32);
impl ::core::marker::Copy for VDS_STORAGE_IDENTIFIER_TYPE {}
impl ::core::clone::Clone for VDS_STORAGE_IDENTIFIER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_STORAGE_IDENTIFIER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_STORAGE_IDENTIFIER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_STORAGE_IDENTIFIER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_IDENTIFIER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_STORAGE_POOL_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPS_UNKNOWN: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPS_ONLINE: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPS_NOT_READY: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPS_OFFLINE: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(4i32);
impl ::core::marker::Copy for VDS_STORAGE_POOL_STATUS {}
impl ::core::clone::Clone for VDS_STORAGE_POOL_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_STORAGE_POOL_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_STORAGE_POOL_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_STORAGE_POOL_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_POOL_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_STORAGE_POOL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPT_UNKNOWN: VDS_STORAGE_POOL_TYPE = VDS_STORAGE_POOL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPT_PRIMORDIAL: VDS_STORAGE_POOL_TYPE = VDS_STORAGE_POOL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SPT_CONCRETE: VDS_STORAGE_POOL_TYPE = VDS_STORAGE_POOL_TYPE(2i32);
impl ::core::marker::Copy for VDS_STORAGE_POOL_TYPE {}
impl ::core::clone::Clone for VDS_STORAGE_POOL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_STORAGE_POOL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_STORAGE_POOL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_STORAGE_POOL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_STORAGE_POOL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_SUB_SYSTEM_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_LUN_MASKING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_LUN_PLEXING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_LUN_REMAPPING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_DRIVE_EXTENT_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_HARDWARE_CHECKSUM_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_RADIUS_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_READ_BACK_VERIFY_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_WRITE_THROUGH_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(512i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_NON_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(1024i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_SIMPLE_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(2048i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_SPAN_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(4096i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_STRIPE_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(8192i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_MIRROR_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(16384i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_PARITY_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(32768i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_AUTH_CHAP: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(65536i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_AUTH_MUTUAL_CHAP: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(131072i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_SIMPLE_TARGET_CONFIG: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(262144i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_LUN_NUMBER: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(524288i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_MIRRORED_CACHE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(1048576i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_READ_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(2097152i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_WRITE_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(4194304i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_MEDIA_SCAN_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(8388608i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_CONSISTENCY_CHECK_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(16777216i32);
impl ::core::marker::Copy for VDS_SUB_SYSTEM_FLAG {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_SUB_SYSTEM_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_SUB_SYSTEM_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_SUB_SYSTEM_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_SUB_SYSTEM_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_SUB_SYSTEM_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_UNKNOWN: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_ONLINE: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_NOT_READY: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_OFFLINE: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_FAILED: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SSS_PARTIALLY_MANAGED: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(9i32);
impl ::core::marker::Copy for VDS_SUB_SYSTEM_STATUS {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_SUB_SYSTEM_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_SUB_SYSTEM_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_SUB_SYSTEM_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_SUB_SYSTEM_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID2_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID3_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID4_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID5_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID6_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID01_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID03_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID05_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID10_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(256i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID15_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(512i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID30_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(1024i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID50_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(2048i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID51_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(4096i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID53_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(8192i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID60_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(16384i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_SF_SUPPORTS_RAID61_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(32768i32);
impl ::core::marker::Copy for VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_TRANSITION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_UNKNOWN: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_STABLE: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_EXTENDING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_SHRINKING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_RECONFIGING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_TS_RESTRIPING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(5i32);
impl ::core::marker::Copy for VDS_TRANSITION_STATE {}
impl ::core::clone::Clone for VDS_TRANSITION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_TRANSITION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_TRANSITION_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_TRANSITION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_TRANSITION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_VDISK_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_UNKNOWN: VDS_VDISK_STATE = VDS_VDISK_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_ADDED: VDS_VDISK_STATE = VDS_VDISK_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_OPEN: VDS_VDISK_STATE = VDS_VDISK_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_ATTACH_PENDING: VDS_VDISK_STATE = VDS_VDISK_STATE(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_ATTACHED_NOT_OPEN: VDS_VDISK_STATE = VDS_VDISK_STATE(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_ATTACHED: VDS_VDISK_STATE = VDS_VDISK_STATE(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_DETACH_PENDING: VDS_VDISK_STATE = VDS_VDISK_STATE(6i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_COMPACTING: VDS_VDISK_STATE = VDS_VDISK_STATE(7i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_MERGING: VDS_VDISK_STATE = VDS_VDISK_STATE(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_EXPANDING: VDS_VDISK_STATE = VDS_VDISK_STATE(9i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_DELETED: VDS_VDISK_STATE = VDS_VDISK_STATE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VST_MAX: VDS_VDISK_STATE = VDS_VDISK_STATE(11i32);
impl ::core::marker::Copy for VDS_VDISK_STATE {}
impl ::core::clone::Clone for VDS_VDISK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_VDISK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_VDISK_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_VDISK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_VDISK_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_VERSION_SUPPORT_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_1_0: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_1_1: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_2_0: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_2_1: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VSF_3_0: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(16i32);
impl ::core::marker::Copy for VDS_VERSION_SUPPORT_FLAG {}
impl ::core::clone::Clone for VDS_VERSION_SUPPORT_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_VERSION_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_VERSION_SUPPORT_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_VERSION_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_VERSION_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_VOLUME_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_SYSTEM_VOLUME: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_BOOT_VOLUME: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(2i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_ACTIVE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(4i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_READONLY: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(8i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_HIDDEN: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(16i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_CAN_EXTEND: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(32i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_CAN_SHRINK: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(64i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_PAGEFILE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(128i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_HIBERNATION: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(256i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_CRASHDUMP: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(512i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_INSTALLABLE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(1024i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_LBN_REMAP_ENABLED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(2048i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_FORMATTING: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(4096i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_NOT_FORMATTABLE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(8192i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_NTFS_NOT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(16384i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_FAT32_NOT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(32768i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_FAT_NOT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(65536i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_NO_DEFAULT_DRIVE_LETTER: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(131072i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_PERMANENTLY_DISMOUNTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(262144i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_PERMANENT_DISMOUNT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(524288i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_SHADOW_COPY: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(1048576i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_FVE_ENABLED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(2097152i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_DIRTY: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(4194304i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_REFS_NOT_SUPPORTED: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(8388608i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_BACKS_BOOT_VOLUME: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(16777216i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VF_BACKED_BY_WIM_IMAGE: VDS_VOLUME_FLAG = VDS_VOLUME_FLAG(33554432i32);
impl ::core::marker::Copy for VDS_VOLUME_FLAG {}
impl ::core::clone::Clone for VDS_VOLUME_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_VOLUME_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_VOLUME_FLAG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_VOLUME_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_VOLUME_PLEX_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPS_UNKNOWN: VDS_VOLUME_PLEX_STATUS = VDS_VOLUME_PLEX_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPS_ONLINE: VDS_VOLUME_PLEX_STATUS = VDS_VOLUME_PLEX_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPS_NO_MEDIA: VDS_VOLUME_PLEX_STATUS = VDS_VOLUME_PLEX_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPS_FAILED: VDS_VOLUME_PLEX_STATUS = VDS_VOLUME_PLEX_STATUS(5i32);
impl ::core::marker::Copy for VDS_VOLUME_PLEX_STATUS {}
impl ::core::clone::Clone for VDS_VOLUME_PLEX_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_VOLUME_PLEX_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_VOLUME_PLEX_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_VOLUME_PLEX_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_PLEX_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_VOLUME_PLEX_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_UNKNOWN: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_SIMPLE: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_SPAN: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_STRIPE: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VPT_PARITY: VDS_VOLUME_PLEX_TYPE = VDS_VOLUME_PLEX_TYPE(14i32);
impl ::core::marker::Copy for VDS_VOLUME_PLEX_TYPE {}
impl ::core::clone::Clone for VDS_VOLUME_PLEX_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_VOLUME_PLEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_VOLUME_PLEX_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_VOLUME_PLEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_PLEX_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_VOLUME_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_UNKNOWN: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_ONLINE: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_NO_MEDIA: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_FAILED: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VS_OFFLINE: VDS_VOLUME_STATUS = VDS_VOLUME_STATUS(4i32);
impl ::core::marker::Copy for VDS_VOLUME_STATUS {}
impl ::core::clone::Clone for VDS_VOLUME_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_VOLUME_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_VOLUME_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_VOLUME_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VDS_VOLUME_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_UNKNOWN: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_SIMPLE: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_SPAN: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_STRIPE: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_MIRROR: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_VT_PARITY: VDS_VOLUME_TYPE = VDS_VOLUME_TYPE(14i32);
impl ::core::marker::Copy for VDS_VOLUME_TYPE {}
impl ::core::clone::Clone for VDS_VOLUME_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VDS_VOLUME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for VDS_VOLUME_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for VDS_VOLUME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VDS_VOLUME_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct __VDS_PARTITION_STYLE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PARTITION_STYLE_MBR: __VDS_PARTITION_STYLE = __VDS_PARTITION_STYLE(0i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PARTITION_STYLE_GPT: __VDS_PARTITION_STYLE = __VDS_PARTITION_STYLE(1i32);
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub const VDS_PARTITION_STYLE_RAW: __VDS_PARTITION_STYLE = __VDS_PARTITION_STYLE(2i32);
impl ::core::marker::Copy for __VDS_PARTITION_STYLE {}
impl ::core::clone::Clone for __VDS_PARTITION_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for __VDS_PARTITION_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for __VDS_PARTITION_STYLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for __VDS_PARTITION_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("__VDS_PARTITION_STYLE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGE_ATTRIBUTES_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CHANGE_ATTRIBUTES_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGE_ATTRIBUTES_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGE_ATTRIBUTES_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CHANGE_ATTRIBUTES_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGE_ATTRIBUTES_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CHANGE_ATTRIBUTES_PARAMETERS_0 {
    pub MbrPartInfo: CHANGE_ATTRIBUTES_PARAMETERS_0_1,
    pub GptPartInfo: CHANGE_ATTRIBUTES_PARAMETERS_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGE_ATTRIBUTES_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGE_ATTRIBUTES_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CHANGE_ATTRIBUTES_PARAMETERS_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGE_ATTRIBUTES_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    pub attributes: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGE_ATTRIBUTES_PARAMETERS_0_0").field("attributes", &self.attributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.attributes == other.attributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGE_ATTRIBUTES_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    pub bootIndicator: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGE_ATTRIBUTES_PARAMETERS_0_1").field("bootIndicator", &self.bootIndicator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.bootIndicator == other.bootIndicator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGE_ATTRIBUTES_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CHANGE_PARTITION_TYPE_PARAMETERS_0,
}
impl ::core::marker::Copy for CHANGE_PARTITION_TYPE_PARAMETERS {}
impl ::core::clone::Clone for CHANGE_PARTITION_TYPE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CHANGE_PARTITION_TYPE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CHANGE_PARTITION_TYPE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    pub MbrPartInfo: CHANGE_PARTITION_TYPE_PARAMETERS_0_1,
    pub GptPartInfo: CHANGE_PARTITION_TYPE_PARAMETERS_0_0,
}
impl ::core::marker::Copy for CHANGE_PARTITION_TYPE_PARAMETERS_0 {}
impl ::core::clone::Clone for CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for CHANGE_PARTITION_TYPE_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    pub partitionType: ::windows::core::GUID,
}
impl ::core::marker::Copy for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {}
impl ::core::clone::Clone for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGE_PARTITION_TYPE_PARAMETERS_0_0").field("partitionType", &self.partitionType).finish()
    }
}
impl ::windows::core::TypeKind for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.partitionType == other.partitionType
    }
}
impl ::core::cmp::Eq for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {}
impl ::core::default::Default for CHANGE_PARTITION_TYPE_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    pub partitionType: u8,
}
impl ::core::marker::Copy for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {}
impl ::core::clone::Clone for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGE_PARTITION_TYPE_PARAMETERS_0_1").field("partitionType", &self.partitionType).finish()
    }
}
impl ::windows::core::TypeKind for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.partitionType == other.partitionType
    }
}
impl ::core::cmp::Eq for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {}
impl ::core::default::Default for CHANGE_PARTITION_TYPE_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_PARTITION_PARAMETERS {
    pub style: VDS_PARTITION_STYLE,
    pub Anonymous: CREATE_PARTITION_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_PARTITION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_PARTITION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CREATE_PARTITION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATE_PARTITION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CREATE_PARTITION_PARAMETERS_0 {
    pub MbrPartInfo: CREATE_PARTITION_PARAMETERS_0_1,
    pub GptPartInfo: CREATE_PARTITION_PARAMETERS_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_PARTITION_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_PARTITION_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CREATE_PARTITION_PARAMETERS_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATE_PARTITION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_PARTITION_PARAMETERS_0_0 {
    pub partitionType: ::windows::core::GUID,
    pub partitionId: ::windows::core::GUID,
    pub attributes: u64,
    pub name: [u16; 36],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_PARTITION_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_PARTITION_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREATE_PARTITION_PARAMETERS_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_PARTITION_PARAMETERS_0_0").field("partitionType", &self.partitionType).field("partitionId", &self.partitionId).field("attributes", &self.attributes).field("name", &self.name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CREATE_PARTITION_PARAMETERS_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREATE_PARTITION_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.partitionType == other.partitionType && self.partitionId == other.partitionId && self.attributes == other.attributes && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREATE_PARTITION_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATE_PARTITION_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_PARTITION_PARAMETERS_0_1 {
    pub partitionType: u8,
    pub bootIndicator: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_PARTITION_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_PARTITION_PARAMETERS_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREATE_PARTITION_PARAMETERS_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_PARTITION_PARAMETERS_0_1").field("partitionType", &self.partitionType).field("bootIndicator", &self.bootIndicator).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CREATE_PARTITION_PARAMETERS_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREATE_PARTITION_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.partitionType == other.partitionType && self.bootIndicator == other.bootIndicator
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREATE_PARTITION_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATE_PARTITION_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ADVANCEDDISK_PROP {
    pub pwszId: ::windows::core::PWSTR,
    pub pwszPathname: ::windows::core::PWSTR,
    pub pwszLocation: ::windows::core::PWSTR,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pswzIdentifier: ::windows::core::PWSTR,
    pub usIdentifierFormat: u16,
    pub ulNumber: u32,
    pub pwszSerialNumber: ::windows::core::PWSTR,
    pub pwszFirmwareVersion: ::windows::core::PWSTR,
    pub pwszManufacturer: ::windows::core::PWSTR,
    pub pwszModel: ::windows::core::PWSTR,
    pub ullTotalSize: u64,
    pub ullAllocatedSize: u64,
    pub ulLogicalSectorSize: u32,
    pub ulPhysicalSectorSize: u32,
    pub ulPartitionCount: u32,
    pub status: VDS_DISK_STATUS,
    pub health: VDS_HEALTH,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_ADVANCEDDISK_PROP_0,
    pub ulFlags: u32,
    pub dwDeviceType: u32,
}
impl ::core::marker::Copy for VDS_ADVANCEDDISK_PROP {}
impl ::core::clone::Clone for VDS_ADVANCEDDISK_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_ADVANCEDDISK_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_ADVANCEDDISK_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_ADVANCEDDISK_PROP_0 {
    pub dwSignature: u32,
    pub DiskGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_ADVANCEDDISK_PROP_0 {}
impl ::core::clone::Clone for VDS_ADVANCEDDISK_PROP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_ADVANCEDDISK_PROP_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_ADVANCEDDISK_PROP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT {
    pub r#type: VDS_ASYNC_OUTPUT_TYPE,
    pub Anonymous: VDS_ASYNC_OUTPUT_0,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_ASYNC_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_ASYNC_OUTPUT_0 {
    pub cp: VDS_ASYNC_OUTPUT_0_2,
    pub cv: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_5>,
    pub bvp: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_0>,
    pub sv: VDS_ASYNC_OUTPUT_0_7,
    pub cl: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_1>,
    pub ct: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_4>,
    pub cpg: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_3>,
    pub cvd: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_6>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_0 {
    pub pVolumeUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_0").field("pVolumeUnk", &self.pVolumeUnk).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pVolumeUnk == other.pVolumeUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_0 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_1 {
    pub pLunUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_1").field("pLunUnk", &self.pLunUnk).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pLunUnk == other.pLunUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_1 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_2 {
    pub ullOffset: u64,
    pub volumeId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_2 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_2").field("ullOffset", &self.ullOffset).field("volumeId", &self.volumeId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_0_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.ullOffset == other.ullOffset && self.volumeId == other.volumeId
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_2 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_3 {
    pub pPortalGroupUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_3 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_3").field("pPortalGroupUnk", &self.pPortalGroupUnk).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_0_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.pPortalGroupUnk == other.pPortalGroupUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_3 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_4 {
    pub pTargetUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_4 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_4").field("pTargetUnk", &self.pTargetUnk).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_0_4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.pTargetUnk == other.pTargetUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_4 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_5 {
    pub pVolumeUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_5 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_5").field("pVolumeUnk", &self.pVolumeUnk).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_0_5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.pVolumeUnk == other.pVolumeUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_5 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_6 {
    pub pVDiskUnk: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_6 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_6").field("pVDiskUnk", &self.pVDiskUnk).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_0_6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.pVDiskUnk == other.pVDiskUnk
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_6 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ASYNC_OUTPUT_0_7 {
    pub ullReclaimedBytes: u64,
}
impl ::core::marker::Copy for VDS_ASYNC_OUTPUT_0_7 {}
impl ::core::clone::Clone for VDS_ASYNC_OUTPUT_0_7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ASYNC_OUTPUT_0_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ASYNC_OUTPUT_0_7").field("ullReclaimedBytes", &self.ullReclaimedBytes).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ASYNC_OUTPUT_0_7 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.ullReclaimedBytes == other.ullReclaimedBytes
    }
}
impl ::core::cmp::Eq for VDS_ASYNC_OUTPUT_0_7 {}
impl ::core::default::Default for VDS_ASYNC_OUTPUT_0_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_CONTROLLER_NOTIFICATION {
    pub ulEvent: VDS_NF_CONTROLLER,
    pub controllerId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_CONTROLLER_NOTIFICATION {}
impl ::core::clone::Clone for VDS_CONTROLLER_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_CONTROLLER_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_CONTROLLER_NOTIFICATION").field("ulEvent", &self.ulEvent).field("controllerId", &self.controllerId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_CONTROLLER_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_CONTROLLER_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.controllerId == other.controllerId
    }
}
impl ::core::cmp::Eq for VDS_CONTROLLER_NOTIFICATION {}
impl ::core::default::Default for VDS_CONTROLLER_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_CONTROLLER_PROP {
    pub id: ::windows::core::GUID,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pwszIdentification: ::windows::core::PWSTR,
    pub status: VDS_CONTROLLER_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfPorts: i16,
}
impl ::core::marker::Copy for VDS_CONTROLLER_PROP {}
impl ::core::clone::Clone for VDS_CONTROLLER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_CONTROLLER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_CONTROLLER_PROP").field("id", &self.id).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("status", &self.status).field("health", &self.health).field("sNumberOfPorts", &self.sNumberOfPorts).finish()
    }
}
impl ::windows::core::TypeKind for VDS_CONTROLLER_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_CONTROLLER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.status == other.status && self.health == other.health && self.sNumberOfPorts == other.sNumberOfPorts
    }
}
impl ::core::cmp::Eq for VDS_CONTROLLER_PROP {}
impl ::core::default::Default for VDS_CONTROLLER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_CREATE_VDISK_PARAMETERS {
    pub UniqueId: ::windows::core::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub pParentPath: ::windows::core::PWSTR,
    pub pSourcePath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VDS_CREATE_VDISK_PARAMETERS {}
impl ::core::clone::Clone for VDS_CREATE_VDISK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_CREATE_VDISK_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_CREATE_VDISK_PARAMETERS").field("UniqueId", &self.UniqueId).field("MaximumSize", &self.MaximumSize).field("BlockSizeInBytes", &self.BlockSizeInBytes).field("SectorSizeInBytes", &self.SectorSizeInBytes).field("pParentPath", &self.pParentPath).field("pSourcePath", &self.pSourcePath).finish()
    }
}
impl ::windows::core::TypeKind for VDS_CREATE_VDISK_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_CREATE_VDISK_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.UniqueId == other.UniqueId && self.MaximumSize == other.MaximumSize && self.BlockSizeInBytes == other.BlockSizeInBytes && self.SectorSizeInBytes == other.SectorSizeInBytes && self.pParentPath == other.pParentPath && self.pSourcePath == other.pSourcePath
    }
}
impl ::core::cmp::Eq for VDS_CREATE_VDISK_PARAMETERS {}
impl ::core::default::Default for VDS_CREATE_VDISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_EXTENT {
    pub diskId: ::windows::core::GUID,
    pub r#type: VDS_DISK_EXTENT_TYPE,
    pub ullOffset: u64,
    pub ullSize: u64,
    pub volumeId: ::windows::core::GUID,
    pub plexId: ::windows::core::GUID,
    pub memberIdx: u32,
}
impl ::core::marker::Copy for VDS_DISK_EXTENT {}
impl ::core::clone::Clone for VDS_DISK_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DISK_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DISK_EXTENT").field("diskId", &self.diskId).field("type", &self.r#type).field("ullOffset", &self.ullOffset).field("ullSize", &self.ullSize).field("volumeId", &self.volumeId).field("plexId", &self.plexId).field("memberIdx", &self.memberIdx).finish()
    }
}
impl ::windows::core::TypeKind for VDS_DISK_EXTENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_DISK_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.diskId == other.diskId && self.r#type == other.r#type && self.ullOffset == other.ullOffset && self.ullSize == other.ullSize && self.volumeId == other.volumeId && self.plexId == other.plexId && self.memberIdx == other.memberIdx
    }
}
impl ::core::cmp::Eq for VDS_DISK_EXTENT {}
impl ::core::default::Default for VDS_DISK_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_FREE_EXTENT {
    pub diskId: ::windows::core::GUID,
    pub ullOffset: u64,
    pub ullSize: u64,
}
impl ::core::marker::Copy for VDS_DISK_FREE_EXTENT {}
impl ::core::clone::Clone for VDS_DISK_FREE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DISK_FREE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DISK_FREE_EXTENT").field("diskId", &self.diskId).field("ullOffset", &self.ullOffset).field("ullSize", &self.ullSize).finish()
    }
}
impl ::windows::core::TypeKind for VDS_DISK_FREE_EXTENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_DISK_FREE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.diskId == other.diskId && self.ullOffset == other.ullOffset && self.ullSize == other.ullSize
    }
}
impl ::core::cmp::Eq for VDS_DISK_FREE_EXTENT {}
impl ::core::default::Default for VDS_DISK_FREE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_NOTIFICATION {
    pub ulEvent: VDS_NF_DISK,
    pub diskId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_DISK_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DISK_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DISK_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DISK_NOTIFICATION").field("ulEvent", &self.ulEvent).field("diskId", &self.diskId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_DISK_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_DISK_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.diskId == other.diskId
    }
}
impl ::core::cmp::Eq for VDS_DISK_NOTIFICATION {}
impl ::core::default::Default for VDS_DISK_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_PROP {
    pub id: ::windows::core::GUID,
    pub status: VDS_DISK_STATUS,
    pub ReserveMode: VDS_LUN_RESERVE_MODE,
    pub health: VDS_HEALTH,
    pub dwDeviceType: u32,
    pub dwMediaType: u32,
    pub ullSize: u64,
    pub ulBytesPerSector: u32,
    pub ulSectorsPerTrack: u32,
    pub ulTracksPerCylinder: u32,
    pub ulFlags: u32,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_DISK_PROP_0,
    pub pwszDiskAddress: ::windows::core::PWSTR,
    pub pwszName: ::windows::core::PWSTR,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pwszAdaptorName: ::windows::core::PWSTR,
    pub pwszDevicePath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VDS_DISK_PROP {}
impl ::core::clone::Clone for VDS_DISK_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_DISK_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_DISK_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_DISK_PROP_0 {
    pub dwSignature: u32,
    pub DiskGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_DISK_PROP_0 {}
impl ::core::clone::Clone for VDS_DISK_PROP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_DISK_PROP_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_DISK_PROP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DISK_PROP2 {
    pub id: ::windows::core::GUID,
    pub status: VDS_DISK_STATUS,
    pub OfflineReason: VDS_DISK_OFFLINE_REASON,
    pub ReserveMode: VDS_LUN_RESERVE_MODE,
    pub health: VDS_HEALTH,
    pub dwDeviceType: u32,
    pub dwMediaType: u32,
    pub ullSize: u64,
    pub ulBytesPerSector: u32,
    pub ulSectorsPerTrack: u32,
    pub ulTracksPerCylinder: u32,
    pub ulFlags: u32,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub Anonymous: VDS_DISK_PROP2_0,
    pub pwszDiskAddress: ::windows::core::PWSTR,
    pub pwszName: ::windows::core::PWSTR,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pwszAdaptorName: ::windows::core::PWSTR,
    pub pwszDevicePath: ::windows::core::PWSTR,
    pub pwszLocationPath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VDS_DISK_PROP2 {}
impl ::core::clone::Clone for VDS_DISK_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_DISK_PROP2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_DISK_PROP2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_DISK_PROP2_0 {
    pub dwSignature: u32,
    pub DiskGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_DISK_PROP2_0 {}
impl ::core::clone::Clone for VDS_DISK_PROP2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_DISK_PROP2_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_DISK_PROP2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_EXTENT {
    pub id: ::windows::core::GUID,
    pub LunId: ::windows::core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_DRIVE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_EXTENT").field("id", &self.id).field("LunId", &self.LunId).field("ullSize", &self.ullSize).field("bUsed", &self.bUsed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_DRIVE_EXTENT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_DRIVE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.LunId == other.LunId && self.ullSize == other.ullSize && self.bUsed == other.bUsed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_DRIVE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DRIVE_LETTER_NOTIFICATION {
    pub ulEvent: u32,
    pub wcLetter: u16,
    pub volumeId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_DRIVE_LETTER_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DRIVE_LETTER_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DRIVE_LETTER_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_LETTER_NOTIFICATION").field("ulEvent", &self.ulEvent).field("wcLetter", &self.wcLetter).field("volumeId", &self.volumeId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_DRIVE_LETTER_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_DRIVE_LETTER_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.wcLetter == other.wcLetter && self.volumeId == other.volumeId
    }
}
impl ::core::cmp::Eq for VDS_DRIVE_LETTER_NOTIFICATION {}
impl ::core::default::Default for VDS_DRIVE_LETTER_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_LETTER_PROP {
    pub wcLetter: u16,
    pub volumeId: ::windows::core::GUID,
    pub ulFlags: u32,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_DRIVE_LETTER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_DRIVE_LETTER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_DRIVE_LETTER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_LETTER_PROP").field("wcLetter", &self.wcLetter).field("volumeId", &self.volumeId).field("ulFlags", &self.ulFlags).field("bUsed", &self.bUsed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_DRIVE_LETTER_PROP {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_DRIVE_LETTER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.wcLetter == other.wcLetter && self.volumeId == other.volumeId && self.ulFlags == other.ulFlags && self.bUsed == other.bUsed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_DRIVE_LETTER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_DRIVE_LETTER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DRIVE_NOTIFICATION {
    pub ulEvent: VDS_NF_DRIVE,
    pub driveId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_DRIVE_NOTIFICATION {}
impl ::core::clone::Clone for VDS_DRIVE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DRIVE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_NOTIFICATION").field("ulEvent", &self.ulEvent).field("driveId", &self.driveId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_DRIVE_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_DRIVE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.driveId == other.driveId
    }
}
impl ::core::cmp::Eq for VDS_DRIVE_NOTIFICATION {}
impl ::core::default::Default for VDS_DRIVE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DRIVE_PROP {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pwszIdentification: ::windows::core::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
}
impl ::core::marker::Copy for VDS_DRIVE_PROP {}
impl ::core::clone::Clone for VDS_DRIVE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DRIVE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_PROP").field("id", &self.id).field("ullSize", &self.ullSize).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("ulFlags", &self.ulFlags).field("status", &self.status).field("health", &self.health).field("sInternalBusNumber", &self.sInternalBusNumber).field("sSlotNumber", &self.sSlotNumber).finish()
    }
}
impl ::windows::core::TypeKind for VDS_DRIVE_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_DRIVE_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.ulFlags == other.ulFlags && self.status == other.status && self.health == other.health && self.sInternalBusNumber == other.sInternalBusNumber && self.sSlotNumber == other.sSlotNumber
    }
}
impl ::core::cmp::Eq for VDS_DRIVE_PROP {}
impl ::core::default::Default for VDS_DRIVE_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_DRIVE_PROP2 {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pwszIdentification: ::windows::core::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
    pub ulEnclosureNumber: u32,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub ulSpindleSpeed: u32,
}
impl ::core::marker::Copy for VDS_DRIVE_PROP2 {}
impl ::core::clone::Clone for VDS_DRIVE_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_DRIVE_PROP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_DRIVE_PROP2")
            .field("id", &self.id)
            .field("ullSize", &self.ullSize)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sInternalBusNumber", &self.sInternalBusNumber)
            .field("sSlotNumber", &self.sSlotNumber)
            .field("ulEnclosureNumber", &self.ulEnclosureNumber)
            .field("busType", &self.busType)
            .field("ulSpindleSpeed", &self.ulSpindleSpeed)
            .finish()
    }
}
impl ::windows::core::TypeKind for VDS_DRIVE_PROP2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_DRIVE_PROP2 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.ulFlags == other.ulFlags && self.status == other.status && self.health == other.health && self.sInternalBusNumber == other.sInternalBusNumber && self.sSlotNumber == other.sSlotNumber && self.ulEnclosureNumber == other.ulEnclosureNumber && self.busType == other.busType && self.ulSpindleSpeed == other.ulSpindleSpeed
    }
}
impl ::core::cmp::Eq for VDS_DRIVE_PROP2 {}
impl ::core::default::Default for VDS_DRIVE_PROP2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    pub ulFlags: u32,
    pub usRevision: u16,
    pub ulDefaultUnitAllocationSize: u32,
    pub rgulAllowedUnitAllocationSizes: [u32; 32],
    pub wszName: [u16; 32],
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP").field("ulFlags", &self.ulFlags).field("usRevision", &self.usRevision).field("ulDefaultUnitAllocationSize", &self.ulDefaultUnitAllocationSize).field("rgulAllowedUnitAllocationSizes", &self.rgulAllowedUnitAllocationSizes).field("wszName", &self.wszName).finish()
    }
}
impl ::windows::core::TypeKind for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.usRevision == other.usRevision && self.ulDefaultUnitAllocationSize == other.ulDefaultUnitAllocationSize && self.rgulAllowedUnitAllocationSizes == other.rgulAllowedUnitAllocationSizes && self.wszName == other.wszName
    }
}
impl ::core::cmp::Eq for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {}
impl ::core::default::Default for VDS_FILE_SYSTEM_FORMAT_SUPPORT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_FILE_SYSTEM_NOTIFICATION {
    pub ulEvent: VDS_NF_FILE_SYSTEM,
    pub volumeId: ::windows::core::GUID,
    pub dwPercentCompleted: u32,
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_NOTIFICATION {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_FILE_SYSTEM_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_FILE_SYSTEM_NOTIFICATION").field("ulEvent", &self.ulEvent).field("volumeId", &self.volumeId).field("dwPercentCompleted", &self.dwPercentCompleted).finish()
    }
}
impl ::windows::core::TypeKind for VDS_FILE_SYSTEM_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_FILE_SYSTEM_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.volumeId == other.volumeId && self.dwPercentCompleted == other.dwPercentCompleted
    }
}
impl ::core::cmp::Eq for VDS_FILE_SYSTEM_NOTIFICATION {}
impl ::core::default::Default for VDS_FILE_SYSTEM_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_FILE_SYSTEM_PROP {
    pub r#type: VDS_FILE_SYSTEM_TYPE,
    pub volumeId: ::windows::core::GUID,
    pub ulFlags: u32,
    pub ullTotalAllocationUnits: u64,
    pub ullAvailableAllocationUnits: u64,
    pub ulAllocationUnitSize: u32,
    pub pwszLabel: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_PROP {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_FILE_SYSTEM_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_FILE_SYSTEM_PROP").field("type", &self.r#type).field("volumeId", &self.volumeId).field("ulFlags", &self.ulFlags).field("ullTotalAllocationUnits", &self.ullTotalAllocationUnits).field("ullAvailableAllocationUnits", &self.ullAvailableAllocationUnits).field("ulAllocationUnitSize", &self.ulAllocationUnitSize).field("pwszLabel", &self.pwszLabel).finish()
    }
}
impl ::windows::core::TypeKind for VDS_FILE_SYSTEM_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_FILE_SYSTEM_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.volumeId == other.volumeId && self.ulFlags == other.ulFlags && self.ullTotalAllocationUnits == other.ullTotalAllocationUnits && self.ullAvailableAllocationUnits == other.ullAvailableAllocationUnits && self.ulAllocationUnitSize == other.ulAllocationUnitSize && self.pwszLabel == other.pwszLabel
    }
}
impl ::core::cmp::Eq for VDS_FILE_SYSTEM_PROP {}
impl ::core::default::Default for VDS_FILE_SYSTEM_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_FILE_SYSTEM_TYPE_PROP {
    pub r#type: VDS_FILE_SYSTEM_TYPE,
    pub wszName: [u16; 8],
    pub ulFlags: u32,
    pub ulCompressionFlags: u32,
    pub ulMaxLableLength: u32,
    pub pwszIllegalLabelCharSet: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VDS_FILE_SYSTEM_TYPE_PROP {}
impl ::core::clone::Clone for VDS_FILE_SYSTEM_TYPE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_FILE_SYSTEM_TYPE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_FILE_SYSTEM_TYPE_PROP").field("type", &self.r#type).field("wszName", &self.wszName).field("ulFlags", &self.ulFlags).field("ulCompressionFlags", &self.ulCompressionFlags).field("ulMaxLableLength", &self.ulMaxLableLength).field("pwszIllegalLabelCharSet", &self.pwszIllegalLabelCharSet).finish()
    }
}
impl ::windows::core::TypeKind for VDS_FILE_SYSTEM_TYPE_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_FILE_SYSTEM_TYPE_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.wszName == other.wszName && self.ulFlags == other.ulFlags && self.ulCompressionFlags == other.ulCompressionFlags && self.ulMaxLableLength == other.ulMaxLableLength && self.pwszIllegalLabelCharSet == other.pwszIllegalLabelCharSet
    }
}
impl ::core::cmp::Eq for VDS_FILE_SYSTEM_TYPE_PROP {}
impl ::core::default::Default for VDS_FILE_SYSTEM_TYPE_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_HBAPORT_PROP {
    pub id: ::windows::core::GUID,
    pub wwnNode: VDS_WWN,
    pub wwnPort: VDS_WWN,
    pub r#type: VDS_HBAPORT_TYPE,
    pub status: VDS_HBAPORT_STATUS,
    pub ulPortSpeed: u32,
    pub ulSupportedPortSpeed: u32,
}
impl ::core::marker::Copy for VDS_HBAPORT_PROP {}
impl ::core::clone::Clone for VDS_HBAPORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_HBAPORT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_HBAPORT_PROP").field("id", &self.id).field("wwnNode", &self.wwnNode).field("wwnPort", &self.wwnPort).field("type", &self.r#type).field("status", &self.status).field("ulPortSpeed", &self.ulPortSpeed).field("ulSupportedPortSpeed", &self.ulSupportedPortSpeed).finish()
    }
}
impl ::windows::core::TypeKind for VDS_HBAPORT_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_HBAPORT_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.wwnNode == other.wwnNode && self.wwnPort == other.wwnPort && self.r#type == other.r#type && self.status == other.status && self.ulPortSpeed == other.ulPortSpeed && self.ulSupportedPortSpeed == other.ulSupportedPortSpeed
    }
}
impl ::core::cmp::Eq for VDS_HBAPORT_PROP {}
impl ::core::default::Default for VDS_HBAPORT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_HINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_HINTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_HINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_HINTS")
            .field("ullHintMask", &self.ullHintMask)
            .field("ullExpectedMaximumSize", &self.ullExpectedMaximumSize)
            .field("ulOptimalReadSize", &self.ulOptimalReadSize)
            .field("ulOptimalReadAlignment", &self.ulOptimalReadAlignment)
            .field("ulOptimalWriteSize", &self.ulOptimalWriteSize)
            .field("ulOptimalWriteAlignment", &self.ulOptimalWriteAlignment)
            .field("ulMaximumDriveCount", &self.ulMaximumDriveCount)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("bFastCrashRecoveryRequired", &self.bFastCrashRecoveryRequired)
            .field("bMostlyReads", &self.bMostlyReads)
            .field("bOptimizeForSequentialReads", &self.bOptimizeForSequentialReads)
            .field("bOptimizeForSequentialWrites", &self.bOptimizeForSequentialWrites)
            .field("bRemapEnabled", &self.bRemapEnabled)
            .field("bReadBackVerifyEnabled", &self.bReadBackVerifyEnabled)
            .field("bWriteThroughCachingEnabled", &self.bWriteThroughCachingEnabled)
            .field("bHardwareChecksumEnabled", &self.bHardwareChecksumEnabled)
            .field("bIsYankable", &self.bIsYankable)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_HINTS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_HINTS {
    fn eq(&self, other: &Self) -> bool {
        self.ullHintMask == other.ullHintMask
            && self.ullExpectedMaximumSize == other.ullExpectedMaximumSize
            && self.ulOptimalReadSize == other.ulOptimalReadSize
            && self.ulOptimalReadAlignment == other.ulOptimalReadAlignment
            && self.ulOptimalWriteSize == other.ulOptimalWriteSize
            && self.ulOptimalWriteAlignment == other.ulOptimalWriteAlignment
            && self.ulMaximumDriveCount == other.ulMaximumDriveCount
            && self.ulStripeSize == other.ulStripeSize
            && self.bFastCrashRecoveryRequired == other.bFastCrashRecoveryRequired
            && self.bMostlyReads == other.bMostlyReads
            && self.bOptimizeForSequentialReads == other.bOptimizeForSequentialReads
            && self.bOptimizeForSequentialWrites == other.bOptimizeForSequentialWrites
            && self.bRemapEnabled == other.bRemapEnabled
            && self.bReadBackVerifyEnabled == other.bReadBackVerifyEnabled
            && self.bWriteThroughCachingEnabled == other.bWriteThroughCachingEnabled
            && self.bHardwareChecksumEnabled == other.bHardwareChecksumEnabled
            && self.bIsYankable == other.bIsYankable
            && self.sRebuildPriority == other.sRebuildPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_HINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_HINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS2 {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub bAllocateHotSpare: super::super::Foundation::BOOL,
    pub bUseMirroredCache: super::super::Foundation::BOOL,
    pub bReadCachingEnabled: super::super::Foundation::BOOL,
    pub bWriteCachingEnabled: super::super::Foundation::BOOL,
    pub bMediaScanEnabled: super::super::Foundation::BOOL,
    pub bConsistencyCheckEnabled: super::super::Foundation::BOOL,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub bReserved3: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_HINTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_HINTS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_HINTS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_HINTS2")
            .field("ullHintMask", &self.ullHintMask)
            .field("ullExpectedMaximumSize", &self.ullExpectedMaximumSize)
            .field("ulOptimalReadSize", &self.ulOptimalReadSize)
            .field("ulOptimalReadAlignment", &self.ulOptimalReadAlignment)
            .field("ulOptimalWriteSize", &self.ulOptimalWriteSize)
            .field("ulOptimalWriteAlignment", &self.ulOptimalWriteAlignment)
            .field("ulMaximumDriveCount", &self.ulMaximumDriveCount)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("ulReserved1", &self.ulReserved1)
            .field("ulReserved2", &self.ulReserved2)
            .field("ulReserved3", &self.ulReserved3)
            .field("bFastCrashRecoveryRequired", &self.bFastCrashRecoveryRequired)
            .field("bMostlyReads", &self.bMostlyReads)
            .field("bOptimizeForSequentialReads", &self.bOptimizeForSequentialReads)
            .field("bOptimizeForSequentialWrites", &self.bOptimizeForSequentialWrites)
            .field("bRemapEnabled", &self.bRemapEnabled)
            .field("bReadBackVerifyEnabled", &self.bReadBackVerifyEnabled)
            .field("bWriteThroughCachingEnabled", &self.bWriteThroughCachingEnabled)
            .field("bHardwareChecksumEnabled", &self.bHardwareChecksumEnabled)
            .field("bIsYankable", &self.bIsYankable)
            .field("bAllocateHotSpare", &self.bAllocateHotSpare)
            .field("bUseMirroredCache", &self.bUseMirroredCache)
            .field("bReadCachingEnabled", &self.bReadCachingEnabled)
            .field("bWriteCachingEnabled", &self.bWriteCachingEnabled)
            .field("bMediaScanEnabled", &self.bMediaScanEnabled)
            .field("bConsistencyCheckEnabled", &self.bConsistencyCheckEnabled)
            .field("BusType", &self.BusType)
            .field("bReserved1", &self.bReserved1)
            .field("bReserved2", &self.bReserved2)
            .field("bReserved3", &self.bReserved3)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_HINTS2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_HINTS2 {
    fn eq(&self, other: &Self) -> bool {
        self.ullHintMask == other.ullHintMask
            && self.ullExpectedMaximumSize == other.ullExpectedMaximumSize
            && self.ulOptimalReadSize == other.ulOptimalReadSize
            && self.ulOptimalReadAlignment == other.ulOptimalReadAlignment
            && self.ulOptimalWriteSize == other.ulOptimalWriteSize
            && self.ulOptimalWriteAlignment == other.ulOptimalWriteAlignment
            && self.ulMaximumDriveCount == other.ulMaximumDriveCount
            && self.ulStripeSize == other.ulStripeSize
            && self.ulReserved1 == other.ulReserved1
            && self.ulReserved2 == other.ulReserved2
            && self.ulReserved3 == other.ulReserved3
            && self.bFastCrashRecoveryRequired == other.bFastCrashRecoveryRequired
            && self.bMostlyReads == other.bMostlyReads
            && self.bOptimizeForSequentialReads == other.bOptimizeForSequentialReads
            && self.bOptimizeForSequentialWrites == other.bOptimizeForSequentialWrites
            && self.bRemapEnabled == other.bRemapEnabled
            && self.bReadBackVerifyEnabled == other.bReadBackVerifyEnabled
            && self.bWriteThroughCachingEnabled == other.bWriteThroughCachingEnabled
            && self.bHardwareChecksumEnabled == other.bHardwareChecksumEnabled
            && self.bIsYankable == other.bIsYankable
            && self.bAllocateHotSpare == other.bAllocateHotSpare
            && self.bUseMirroredCache == other.bUseMirroredCache
            && self.bReadCachingEnabled == other.bReadCachingEnabled
            && self.bWriteCachingEnabled == other.bWriteCachingEnabled
            && self.bMediaScanEnabled == other.bMediaScanEnabled
            && self.bConsistencyCheckEnabled == other.bConsistencyCheckEnabled
            && self.BusType == other.BusType
            && self.bReserved1 == other.bReserved1
            && self.bReserved2 == other.bReserved2
            && self.bReserved3 == other.bReserved3
            && self.sRebuildPriority == other.sRebuildPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_HINTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_HINTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_INPUT_DISK {
    pub diskId: ::windows::core::GUID,
    pub ullSize: u64,
    pub plexId: ::windows::core::GUID,
    pub memberIdx: u32,
}
impl ::core::marker::Copy for VDS_INPUT_DISK {}
impl ::core::clone::Clone for VDS_INPUT_DISK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_INPUT_DISK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_INPUT_DISK").field("diskId", &self.diskId).field("ullSize", &self.ullSize).field("plexId", &self.plexId).field("memberIdx", &self.memberIdx).finish()
    }
}
impl ::windows::core::TypeKind for VDS_INPUT_DISK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_INPUT_DISK {
    fn eq(&self, other: &Self) -> bool {
        self.diskId == other.diskId && self.ullSize == other.ullSize && self.plexId == other.plexId && self.memberIdx == other.memberIdx
    }
}
impl ::core::cmp::Eq for VDS_INPUT_DISK {}
impl ::core::default::Default for VDS_INPUT_DISK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_INTERCONNECT {
    pub m_addressType: VDS_INTERCONNECT_ADDRESS_TYPE,
    pub m_cbPort: u32,
    pub m_pbPort: *mut u8,
    pub m_cbAddress: u32,
    pub m_pbAddress: *mut u8,
}
impl ::core::marker::Copy for VDS_INTERCONNECT {}
impl ::core::clone::Clone for VDS_INTERCONNECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_INTERCONNECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_INTERCONNECT").field("m_addressType", &self.m_addressType).field("m_cbPort", &self.m_cbPort).field("m_pbPort", &self.m_pbPort).field("m_cbAddress", &self.m_cbAddress).field("m_pbAddress", &self.m_pbAddress).finish()
    }
}
impl ::windows::core::TypeKind for VDS_INTERCONNECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_INTERCONNECT {
    fn eq(&self, other: &Self) -> bool {
        self.m_addressType == other.m_addressType && self.m_cbPort == other.m_cbPort && self.m_pbPort == other.m_pbPort && self.m_cbAddress == other.m_cbAddress && self.m_pbAddress == other.m_pbAddress
    }
}
impl ::core::cmp::Eq for VDS_INTERCONNECT {}
impl ::core::default::Default for VDS_INTERCONNECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_IPADDRESS {
    pub r#type: VDS_IPADDRESS_TYPE,
    pub ipv4Address: u32,
    pub ipv6Address: [u8; 16],
    pub ulIpv6FlowInfo: u32,
    pub ulIpv6ScopeId: u32,
    pub wszTextAddress: [u16; 257],
    pub ulPort: u32,
}
impl ::core::marker::Copy for VDS_IPADDRESS {}
impl ::core::clone::Clone for VDS_IPADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_IPADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_IPADDRESS").field("type", &self.r#type).field("ipv4Address", &self.ipv4Address).field("ipv6Address", &self.ipv6Address).field("ulIpv6FlowInfo", &self.ulIpv6FlowInfo).field("ulIpv6ScopeId", &self.ulIpv6ScopeId).field("wszTextAddress", &self.wszTextAddress).field("ulPort", &self.ulPort).finish()
    }
}
impl ::windows::core::TypeKind for VDS_IPADDRESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_IPADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.ipv4Address == other.ipv4Address && self.ipv6Address == other.ipv6Address && self.ulIpv6FlowInfo == other.ulIpv6FlowInfo && self.ulIpv6ScopeId == other.ulIpv6ScopeId && self.wszTextAddress == other.wszTextAddress && self.ulPort == other.ulPort
    }
}
impl ::core::cmp::Eq for VDS_IPADDRESS {}
impl ::core::default::Default for VDS_IPADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    pub id: ::windows::core::GUID,
    pub pwszName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VDS_ISCSI_INITIATOR_ADAPTER_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_INITIATOR_ADAPTER_PROP").field("id", &self.id).field("pwszName", &self.pwszName).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszName == other.pwszName
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_INITIATOR_ADAPTER_PROP {}
impl ::core::default::Default for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_INITIATOR_PORTAL_PROP {
    pub id: ::windows::core::GUID,
    pub address: VDS_IPADDRESS,
    pub ulPortIndex: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_INITIATOR_PORTAL_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_INITIATOR_PORTAL_PROP").field("id", &self.id).field("address", &self.address).field("ulPortIndex", &self.ulPortIndex).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.address == other.address && self.ulPortIndex == other.ulPortIndex
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_INITIATOR_PORTAL_PROP {}
impl ::core::default::Default for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_IPSEC_KEY {
    pub pKey: *mut u8,
    pub ulKeySize: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_IPSEC_KEY {}
impl ::core::clone::Clone for VDS_ISCSI_IPSEC_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_IPSEC_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_IPSEC_KEY").field("pKey", &self.pKey).field("ulKeySize", &self.ulKeySize).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_IPSEC_KEY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_IPSEC_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.pKey == other.pKey && self.ulKeySize == other.ulKeySize
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_IPSEC_KEY {}
impl ::core::default::Default for VDS_ISCSI_IPSEC_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_PORTALGROUP_PROP {
    pub id: ::windows::core::GUID,
    pub tag: u16,
}
impl ::core::marker::Copy for VDS_ISCSI_PORTALGROUP_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_PORTALGROUP_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_PORTALGROUP_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_PORTALGROUP_PROP").field("id", &self.id).field("tag", &self.tag).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_PORTALGROUP_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_PORTALGROUP_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.tag == other.tag
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_PORTALGROUP_PROP {}
impl ::core::default::Default for VDS_ISCSI_PORTALGROUP_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_PORTAL_PROP {
    pub id: ::windows::core::GUID,
    pub address: VDS_IPADDRESS,
    pub status: VDS_ISCSI_PORTAL_STATUS,
}
impl ::core::marker::Copy for VDS_ISCSI_PORTAL_PROP {}
impl ::core::clone::Clone for VDS_ISCSI_PORTAL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_PORTAL_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_PORTAL_PROP").field("id", &self.id).field("address", &self.address).field("status", &self.status).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_PORTAL_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_PORTAL_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.address == other.address && self.status == other.status
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_PORTAL_PROP {}
impl ::core::default::Default for VDS_ISCSI_PORTAL_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_ISCSI_SHARED_SECRET {
    pub pSharedSecret: *mut u8,
    pub ulSharedSecretSize: u32,
}
impl ::core::marker::Copy for VDS_ISCSI_SHARED_SECRET {}
impl ::core::clone::Clone for VDS_ISCSI_SHARED_SECRET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_ISCSI_SHARED_SECRET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_SHARED_SECRET").field("pSharedSecret", &self.pSharedSecret).field("ulSharedSecretSize", &self.ulSharedSecretSize).finish()
    }
}
impl ::windows::core::TypeKind for VDS_ISCSI_SHARED_SECRET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_ISCSI_SHARED_SECRET {
    fn eq(&self, other: &Self) -> bool {
        self.pSharedSecret == other.pSharedSecret && self.ulSharedSecretSize == other.ulSharedSecretSize
    }
}
impl ::core::cmp::Eq for VDS_ISCSI_SHARED_SECRET {}
impl ::core::default::Default for VDS_ISCSI_SHARED_SECRET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_ISCSI_TARGET_PROP {
    pub id: ::windows::core::GUID,
    pub pwszIscsiName: ::windows::core::PWSTR,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub bChapEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_ISCSI_TARGET_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_ISCSI_TARGET_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_ISCSI_TARGET_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_ISCSI_TARGET_PROP").field("id", &self.id).field("pwszIscsiName", &self.pwszIscsiName).field("pwszFriendlyName", &self.pwszFriendlyName).field("bChapEnabled", &self.bChapEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_ISCSI_TARGET_PROP {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_ISCSI_TARGET_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszIscsiName == other.pwszIscsiName && self.pwszFriendlyName == other.pwszFriendlyName && self.bChapEnabled == other.bChapEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_ISCSI_TARGET_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_ISCSI_TARGET_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_LUN_INFORMATION {
    pub m_version: u32,
    pub m_DeviceType: u8,
    pub m_DeviceTypeModifier: u8,
    pub m_bCommandQueueing: super::super::Foundation::BOOL,
    pub m_BusType: VDS_STORAGE_BUS_TYPE,
    pub m_szVendorId: *mut u8,
    pub m_szProductId: *mut u8,
    pub m_szProductRevision: *mut u8,
    pub m_szSerialNumber: *mut u8,
    pub m_diskSignature: ::windows::core::GUID,
    pub m_deviceIdDescriptor: VDS_STORAGE_DEVICE_ID_DESCRIPTOR,
    pub m_cInterconnects: u32,
    pub m_rgInterconnects: *mut VDS_INTERCONNECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_LUN_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_LUN_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_LUN_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_LUN_INFORMATION")
            .field("m_version", &self.m_version)
            .field("m_DeviceType", &self.m_DeviceType)
            .field("m_DeviceTypeModifier", &self.m_DeviceTypeModifier)
            .field("m_bCommandQueueing", &self.m_bCommandQueueing)
            .field("m_BusType", &self.m_BusType)
            .field("m_szVendorId", &self.m_szVendorId)
            .field("m_szProductId", &self.m_szProductId)
            .field("m_szProductRevision", &self.m_szProductRevision)
            .field("m_szSerialNumber", &self.m_szSerialNumber)
            .field("m_diskSignature", &self.m_diskSignature)
            .field("m_deviceIdDescriptor", &self.m_deviceIdDescriptor)
            .field("m_cInterconnects", &self.m_cInterconnects)
            .field("m_rgInterconnects", &self.m_rgInterconnects)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_LUN_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_LUN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.m_version == other.m_version && self.m_DeviceType == other.m_DeviceType && self.m_DeviceTypeModifier == other.m_DeviceTypeModifier && self.m_bCommandQueueing == other.m_bCommandQueueing && self.m_BusType == other.m_BusType && self.m_szVendorId == other.m_szVendorId && self.m_szProductId == other.m_szProductId && self.m_szProductRevision == other.m_szProductRevision && self.m_szSerialNumber == other.m_szSerialNumber && self.m_diskSignature == other.m_diskSignature && self.m_deviceIdDescriptor == other.m_deviceIdDescriptor && self.m_cInterconnects == other.m_cInterconnects && self.m_rgInterconnects == other.m_rgInterconnects
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_LUN_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_LUN_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_LUN_NOTIFICATION {
    pub ulEvent: VDS_NF_LUN,
    pub LunId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_LUN_NOTIFICATION {}
impl ::core::clone::Clone for VDS_LUN_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_LUN_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_LUN_NOTIFICATION").field("ulEvent", &self.ulEvent).field("LunId", &self.LunId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_LUN_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_LUN_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.LunId == other.LunId
    }
}
impl ::core::cmp::Eq for VDS_LUN_NOTIFICATION {}
impl ::core::default::Default for VDS_LUN_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_LUN_PLEX_PROP {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub r#type: VDS_LUN_PLEX_TYPE,
    pub status: VDS_LUN_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ulFlags: u32,
    pub ulStripeSize: u32,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_LUN_PLEX_PROP {}
impl ::core::clone::Clone for VDS_LUN_PLEX_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_LUN_PLEX_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_LUN_PLEX_PROP").field("id", &self.id).field("ullSize", &self.ullSize).field("type", &self.r#type).field("status", &self.status).field("health", &self.health).field("TransitionState", &self.TransitionState).field("ulFlags", &self.ulFlags).field("ulStripeSize", &self.ulStripeSize).field("sRebuildPriority", &self.sRebuildPriority).finish()
    }
}
impl ::windows::core::TypeKind for VDS_LUN_PLEX_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_LUN_PLEX_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.r#type == other.r#type && self.status == other.status && self.health == other.health && self.TransitionState == other.TransitionState && self.ulFlags == other.ulFlags && self.ulStripeSize == other.ulStripeSize && self.sRebuildPriority == other.sRebuildPriority
    }
}
impl ::core::cmp::Eq for VDS_LUN_PLEX_PROP {}
impl ::core::default::Default for VDS_LUN_PLEX_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_LUN_PROP {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pwszIdentification: ::windows::core::PWSTR,
    pub pwszUnmaskingList: ::windows::core::PWSTR,
    pub ulFlags: u32,
    pub r#type: VDS_LUN_TYPE,
    pub status: VDS_LUN_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_LUN_PROP {}
impl ::core::clone::Clone for VDS_LUN_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_LUN_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_LUN_PROP").field("id", &self.id).field("ullSize", &self.ullSize).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("pwszUnmaskingList", &self.pwszUnmaskingList).field("ulFlags", &self.ulFlags).field("type", &self.r#type).field("status", &self.status).field("health", &self.health).field("TransitionState", &self.TransitionState).field("sRebuildPriority", &self.sRebuildPriority).finish()
    }
}
impl ::windows::core::TypeKind for VDS_LUN_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_LUN_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.pwszUnmaskingList == other.pwszUnmaskingList && self.ulFlags == other.ulFlags && self.r#type == other.r#type && self.status == other.status && self.health == other.health && self.TransitionState == other.TransitionState && self.sRebuildPriority == other.sRebuildPriority
    }
}
impl ::core::cmp::Eq for VDS_LUN_PROP {}
impl ::core::default::Default for VDS_LUN_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_MOUNT_POINT_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_MOUNT_POINT_NOTIFICATION {}
impl ::core::clone::Clone for VDS_MOUNT_POINT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_MOUNT_POINT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_MOUNT_POINT_NOTIFICATION").field("ulEvent", &self.ulEvent).field("volumeId", &self.volumeId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_MOUNT_POINT_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_MOUNT_POINT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.volumeId == other.volumeId
    }
}
impl ::core::cmp::Eq for VDS_MOUNT_POINT_NOTIFICATION {}
impl ::core::default::Default for VDS_MOUNT_POINT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_NOTIFICATION {
    pub objectType: VDS_NOTIFICATION_TARGET_TYPE,
    pub Anonymous: VDS_NOTIFICATION_0,
}
impl ::core::marker::Copy for VDS_NOTIFICATION {}
impl ::core::clone::Clone for VDS_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_NOTIFICATION_0 {
    pub Pack: VDS_PACK_NOTIFICATION,
    pub Disk: VDS_DISK_NOTIFICATION,
    pub Volume: VDS_VOLUME_NOTIFICATION,
    pub Partition: VDS_PARTITION_NOTIFICATION,
    pub Letter: VDS_DRIVE_LETTER_NOTIFICATION,
    pub FileSystem: VDS_FILE_SYSTEM_NOTIFICATION,
    pub MountPoint: VDS_MOUNT_POINT_NOTIFICATION,
    pub SubSystem: VDS_SUB_SYSTEM_NOTIFICATION,
    pub Controller: VDS_CONTROLLER_NOTIFICATION,
    pub Drive: VDS_DRIVE_NOTIFICATION,
    pub Lun: VDS_LUN_NOTIFICATION,
    pub Port: VDS_PORT_NOTIFICATION,
    pub Portal: VDS_PORTAL_NOTIFICATION,
    pub Target: VDS_TARGET_NOTIFICATION,
    pub PortalGroup: VDS_PORTAL_GROUP_NOTIFICATION,
    pub Service: VDS_SERVICE_NOTIFICATION,
}
impl ::core::marker::Copy for VDS_NOTIFICATION_0 {}
impl ::core::clone::Clone for VDS_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_NOTIFICATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PACK_NOTIFICATION {
    pub ulEvent: VDS_NF_PACK,
    pub packId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PACK_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PACK_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PACK_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PACK_NOTIFICATION").field("ulEvent", &self.ulEvent).field("packId", &self.packId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PACK_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PACK_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.packId == other.packId
    }
}
impl ::core::cmp::Eq for VDS_PACK_NOTIFICATION {}
impl ::core::default::Default for VDS_PACK_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PACK_PROP {
    pub id: ::windows::core::GUID,
    pub pwszName: ::windows::core::PWSTR,
    pub status: VDS_PACK_STATUS,
    pub ulFlags: u32,
}
impl ::core::marker::Copy for VDS_PACK_PROP {}
impl ::core::clone::Clone for VDS_PACK_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PACK_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PACK_PROP").field("id", &self.id).field("pwszName", &self.pwszName).field("status", &self.status).field("ulFlags", &self.ulFlags).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PACK_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PACK_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszName == other.pwszName && self.status == other.status && self.ulFlags == other.ulFlags
    }
}
impl ::core::cmp::Eq for VDS_PACK_PROP {}
impl ::core::default::Default for VDS_PACK_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PARTITION_INFORMATION_EX {
    pub dwPartitionStyle: __VDS_PARTITION_STYLE,
    pub ullStartingOffset: u64,
    pub ullPartitionLength: u64,
    pub dwPartitionNumber: u32,
    pub bRewritePartition: super::super::Foundation::BOOLEAN,
    pub Anonymous: VDS_PARTITION_INFORMATION_EX_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_INFORMATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_PARTITION_INFORMATION_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union VDS_PARTITION_INFORMATION_EX_0 {
    pub Mbr: VDS_PARTITION_INFO_MBR,
    pub Gpt: VDS_PARTITION_INFO_GPT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_INFORMATION_EX_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_INFORMATION_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_PARTITION_INFORMATION_EX_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PARTITION_INFO_GPT {
    pub partitionType: ::windows::core::GUID,
    pub partitionId: ::windows::core::GUID,
    pub attributes: u64,
    pub name: [u16; 36],
}
impl ::core::marker::Copy for VDS_PARTITION_INFO_GPT {}
impl ::core::clone::Clone for VDS_PARTITION_INFO_GPT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PARTITION_INFO_GPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PARTITION_INFO_GPT").field("partitionType", &self.partitionType).field("partitionId", &self.partitionId).field("attributes", &self.attributes).field("name", &self.name).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PARTITION_INFO_GPT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PARTITION_INFO_GPT {
    fn eq(&self, other: &Self) -> bool {
        self.partitionType == other.partitionType && self.partitionId == other.partitionId && self.attributes == other.attributes && self.name == other.name
    }
}
impl ::core::cmp::Eq for VDS_PARTITION_INFO_GPT {}
impl ::core::default::Default for VDS_PARTITION_INFO_GPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PARTITION_INFO_MBR {
    pub partitionType: u8,
    pub bootIndicator: super::super::Foundation::BOOLEAN,
    pub recognizedPartition: super::super::Foundation::BOOLEAN,
    pub hiddenSectors: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_INFO_MBR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_INFO_MBR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_PARTITION_INFO_MBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PARTITION_INFO_MBR").field("partitionType", &self.partitionType).field("bootIndicator", &self.bootIndicator).field("recognizedPartition", &self.recognizedPartition).field("hiddenSectors", &self.hiddenSectors).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_PARTITION_INFO_MBR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_PARTITION_INFO_MBR {
    fn eq(&self, other: &Self) -> bool {
        self.partitionType == other.partitionType && self.bootIndicator == other.bootIndicator && self.recognizedPartition == other.recognizedPartition && self.hiddenSectors == other.hiddenSectors
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_PARTITION_INFO_MBR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_PARTITION_INFO_MBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PARTITION_NOTIFICATION {
    pub ulEvent: u32,
    pub diskId: ::windows::core::GUID,
    pub ullOffset: u64,
}
impl ::core::marker::Copy for VDS_PARTITION_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PARTITION_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PARTITION_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PARTITION_NOTIFICATION").field("ulEvent", &self.ulEvent).field("diskId", &self.diskId).field("ullOffset", &self.ullOffset).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PARTITION_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PARTITION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.diskId == other.diskId && self.ullOffset == other.ullOffset
    }
}
impl ::core::cmp::Eq for VDS_PARTITION_NOTIFICATION {}
impl ::core::default::Default for VDS_PARTITION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PARTITION_PROP {
    pub PartitionStyle: VDS_PARTITION_STYLE,
    pub ulFlags: u32,
    pub ulPartitionNumber: u32,
    pub ullOffset: u64,
    pub ullSize: u64,
    pub Anonymous: VDS_PARTITION_PROP_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_PARTITION_PROP {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_PARTITION_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union VDS_PARTITION_PROP_0 {
    pub Mbr: VDS_PARTITION_INFO_MBR,
    pub Gpt: VDS_PARTITION_INFO_GPT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PARTITION_PROP_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PARTITION_PROP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_PARTITION_PROP_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_PARTITION_PROP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PATH_ID {
    pub ullSourceId: u64,
    pub ullPathId: u64,
}
impl ::core::marker::Copy for VDS_PATH_ID {}
impl ::core::clone::Clone for VDS_PATH_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PATH_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PATH_ID").field("ullSourceId", &self.ullSourceId).field("ullPathId", &self.ullPathId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PATH_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PATH_ID {
    fn eq(&self, other: &Self) -> bool {
        self.ullSourceId == other.ullSourceId && self.ullPathId == other.ullPathId
    }
}
impl ::core::cmp::Eq for VDS_PATH_ID {}
impl ::core::default::Default for VDS_PATH_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PATH_INFO {
    pub pathId: VDS_PATH_ID,
    pub r#type: VDS_HWPROVIDER_TYPE,
    pub status: VDS_PATH_STATUS,
    pub Anonymous1: VDS_PATH_INFO_0,
    pub Anonymous2: VDS_PATH_INFO_1,
    pub Anonymous3: VDS_PATH_INFO_2,
}
impl ::core::marker::Copy for VDS_PATH_INFO {}
impl ::core::clone::Clone for VDS_PATH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_PATH_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_PATH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_PATH_INFO_0 {
    pub controllerPortId: ::windows::core::GUID,
    pub targetPortalId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PATH_INFO_0 {}
impl ::core::clone::Clone for VDS_PATH_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_PATH_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_PATH_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_PATH_INFO_1 {
    pub hbaPortId: ::windows::core::GUID,
    pub initiatorAdapterId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PATH_INFO_1 {}
impl ::core::clone::Clone for VDS_PATH_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_PATH_INFO_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_PATH_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub union VDS_PATH_INFO_2 {
    pub pHbaPortProp: *mut VDS_HBAPORT_PROP,
    pub pInitiatorPortalIpAddr: *mut VDS_IPADDRESS,
}
impl ::core::marker::Copy for VDS_PATH_INFO_2 {}
impl ::core::clone::Clone for VDS_PATH_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VDS_PATH_INFO_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VDS_PATH_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PATH_POLICY {
    pub pathId: VDS_PATH_ID,
    pub bPrimaryPath: super::super::Foundation::BOOL,
    pub ulWeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_PATH_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_PATH_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_PATH_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PATH_POLICY").field("pathId", &self.pathId).field("bPrimaryPath", &self.bPrimaryPath).field("ulWeight", &self.ulWeight).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_PATH_POLICY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_PATH_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.pathId == other.pathId && self.bPrimaryPath == other.bPrimaryPath && self.ulWeight == other.ulWeight
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_PATH_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_PATH_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_POOL_ATTRIBUTES {
    pub ullAttributeMask: u64,
    pub raidType: VDS_RAID_TYPE,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub pwszIntendedUsage: ::windows::core::PWSTR,
    pub bSpinDown: super::super::Foundation::BOOL,
    pub bIsThinProvisioned: super::super::Foundation::BOOL,
    pub ullProvisionedSpace: u64,
    pub bNoSinglePointOfFailure: super::super::Foundation::BOOL,
    pub ulDataRedundancyMax: u32,
    pub ulDataRedundancyMin: u32,
    pub ulDataRedundancyDefault: u32,
    pub ulPackageRedundancyMax: u32,
    pub ulPackageRedundancyMin: u32,
    pub ulPackageRedundancyDefault: u32,
    pub ulStripeSize: u32,
    pub ulStripeSizeMax: u32,
    pub ulStripeSizeMin: u32,
    pub ulDefaultStripeSize: u32,
    pub ulNumberOfColumns: u32,
    pub ulNumberOfColumnsMax: u32,
    pub ulNumberOfColumnsMin: u32,
    pub ulDefaultNumberofColumns: u32,
    pub ulDataAvailabilityHint: u32,
    pub ulAccessRandomnessHint: u32,
    pub ulAccessDirectionHint: u32,
    pub ulAccessSizeHint: u32,
    pub ulAccessLatencyHint: u32,
    pub ulAccessBandwidthWeightHint: u32,
    pub ulStorageCostHint: u32,
    pub ulStorageEfficiencyHint: u32,
    pub ulNumOfCustomAttributes: u32,
    pub pPoolCustomAttributes: *mut VDS_POOL_CUSTOM_ATTRIBUTES,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ullReserved1: u64,
    pub ullReserved2: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_POOL_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_POOL_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_POOL_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_POOL_ATTRIBUTES")
            .field("ullAttributeMask", &self.ullAttributeMask)
            .field("raidType", &self.raidType)
            .field("busType", &self.busType)
            .field("pwszIntendedUsage", &self.pwszIntendedUsage)
            .field("bSpinDown", &self.bSpinDown)
            .field("bIsThinProvisioned", &self.bIsThinProvisioned)
            .field("ullProvisionedSpace", &self.ullProvisionedSpace)
            .field("bNoSinglePointOfFailure", &self.bNoSinglePointOfFailure)
            .field("ulDataRedundancyMax", &self.ulDataRedundancyMax)
            .field("ulDataRedundancyMin", &self.ulDataRedundancyMin)
            .field("ulDataRedundancyDefault", &self.ulDataRedundancyDefault)
            .field("ulPackageRedundancyMax", &self.ulPackageRedundancyMax)
            .field("ulPackageRedundancyMin", &self.ulPackageRedundancyMin)
            .field("ulPackageRedundancyDefault", &self.ulPackageRedundancyDefault)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("ulStripeSizeMax", &self.ulStripeSizeMax)
            .field("ulStripeSizeMin", &self.ulStripeSizeMin)
            .field("ulDefaultStripeSize", &self.ulDefaultStripeSize)
            .field("ulNumberOfColumns", &self.ulNumberOfColumns)
            .field("ulNumberOfColumnsMax", &self.ulNumberOfColumnsMax)
            .field("ulNumberOfColumnsMin", &self.ulNumberOfColumnsMin)
            .field("ulDefaultNumberofColumns", &self.ulDefaultNumberofColumns)
            .field("ulDataAvailabilityHint", &self.ulDataAvailabilityHint)
            .field("ulAccessRandomnessHint", &self.ulAccessRandomnessHint)
            .field("ulAccessDirectionHint", &self.ulAccessDirectionHint)
            .field("ulAccessSizeHint", &self.ulAccessSizeHint)
            .field("ulAccessLatencyHint", &self.ulAccessLatencyHint)
            .field("ulAccessBandwidthWeightHint", &self.ulAccessBandwidthWeightHint)
            .field("ulStorageCostHint", &self.ulStorageCostHint)
            .field("ulStorageEfficiencyHint", &self.ulStorageEfficiencyHint)
            .field("ulNumOfCustomAttributes", &self.ulNumOfCustomAttributes)
            .field("pPoolCustomAttributes", &self.pPoolCustomAttributes)
            .field("bReserved1", &self.bReserved1)
            .field("bReserved2", &self.bReserved2)
            .field("ulReserved1", &self.ulReserved1)
            .field("ulReserved2", &self.ulReserved2)
            .field("ullReserved1", &self.ullReserved1)
            .field("ullReserved2", &self.ullReserved2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_POOL_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_POOL_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.ullAttributeMask == other.ullAttributeMask
            && self.raidType == other.raidType
            && self.busType == other.busType
            && self.pwszIntendedUsage == other.pwszIntendedUsage
            && self.bSpinDown == other.bSpinDown
            && self.bIsThinProvisioned == other.bIsThinProvisioned
            && self.ullProvisionedSpace == other.ullProvisionedSpace
            && self.bNoSinglePointOfFailure == other.bNoSinglePointOfFailure
            && self.ulDataRedundancyMax == other.ulDataRedundancyMax
            && self.ulDataRedundancyMin == other.ulDataRedundancyMin
            && self.ulDataRedundancyDefault == other.ulDataRedundancyDefault
            && self.ulPackageRedundancyMax == other.ulPackageRedundancyMax
            && self.ulPackageRedundancyMin == other.ulPackageRedundancyMin
            && self.ulPackageRedundancyDefault == other.ulPackageRedundancyDefault
            && self.ulStripeSize == other.ulStripeSize
            && self.ulStripeSizeMax == other.ulStripeSizeMax
            && self.ulStripeSizeMin == other.ulStripeSizeMin
            && self.ulDefaultStripeSize == other.ulDefaultStripeSize
            && self.ulNumberOfColumns == other.ulNumberOfColumns
            && self.ulNumberOfColumnsMax == other.ulNumberOfColumnsMax
            && self.ulNumberOfColumnsMin == other.ulNumberOfColumnsMin
            && self.ulDefaultNumberofColumns == other.ulDefaultNumberofColumns
            && self.ulDataAvailabilityHint == other.ulDataAvailabilityHint
            && self.ulAccessRandomnessHint == other.ulAccessRandomnessHint
            && self.ulAccessDirectionHint == other.ulAccessDirectionHint
            && self.ulAccessSizeHint == other.ulAccessSizeHint
            && self.ulAccessLatencyHint == other.ulAccessLatencyHint
            && self.ulAccessBandwidthWeightHint == other.ulAccessBandwidthWeightHint
            && self.ulStorageCostHint == other.ulStorageCostHint
            && self.ulStorageEfficiencyHint == other.ulStorageEfficiencyHint
            && self.ulNumOfCustomAttributes == other.ulNumOfCustomAttributes
            && self.pPoolCustomAttributes == other.pPoolCustomAttributes
            && self.bReserved1 == other.bReserved1
            && self.bReserved2 == other.bReserved2
            && self.ulReserved1 == other.ulReserved1
            && self.ulReserved2 == other.ulReserved2
            && self.ullReserved1 == other.ullReserved1
            && self.ullReserved2 == other.ullReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_POOL_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_POOL_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_POOL_CUSTOM_ATTRIBUTES {
    pub pwszName: ::windows::core::PWSTR,
    pub pwszValue: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VDS_POOL_CUSTOM_ATTRIBUTES {}
impl ::core::clone::Clone for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_POOL_CUSTOM_ATTRIBUTES").field("pwszName", &self.pwszName).field("pwszValue", &self.pwszValue).finish()
    }
}
impl ::windows::core::TypeKind for VDS_POOL_CUSTOM_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.pwszValue == other.pwszValue
    }
}
impl ::core::cmp::Eq for VDS_POOL_CUSTOM_ATTRIBUTES {}
impl ::core::default::Default for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PORTAL_GROUP_NOTIFICATION {
    pub ulEvent: u32,
    pub portalGroupId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PORTAL_GROUP_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORTAL_GROUP_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PORTAL_GROUP_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PORTAL_GROUP_NOTIFICATION").field("ulEvent", &self.ulEvent).field("portalGroupId", &self.portalGroupId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PORTAL_GROUP_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PORTAL_GROUP_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.portalGroupId == other.portalGroupId
    }
}
impl ::core::cmp::Eq for VDS_PORTAL_GROUP_NOTIFICATION {}
impl ::core::default::Default for VDS_PORTAL_GROUP_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PORTAL_NOTIFICATION {
    pub ulEvent: u32,
    pub portalId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PORTAL_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORTAL_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PORTAL_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PORTAL_NOTIFICATION").field("ulEvent", &self.ulEvent).field("portalId", &self.portalId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PORTAL_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PORTAL_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.portalId == other.portalId
    }
}
impl ::core::cmp::Eq for VDS_PORTAL_NOTIFICATION {}
impl ::core::default::Default for VDS_PORTAL_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PORT_NOTIFICATION {
    pub ulEvent: VDS_NF_PORT,
    pub portId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_PORT_NOTIFICATION {}
impl ::core::clone::Clone for VDS_PORT_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PORT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PORT_NOTIFICATION").field("ulEvent", &self.ulEvent).field("portId", &self.portId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PORT_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PORT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.portId == other.portId
    }
}
impl ::core::cmp::Eq for VDS_PORT_NOTIFICATION {}
impl ::core::default::Default for VDS_PORT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PORT_PROP {
    pub id: ::windows::core::GUID,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pwszIdentification: ::windows::core::PWSTR,
    pub status: VDS_PORT_STATUS,
}
impl ::core::marker::Copy for VDS_PORT_PROP {}
impl ::core::clone::Clone for VDS_PORT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PORT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PORT_PROP").field("id", &self.id).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("status", &self.status).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PORT_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PORT_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.status == other.status
    }
}
impl ::core::cmp::Eq for VDS_PORT_PROP {}
impl ::core::default::Default for VDS_PORT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_PROVIDER_PROP {
    pub id: ::windows::core::GUID,
    pub pwszName: ::windows::core::PWSTR,
    pub guidVersionId: ::windows::core::GUID,
    pub pwszVersion: ::windows::core::PWSTR,
    pub r#type: VDS_PROVIDER_TYPE,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_PROVIDER_PROP {}
impl ::core::clone::Clone for VDS_PROVIDER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_PROVIDER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_PROVIDER_PROP").field("id", &self.id).field("pwszName", &self.pwszName).field("guidVersionId", &self.guidVersionId).field("pwszVersion", &self.pwszVersion).field("type", &self.r#type).field("ulFlags", &self.ulFlags).field("ulStripeSizeFlags", &self.ulStripeSizeFlags).field("sRebuildPriority", &self.sRebuildPriority).finish()
    }
}
impl ::windows::core::TypeKind for VDS_PROVIDER_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_PROVIDER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszName == other.pwszName && self.guidVersionId == other.guidVersionId && self.pwszVersion == other.pwszVersion && self.r#type == other.r#type && self.ulFlags == other.ulFlags && self.ulStripeSizeFlags == other.ulStripeSizeFlags && self.sRebuildPriority == other.sRebuildPriority
    }
}
impl ::core::cmp::Eq for VDS_PROVIDER_PROP {}
impl ::core::default::Default for VDS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_REPARSE_POINT_PROP {
    pub SourceVolumeId: ::windows::core::GUID,
    pub pwszPath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VDS_REPARSE_POINT_PROP {}
impl ::core::clone::Clone for VDS_REPARSE_POINT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_REPARSE_POINT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_REPARSE_POINT_PROP").field("SourceVolumeId", &self.SourceVolumeId).field("pwszPath", &self.pwszPath).finish()
    }
}
impl ::windows::core::TypeKind for VDS_REPARSE_POINT_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_REPARSE_POINT_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.SourceVolumeId == other.SourceVolumeId && self.pwszPath == other.pwszPath
    }
}
impl ::core::cmp::Eq for VDS_REPARSE_POINT_PROP {}
impl ::core::default::Default for VDS_REPARSE_POINT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SERVICE_NOTIFICATION {
    pub ulEvent: u32,
    pub action: VDS_RECOVER_ACTION,
}
impl ::core::marker::Copy for VDS_SERVICE_NOTIFICATION {}
impl ::core::clone::Clone for VDS_SERVICE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_SERVICE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_SERVICE_NOTIFICATION").field("ulEvent", &self.ulEvent).field("action", &self.action).finish()
    }
}
impl ::windows::core::TypeKind for VDS_SERVICE_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_SERVICE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.action == other.action
    }
}
impl ::core::cmp::Eq for VDS_SERVICE_NOTIFICATION {}
impl ::core::default::Default for VDS_SERVICE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SERVICE_PROP {
    pub pwszVersion: ::windows::core::PWSTR,
    pub ulFlags: u32,
}
impl ::core::marker::Copy for VDS_SERVICE_PROP {}
impl ::core::clone::Clone for VDS_SERVICE_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_SERVICE_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_SERVICE_PROP").field("pwszVersion", &self.pwszVersion).field("ulFlags", &self.ulFlags).finish()
    }
}
impl ::windows::core::TypeKind for VDS_SERVICE_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_SERVICE_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.pwszVersion == other.pwszVersion && self.ulFlags == other.ulFlags
    }
}
impl ::core::cmp::Eq for VDS_SERVICE_PROP {}
impl ::core::default::Default for VDS_SERVICE_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    pub m_version: u32,
    pub m_cIdentifiers: u32,
    pub m_rgIdentifiers: *mut VDS_STORAGE_IDENTIFIER,
}
impl ::core::marker::Copy for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::core::clone::Clone for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_STORAGE_DEVICE_ID_DESCRIPTOR").field("m_version", &self.m_version).field("m_cIdentifiers", &self.m_cIdentifiers).field("m_rgIdentifiers", &self.m_rgIdentifiers).finish()
    }
}
impl ::windows::core::TypeKind for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.m_version == other.m_version && self.m_cIdentifiers == other.m_cIdentifiers && self.m_rgIdentifiers == other.m_rgIdentifiers
    }
}
impl ::core::cmp::Eq for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::core::default::Default for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_STORAGE_IDENTIFIER {
    pub m_CodeSet: VDS_STORAGE_IDENTIFIER_CODE_SET,
    pub m_Type: VDS_STORAGE_IDENTIFIER_TYPE,
    pub m_cbIdentifier: u32,
    pub m_rgbIdentifier: *mut u8,
}
impl ::core::marker::Copy for VDS_STORAGE_IDENTIFIER {}
impl ::core::clone::Clone for VDS_STORAGE_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_STORAGE_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_STORAGE_IDENTIFIER").field("m_CodeSet", &self.m_CodeSet).field("m_Type", &self.m_Type).field("m_cbIdentifier", &self.m_cbIdentifier).field("m_rgbIdentifier", &self.m_rgbIdentifier).finish()
    }
}
impl ::windows::core::TypeKind for VDS_STORAGE_IDENTIFIER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_STORAGE_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.m_CodeSet == other.m_CodeSet && self.m_Type == other.m_Type && self.m_cbIdentifier == other.m_cbIdentifier && self.m_rgbIdentifier == other.m_rgbIdentifier
    }
}
impl ::core::cmp::Eq for VDS_STORAGE_IDENTIFIER {}
impl ::core::default::Default for VDS_STORAGE_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_STORAGE_POOL_DRIVE_EXTENT {
    pub id: ::windows::core::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VDS_STORAGE_POOL_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_STORAGE_POOL_DRIVE_EXTENT").field("id", &self.id).field("ullSize", &self.ullSize).field("bUsed", &self.bUsed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VDS_STORAGE_POOL_DRIVE_EXTENT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.bUsed == other.bUsed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDS_STORAGE_POOL_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_STORAGE_POOL_PROP {
    pub id: ::windows::core::GUID,
    pub status: VDS_STORAGE_POOL_STATUS,
    pub health: VDS_HEALTH,
    pub r#type: VDS_STORAGE_POOL_TYPE,
    pub pwszName: ::windows::core::PWSTR,
    pub pwszDescription: ::windows::core::PWSTR,
    pub ullTotalConsumedSpace: u64,
    pub ullTotalManagedSpace: u64,
    pub ullRemainingFreeSpace: u64,
}
impl ::core::marker::Copy for VDS_STORAGE_POOL_PROP {}
impl ::core::clone::Clone for VDS_STORAGE_POOL_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_STORAGE_POOL_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_STORAGE_POOL_PROP").field("id", &self.id).field("status", &self.status).field("health", &self.health).field("type", &self.r#type).field("pwszName", &self.pwszName).field("pwszDescription", &self.pwszDescription).field("ullTotalConsumedSpace", &self.ullTotalConsumedSpace).field("ullTotalManagedSpace", &self.ullTotalManagedSpace).field("ullRemainingFreeSpace", &self.ullRemainingFreeSpace).finish()
    }
}
impl ::windows::core::TypeKind for VDS_STORAGE_POOL_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_STORAGE_POOL_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.status == other.status && self.health == other.health && self.r#type == other.r#type && self.pwszName == other.pwszName && self.pwszDescription == other.pwszDescription && self.ullTotalConsumedSpace == other.ullTotalConsumedSpace && self.ullTotalManagedSpace == other.ullTotalManagedSpace && self.ullRemainingFreeSpace == other.ullRemainingFreeSpace
    }
}
impl ::core::cmp::Eq for VDS_STORAGE_POOL_PROP {}
impl ::core::default::Default for VDS_STORAGE_POOL_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SUB_SYSTEM_NOTIFICATION {
    pub ulEvent: u32,
    pub subSystemId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_SUB_SYSTEM_NOTIFICATION {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_SUB_SYSTEM_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_SUB_SYSTEM_NOTIFICATION").field("ulEvent", &self.ulEvent).field("subSystemId", &self.subSystemId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_SUB_SYSTEM_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_SUB_SYSTEM_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.subSystemId == other.subSystemId
    }
}
impl ::core::cmp::Eq for VDS_SUB_SYSTEM_NOTIFICATION {}
impl ::core::default::Default for VDS_SUB_SYSTEM_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SUB_SYSTEM_PROP {
    pub id: ::windows::core::GUID,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pwszIdentification: ::windows::core::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
}
impl ::core::marker::Copy for VDS_SUB_SYSTEM_PROP {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_SUB_SYSTEM_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_SUB_SYSTEM_PROP")
            .field("id", &self.id)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("ulStripeSizeFlags", &self.ulStripeSizeFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sNumberOfInternalBuses", &self.sNumberOfInternalBuses)
            .field("sMaxNumberOfSlotsEachBus", &self.sMaxNumberOfSlotsEachBus)
            .field("sMaxNumberOfControllers", &self.sMaxNumberOfControllers)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
impl ::windows::core::TypeKind for VDS_SUB_SYSTEM_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_SUB_SYSTEM_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.ulFlags == other.ulFlags && self.ulStripeSizeFlags == other.ulStripeSizeFlags && self.status == other.status && self.health == other.health && self.sNumberOfInternalBuses == other.sNumberOfInternalBuses && self.sMaxNumberOfSlotsEachBus == other.sMaxNumberOfSlotsEachBus && self.sMaxNumberOfControllers == other.sMaxNumberOfControllers && self.sRebuildPriority == other.sRebuildPriority
    }
}
impl ::core::cmp::Eq for VDS_SUB_SYSTEM_PROP {}
impl ::core::default::Default for VDS_SUB_SYSTEM_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_SUB_SYSTEM_PROP2 {
    pub id: ::windows::core::GUID,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub pwszIdentification: ::windows::core::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub ulSupportedRaidTypeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
    pub ulNumberOfEnclosures: u32,
}
impl ::core::marker::Copy for VDS_SUB_SYSTEM_PROP2 {}
impl ::core::clone::Clone for VDS_SUB_SYSTEM_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_SUB_SYSTEM_PROP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_SUB_SYSTEM_PROP2")
            .field("id", &self.id)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("ulStripeSizeFlags", &self.ulStripeSizeFlags)
            .field("ulSupportedRaidTypeFlags", &self.ulSupportedRaidTypeFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sNumberOfInternalBuses", &self.sNumberOfInternalBuses)
            .field("sMaxNumberOfSlotsEachBus", &self.sMaxNumberOfSlotsEachBus)
            .field("sMaxNumberOfControllers", &self.sMaxNumberOfControllers)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .field("ulNumberOfEnclosures", &self.ulNumberOfEnclosures)
            .finish()
    }
}
impl ::windows::core::TypeKind for VDS_SUB_SYSTEM_PROP2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_SUB_SYSTEM_PROP2 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.ulFlags == other.ulFlags && self.ulStripeSizeFlags == other.ulStripeSizeFlags && self.ulSupportedRaidTypeFlags == other.ulSupportedRaidTypeFlags && self.status == other.status && self.health == other.health && self.sNumberOfInternalBuses == other.sNumberOfInternalBuses && self.sMaxNumberOfSlotsEachBus == other.sMaxNumberOfSlotsEachBus && self.sMaxNumberOfControllers == other.sMaxNumberOfControllers && self.sRebuildPriority == other.sRebuildPriority && self.ulNumberOfEnclosures == other.ulNumberOfEnclosures
    }
}
impl ::core::cmp::Eq for VDS_SUB_SYSTEM_PROP2 {}
impl ::core::default::Default for VDS_SUB_SYSTEM_PROP2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_TARGET_NOTIFICATION {
    pub ulEvent: u32,
    pub targetId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VDS_TARGET_NOTIFICATION {}
impl ::core::clone::Clone for VDS_TARGET_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_TARGET_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_TARGET_NOTIFICATION").field("ulEvent", &self.ulEvent).field("targetId", &self.targetId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_TARGET_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_TARGET_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.targetId == other.targetId
    }
}
impl ::core::cmp::Eq for VDS_TARGET_NOTIFICATION {}
impl ::core::default::Default for VDS_TARGET_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Vhd\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
pub struct VDS_VDISK_PROPERTIES {
    pub Id: ::windows::core::GUID,
    pub State: VDS_VDISK_STATE,
    pub VirtualDeviceType: super::Vhd::VIRTUAL_STORAGE_TYPE,
    pub VirtualSize: u64,
    pub PhysicalSize: u64,
    pub pPath: ::windows::core::PWSTR,
    pub pDeviceName: ::windows::core::PWSTR,
    pub DiskFlag: super::Vhd::DEPENDENT_DISK_FLAG,
    pub bIsChild: super::super::Foundation::BOOL,
    pub pParentPath: ::windows::core::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::core::marker::Copy for VDS_VDISK_PROPERTIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::core::clone::Clone for VDS_VDISK_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::core::fmt::Debug for VDS_VDISK_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_VDISK_PROPERTIES").field("Id", &self.Id).field("State", &self.State).field("VirtualDeviceType", &self.VirtualDeviceType).field("VirtualSize", &self.VirtualSize).field("PhysicalSize", &self.PhysicalSize).field("pPath", &self.pPath).field("pDeviceName", &self.pDeviceName).field("DiskFlag", &self.DiskFlag).field("bIsChild", &self.bIsChild).field("pParentPath", &self.pParentPath).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::windows::core::TypeKind for VDS_VDISK_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::core::cmp::PartialEq for VDS_VDISK_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.State == other.State && self.VirtualDeviceType == other.VirtualDeviceType && self.VirtualSize == other.VirtualSize && self.PhysicalSize == other.PhysicalSize && self.pPath == other.pPath && self.pDeviceName == other.pDeviceName && self.DiskFlag == other.DiskFlag && self.bIsChild == other.bIsChild && self.pParentPath == other.pParentPath
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::core::cmp::Eq for VDS_VDISK_PROPERTIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Vhd"))]
impl ::core::default::Default for VDS_VDISK_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_VOLUME_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows::core::GUID,
    pub plexId: ::windows::core::GUID,
    pub ulPercentCompleted: u32,
}
impl ::core::marker::Copy for VDS_VOLUME_NOTIFICATION {}
impl ::core::clone::Clone for VDS_VOLUME_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_VOLUME_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_VOLUME_NOTIFICATION").field("ulEvent", &self.ulEvent).field("volumeId", &self.volumeId).field("plexId", &self.plexId).field("ulPercentCompleted", &self.ulPercentCompleted).finish()
    }
}
impl ::windows::core::TypeKind for VDS_VOLUME_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_VOLUME_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.volumeId == other.volumeId && self.plexId == other.plexId && self.ulPercentCompleted == other.ulPercentCompleted
    }
}
impl ::core::cmp::Eq for VDS_VOLUME_NOTIFICATION {}
impl ::core::default::Default for VDS_VOLUME_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_VOLUME_PLEX_PROP {
    pub id: ::windows::core::GUID,
    pub r#type: VDS_VOLUME_PLEX_TYPE,
    pub status: VDS_VOLUME_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulStripeSize: u32,
    pub ulNumberOfMembers: u32,
}
impl ::core::marker::Copy for VDS_VOLUME_PLEX_PROP {}
impl ::core::clone::Clone for VDS_VOLUME_PLEX_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_VOLUME_PLEX_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_VOLUME_PLEX_PROP").field("id", &self.id).field("type", &self.r#type).field("status", &self.status).field("health", &self.health).field("TransitionState", &self.TransitionState).field("ullSize", &self.ullSize).field("ulStripeSize", &self.ulStripeSize).field("ulNumberOfMembers", &self.ulNumberOfMembers).finish()
    }
}
impl ::windows::core::TypeKind for VDS_VOLUME_PLEX_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_VOLUME_PLEX_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.r#type == other.r#type && self.status == other.status && self.health == other.health && self.TransitionState == other.TransitionState && self.ullSize == other.ullSize && self.ulStripeSize == other.ulStripeSize && self.ulNumberOfMembers == other.ulNumberOfMembers
    }
}
impl ::core::cmp::Eq for VDS_VOLUME_PLEX_PROP {}
impl ::core::default::Default for VDS_VOLUME_PLEX_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_VOLUME_PROP {
    pub id: ::windows::core::GUID,
    pub r#type: VDS_VOLUME_TYPE,
    pub status: VDS_VOLUME_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulFlags: u32,
    pub RecommendedFileSystemType: VDS_FILE_SYSTEM_TYPE,
    pub pwszName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for VDS_VOLUME_PROP {}
impl ::core::clone::Clone for VDS_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_VOLUME_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_VOLUME_PROP").field("id", &self.id).field("type", &self.r#type).field("status", &self.status).field("health", &self.health).field("TransitionState", &self.TransitionState).field("ullSize", &self.ullSize).field("ulFlags", &self.ulFlags).field("RecommendedFileSystemType", &self.RecommendedFileSystemType).field("pwszName", &self.pwszName).finish()
    }
}
impl ::windows::core::TypeKind for VDS_VOLUME_PROP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.r#type == other.r#type && self.status == other.status && self.health == other.health && self.TransitionState == other.TransitionState && self.ullSize == other.ullSize && self.ulFlags == other.ulFlags && self.RecommendedFileSystemType == other.RecommendedFileSystemType && self.pwszName == other.pwszName
    }
}
impl ::core::cmp::Eq for VDS_VOLUME_PROP {}
impl ::core::default::Default for VDS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_VOLUME_PROP2 {
    pub id: ::windows::core::GUID,
    pub r#type: VDS_VOLUME_TYPE,
    pub status: VDS_VOLUME_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ullSize: u64,
    pub ulFlags: u32,
    pub RecommendedFileSystemType: VDS_FILE_SYSTEM_TYPE,
    pub cbUniqueId: u32,
    pub pwszName: ::windows::core::PWSTR,
    pub pUniqueId: *mut u8,
}
impl ::core::marker::Copy for VDS_VOLUME_PROP2 {}
impl ::core::clone::Clone for VDS_VOLUME_PROP2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_VOLUME_PROP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_VOLUME_PROP2").field("id", &self.id).field("type", &self.r#type).field("status", &self.status).field("health", &self.health).field("TransitionState", &self.TransitionState).field("ullSize", &self.ullSize).field("ulFlags", &self.ulFlags).field("RecommendedFileSystemType", &self.RecommendedFileSystemType).field("cbUniqueId", &self.cbUniqueId).field("pwszName", &self.pwszName).field("pUniqueId", &self.pUniqueId).finish()
    }
}
impl ::windows::core::TypeKind for VDS_VOLUME_PROP2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_VOLUME_PROP2 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.r#type == other.r#type && self.status == other.status && self.health == other.health && self.TransitionState == other.TransitionState && self.ullSize == other.ullSize && self.ulFlags == other.ulFlags && self.RecommendedFileSystemType == other.RecommendedFileSystemType && self.cbUniqueId == other.cbUniqueId && self.pwszName == other.pwszName && self.pUniqueId == other.pUniqueId
    }
}
impl ::core::cmp::Eq for VDS_VOLUME_PROP2 {}
impl ::core::default::Default for VDS_VOLUME_PROP2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_VirtualDiskService\"`*"]
pub struct VDS_WWN {
    pub rguchWwn: [u8; 8],
}
impl ::core::marker::Copy for VDS_WWN {}
impl ::core::clone::Clone for VDS_WWN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VDS_WWN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDS_WWN").field("rguchWwn", &self.rguchWwn).finish()
    }
}
impl ::windows::core::TypeKind for VDS_WWN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for VDS_WWN {
    fn eq(&self, other: &Self) -> bool {
        self.rguchWwn == other.rguchWwn
    }
}
impl ::core::cmp::Eq for VDS_WWN {}
impl ::core::default::Default for VDS_WWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
