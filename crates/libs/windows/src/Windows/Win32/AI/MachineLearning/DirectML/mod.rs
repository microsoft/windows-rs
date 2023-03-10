#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[inline]
pub unsafe fn DMLCreateDevice<P0, T>(d3d12device: P0, flags: DML_CREATE_DEVICE_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "directml.dll""system" fn DMLCreateDevice ( d3d12device : * mut::core::ffi::c_void , flags : DML_CREATE_DEVICE_FLAGS , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DMLCreateDevice(d3d12device.into_param().abi(), flags, &<T as ::windows::core::ComInterface>::IID, result__ as *mut _ as *mut _).ok()
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
#[inline]
pub unsafe fn DMLCreateDevice1<P0, T>(d3d12device: P0, flags: DML_CREATE_DEVICE_FLAGS, minimumfeaturelevel: DML_FEATURE_LEVEL, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12Device>,
    T: ::windows::core::ComInterface,
{
    ::windows::imp::link ! ( "directml.dll""system" fn DMLCreateDevice1 ( d3d12device : * mut::core::ffi::c_void , flags : DML_CREATE_DEVICE_FLAGS , minimumfeaturelevel : DML_FEATURE_LEVEL , riid : *const :: windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    DMLCreateDevice1(d3d12device.into_param().abi(), flags, minimumfeaturelevel, &<T as ::windows::core::ComInterface>::IID, result__ as *mut _ as *mut _).ok()
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLBindingTable(::windows::core::IUnknown);
impl IDMLBindingTable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn BindInputs(&self, bindings: ::core::option::Option<&[DML_BINDING_DESC]>) {
        (::windows::core::Interface::vtable(self).BindInputs)(::windows::core::Interface::as_raw(self), bindings.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn BindOutputs(&self, bindings: ::core::option::Option<&[DML_BINDING_DESC]>) {
        (::windows::core::Interface::vtable(self).BindOutputs)(::windows::core::Interface::as_raw(self), bindings.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(bindings.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn BindTemporaryResource(&self, binding: ::core::option::Option<*const DML_BINDING_DESC>) {
        (::windows::core::Interface::vtable(self).BindTemporaryResource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(binding.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BindPersistentResource(&self, binding: ::core::option::Option<*const DML_BINDING_DESC>) {
        (::windows::core::Interface::vtable(self).BindPersistentResource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(binding.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn Reset(&self, desc: ::core::option::Option<*const DML_BINDING_TABLE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(desc.unwrap_or(::std::ptr::null()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IDMLBindingTable, ::windows::core::IUnknown, IDMLObject, IDMLDeviceChild);
impl ::core::cmp::PartialEq for IDMLBindingTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLBindingTable {}
impl ::core::fmt::Debug for IDMLBindingTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLBindingTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLBindingTable {
    type Vtable = IDMLBindingTable_Vtbl;
}
impl ::core::clone::Clone for IDMLBindingTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLBindingTable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29c687dc_de74_4e3b_ab00_1168f2fc3cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLBindingTable_Vtbl {
    pub base__: IDMLDeviceChild_Vtbl,
    pub BindInputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC),
    pub BindOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bindingcount: u32, bindings: *const DML_BINDING_DESC),
    pub BindTemporaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC),
    pub BindPersistentResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binding: *const DML_BINDING_DESC),
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    Reset: usize,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLCommandRecorder(::windows::core::IUnknown);
impl IDMLCommandRecorder {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn RecordDispatch<P0, P1, P2>(&self, commandlist: P0, dispatchable: P1, bindings: P2)
    where
        P0: ::windows::core::IntoParam<super::super::super::Graphics::Direct3D12::ID3D12CommandList>,
        P1: ::windows::core::IntoParam<IDMLDispatchable>,
        P2: ::windows::core::IntoParam<IDMLBindingTable>,
    {
        (::windows::core::Interface::vtable(self).RecordDispatch)(::windows::core::Interface::as_raw(self), commandlist.into_param().abi(), dispatchable.into_param().abi(), bindings.into_param().abi())
    }
}
::windows::imp::interface_hierarchy!(IDMLCommandRecorder, ::windows::core::IUnknown, IDMLObject, IDMLDeviceChild);
impl ::core::cmp::PartialEq for IDMLCommandRecorder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLCommandRecorder {}
impl ::core::fmt::Debug for IDMLCommandRecorder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLCommandRecorder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLCommandRecorder {
    type Vtable = IDMLCommandRecorder_Vtbl;
}
impl ::core::clone::Clone for IDMLCommandRecorder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLCommandRecorder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6857a76_2e3e_4fdd_bff4_5d2ba10fb453);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLCommandRecorder_Vtbl {
    pub base__: IDMLDeviceChild_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub RecordDispatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commandlist: *mut ::core::ffi::c_void, dispatchable: *mut ::core::ffi::c_void, bindings: *mut ::core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    RecordDispatch: usize,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLCompiledOperator(::windows::core::IUnknown);
impl IDMLCompiledOperator {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetBindingProperties)(::windows::core::Interface::as_raw(self), &mut result__);
        result__
    }
}
::windows::imp::interface_hierarchy!(IDMLCompiledOperator, ::windows::core::IUnknown, IDMLObject, IDMLDeviceChild, IDMLPageable, IDMLDispatchable);
impl ::core::cmp::PartialEq for IDMLCompiledOperator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLCompiledOperator {}
impl ::core::fmt::Debug for IDMLCompiledOperator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLCompiledOperator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLCompiledOperator {
    type Vtable = IDMLCompiledOperator_Vtbl;
}
impl ::core::clone::Clone for IDMLCompiledOperator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLCompiledOperator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b15e56a_bf5c_4902_92d8_da3a650afea4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLCompiledOperator_Vtbl {
    pub base__: IDMLDispatchable_Vtbl,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLDebugDevice(::windows::core::IUnknown);
impl IDMLDebugDevice {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuteDebugOutput<P0>(&self, mute: P0)
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetMuteDebugOutput)(::windows::core::Interface::as_raw(self), mute.into_param().abi())
    }
}
::windows::imp::interface_hierarchy!(IDMLDebugDevice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDMLDebugDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDebugDevice {}
impl ::core::fmt::Debug for IDMLDebugDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDebugDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLDebugDevice {
    type Vtable = IDMLDebugDevice_Vtbl;
}
impl ::core::clone::Clone for IDMLDebugDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLDebugDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d6f3ac9_394a_4ac3_92a7_390cc57a8217);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDebugDevice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMuteDebugOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mute: super::super::super::Foundation::BOOL),
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMuteDebugOutput: usize,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLDevice(::windows::core::IUnknown);
impl IDMLDevice {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: ::core::option::Option<*const ::core::ffi::c_void>, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CheckFeatureSupport)(::windows::core::Interface::as_raw(self), feature, featurequerydatasize, ::core::mem::transmute(featurequerydata.unwrap_or(::std::ptr::null())), featuresupportdatasize, featuresupportdata).ok()
    }
    pub unsafe fn CreateOperator<T>(&self, desc: *const DML_OPERATOR_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::ComInterface,
    {
        (::windows::core::Interface::vtable(self).CreateOperator)(::windows::core::Interface::as_raw(self), desc, &<T as ::windows::core::ComInterface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CompileOperator<P0, T>(&self, op: P0, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDMLOperator>,
        T: ::windows::core::ComInterface,
    {
        (::windows::core::Interface::vtable(self).CompileOperator)(::windows::core::Interface::as_raw(self), op.into_param().abi(), flags, &<T as ::windows::core::ComInterface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateOperatorInitializer<T>(&self, operators: ::core::option::Option<&[::core::option::Option<IDMLCompiledOperator>]>) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateOperatorInitializer)(::windows::core::Interface::as_raw(self), operators.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(operators.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateCommandRecorder<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateCommandRecorder)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateBindingTable<T>(&self, desc: ::core::option::Option<*const DML_BINDING_TABLE_DESC>) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateBindingTable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(desc.unwrap_or(::std::ptr::null())), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Evict(&self, ppobjects: &[::core::option::Option<IDMLPageable>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Evict)(::windows::core::Interface::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[::core::option::Option<IDMLPageable>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MakeResident)(::windows::core::Interface::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceRemovedReason)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetParentDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetParentDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDMLDevice, ::windows::core::IUnknown, IDMLObject);
impl ::core::cmp::PartialEq for IDMLDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDevice {}
impl ::core::fmt::Debug for IDMLDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLDevice {
    type Vtable = IDMLDevice_Vtbl;
}
impl ::core::clone::Clone for IDMLDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dbd6437_96fd_423f_a98c_ae5e7c2a573f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDevice_Vtbl {
    pub base__: IDMLObject_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: *const ::core::ffi::c_void, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateOperator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_OPERATOR_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompileOperator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, op: *mut ::core::ffi::c_void, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateOperatorInitializer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateCommandRecorder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub CreateBindingTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_BINDING_TABLE_DESC, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))]
    CreateBindingTable: usize,
    pub Evict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MakeResident: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, ppobjects: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceRemovedReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParentDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLDevice1(::windows::core::IUnknown);
impl IDMLDevice1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DML_FEATURE, featurequerydatasize: u32, featurequerydata: ::core::option::Option<*const ::core::ffi::c_void>, featuresupportdatasize: u32, featuresupportdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CheckFeatureSupport)(::windows::core::Interface::as_raw(self), feature, featurequerydatasize, ::core::mem::transmute(featurequerydata.unwrap_or(::std::ptr::null())), featuresupportdatasize, featuresupportdata).ok()
    }
    pub unsafe fn CreateOperator<T>(&self, desc: *const DML_OPERATOR_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::ComInterface,
    {
        (::windows::core::Interface::vtable(self).base__.CreateOperator)(::windows::core::Interface::as_raw(self), desc, &<T as ::windows::core::ComInterface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CompileOperator<P0, T>(&self, op: P0, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDMLOperator>,
        T: ::windows::core::ComInterface,
    {
        (::windows::core::Interface::vtable(self).base__.CompileOperator)(::windows::core::Interface::as_raw(self), op.into_param().abi(), flags, &<T as ::windows::core::ComInterface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateOperatorInitializer<T>(&self, operators: ::core::option::Option<&[::core::option::Option<IDMLCompiledOperator>]>) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.CreateOperatorInitializer)(::windows::core::Interface::as_raw(self), operators.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(operators.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateCommandRecorder<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.CreateCommandRecorder)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateBindingTable<T>(&self, desc: ::core::option::Option<*const DML_BINDING_TABLE_DESC>) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.CreateBindingTable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(desc.unwrap_or(::std::ptr::null())), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn Evict(&self, ppobjects: &[::core::option::Option<IDMLPageable>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Evict)(::windows::core::Interface::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[::core::option::Option<IDMLPageable>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.MakeResident)(::windows::core::Interface::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceRemovedReason)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetParentDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetParentDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn CompileGraph<T>(&self, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::ComInterface,
    {
        (::windows::core::Interface::vtable(self).CompileGraph)(::windows::core::Interface::as_raw(self), desc, flags, &<T as ::windows::core::ComInterface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
::windows::imp::interface_hierarchy!(IDMLDevice1, ::windows::core::IUnknown, IDMLObject, IDMLDevice);
impl ::core::cmp::PartialEq for IDMLDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDevice1 {}
impl ::core::fmt::Debug for IDMLDevice1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDevice1").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLDevice1 {
    type Vtable = IDMLDevice1_Vtbl;
}
impl ::core::clone::Clone for IDMLDevice1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLDevice1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0884f9a_d2be_4355_aa5d_5901281ad1d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDevice1_Vtbl {
    pub base__: IDMLDevice_Vtbl,
    pub CompileGraph: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desc: *const DML_GRAPH_DESC, flags: DML_EXECUTION_FLAGS, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLDeviceChild(::windows::core::IUnknown);
impl IDMLDeviceChild {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDMLDeviceChild, ::windows::core::IUnknown, IDMLObject);
impl ::core::cmp::PartialEq for IDMLDeviceChild {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDeviceChild {}
impl ::core::fmt::Debug for IDMLDeviceChild {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDeviceChild").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLDeviceChild {
    type Vtable = IDMLDeviceChild_Vtbl;
}
impl ::core::clone::Clone for IDMLDeviceChild {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLDeviceChild {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27e83142_8165_49e3_974e_2fd66e4cb69d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDeviceChild_Vtbl {
    pub base__: IDMLObject_Vtbl,
    pub GetDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLDispatchable(::windows::core::IUnknown);
impl IDMLDispatchable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetBindingProperties)(::windows::core::Interface::as_raw(self), &mut result__);
        result__
    }
}
::windows::imp::interface_hierarchy!(IDMLDispatchable, ::windows::core::IUnknown, IDMLObject, IDMLDeviceChild, IDMLPageable);
impl ::core::cmp::PartialEq for IDMLDispatchable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLDispatchable {}
impl ::core::fmt::Debug for IDMLDispatchable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLDispatchable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLDispatchable {
    type Vtable = IDMLDispatchable_Vtbl;
}
impl ::core::clone::Clone for IDMLDispatchable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLDispatchable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcb821a8_1039_441e_9f1c_b1759c2f3cec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLDispatchable_Vtbl {
    pub base__: IDMLPageable_Vtbl,
    pub GetBindingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DML_BINDING_PROPERTIES),
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLObject(::windows::core::IUnknown);
impl IDMLObject {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDMLObject, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDMLObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLObject {}
impl ::core::fmt::Debug for IDMLObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLObject {
    type Vtable = IDMLObject_Vtbl;
}
impl ::core::clone::Clone for IDMLObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8263aac_9e0c_4a2d_9b8e_007521a3317c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLObject_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: *mut u32, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, datasize: u32, data: *const ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, data: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLOperator(::windows::core::IUnknown);
impl IDMLOperator {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDMLOperator, ::windows::core::IUnknown, IDMLObject, IDMLDeviceChild);
impl ::core::cmp::PartialEq for IDMLOperator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLOperator {}
impl ::core::fmt::Debug for IDMLOperator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLOperator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLOperator {
    type Vtable = IDMLOperator_Vtbl;
}
impl ::core::clone::Clone for IDMLOperator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLOperator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26caae7a_3081_4633_9581_226fbe57695d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLOperator_Vtbl {
    pub base__: IDMLDeviceChild_Vtbl,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLOperatorInitializer(::windows::core::IUnknown);
impl IDMLOperatorInitializer {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBindingProperties(&self) -> DML_BINDING_PROPERTIES {
        let mut result__: DML_BINDING_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetBindingProperties)(::windows::core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn Reset(&self, operators: ::core::option::Option<&[::core::option::Option<IDMLCompiledOperator>]>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self), operators.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(operators.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
::windows::imp::interface_hierarchy!(IDMLOperatorInitializer, ::windows::core::IUnknown, IDMLObject, IDMLDeviceChild, IDMLPageable, IDMLDispatchable);
impl ::core::cmp::PartialEq for IDMLOperatorInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLOperatorInitializer {}
impl ::core::fmt::Debug for IDMLOperatorInitializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLOperatorInitializer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLOperatorInitializer {
    type Vtable = IDMLOperatorInitializer_Vtbl;
}
impl ::core::clone::Clone for IDMLOperatorInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLOperatorInitializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x427c1113_435c_469c_8676_4d5dd072f813);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLOperatorInitializer_Vtbl {
    pub base__: IDMLDispatchable_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operatorcount: u32, operators: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
pub struct IDMLPageable(::windows::core::IUnknown);
impl IDMLPageable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: *mut u32, data: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateData)(::windows::core::Interface::as_raw(self), guid, datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Interface::as_raw(self), guid, data.into_param().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).base__.GetDevice)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDMLPageable, ::windows::core::IUnknown, IDMLObject, IDMLDeviceChild);
impl ::core::cmp::PartialEq for IDMLPageable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMLPageable {}
impl ::core::fmt::Debug for IDMLPageable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMLPageable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDMLPageable {
    type Vtable = IDMLPageable_Vtbl;
}
impl ::core::clone::Clone for IDMLPageable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDMLPageable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1ab0825_4542_4a4b_8617_6dde6e8f6201);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDMLPageable_Vtbl {
    pub base__: IDMLDeviceChild_Vtbl,
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_MINIMUM_BUFFER_TENSOR_ALIGNMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_PERSISTENT_BUFFER_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TARGET_VERSION: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TEMPORARY_BUFFER_ALIGNMENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DIMENSION_COUNT_MAX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DIMENSION_COUNT_MAX1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_AXIS_DIRECTION(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_AXIS_DIRECTION_INCREASING: DML_AXIS_DIRECTION = DML_AXIS_DIRECTION(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_AXIS_DIRECTION_DECREASING: DML_AXIS_DIRECTION = DML_AXIS_DIRECTION(1i32);
impl ::core::marker::Copy for DML_AXIS_DIRECTION {}
impl ::core::clone::Clone for DML_AXIS_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_AXIS_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_AXIS_DIRECTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_AXIS_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_AXIS_DIRECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_BINDING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_BINDING_TYPE_NONE: DML_BINDING_TYPE = DML_BINDING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_BINDING_TYPE_BUFFER: DML_BINDING_TYPE = DML_BINDING_TYPE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_BINDING_TYPE_BUFFER_ARRAY: DML_BINDING_TYPE = DML_BINDING_TYPE(2i32);
impl ::core::marker::Copy for DML_BINDING_TYPE {}
impl ::core::clone::Clone for DML_BINDING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_BINDING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_BINDING_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_BINDING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_BINDING_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_CONVOLUTION_DIRECTION(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_CONVOLUTION_DIRECTION_FORWARD: DML_CONVOLUTION_DIRECTION = DML_CONVOLUTION_DIRECTION(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_CONVOLUTION_DIRECTION_BACKWARD: DML_CONVOLUTION_DIRECTION = DML_CONVOLUTION_DIRECTION(1i32);
impl ::core::marker::Copy for DML_CONVOLUTION_DIRECTION {}
impl ::core::clone::Clone for DML_CONVOLUTION_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_CONVOLUTION_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_CONVOLUTION_DIRECTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_CONVOLUTION_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_CONVOLUTION_DIRECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_CONVOLUTION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_CONVOLUTION_MODE_CONVOLUTION: DML_CONVOLUTION_MODE = DML_CONVOLUTION_MODE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_CONVOLUTION_MODE_CROSS_CORRELATION: DML_CONVOLUTION_MODE = DML_CONVOLUTION_MODE(1i32);
impl ::core::marker::Copy for DML_CONVOLUTION_MODE {}
impl ::core::clone::Clone for DML_CONVOLUTION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_CONVOLUTION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_CONVOLUTION_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_CONVOLUTION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_CONVOLUTION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_CREATE_DEVICE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_CREATE_DEVICE_FLAG_NONE: DML_CREATE_DEVICE_FLAGS = DML_CREATE_DEVICE_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_CREATE_DEVICE_FLAG_DEBUG: DML_CREATE_DEVICE_FLAGS = DML_CREATE_DEVICE_FLAGS(1i32);
impl ::core::marker::Copy for DML_CREATE_DEVICE_FLAGS {}
impl ::core::clone::Clone for DML_CREATE_DEVICE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_CREATE_DEVICE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_CREATE_DEVICE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_CREATE_DEVICE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_CREATE_DEVICE_FLAGS").field(&self.0).finish()
    }
}
impl DML_CREATE_DEVICE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DML_CREATE_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DML_CREATE_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DML_CREATE_DEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DML_CREATE_DEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DML_CREATE_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_DEPTH_SPACE_ORDER(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_DEPTH_SPACE_ORDER_DEPTH_COLUMN_ROW: DML_DEPTH_SPACE_ORDER = DML_DEPTH_SPACE_ORDER(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_DEPTH_SPACE_ORDER_COLUMN_ROW_DEPTH: DML_DEPTH_SPACE_ORDER = DML_DEPTH_SPACE_ORDER(1i32);
impl ::core::marker::Copy for DML_DEPTH_SPACE_ORDER {}
impl ::core::clone::Clone for DML_DEPTH_SPACE_ORDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_DEPTH_SPACE_ORDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_DEPTH_SPACE_ORDER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_DEPTH_SPACE_ORDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_DEPTH_SPACE_ORDER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_EXECUTION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_EXECUTION_FLAG_NONE: DML_EXECUTION_FLAGS = DML_EXECUTION_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_EXECUTION_FLAG_ALLOW_HALF_PRECISION_COMPUTATION: DML_EXECUTION_FLAGS = DML_EXECUTION_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_EXECUTION_FLAG_DISABLE_META_COMMANDS: DML_EXECUTION_FLAGS = DML_EXECUTION_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_EXECUTION_FLAG_DESCRIPTORS_VOLATILE: DML_EXECUTION_FLAGS = DML_EXECUTION_FLAGS(4i32);
impl ::core::marker::Copy for DML_EXECUTION_FLAGS {}
impl ::core::clone::Clone for DML_EXECUTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_EXECUTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_EXECUTION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_EXECUTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_EXECUTION_FLAGS").field(&self.0).finish()
    }
}
impl DML_EXECUTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DML_EXECUTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DML_EXECUTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DML_EXECUTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DML_EXECUTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DML_EXECUTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_FEATURE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_FEATURE_TENSOR_DATA_TYPE_SUPPORT: DML_FEATURE = DML_FEATURE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_FEATURE_FEATURE_LEVELS: DML_FEATURE = DML_FEATURE(1i32);
impl ::core::marker::Copy for DML_FEATURE {}
impl ::core::clone::Clone for DML_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_FEATURE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_FEATURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_FEATURE_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_FEATURE_LEVEL_1_0: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(4096i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_FEATURE_LEVEL_2_0: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(8192i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_FEATURE_LEVEL_2_1: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(8448i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_FEATURE_LEVEL_3_0: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(12288i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_FEATURE_LEVEL_3_1: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(12544i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_FEATURE_LEVEL_4_0: DML_FEATURE_LEVEL = DML_FEATURE_LEVEL(16384i32);
impl ::core::marker::Copy for DML_FEATURE_LEVEL {}
impl ::core::clone::Clone for DML_FEATURE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_FEATURE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_FEATURE_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_FEATURE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_FEATURE_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_GRAPH_EDGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_GRAPH_EDGE_TYPE_INVALID: DML_GRAPH_EDGE_TYPE = DML_GRAPH_EDGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_GRAPH_EDGE_TYPE_INPUT: DML_GRAPH_EDGE_TYPE = DML_GRAPH_EDGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_GRAPH_EDGE_TYPE_OUTPUT: DML_GRAPH_EDGE_TYPE = DML_GRAPH_EDGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_GRAPH_EDGE_TYPE_INTERMEDIATE: DML_GRAPH_EDGE_TYPE = DML_GRAPH_EDGE_TYPE(3i32);
impl ::core::marker::Copy for DML_GRAPH_EDGE_TYPE {}
impl ::core::clone::Clone for DML_GRAPH_EDGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_GRAPH_EDGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_GRAPH_EDGE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_GRAPH_EDGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_GRAPH_EDGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_GRAPH_NODE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_GRAPH_NODE_TYPE_INVALID: DML_GRAPH_NODE_TYPE = DML_GRAPH_NODE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_GRAPH_NODE_TYPE_OPERATOR: DML_GRAPH_NODE_TYPE = DML_GRAPH_NODE_TYPE(1i32);
impl ::core::marker::Copy for DML_GRAPH_NODE_TYPE {}
impl ::core::clone::Clone for DML_GRAPH_NODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_GRAPH_NODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_GRAPH_NODE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_GRAPH_NODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_GRAPH_NODE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_INTERPOLATION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DML_INTERPOLATION_MODE = DML_INTERPOLATION_MODE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_INTERPOLATION_MODE_LINEAR: DML_INTERPOLATION_MODE = DML_INTERPOLATION_MODE(1i32);
impl ::core::marker::Copy for DML_INTERPOLATION_MODE {}
impl ::core::clone::Clone for DML_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_INTERPOLATION_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_IS_INFINITY_MODE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_IS_INFINITY_MODE_EITHER: DML_IS_INFINITY_MODE = DML_IS_INFINITY_MODE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_IS_INFINITY_MODE_POSITIVE: DML_IS_INFINITY_MODE = DML_IS_INFINITY_MODE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_IS_INFINITY_MODE_NEGATIVE: DML_IS_INFINITY_MODE = DML_IS_INFINITY_MODE(2i32);
impl ::core::marker::Copy for DML_IS_INFINITY_MODE {}
impl ::core::clone::Clone for DML_IS_INFINITY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_IS_INFINITY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_IS_INFINITY_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_IS_INFINITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_IS_INFINITY_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_MATRIX_TRANSFORM(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_MATRIX_TRANSFORM_NONE: DML_MATRIX_TRANSFORM = DML_MATRIX_TRANSFORM(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_MATRIX_TRANSFORM_TRANSPOSE: DML_MATRIX_TRANSFORM = DML_MATRIX_TRANSFORM(1i32);
impl ::core::marker::Copy for DML_MATRIX_TRANSFORM {}
impl ::core::clone::Clone for DML_MATRIX_TRANSFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_MATRIX_TRANSFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_MATRIX_TRANSFORM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_MATRIX_TRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_MATRIX_TRANSFORM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_OPERATOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_INVALID: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_IDENTITY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ABS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(2i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ACOS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(3i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ADD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(4i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ASIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(5i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ATAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(6i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_CEIL: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(7i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_CLIP: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(8i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_COS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(9i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_DIVIDE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(10i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_EXP: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(11i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_FLOOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(12i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOG: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(13i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_AND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(14i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_EQUALS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(15i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_GREATER_THAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(16i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_LESS_THAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(17i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_NOT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(18i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_OR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(19i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_XOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(20i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_MAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(21i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_MEAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(22i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_MIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(23i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_MULTIPLY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(24i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_POW: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(25i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_CONSTANT_POW: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(26i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_RECIP: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(27i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_SIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(28i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_SQRT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(29i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_SUBTRACT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(30i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_TAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(31i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_THRESHOLD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(32i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_QUANTIZE_LINEAR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(33i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_DEQUANTIZE_LINEAR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(34i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_ELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(35i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_HARDMAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(36i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_HARD_SIGMOID: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(37i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_IDENTITY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(38i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_LEAKY_RELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(39i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_LINEAR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(40i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_LOG_SOFTMAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(41i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_PARAMETERIZED_RELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(42i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_PARAMETRIC_SOFTPLUS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(43i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_RELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(44i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_SCALED_ELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(45i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_SCALED_TANH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(46i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_SIGMOID: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(47i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_SOFTMAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(48i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_SOFTPLUS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(49i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_SOFTSIGN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(50i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_TANH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(51i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_THRESHOLDED_RELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(52i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_CONVOLUTION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(53i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_GEMM: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(54i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_REDUCE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(55i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_AVERAGE_POOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(56i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_LP_POOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(57i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_MAX_POOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(58i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ROI_POOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(59i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_SLICE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(60i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_CAST: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(61i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_SPLIT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(62i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_JOIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(63i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_PADDING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(64i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_VALUE_SCALE_2D: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(65i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_UPSAMPLE_2D: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(66i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_GATHER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(67i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_SPACE_TO_DEPTH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(68i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_DEPTH_TO_SPACE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(69i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_TILE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(70i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_TOP_K: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(71i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_BATCH_NORMALIZATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(72i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(73i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_LOCAL_RESPONSE_NORMALIZATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(74i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_LP_NORMALIZATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(75i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_RNN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(76i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_LSTM: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(77i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_GRU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(78i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_SIGN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(79i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_IS_NAN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(80i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ERF: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(81i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_SINH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(82i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_COSH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(83i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_TANH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(84i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ASINH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(85i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ACOSH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(86i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ATANH: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(87i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_IF: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(88i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ADD1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(89i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_SHRINK: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(90i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_MAX_POOLING1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(91i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_MAX_UNPOOLING: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(92i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_DIAGONAL_MATRIX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(93i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_SCATTER_ELEMENTS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(94i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_SCATTER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(94i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ONE_HOT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(95i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_RESAMPLE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(96i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_BIT_SHIFT_LEFT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(97i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_BIT_SHIFT_RIGHT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(98i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ROUND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(99i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_IS_INFINITY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(100i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_MODULUS_TRUNCATE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(101i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_MODULUS_FLOOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(102i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_FILL_VALUE_CONSTANT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(103i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_FILL_VALUE_SEQUENCE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(104i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_CUMULATIVE_SUMMATION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(105i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_REVERSE_SUBSEQUENCES: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(106i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_GATHER_ELEMENTS: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(107i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_GATHER_ND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(108i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_SCATTER_ND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(109i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_MAX_POOLING2: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(110i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_SLICE1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(111i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_TOP_K1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(112i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_DEPTH_TO_SPACE1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(113i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_SPACE_TO_DEPTH1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(114i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_MEAN_VARIANCE_NORMALIZATION1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(115i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_RESAMPLE1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(116i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_MATRIX_MULTIPLY_INTEGER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(117i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_QUANTIZED_LINEAR_MATRIX_MULTIPLY: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(118i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_CONVOLUTION_INTEGER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(119i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_QUANTIZED_LINEAR_CONVOLUTION: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(120i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_BIT_AND: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(121i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_BIT_OR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(122i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_BIT_XOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(123i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_BIT_NOT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(124i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_BIT_COUNT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(125i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(126i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(127i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_CELU: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(128i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ACTIVATION_RELU_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(129i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_AVERAGE_POOLING_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(130i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_MAX_POOLING_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(131i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_RANDOM_GENERATOR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(132i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_NONZERO_COORDINATES: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(133i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_RESAMPLE_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(134i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_SLICE_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(135i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ADAM_OPTIMIZER: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(136i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ARGMIN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(137i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ARGMAX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(138i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ROI_ALIGN: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(139i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_GATHER_ND1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(140i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_ATAN_YX: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(141i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_CLIP_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(142i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_DIFFERENCE_SQUARE: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(143i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_LOCAL_RESPONSE_NORMALIZATION_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(144i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_CUMULATIVE_PRODUCT: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(145i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_BATCH_NORMALIZATION_GRAD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(146i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ELEMENT_WISE_QUANTIZED_LINEAR_ADD: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(147i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_DYNAMIC_QUANTIZE_LINEAR: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(148i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_OPERATOR_ROI_ALIGN1: DML_OPERATOR_TYPE = DML_OPERATOR_TYPE(149i32);
impl ::core::marker::Copy for DML_OPERATOR_TYPE {}
impl ::core::clone::Clone for DML_OPERATOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_OPERATOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_OPERATOR_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_OPERATOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_OPERATOR_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_PADDING_MODE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_PADDING_MODE_CONSTANT: DML_PADDING_MODE = DML_PADDING_MODE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_PADDING_MODE_EDGE: DML_PADDING_MODE = DML_PADDING_MODE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_PADDING_MODE_REFLECTION: DML_PADDING_MODE = DML_PADDING_MODE(2i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_PADDING_MODE_SYMMETRIC: DML_PADDING_MODE = DML_PADDING_MODE(3i32);
impl ::core::marker::Copy for DML_PADDING_MODE {}
impl ::core::clone::Clone for DML_PADDING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_PADDING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_PADDING_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_PADDING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_PADDING_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_RANDOM_GENERATOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_RANDOM_GENERATOR_TYPE_PHILOX_4X32_10: DML_RANDOM_GENERATOR_TYPE = DML_RANDOM_GENERATOR_TYPE(0i32);
impl ::core::marker::Copy for DML_RANDOM_GENERATOR_TYPE {}
impl ::core::clone::Clone for DML_RANDOM_GENERATOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_RANDOM_GENERATOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_RANDOM_GENERATOR_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_RANDOM_GENERATOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_RANDOM_GENERATOR_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_RECURRENT_NETWORK_DIRECTION(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_RECURRENT_NETWORK_DIRECTION_FORWARD: DML_RECURRENT_NETWORK_DIRECTION = DML_RECURRENT_NETWORK_DIRECTION(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_RECURRENT_NETWORK_DIRECTION_BACKWARD: DML_RECURRENT_NETWORK_DIRECTION = DML_RECURRENT_NETWORK_DIRECTION(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_RECURRENT_NETWORK_DIRECTION_BIDIRECTIONAL: DML_RECURRENT_NETWORK_DIRECTION = DML_RECURRENT_NETWORK_DIRECTION(2i32);
impl ::core::marker::Copy for DML_RECURRENT_NETWORK_DIRECTION {}
impl ::core::clone::Clone for DML_RECURRENT_NETWORK_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_RECURRENT_NETWORK_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_RECURRENT_NETWORK_DIRECTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_RECURRENT_NETWORK_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_RECURRENT_NETWORK_DIRECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_REDUCE_FUNCTION(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_ARGMAX: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_ARGMIN: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_AVERAGE: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(2i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_L1: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(3i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_L2: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(4i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_LOG_SUM: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(5i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_LOG_SUM_EXP: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(6i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_MAX: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(7i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_MIN: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(8i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_MULTIPLY: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(9i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_SUM: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(10i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_REDUCE_FUNCTION_SUM_SQUARE: DML_REDUCE_FUNCTION = DML_REDUCE_FUNCTION(11i32);
impl ::core::marker::Copy for DML_REDUCE_FUNCTION {}
impl ::core::clone::Clone for DML_REDUCE_FUNCTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_REDUCE_FUNCTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_REDUCE_FUNCTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_REDUCE_FUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_REDUCE_FUNCTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_ROUNDING_MODE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_ROUNDING_MODE_HALVES_TO_NEAREST_EVEN: DML_ROUNDING_MODE = DML_ROUNDING_MODE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_ROUNDING_MODE_TOWARD_ZERO: DML_ROUNDING_MODE = DML_ROUNDING_MODE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_ROUNDING_MODE_TOWARD_INFINITY: DML_ROUNDING_MODE = DML_ROUNDING_MODE(2i32);
impl ::core::marker::Copy for DML_ROUNDING_MODE {}
impl ::core::clone::Clone for DML_ROUNDING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_ROUNDING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_ROUNDING_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_ROUNDING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_ROUNDING_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_TENSOR_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_UNKNOWN: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_FLOAT32: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_FLOAT16: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_UINT32: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_UINT16: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_UINT8: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(5i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_INT32: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(6i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_INT16: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(7i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_INT8: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(8i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_FLOAT64: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(9i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_UINT64: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(10i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_DATA_TYPE_INT64: DML_TENSOR_DATA_TYPE = DML_TENSOR_DATA_TYPE(11i32);
impl ::core::marker::Copy for DML_TENSOR_DATA_TYPE {}
impl ::core::clone::Clone for DML_TENSOR_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_TENSOR_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_TENSOR_DATA_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_TENSOR_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_TENSOR_DATA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_TENSOR_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_FLAG_NONE: DML_TENSOR_FLAGS = DML_TENSOR_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_FLAG_OWNED_BY_DML: DML_TENSOR_FLAGS = DML_TENSOR_FLAGS(1i32);
impl ::core::marker::Copy for DML_TENSOR_FLAGS {}
impl ::core::clone::Clone for DML_TENSOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_TENSOR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_TENSOR_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_TENSOR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_TENSOR_FLAGS").field(&self.0).finish()
    }
}
impl DML_TENSOR_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DML_TENSOR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DML_TENSOR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DML_TENSOR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DML_TENSOR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DML_TENSOR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DML_TENSOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_TYPE_INVALID: DML_TENSOR_TYPE = DML_TENSOR_TYPE(0i32);
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub const DML_TENSOR_TYPE_BUFFER: DML_TENSOR_TYPE = DML_TENSOR_TYPE(1i32);
impl ::core::marker::Copy for DML_TENSOR_TYPE {}
impl ::core::clone::Clone for DML_TENSOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DML_TENSOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DML_TENSOR_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DML_TENSOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DML_TENSOR_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_CELU_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_CELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_CELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_CELU_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_CELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_CELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_ELU_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_ELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_ELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_ELU_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_ELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_ELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_HARDMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_HARDMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_HARD_SIGMOID_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_IDENTITY_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_IDENTITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_LEAKY_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_LOG_SOFTMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub SlopeTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("SlopeTensor", &self.SlopeTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.SlopeTensor == other.SlopeTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_PARAMETERIZED_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_PARAMETRIC_SOFTPLUS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub InputGradientTensor: *const DML_TENSOR_DESC,
    pub OutputGradientTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_RELU_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_RELU_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_RELU_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Gamma: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Gamma", &self.Gamma).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Gamma == other.Gamma
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SCALED_ELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
    pub Beta: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).field("Beta", &self.Beta).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha && self.Beta == other.Beta
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SCALED_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Bias: f32,
    pub Threshold: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SHRINK_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SHRINK_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Bias", &self.Bias).field("Threshold", &self.Threshold).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Bias == other.Bias && self.Threshold == other.Threshold
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SHRINK_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SHRINK_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SIGMOID_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SIGMOID_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SOFTMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SOFTMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Steepness: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Steepness", &self.Steepness).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Steepness == other.Steepness
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SOFTPLUS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_SOFTSIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_TANH_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ACTIVATION_TANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_TANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_TANH_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_TANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Alpha: f32,
}
impl ::core::marker::Copy for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Alpha", &self.Alpha).finish()
    }
}
impl ::windows::core::TypeKind for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {}
impl ::core::default::Default for DML_ACTIVATION_THRESHOLDED_RELU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    pub InputParametersTensor: *const DML_TENSOR_DESC,
    pub InputFirstMomentTensor: *const DML_TENSOR_DESC,
    pub InputSecondMomentTensor: *const DML_TENSOR_DESC,
    pub GradientTensor: *const DML_TENSOR_DESC,
    pub TrainingStepTensor: *const DML_TENSOR_DESC,
    pub OutputParametersTensor: *const DML_TENSOR_DESC,
    pub OutputFirstMomentTensor: *const DML_TENSOR_DESC,
    pub OutputSecondMomentTensor: *const DML_TENSOR_DESC,
    pub LearningRate: f32,
    pub Beta1: f32,
    pub Beta2: f32,
    pub Epsilon: f32,
}
impl ::core::marker::Copy for DML_ADAM_OPTIMIZER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ADAM_OPTIMIZER_OPERATOR_DESC")
            .field("InputParametersTensor", &self.InputParametersTensor)
            .field("InputFirstMomentTensor", &self.InputFirstMomentTensor)
            .field("InputSecondMomentTensor", &self.InputSecondMomentTensor)
            .field("GradientTensor", &self.GradientTensor)
            .field("TrainingStepTensor", &self.TrainingStepTensor)
            .field("OutputParametersTensor", &self.OutputParametersTensor)
            .field("OutputFirstMomentTensor", &self.OutputFirstMomentTensor)
            .field("OutputSecondMomentTensor", &self.OutputSecondMomentTensor)
            .field("LearningRate", &self.LearningRate)
            .field("Beta1", &self.Beta1)
            .field("Beta2", &self.Beta2)
            .field("Epsilon", &self.Epsilon)
            .finish()
    }
}
impl ::windows::core::TypeKind for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputParametersTensor == other.InputParametersTensor && self.InputFirstMomentTensor == other.InputFirstMomentTensor && self.InputSecondMomentTensor == other.InputSecondMomentTensor && self.GradientTensor == other.GradientTensor && self.TrainingStepTensor == other.TrainingStepTensor && self.OutputParametersTensor == other.OutputParametersTensor && self.OutputFirstMomentTensor == other.OutputFirstMomentTensor && self.OutputSecondMomentTensor == other.OutputSecondMomentTensor && self.LearningRate == other.LearningRate && self.Beta1 == other.Beta1 && self.Beta2 == other.Beta2 && self.Epsilon == other.Epsilon
    }
}
impl ::core::cmp::Eq for DML_ADAM_OPTIMIZER_OPERATOR_DESC {}
impl ::core::default::Default for DML_ADAM_OPTIMIZER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ARGMAX_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *const u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl ::core::marker::Copy for DML_ARGMAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ARGMAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ARGMAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ARGMAX_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).field("AxisDirection", &self.AxisDirection).finish()
    }
}
impl ::windows::core::TypeKind for DML_ARGMAX_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ARGMAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes && self.AxisDirection == other.AxisDirection
    }
}
impl ::core::cmp::Eq for DML_ARGMAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ARGMAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ARGMIN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *const u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl ::core::marker::Copy for DML_ARGMIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ARGMIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ARGMIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ARGMIN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).field("AxisDirection", &self.AxisDirection).finish()
    }
}
impl ::windows::core::TypeKind for DML_ARGMIN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ARGMIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes && self.AxisDirection == other.AxisDirection
    }
}
impl ::core::cmp::Eq for DML_ARGMIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ARGMIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *const DML_TENSOR_DESC,
    pub OutputGradientTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub IncludePadding: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC").field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("IncludePadding", &self.IncludePadding).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.IncludePadding == other.IncludePadding
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_AVERAGE_POOLING_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_AVERAGE_POOLING_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub IncludePadding: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_AVERAGE_POOLING_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_AVERAGE_POOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("IncludePadding", &self.IncludePadding).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_AVERAGE_POOLING_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.IncludePadding == other.IncludePadding
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_AVERAGE_POOLING_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_AVERAGE_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub InputGradientTensor: *const DML_TENSOR_DESC,
    pub MeanTensor: *const DML_TENSOR_DESC,
    pub VarianceTensor: *const DML_TENSOR_DESC,
    pub ScaleTensor: *const DML_TENSOR_DESC,
    pub OutputGradientTensor: *const DML_TENSOR_DESC,
    pub OutputScaleGradientTensor: *const DML_TENSOR_DESC,
    pub OutputBiasGradientTensor: *const DML_TENSOR_DESC,
    pub Epsilon: f32,
}
impl ::core::marker::Copy for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("InputGradientTensor", &self.InputGradientTensor)
            .field("MeanTensor", &self.MeanTensor)
            .field("VarianceTensor", &self.VarianceTensor)
            .field("ScaleTensor", &self.ScaleTensor)
            .field("OutputGradientTensor", &self.OutputGradientTensor)
            .field("OutputScaleGradientTensor", &self.OutputScaleGradientTensor)
            .field("OutputBiasGradientTensor", &self.OutputBiasGradientTensor)
            .field("Epsilon", &self.Epsilon)
            .finish()
    }
}
impl ::windows::core::TypeKind for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.MeanTensor == other.MeanTensor && self.VarianceTensor == other.VarianceTensor && self.ScaleTensor == other.ScaleTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.OutputScaleGradientTensor == other.OutputScaleGradientTensor && self.OutputBiasGradientTensor == other.OutputBiasGradientTensor && self.Epsilon == other.Epsilon
    }
}
impl ::core::cmp::Eq for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_BATCH_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub MeanTensor: *const DML_TENSOR_DESC,
    pub VarianceTensor: *const DML_TENSOR_DESC,
    pub ScaleTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Spatial: super::super::super::Foundation::BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_BATCH_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BATCH_NORMALIZATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("MeanTensor", &self.MeanTensor).field("VarianceTensor", &self.VarianceTensor).field("ScaleTensor", &self.ScaleTensor).field("BiasTensor", &self.BiasTensor).field("OutputTensor", &self.OutputTensor).field("Spatial", &self.Spatial).field("Epsilon", &self.Epsilon).field("FusedActivation", &self.FusedActivation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.MeanTensor == other.MeanTensor && self.VarianceTensor == other.VarianceTensor && self.ScaleTensor == other.ScaleTensor && self.BiasTensor == other.BiasTensor && self.OutputTensor == other.OutputTensor && self.Spatial == other.Spatial && self.Epsilon == other.Epsilon && self.FusedActivation == other.FusedActivation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_BATCH_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_BATCH_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_BINDING_DESC {
    pub Type: DML_BINDING_TYPE,
    pub Desc: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_BINDING_DESC {}
impl ::core::clone::Clone for DML_BINDING_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_BINDING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BINDING_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::windows::core::TypeKind for DML_BINDING_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_BINDING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_BINDING_DESC {}
impl ::core::default::Default for DML_BINDING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_BINDING_PROPERTIES {
    pub RequiredDescriptorCount: u32,
    pub TemporaryResourceSize: u64,
    pub PersistentResourceSize: u64,
}
impl ::core::marker::Copy for DML_BINDING_PROPERTIES {}
impl ::core::clone::Clone for DML_BINDING_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_BINDING_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BINDING_PROPERTIES").field("RequiredDescriptorCount", &self.RequiredDescriptorCount).field("TemporaryResourceSize", &self.TemporaryResourceSize).field("PersistentResourceSize", &self.PersistentResourceSize).finish()
    }
}
impl ::windows::core::TypeKind for DML_BINDING_PROPERTIES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_BINDING_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.RequiredDescriptorCount == other.RequiredDescriptorCount && self.TemporaryResourceSize == other.TemporaryResourceSize && self.PersistentResourceSize == other.PersistentResourceSize
    }
}
impl ::core::cmp::Eq for DML_BINDING_PROPERTIES {}
impl ::core::default::Default for DML_BINDING_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct DML_BINDING_TABLE_DESC {
    pub Dispatchable: ::std::mem::ManuallyDrop<::core::option::Option<IDMLDispatchable>>,
    pub CPUDescriptorHandle: super::super::super::Graphics::Direct3D12::D3D12_CPU_DESCRIPTOR_HANDLE,
    pub GPUDescriptorHandle: super::super::super::Graphics::Direct3D12::D3D12_GPU_DESCRIPTOR_HANDLE,
    pub SizeInDescriptors: u32,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for DML_BINDING_TABLE_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for DML_BINDING_TABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BINDING_TABLE_DESC").field("Dispatchable", &self.Dispatchable).field("CPUDescriptorHandle", &self.CPUDescriptorHandle).field("GPUDescriptorHandle", &self.GPUDescriptorHandle).field("SizeInDescriptors", &self.SizeInDescriptors).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::TypeKind for DML_BINDING_TABLE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for DML_BINDING_TABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Dispatchable == other.Dispatchable && self.CPUDescriptorHandle == other.CPUDescriptorHandle && self.GPUDescriptorHandle == other.GPUDescriptorHandle && self.SizeInDescriptors == other.SizeInDescriptors
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for DML_BINDING_TABLE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BINDING_TABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct DML_BUFFER_ARRAY_BINDING {
    pub BindingCount: u32,
    pub Bindings: *const DML_BUFFER_BINDING,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::marker::Copy for DML_BUFFER_ARRAY_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for DML_BUFFER_ARRAY_BINDING {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for DML_BUFFER_ARRAY_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BUFFER_ARRAY_BINDING").field("BindingCount", &self.BindingCount).field("Bindings", &self.Bindings).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::TypeKind for DML_BUFFER_ARRAY_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for DML_BUFFER_ARRAY_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.BindingCount == other.BindingCount && self.Bindings == other.Bindings
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for DML_BUFFER_ARRAY_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BUFFER_ARRAY_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Graphics_Direct3D12\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D12")]
pub struct DML_BUFFER_BINDING {
    pub Buffer: ::std::mem::ManuallyDrop<::core::option::Option<super::super::super::Graphics::Direct3D12::ID3D12Resource>>,
    pub Offset: u64,
    pub SizeInBytes: u64,
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::clone::Clone for DML_BUFFER_BINDING {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for DML_BUFFER_BINDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BUFFER_BINDING").field("Buffer", &self.Buffer).field("Offset", &self.Offset).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::windows::core::TypeKind for DML_BUFFER_BINDING {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for DML_BUFFER_BINDING {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.Offset == other.Offset && self.SizeInBytes == other.SizeInBytes
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for DML_BUFFER_BINDING {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for DML_BUFFER_BINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_BUFFER_TENSOR_DESC {
    pub DataType: DML_TENSOR_DATA_TYPE,
    pub Flags: DML_TENSOR_FLAGS,
    pub DimensionCount: u32,
    pub Sizes: *const u32,
    pub Strides: *const u32,
    pub TotalTensorSizeInBytes: u64,
    pub GuaranteedBaseOffsetAlignment: u32,
}
impl ::core::marker::Copy for DML_BUFFER_TENSOR_DESC {}
impl ::core::clone::Clone for DML_BUFFER_TENSOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_BUFFER_TENSOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_BUFFER_TENSOR_DESC").field("DataType", &self.DataType).field("Flags", &self.Flags).field("DimensionCount", &self.DimensionCount).field("Sizes", &self.Sizes).field("Strides", &self.Strides).field("TotalTensorSizeInBytes", &self.TotalTensorSizeInBytes).field("GuaranteedBaseOffsetAlignment", &self.GuaranteedBaseOffsetAlignment).finish()
    }
}
impl ::windows::core::TypeKind for DML_BUFFER_TENSOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_BUFFER_TENSOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType && self.Flags == other.Flags && self.DimensionCount == other.DimensionCount && self.Sizes == other.Sizes && self.Strides == other.Strides && self.TotalTensorSizeInBytes == other.TotalTensorSizeInBytes && self.GuaranteedBaseOffsetAlignment == other.GuaranteedBaseOffsetAlignment
    }
}
impl ::core::cmp::Eq for DML_BUFFER_TENSOR_DESC {}
impl ::core::default::Default for DML_BUFFER_TENSOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_CAST_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_CAST_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_CAST_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_CAST_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CAST_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_CAST_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_CAST_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_CAST_OPERATOR_DESC {}
impl ::core::default::Default for DML_CAST_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub InputZeroPointTensor: *const DML_TENSOR_DESC,
    pub FilterTensor: *const DML_TENSOR_DESC,
    pub FilterZeroPointTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub Dilations: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub GroupCount: u32,
}
impl ::core::marker::Copy for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CONVOLUTION_INTEGER_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("InputZeroPointTensor", &self.InputZeroPointTensor)
            .field("FilterTensor", &self.FilterTensor)
            .field("FilterZeroPointTensor", &self.FilterZeroPointTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("Dilations", &self.Dilations)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("GroupCount", &self.GroupCount)
            .finish()
    }
}
impl ::windows::core::TypeKind for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputZeroPointTensor == other.InputZeroPointTensor && self.FilterTensor == other.FilterTensor && self.FilterZeroPointTensor == other.FilterZeroPointTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.Dilations == other.Dilations && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.GroupCount == other.GroupCount
    }
}
impl ::core::cmp::Eq for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {}
impl ::core::default::Default for DML_CONVOLUTION_INTEGER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_CONVOLUTION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub FilterTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Mode: DML_CONVOLUTION_MODE,
    pub Direction: DML_CONVOLUTION_DIRECTION,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub Dilations: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub OutputPadding: *const u32,
    pub GroupCount: u32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}
impl ::core::marker::Copy for DML_CONVOLUTION_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_CONVOLUTION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_CONVOLUTION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CONVOLUTION_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("FilterTensor", &self.FilterTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("Mode", &self.Mode)
            .field("Direction", &self.Direction)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("Dilations", &self.Dilations)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("OutputPadding", &self.OutputPadding)
            .field("GroupCount", &self.GroupCount)
            .field("FusedActivation", &self.FusedActivation)
            .finish()
    }
}
impl ::windows::core::TypeKind for DML_CONVOLUTION_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_CONVOLUTION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.FilterTensor == other.FilterTensor && self.BiasTensor == other.BiasTensor && self.OutputTensor == other.OutputTensor && self.Mode == other.Mode && self.Direction == other.Direction && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.Dilations == other.Dilations && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.OutputPadding == other.OutputPadding && self.GroupCount == other.GroupCount && self.FusedActivation == other.FusedActivation
    }
}
impl ::core::cmp::Eq for DML_CONVOLUTION_OPERATOR_DESC {}
impl ::core::default::Default for DML_CONVOLUTION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
    pub HasExclusiveProduct: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CUMULATIVE_PRODUCT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("AxisDirection", &self.AxisDirection).field("HasExclusiveProduct", &self.HasExclusiveProduct).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.AxisDirection == other.AxisDirection && self.HasExclusiveProduct == other.HasExclusiveProduct
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_CUMULATIVE_PRODUCT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
    pub HasExclusiveSum: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_CUMULATIVE_SUMMATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("AxisDirection", &self.AxisDirection).field("HasExclusiveSum", &self.HasExclusiveSum).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.AxisDirection == other.AxisDirection && self.HasExclusiveSum == other.HasExclusiveSum
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_CUMULATIVE_SUMMATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub BlockSize: u32,
    pub Order: DML_DEPTH_SPACE_ORDER,
}
impl ::core::marker::Copy for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_DEPTH_TO_SPACE1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).field("Order", &self.Order).finish()
    }
}
impl ::windows::core::TypeKind for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize && self.Order == other.Order
    }
}
impl ::core::cmp::Eq for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {}
impl ::core::default::Default for DML_DEPTH_TO_SPACE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub BlockSize: u32,
}
impl ::core::marker::Copy for DML_DEPTH_TO_SPACE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_DEPTH_TO_SPACE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).finish()
    }
}
impl ::windows::core::TypeKind for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize
    }
}
impl ::core::cmp::Eq for DML_DEPTH_TO_SPACE_OPERATOR_DESC {}
impl ::core::default::Default for DML_DEPTH_TO_SPACE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Offset: i32,
    pub Value: f32,
}
impl ::core::marker::Copy for DML_DIAGONAL_MATRIX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_DIAGONAL_MATRIX_OPERATOR_DESC").field("OutputTensor", &self.OutputTensor).field("Offset", &self.Offset).field("Value", &self.Value).finish()
    }
}
impl ::windows::core::TypeKind for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.OutputTensor == other.OutputTensor && self.Offset == other.Offset && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DML_DIAGONAL_MATRIX_OPERATOR_DESC {}
impl ::core::default::Default for DML_DIAGONAL_MATRIX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub OutputScaleTensor: *const DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("OutputScaleTensor", &self.OutputScaleTensor).field("OutputZeroPointTensor", &self.OutputZeroPointTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor
    }
}
impl ::core::cmp::Eq for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_DYNAMIC_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ABS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ABS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ACOSH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ACOS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ACOS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ADD1_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).field("FusedActivation", &self.FusedActivation).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor && self.FusedActivation == other.FusedActivation
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ADD1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ADD_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ADD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ASINH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ASINH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ASIN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ASIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ATANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ATANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ATAN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ATAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ATAN_YX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_AND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_COUNT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_NOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_OR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_SHIFT_LEFT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_SHIFT_RIGHT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_BIT_XOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_CEIL_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CEIL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub InputGradientTensor: *const DML_TENSOR_DESC,
    pub OutputGradientTensor: *const DML_TENSOR_DESC,
    pub Min: f32,
    pub Max: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.Min == other.Min && self.Max == other.Max
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CLIP_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
    pub Min: f32,
    pub Max: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_CLIP_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias && self.Min == other.Min && self.Max == other.Max
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CLIP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
    pub Exponent: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).field("Exponent", &self.Exponent).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias && self.Exponent == other.Exponent
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_CONSTANT_POW_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_COSH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_COSH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_COS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_COS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_COS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_COS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ScaleTensor: *const DML_TENSOR_DESC,
    pub ZeroPointTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("ZeroPointTensor", &self.ZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.ZeroPointTensor == other.ZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_DEQUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_DIFFERENCE_SQUARE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_DIVIDE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ERF_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ERF_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_EXP_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_EXP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_FLOOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IDENTITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    pub ConditionTensor: *const DML_TENSOR_DESC,
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IF_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_IF_OPERATOR_DESC").field("ConditionTensor", &self.ConditionTensor).field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ConditionTensor == other.ConditionTensor && self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IF_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IF_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub InfinityMode: DML_IS_INFINITY_MODE,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("InfinityMode", &self.InfinityMode).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.InfinityMode == other.InfinityMode
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IS_INFINITY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_IS_NAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_AND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_EQUALS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_GREATER_THAN_OR_EQUAL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_LESS_THAN_OR_EQUAL_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_NOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_OR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOGICAL_XOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_LOG_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_LOG_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MAX_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MAX_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MEAN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MEAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MIN_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MODULUS_FLOOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MODULUS_TRUNCATE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_MULTIPLY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ExponentTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_POW_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_POW_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ExponentTensor", &self.ExponentTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ExponentTensor == other.ExponentTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_POW_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_POW_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub AScaleTensor: *const DML_TENSOR_DESC,
    pub AZeroPointTensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub BScaleTensor: *const DML_TENSOR_DESC,
    pub BZeroPointTensor: *const DML_TENSOR_DESC,
    pub OutputScaleTensor: *const DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC").field("ATensor", &self.ATensor).field("AScaleTensor", &self.AScaleTensor).field("AZeroPointTensor", &self.AZeroPointTensor).field("BTensor", &self.BTensor).field("BScaleTensor", &self.BScaleTensor).field("BZeroPointTensor", &self.BZeroPointTensor).field("OutputScaleTensor", &self.OutputScaleTensor).field("OutputZeroPointTensor", &self.OutputZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.AScaleTensor == other.AScaleTensor && self.AZeroPointTensor == other.AZeroPointTensor && self.BTensor == other.BTensor && self.BScaleTensor == other.BScaleTensor && self.BZeroPointTensor == other.BZeroPointTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_QUANTIZED_LINEAR_ADD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ScaleTensor: *const DML_TENSOR_DESC,
    pub ZeroPointTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("ZeroPointTensor", &self.ZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.ZeroPointTensor == other.ZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_QUANTIZE_LINEAR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_RECIP_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_RECIP_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub RoundingMode: DML_ROUNDING_MODE,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_ROUND_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("RoundingMode", &self.RoundingMode).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.RoundingMode == other.RoundingMode
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_ROUND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SIGN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SINH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SINH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SIN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SQRT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SQRT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_SUBTRACT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_TANH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_TANH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_TAN_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_TAN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleBias: *const DML_SCALE_BIAS,
    pub Min: f32,
}
impl ::core::marker::Copy for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleBias", &self.ScaleBias).field("Min", &self.Min).finish()
    }
}
impl ::windows::core::TypeKind for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleBias == other.ScaleBias && self.Min == other.Min
    }
}
impl ::core::cmp::Eq for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {}
impl ::core::default::Default for DML_ELEMENT_WISE_THRESHOLD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_FEATURE_DATA_FEATURE_LEVELS {
    pub MaxSupportedFeatureLevel: DML_FEATURE_LEVEL,
}
impl ::core::marker::Copy for DML_FEATURE_DATA_FEATURE_LEVELS {}
impl ::core::clone::Clone for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_FEATURE_DATA_FEATURE_LEVELS").field("MaxSupportedFeatureLevel", &self.MaxSupportedFeatureLevel).finish()
    }
}
impl ::windows::core::TypeKind for DML_FEATURE_DATA_FEATURE_LEVELS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.MaxSupportedFeatureLevel == other.MaxSupportedFeatureLevel
    }
}
impl ::core::cmp::Eq for DML_FEATURE_DATA_FEATURE_LEVELS {}
impl ::core::default::Default for DML_FEATURE_DATA_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    pub IsSupported: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT").field("IsSupported", &self.IsSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.IsSupported == other.IsSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_FEATURE_DATA_TENSOR_DATA_TYPE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_FEATURE_QUERY_FEATURE_LEVELS {
    pub RequestedFeatureLevelCount: u32,
    pub RequestedFeatureLevels: *const DML_FEATURE_LEVEL,
}
impl ::core::marker::Copy for DML_FEATURE_QUERY_FEATURE_LEVELS {}
impl ::core::clone::Clone for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_FEATURE_QUERY_FEATURE_LEVELS").field("RequestedFeatureLevelCount", &self.RequestedFeatureLevelCount).field("RequestedFeatureLevels", &self.RequestedFeatureLevels).finish()
    }
}
impl ::windows::core::TypeKind for DML_FEATURE_QUERY_FEATURE_LEVELS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.RequestedFeatureLevelCount == other.RequestedFeatureLevelCount && self.RequestedFeatureLevels == other.RequestedFeatureLevels
    }
}
impl ::core::cmp::Eq for DML_FEATURE_QUERY_FEATURE_LEVELS {}
impl ::core::default::Default for DML_FEATURE_QUERY_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    pub DataType: DML_TENSOR_DATA_TYPE,
}
impl ::core::marker::Copy for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {}
impl ::core::clone::Clone for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT").field("DataType", &self.DataType).finish()
    }
}
impl ::windows::core::TypeKind for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.DataType == other.DataType
    }
}
impl ::core::cmp::Eq for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {}
impl ::core::default::Default for DML_FEATURE_QUERY_TENSOR_DATA_TYPE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ValueDataType: DML_TENSOR_DATA_TYPE,
    pub Value: DML_SCALAR_UNION,
}
impl ::core::marker::Copy for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DML_FILL_VALUE_CONSTANT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ValueDataType: DML_TENSOR_DATA_TYPE,
    pub ValueStart: DML_SCALAR_UNION,
    pub ValueDelta: DML_SCALAR_UNION,
}
impl ::core::marker::Copy for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DML_FILL_VALUE_SEQUENCE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_GATHER_ELEMENTS_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub IndicesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_GATHER_ELEMENTS_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GATHER_ELEMENTS_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::windows::core::TypeKind for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_GATHER_ELEMENTS_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_ELEMENTS_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_GATHER_ND1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub IndicesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
    pub BatchDimensionCount: u32,
}
impl ::core::marker::Copy for DML_GATHER_ND1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_ND1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_GATHER_ND1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GATHER_ND1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("InputDimensionCount", &self.InputDimensionCount).field("IndicesDimensionCount", &self.IndicesDimensionCount).field("BatchDimensionCount", &self.BatchDimensionCount).finish()
    }
}
impl ::windows::core::TypeKind for DML_GATHER_ND1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_GATHER_ND1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.InputDimensionCount == other.InputDimensionCount && self.IndicesDimensionCount == other.IndicesDimensionCount && self.BatchDimensionCount == other.BatchDimensionCount
    }
}
impl ::core::cmp::Eq for DML_GATHER_ND1_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_ND1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_GATHER_ND_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub IndicesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
}
impl ::core::marker::Copy for DML_GATHER_ND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_ND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_GATHER_ND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GATHER_ND_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("InputDimensionCount", &self.InputDimensionCount).field("IndicesDimensionCount", &self.IndicesDimensionCount).finish()
    }
}
impl ::windows::core::TypeKind for DML_GATHER_ND_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_GATHER_ND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.InputDimensionCount == other.InputDimensionCount && self.IndicesDimensionCount == other.IndicesDimensionCount
    }
}
impl ::core::cmp::Eq for DML_GATHER_ND_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_ND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_GATHER_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub IndicesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
    pub IndexDimensions: u32,
}
impl ::core::marker::Copy for DML_GATHER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GATHER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_GATHER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GATHER_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("IndexDimensions", &self.IndexDimensions).finish()
    }
}
impl ::windows::core::TypeKind for DML_GATHER_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_GATHER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.IndexDimensions == other.IndexDimensions
    }
}
impl ::core::cmp::Eq for DML_GATHER_OPERATOR_DESC {}
impl ::core::default::Default for DML_GATHER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_GEMM_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub CTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub TransA: DML_MATRIX_TRANSFORM,
    pub TransB: DML_MATRIX_TRANSFORM,
    pub Alpha: f32,
    pub Beta: f32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}
impl ::core::marker::Copy for DML_GEMM_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_GEMM_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_GEMM_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GEMM_OPERATOR_DESC").field("ATensor", &self.ATensor).field("BTensor", &self.BTensor).field("CTensor", &self.CTensor).field("OutputTensor", &self.OutputTensor).field("TransA", &self.TransA).field("TransB", &self.TransB).field("Alpha", &self.Alpha).field("Beta", &self.Beta).field("FusedActivation", &self.FusedActivation).finish()
    }
}
impl ::windows::core::TypeKind for DML_GEMM_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_GEMM_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.BTensor == other.BTensor && self.CTensor == other.CTensor && self.OutputTensor == other.OutputTensor && self.TransA == other.TransA && self.TransB == other.TransB && self.Alpha == other.Alpha && self.Beta == other.Beta && self.FusedActivation == other.FusedActivation
    }
}
impl ::core::cmp::Eq for DML_GEMM_OPERATOR_DESC {}
impl ::core::default::Default for DML_GEMM_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_GRAPH_DESC {
    pub InputCount: u32,
    pub OutputCount: u32,
    pub NodeCount: u32,
    pub Nodes: *const DML_GRAPH_NODE_DESC,
    pub InputEdgeCount: u32,
    pub InputEdges: *const DML_GRAPH_EDGE_DESC,
    pub OutputEdgeCount: u32,
    pub OutputEdges: *const DML_GRAPH_EDGE_DESC,
    pub IntermediateEdgeCount: u32,
    pub IntermediateEdges: *const DML_GRAPH_EDGE_DESC,
}
impl ::core::marker::Copy for DML_GRAPH_DESC {}
impl ::core::clone::Clone for DML_GRAPH_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_GRAPH_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GRAPH_DESC").field("InputCount", &self.InputCount).field("OutputCount", &self.OutputCount).field("NodeCount", &self.NodeCount).field("Nodes", &self.Nodes).field("InputEdgeCount", &self.InputEdgeCount).field("InputEdges", &self.InputEdges).field("OutputEdgeCount", &self.OutputEdgeCount).field("OutputEdges", &self.OutputEdges).field("IntermediateEdgeCount", &self.IntermediateEdgeCount).field("IntermediateEdges", &self.IntermediateEdges).finish()
    }
}
impl ::windows::core::TypeKind for DML_GRAPH_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_GRAPH_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputCount == other.InputCount && self.OutputCount == other.OutputCount && self.NodeCount == other.NodeCount && self.Nodes == other.Nodes && self.InputEdgeCount == other.InputEdgeCount && self.InputEdges == other.InputEdges && self.OutputEdgeCount == other.OutputEdgeCount && self.OutputEdges == other.OutputEdges && self.IntermediateEdgeCount == other.IntermediateEdgeCount && self.IntermediateEdges == other.IntermediateEdges
    }
}
impl ::core::cmp::Eq for DML_GRAPH_DESC {}
impl ::core::default::Default for DML_GRAPH_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_GRAPH_EDGE_DESC {
    pub Type: DML_GRAPH_EDGE_TYPE,
    pub Desc: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_GRAPH_EDGE_DESC {}
impl ::core::clone::Clone for DML_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_GRAPH_EDGE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GRAPH_EDGE_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::windows::core::TypeKind for DML_GRAPH_EDGE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_GRAPH_EDGE_DESC {}
impl ::core::default::Default for DML_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_GRAPH_NODE_DESC {
    pub Type: DML_GRAPH_NODE_TYPE,
    pub Desc: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_GRAPH_NODE_DESC {}
impl ::core::clone::Clone for DML_GRAPH_NODE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_GRAPH_NODE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GRAPH_NODE_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::windows::core::TypeKind for DML_GRAPH_NODE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_GRAPH_NODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_GRAPH_NODE_DESC {}
impl ::core::default::Default for DML_GRAPH_NODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_GRU_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub WeightTensor: *const DML_TENSOR_DESC,
    pub RecurrenceTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub HiddenInitTensor: *const DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *const DML_TENSOR_DESC,
    pub OutputSequenceTensor: *const DML_TENSOR_DESC,
    pub OutputSingleTensor: *const DML_TENSOR_DESC,
    pub ActivationDescCount: u32,
    pub ActivationDescs: *const DML_OPERATOR_DESC,
    pub Direction: DML_RECURRENT_NETWORK_DIRECTION,
    pub LinearBeforeReset: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_GRU_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_GRU_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_GRU_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_GRU_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("WeightTensor", &self.WeightTensor)
            .field("RecurrenceTensor", &self.RecurrenceTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("HiddenInitTensor", &self.HiddenInitTensor)
            .field("SequenceLengthsTensor", &self.SequenceLengthsTensor)
            .field("OutputSequenceTensor", &self.OutputSequenceTensor)
            .field("OutputSingleTensor", &self.OutputSingleTensor)
            .field("ActivationDescCount", &self.ActivationDescCount)
            .field("ActivationDescs", &self.ActivationDescs)
            .field("Direction", &self.Direction)
            .field("LinearBeforeReset", &self.LinearBeforeReset)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_GRU_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_GRU_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.WeightTensor == other.WeightTensor && self.RecurrenceTensor == other.RecurrenceTensor && self.BiasTensor == other.BiasTensor && self.HiddenInitTensor == other.HiddenInitTensor && self.SequenceLengthsTensor == other.SequenceLengthsTensor && self.OutputSequenceTensor == other.OutputSequenceTensor && self.OutputSingleTensor == other.OutputSingleTensor && self.ActivationDescCount == other.ActivationDescCount && self.ActivationDescs == other.ActivationDescs && self.Direction == other.Direction && self.LinearBeforeReset == other.LinearBeforeReset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_GRU_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_GRU_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_INPUT_GRAPH_EDGE_DESC {
    pub GraphInputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub Name: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for DML_INPUT_GRAPH_EDGE_DESC {}
impl ::core::clone::Clone for DML_INPUT_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_INPUT_GRAPH_EDGE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_INPUT_GRAPH_EDGE_DESC").field("GraphInputIndex", &self.GraphInputIndex).field("ToNodeIndex", &self.ToNodeIndex).field("ToNodeInputIndex", &self.ToNodeInputIndex).field("Name", &self.Name).finish()
    }
}
impl ::windows::core::TypeKind for DML_INPUT_GRAPH_EDGE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_INPUT_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.GraphInputIndex == other.GraphInputIndex && self.ToNodeIndex == other.ToNodeIndex && self.ToNodeInputIndex == other.ToNodeInputIndex && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for DML_INPUT_GRAPH_EDGE_DESC {}
impl ::core::default::Default for DML_INPUT_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub ToNodeIndex: u32,
    pub ToNodeInputIndex: u32,
    pub Name: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for DML_INTERMEDIATE_GRAPH_EDGE_DESC {}
impl ::core::clone::Clone for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_INTERMEDIATE_GRAPH_EDGE_DESC").field("FromNodeIndex", &self.FromNodeIndex).field("FromNodeOutputIndex", &self.FromNodeOutputIndex).field("ToNodeIndex", &self.ToNodeIndex).field("ToNodeInputIndex", &self.ToNodeInputIndex).field("Name", &self.Name).finish()
    }
}
impl ::windows::core::TypeKind for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.FromNodeIndex == other.FromNodeIndex && self.FromNodeOutputIndex == other.FromNodeOutputIndex && self.ToNodeIndex == other.ToNodeIndex && self.ToNodeInputIndex == other.ToNodeInputIndex && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for DML_INTERMEDIATE_GRAPH_EDGE_DESC {}
impl ::core::default::Default for DML_INTERMEDIATE_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_JOIN_OPERATOR_DESC {
    pub InputCount: u32,
    pub InputTensors: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_JOIN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_JOIN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_JOIN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_JOIN_OPERATOR_DESC").field("InputCount", &self.InputCount).field("InputTensors", &self.InputTensors).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::windows::core::TypeKind for DML_JOIN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_JOIN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputCount == other.InputCount && self.InputTensors == other.InputTensors && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_JOIN_OPERATOR_DESC {}
impl ::core::default::Default for DML_JOIN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub InputGradientTensor: *const DML_TENSOR_DESC,
    pub OutputGradientTensor: *const DML_TENSOR_DESC,
    pub CrossChannel: super::super::super::Foundation::BOOL,
    pub LocalSize: u32,
    pub Alpha: f32,
    pub Beta: f32,
    pub Bias: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("CrossChannel", &self.CrossChannel).field("LocalSize", &self.LocalSize).field("Alpha", &self.Alpha).field("Beta", &self.Beta).field("Bias", &self.Bias).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.CrossChannel == other.CrossChannel && self.LocalSize == other.LocalSize && self.Alpha == other.Alpha && self.Beta == other.Beta && self.Bias == other.Bias
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LOCAL_RESPONSE_NORMALIZATION_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub CrossChannel: super::super::super::Foundation::BOOL,
    pub LocalSize: u32,
    pub Alpha: f32,
    pub Beta: f32,
    pub Bias: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("CrossChannel", &self.CrossChannel).field("LocalSize", &self.LocalSize).field("Alpha", &self.Alpha).field("Beta", &self.Beta).field("Bias", &self.Bias).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.CrossChannel == other.CrossChannel && self.LocalSize == other.LocalSize && self.Alpha == other.Alpha && self.Beta == other.Beta && self.Bias == other.Bias
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LOCAL_RESPONSE_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_LP_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
    pub Epsilon: f32,
    pub P: u32,
}
impl ::core::marker::Copy for DML_LP_NORMALIZATION_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LP_NORMALIZATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).field("Epsilon", &self.Epsilon).field("P", &self.P).finish()
    }
}
impl ::windows::core::TypeKind for DML_LP_NORMALIZATION_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis && self.Epsilon == other.Epsilon && self.P == other.P
    }
}
impl ::core::cmp::Eq for DML_LP_NORMALIZATION_OPERATOR_DESC {}
impl ::core::default::Default for DML_LP_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_LP_POOLING_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub P: u32,
}
impl ::core::marker::Copy for DML_LP_POOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_LP_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_LP_POOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LP_POOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("P", &self.P).finish()
    }
}
impl ::windows::core::TypeKind for DML_LP_POOLING_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_LP_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.P == other.P
    }
}
impl ::core::cmp::Eq for DML_LP_POOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_LP_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_LSTM_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub WeightTensor: *const DML_TENSOR_DESC,
    pub RecurrenceTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub HiddenInitTensor: *const DML_TENSOR_DESC,
    pub CellMemInitTensor: *const DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *const DML_TENSOR_DESC,
    pub PeepholeTensor: *const DML_TENSOR_DESC,
    pub OutputSequenceTensor: *const DML_TENSOR_DESC,
    pub OutputSingleTensor: *const DML_TENSOR_DESC,
    pub OutputCellSingleTensor: *const DML_TENSOR_DESC,
    pub ActivationDescCount: u32,
    pub ActivationDescs: *const DML_OPERATOR_DESC,
    pub Direction: DML_RECURRENT_NETWORK_DIRECTION,
    pub ClipThreshold: f32,
    pub UseClipThreshold: super::super::super::Foundation::BOOL,
    pub CoupleInputForget: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_LSTM_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_LSTM_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_LSTM_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_LSTM_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("WeightTensor", &self.WeightTensor)
            .field("RecurrenceTensor", &self.RecurrenceTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("HiddenInitTensor", &self.HiddenInitTensor)
            .field("CellMemInitTensor", &self.CellMemInitTensor)
            .field("SequenceLengthsTensor", &self.SequenceLengthsTensor)
            .field("PeepholeTensor", &self.PeepholeTensor)
            .field("OutputSequenceTensor", &self.OutputSequenceTensor)
            .field("OutputSingleTensor", &self.OutputSingleTensor)
            .field("OutputCellSingleTensor", &self.OutputCellSingleTensor)
            .field("ActivationDescCount", &self.ActivationDescCount)
            .field("ActivationDescs", &self.ActivationDescs)
            .field("Direction", &self.Direction)
            .field("ClipThreshold", &self.ClipThreshold)
            .field("UseClipThreshold", &self.UseClipThreshold)
            .field("CoupleInputForget", &self.CoupleInputForget)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_LSTM_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_LSTM_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor
            && self.WeightTensor == other.WeightTensor
            && self.RecurrenceTensor == other.RecurrenceTensor
            && self.BiasTensor == other.BiasTensor
            && self.HiddenInitTensor == other.HiddenInitTensor
            && self.CellMemInitTensor == other.CellMemInitTensor
            && self.SequenceLengthsTensor == other.SequenceLengthsTensor
            && self.PeepholeTensor == other.PeepholeTensor
            && self.OutputSequenceTensor == other.OutputSequenceTensor
            && self.OutputSingleTensor == other.OutputSingleTensor
            && self.OutputCellSingleTensor == other.OutputCellSingleTensor
            && self.ActivationDescCount == other.ActivationDescCount
            && self.ActivationDescs == other.ActivationDescs
            && self.Direction == other.Direction
            && self.ClipThreshold == other.ClipThreshold
            && self.UseClipThreshold == other.UseClipThreshold
            && self.CoupleInputForget == other.CoupleInputForget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_LSTM_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_LSTM_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub AZeroPointTensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub BZeroPointTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC").field("ATensor", &self.ATensor).field("AZeroPointTensor", &self.AZeroPointTensor).field("BTensor", &self.BTensor).field("BZeroPointTensor", &self.BZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.AZeroPointTensor == other.AZeroPointTensor && self.BTensor == other.BTensor && self.BZeroPointTensor == other.BZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {}
impl ::core::default::Default for DML_MATRIX_MULTIPLY_INTEGER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_MAX_POOLING1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub OutputIndicesTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
}
impl ::core::marker::Copy for DML_MAX_POOLING1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_MAX_POOLING1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_POOLING1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("OutputIndicesTensor", &self.OutputIndicesTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).finish()
    }
}
impl ::windows::core::TypeKind for DML_MAX_POOLING1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.OutputIndicesTensor == other.OutputIndicesTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING1_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_MAX_POOLING2_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub OutputIndicesTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub Dilations: *const u32,
}
impl ::core::marker::Copy for DML_MAX_POOLING2_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING2_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_MAX_POOLING2_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_POOLING2_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("OutputIndicesTensor", &self.OutputIndicesTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("Dilations", &self.Dilations).finish()
    }
}
impl ::windows::core::TypeKind for DML_MAX_POOLING2_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING2_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.OutputIndicesTensor == other.OutputIndicesTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.Dilations == other.Dilations
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING2_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING2_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub InputGradientTensor: *const DML_TENSOR_DESC,
    pub OutputGradientTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub Dilations: *const u32,
}
impl ::core::marker::Copy for DML_MAX_POOLING_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_POOLING_GRAD_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).field("Dilations", &self.Dilations).finish()
    }
}
impl ::windows::core::TypeKind for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.Dilations == other.Dilations
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_MAX_POOLING_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub WindowSize: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
}
impl ::core::marker::Copy for DML_MAX_POOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_MAX_POOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_POOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("Strides", &self.Strides).field("WindowSize", &self.WindowSize).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).finish()
    }
}
impl ::windows::core::TypeKind for DML_MAX_POOLING_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_MAX_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.WindowSize == other.WindowSize && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding
    }
}
impl ::core::cmp::Eq for DML_MAX_POOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_MAX_UNPOOLING_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub IndicesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_MAX_UNPOOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MAX_UNPOOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_MAX_UNPOOLING_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_MAX_UNPOOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_MAX_UNPOOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ScaleTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *const u32,
    pub NormalizeVariance: super::super::super::Foundation::BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("BiasTensor", &self.BiasTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).field("NormalizeVariance", &self.NormalizeVariance).field("Epsilon", &self.Epsilon).field("FusedActivation", &self.FusedActivation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.BiasTensor == other.BiasTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes && self.NormalizeVariance == other.NormalizeVariance && self.Epsilon == other.Epsilon && self.FusedActivation == other.FusedActivation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_MEAN_VARIANCE_NORMALIZATION1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ScaleTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub CrossChannel: super::super::super::Foundation::BOOL,
    pub NormalizeVariance: super::super::super::Foundation::BOOL,
    pub Epsilon: f32,
    pub FusedActivation: *const DML_OPERATOR_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ScaleTensor", &self.ScaleTensor).field("BiasTensor", &self.BiasTensor).field("OutputTensor", &self.OutputTensor).field("CrossChannel", &self.CrossChannel).field("NormalizeVariance", &self.NormalizeVariance).field("Epsilon", &self.Epsilon).field("FusedActivation", &self.FusedActivation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ScaleTensor == other.ScaleTensor && self.BiasTensor == other.BiasTensor && self.OutputTensor == other.OutputTensor && self.CrossChannel == other.CrossChannel && self.NormalizeVariance == other.NormalizeVariance && self.Epsilon == other.Epsilon && self.FusedActivation == other.FusedActivation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_MEAN_VARIANCE_NORMALIZATION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_NONZERO_COORDINATES_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputCountTensor: *const DML_TENSOR_DESC,
    pub OutputCoordinatesTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_NONZERO_COORDINATES_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_NONZERO_COORDINATES_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputCountTensor", &self.OutputCountTensor).field("OutputCoordinatesTensor", &self.OutputCoordinatesTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputCountTensor == other.OutputCountTensor && self.OutputCoordinatesTensor == other.OutputCoordinatesTensor
    }
}
impl ::core::cmp::Eq for DML_NONZERO_COORDINATES_OPERATOR_DESC {}
impl ::core::default::Default for DML_NONZERO_COORDINATES_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ONE_HOT_OPERATOR_DESC {
    pub IndicesTensor: *const DML_TENSOR_DESC,
    pub ValuesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_ONE_HOT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ONE_HOT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ONE_HOT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ONE_HOT_OPERATOR_DESC").field("IndicesTensor", &self.IndicesTensor).field("ValuesTensor", &self.ValuesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::windows::core::TypeKind for DML_ONE_HOT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ONE_HOT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.IndicesTensor == other.IndicesTensor && self.ValuesTensor == other.ValuesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_ONE_HOT_OPERATOR_DESC {}
impl ::core::default::Default for DML_ONE_HOT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_OPERATOR_DESC {
    pub Type: DML_OPERATOR_TYPE,
    pub Desc: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_OPERATOR_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::windows::core::TypeKind for DML_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_OPERATOR_DESC {}
impl ::core::default::Default for DML_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_OPERATOR_GRAPH_NODE_DESC {
    pub Operator: ::std::mem::ManuallyDrop<::core::option::Option<IDMLOperator>>,
    pub Name: ::windows::core::PCSTR,
}
impl ::core::clone::Clone for DML_OPERATOR_GRAPH_NODE_DESC {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for DML_OPERATOR_GRAPH_NODE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_OPERATOR_GRAPH_NODE_DESC").field("Operator", &self.Operator).field("Name", &self.Name).finish()
    }
}
impl ::windows::core::TypeKind for DML_OPERATOR_GRAPH_NODE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_OPERATOR_GRAPH_NODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Operator == other.Operator && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for DML_OPERATOR_GRAPH_NODE_DESC {}
impl ::core::default::Default for DML_OPERATOR_GRAPH_NODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_OUTPUT_GRAPH_EDGE_DESC {
    pub FromNodeIndex: u32,
    pub FromNodeOutputIndex: u32,
    pub GraphOutputIndex: u32,
    pub Name: ::windows::core::PCSTR,
}
impl ::core::marker::Copy for DML_OUTPUT_GRAPH_EDGE_DESC {}
impl ::core::clone::Clone for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_OUTPUT_GRAPH_EDGE_DESC").field("FromNodeIndex", &self.FromNodeIndex).field("FromNodeOutputIndex", &self.FromNodeOutputIndex).field("GraphOutputIndex", &self.GraphOutputIndex).field("Name", &self.Name).finish()
    }
}
impl ::windows::core::TypeKind for DML_OUTPUT_GRAPH_EDGE_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.FromNodeIndex == other.FromNodeIndex && self.FromNodeOutputIndex == other.FromNodeOutputIndex && self.GraphOutputIndex == other.GraphOutputIndex && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for DML_OUTPUT_GRAPH_EDGE_DESC {}
impl ::core::default::Default for DML_OUTPUT_GRAPH_EDGE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_PADDING_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub PaddingMode: DML_PADDING_MODE,
    pub PaddingValue: f32,
    pub DimensionCount: u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
}
impl ::core::marker::Copy for DML_PADDING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_PADDING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_PADDING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_PADDING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("PaddingMode", &self.PaddingMode).field("PaddingValue", &self.PaddingValue).field("DimensionCount", &self.DimensionCount).field("StartPadding", &self.StartPadding).field("EndPadding", &self.EndPadding).finish()
    }
}
impl ::windows::core::TypeKind for DML_PADDING_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_PADDING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.PaddingMode == other.PaddingMode && self.PaddingValue == other.PaddingValue && self.DimensionCount == other.DimensionCount && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding
    }
}
impl ::core::cmp::Eq for DML_PADDING_OPERATOR_DESC {}
impl ::core::default::Default for DML_PADDING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub InputScaleTensor: *const DML_TENSOR_DESC,
    pub InputZeroPointTensor: *const DML_TENSOR_DESC,
    pub FilterTensor: *const DML_TENSOR_DESC,
    pub FilterScaleTensor: *const DML_TENSOR_DESC,
    pub FilterZeroPointTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub OutputScaleTensor: *const DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Strides: *const u32,
    pub Dilations: *const u32,
    pub StartPadding: *const u32,
    pub EndPadding: *const u32,
    pub GroupCount: u32,
}
impl ::core::marker::Copy for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("InputScaleTensor", &self.InputScaleTensor)
            .field("InputZeroPointTensor", &self.InputZeroPointTensor)
            .field("FilterTensor", &self.FilterTensor)
            .field("FilterScaleTensor", &self.FilterScaleTensor)
            .field("FilterZeroPointTensor", &self.FilterZeroPointTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("OutputScaleTensor", &self.OutputScaleTensor)
            .field("OutputZeroPointTensor", &self.OutputZeroPointTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("DimensionCount", &self.DimensionCount)
            .field("Strides", &self.Strides)
            .field("Dilations", &self.Dilations)
            .field("StartPadding", &self.StartPadding)
            .field("EndPadding", &self.EndPadding)
            .field("GroupCount", &self.GroupCount)
            .finish()
    }
}
impl ::windows::core::TypeKind for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.InputScaleTensor == other.InputScaleTensor && self.InputZeroPointTensor == other.InputZeroPointTensor && self.FilterTensor == other.FilterTensor && self.FilterScaleTensor == other.FilterScaleTensor && self.FilterZeroPointTensor == other.FilterZeroPointTensor && self.BiasTensor == other.BiasTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Strides == other.Strides && self.Dilations == other.Dilations && self.StartPadding == other.StartPadding && self.EndPadding == other.EndPadding && self.GroupCount == other.GroupCount
    }
}
impl ::core::cmp::Eq for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {}
impl ::core::default::Default for DML_QUANTIZED_LINEAR_CONVOLUTION_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    pub ATensor: *const DML_TENSOR_DESC,
    pub AScaleTensor: *const DML_TENSOR_DESC,
    pub AZeroPointTensor: *const DML_TENSOR_DESC,
    pub BTensor: *const DML_TENSOR_DESC,
    pub BScaleTensor: *const DML_TENSOR_DESC,
    pub BZeroPointTensor: *const DML_TENSOR_DESC,
    pub OutputScaleTensor: *const DML_TENSOR_DESC,
    pub OutputZeroPointTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
}
impl ::core::marker::Copy for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC").field("ATensor", &self.ATensor).field("AScaleTensor", &self.AScaleTensor).field("AZeroPointTensor", &self.AZeroPointTensor).field("BTensor", &self.BTensor).field("BScaleTensor", &self.BScaleTensor).field("BZeroPointTensor", &self.BZeroPointTensor).field("OutputScaleTensor", &self.OutputScaleTensor).field("OutputZeroPointTensor", &self.OutputZeroPointTensor).field("OutputTensor", &self.OutputTensor).finish()
    }
}
impl ::windows::core::TypeKind for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ATensor == other.ATensor && self.AScaleTensor == other.AScaleTensor && self.AZeroPointTensor == other.AZeroPointTensor && self.BTensor == other.BTensor && self.BScaleTensor == other.BScaleTensor && self.BZeroPointTensor == other.BZeroPointTensor && self.OutputScaleTensor == other.OutputScaleTensor && self.OutputZeroPointTensor == other.OutputZeroPointTensor && self.OutputTensor == other.OutputTensor
    }
}
impl ::core::cmp::Eq for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {}
impl ::core::default::Default for DML_QUANTIZED_LINEAR_MATRIX_MULTIPLY_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_RANDOM_GENERATOR_OPERATOR_DESC {
    pub InputStateTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub OutputStateTensor: *const DML_TENSOR_DESC,
    pub Type: DML_RANDOM_GENERATOR_TYPE,
}
impl ::core::marker::Copy for DML_RANDOM_GENERATOR_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RANDOM_GENERATOR_OPERATOR_DESC").field("InputStateTensor", &self.InputStateTensor).field("OutputTensor", &self.OutputTensor).field("OutputStateTensor", &self.OutputStateTensor).field("Type", &self.Type).finish()
    }
}
impl ::windows::core::TypeKind for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputStateTensor == other.InputStateTensor && self.OutputTensor == other.OutputTensor && self.OutputStateTensor == other.OutputStateTensor && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for DML_RANDOM_GENERATOR_OPERATOR_DESC {}
impl ::core::default::Default for DML_RANDOM_GENERATOR_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_REDUCE_OPERATOR_DESC {
    pub Function: DML_REDUCE_FUNCTION,
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub AxisCount: u32,
    pub Axes: *const u32,
}
impl ::core::marker::Copy for DML_REDUCE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_REDUCE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_REDUCE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_REDUCE_OPERATOR_DESC").field("Function", &self.Function).field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("AxisCount", &self.AxisCount).field("Axes", &self.Axes).finish()
    }
}
impl ::windows::core::TypeKind for DML_REDUCE_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_REDUCE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function && self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.AxisCount == other.AxisCount && self.Axes == other.Axes
    }
}
impl ::core::cmp::Eq for DML_REDUCE_OPERATOR_DESC {}
impl ::core::default::Default for DML_REDUCE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_RESAMPLE1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub DimensionCount: u32,
    pub Scales: *const f32,
    pub InputPixelOffsets: *const f32,
    pub OutputPixelOffsets: *const f32,
}
impl ::core::marker::Copy for DML_RESAMPLE1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RESAMPLE1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_RESAMPLE1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RESAMPLE1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("InterpolationMode", &self.InterpolationMode).field("DimensionCount", &self.DimensionCount).field("Scales", &self.Scales).field("InputPixelOffsets", &self.InputPixelOffsets).field("OutputPixelOffsets", &self.OutputPixelOffsets).finish()
    }
}
impl ::windows::core::TypeKind for DML_RESAMPLE1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_RESAMPLE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.InterpolationMode == other.InterpolationMode && self.DimensionCount == other.DimensionCount && self.Scales == other.Scales && self.InputPixelOffsets == other.InputPixelOffsets && self.OutputPixelOffsets == other.OutputPixelOffsets
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE1_OPERATOR_DESC {}
impl ::core::default::Default for DML_RESAMPLE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_RESAMPLE_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *const DML_TENSOR_DESC,
    pub OutputGradientTensor: *const DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub DimensionCount: u32,
    pub Scales: *const f32,
    pub InputPixelOffsets: *const f32,
    pub OutputPixelOffsets: *const f32,
}
impl ::core::marker::Copy for DML_RESAMPLE_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RESAMPLE_GRAD_OPERATOR_DESC").field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("InterpolationMode", &self.InterpolationMode).field("DimensionCount", &self.DimensionCount).field("Scales", &self.Scales).field("InputPixelOffsets", &self.InputPixelOffsets).field("OutputPixelOffsets", &self.OutputPixelOffsets).finish()
    }
}
impl ::windows::core::TypeKind for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.InterpolationMode == other.InterpolationMode && self.DimensionCount == other.DimensionCount && self.Scales == other.Scales && self.InputPixelOffsets == other.InputPixelOffsets && self.OutputPixelOffsets == other.OutputPixelOffsets
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_RESAMPLE_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_RESAMPLE_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub ScaleCount: u32,
    pub Scales: *const f32,
}
impl ::core::marker::Copy for DML_RESAMPLE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RESAMPLE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_RESAMPLE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RESAMPLE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("InterpolationMode", &self.InterpolationMode).field("ScaleCount", &self.ScaleCount).field("Scales", &self.Scales).finish()
    }
}
impl ::windows::core::TypeKind for DML_RESAMPLE_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_RESAMPLE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.InterpolationMode == other.InterpolationMode && self.ScaleCount == other.ScaleCount && self.Scales == other.Scales
    }
}
impl ::core::cmp::Eq for DML_RESAMPLE_OPERATOR_DESC {}
impl ::core::default::Default for DML_RESAMPLE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("SequenceLengthsTensor", &self.SequenceLengthsTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::windows::core::TypeKind for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.SequenceLengthsTensor == other.SequenceLengthsTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {}
impl ::core::default::Default for DML_REVERSE_SUBSEQUENCES_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_RNN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub WeightTensor: *const DML_TENSOR_DESC,
    pub RecurrenceTensor: *const DML_TENSOR_DESC,
    pub BiasTensor: *const DML_TENSOR_DESC,
    pub HiddenInitTensor: *const DML_TENSOR_DESC,
    pub SequenceLengthsTensor: *const DML_TENSOR_DESC,
    pub OutputSequenceTensor: *const DML_TENSOR_DESC,
    pub OutputSingleTensor: *const DML_TENSOR_DESC,
    pub ActivationDescCount: u32,
    pub ActivationDescs: *const DML_OPERATOR_DESC,
    pub Direction: DML_RECURRENT_NETWORK_DIRECTION,
}
impl ::core::marker::Copy for DML_RNN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_RNN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_RNN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_RNN_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("WeightTensor", &self.WeightTensor)
            .field("RecurrenceTensor", &self.RecurrenceTensor)
            .field("BiasTensor", &self.BiasTensor)
            .field("HiddenInitTensor", &self.HiddenInitTensor)
            .field("SequenceLengthsTensor", &self.SequenceLengthsTensor)
            .field("OutputSequenceTensor", &self.OutputSequenceTensor)
            .field("OutputSingleTensor", &self.OutputSingleTensor)
            .field("ActivationDescCount", &self.ActivationDescCount)
            .field("ActivationDescs", &self.ActivationDescs)
            .field("Direction", &self.Direction)
            .finish()
    }
}
impl ::windows::core::TypeKind for DML_RNN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_RNN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.WeightTensor == other.WeightTensor && self.RecurrenceTensor == other.RecurrenceTensor && self.BiasTensor == other.BiasTensor && self.HiddenInitTensor == other.HiddenInitTensor && self.SequenceLengthsTensor == other.SequenceLengthsTensor && self.OutputSequenceTensor == other.OutputSequenceTensor && self.OutputSingleTensor == other.OutputSingleTensor && self.ActivationDescCount == other.ActivationDescCount && self.ActivationDescs == other.ActivationDescs && self.Direction == other.Direction
    }
}
impl ::core::cmp::Eq for DML_RNN_OPERATOR_DESC {}
impl ::core::default::Default for DML_RNN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DML_ROI_ALIGN1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ROITensor: *const DML_TENSOR_DESC,
    pub BatchIndicesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ReductionFunction: DML_REDUCE_FUNCTION,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub SpatialScaleX: f32,
    pub SpatialScaleY: f32,
    pub InputPixelOffset: f32,
    pub OutputPixelOffset: f32,
    pub OutOfBoundsInputValue: f32,
    pub MinimumSamplesPerOutput: u32,
    pub MaximumSamplesPerOutput: u32,
    pub AlignRegionsToCorners: super::super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DML_ROI_ALIGN1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ROI_ALIGN1_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("ROITensor", &self.ROITensor)
            .field("BatchIndicesTensor", &self.BatchIndicesTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("ReductionFunction", &self.ReductionFunction)
            .field("InterpolationMode", &self.InterpolationMode)
            .field("SpatialScaleX", &self.SpatialScaleX)
            .field("SpatialScaleY", &self.SpatialScaleY)
            .field("InputPixelOffset", &self.InputPixelOffset)
            .field("OutputPixelOffset", &self.OutputPixelOffset)
            .field("OutOfBoundsInputValue", &self.OutOfBoundsInputValue)
            .field("MinimumSamplesPerOutput", &self.MinimumSamplesPerOutput)
            .field("MaximumSamplesPerOutput", &self.MaximumSamplesPerOutput)
            .field("AlignRegionsToCorners", &self.AlignRegionsToCorners)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DML_ROI_ALIGN1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ROITensor == other.ROITensor && self.BatchIndicesTensor == other.BatchIndicesTensor && self.OutputTensor == other.OutputTensor && self.ReductionFunction == other.ReductionFunction && self.InterpolationMode == other.InterpolationMode && self.SpatialScaleX == other.SpatialScaleX && self.SpatialScaleY == other.SpatialScaleY && self.InputPixelOffset == other.InputPixelOffset && self.OutputPixelOffset == other.OutputPixelOffset && self.OutOfBoundsInputValue == other.OutOfBoundsInputValue && self.MinimumSamplesPerOutput == other.MinimumSamplesPerOutput && self.MaximumSamplesPerOutput == other.MaximumSamplesPerOutput && self.AlignRegionsToCorners == other.AlignRegionsToCorners
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DML_ROI_ALIGN1_OPERATOR_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DML_ROI_ALIGN1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ROI_ALIGN_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ROITensor: *const DML_TENSOR_DESC,
    pub BatchIndicesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ReductionFunction: DML_REDUCE_FUNCTION,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
    pub SpatialScaleX: f32,
    pub SpatialScaleY: f32,
    pub OutOfBoundsInputValue: f32,
    pub MinimumSamplesPerOutput: u32,
    pub MaximumSamplesPerOutput: u32,
}
impl ::core::marker::Copy for DML_ROI_ALIGN_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ROI_ALIGN_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ROI_ALIGN_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ROI_ALIGN_OPERATOR_DESC")
            .field("InputTensor", &self.InputTensor)
            .field("ROITensor", &self.ROITensor)
            .field("BatchIndicesTensor", &self.BatchIndicesTensor)
            .field("OutputTensor", &self.OutputTensor)
            .field("ReductionFunction", &self.ReductionFunction)
            .field("InterpolationMode", &self.InterpolationMode)
            .field("SpatialScaleX", &self.SpatialScaleX)
            .field("SpatialScaleY", &self.SpatialScaleY)
            .field("OutOfBoundsInputValue", &self.OutOfBoundsInputValue)
            .field("MinimumSamplesPerOutput", &self.MinimumSamplesPerOutput)
            .field("MaximumSamplesPerOutput", &self.MaximumSamplesPerOutput)
            .finish()
    }
}
impl ::windows::core::TypeKind for DML_ROI_ALIGN_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ROI_ALIGN_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ROITensor == other.ROITensor && self.BatchIndicesTensor == other.BatchIndicesTensor && self.OutputTensor == other.OutputTensor && self.ReductionFunction == other.ReductionFunction && self.InterpolationMode == other.InterpolationMode && self.SpatialScaleX == other.SpatialScaleX && self.SpatialScaleY == other.SpatialScaleY && self.OutOfBoundsInputValue == other.OutOfBoundsInputValue && self.MinimumSamplesPerOutput == other.MinimumSamplesPerOutput && self.MaximumSamplesPerOutput == other.MaximumSamplesPerOutput
    }
}
impl ::core::cmp::Eq for DML_ROI_ALIGN_OPERATOR_DESC {}
impl ::core::default::Default for DML_ROI_ALIGN_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_ROI_POOLING_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub ROITensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub SpatialScale: f32,
    pub PooledSize: DML_SIZE_2D,
}
impl ::core::marker::Copy for DML_ROI_POOLING_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_ROI_POOLING_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_ROI_POOLING_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_ROI_POOLING_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("ROITensor", &self.ROITensor).field("OutputTensor", &self.OutputTensor).field("SpatialScale", &self.SpatialScale).field("PooledSize", &self.PooledSize).finish()
    }
}
impl ::windows::core::TypeKind for DML_ROI_POOLING_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_ROI_POOLING_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.ROITensor == other.ROITensor && self.OutputTensor == other.OutputTensor && self.SpatialScale == other.SpatialScale && self.PooledSize == other.PooledSize
    }
}
impl ::core::cmp::Eq for DML_ROI_POOLING_OPERATOR_DESC {}
impl ::core::default::Default for DML_ROI_POOLING_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub union DML_SCALAR_UNION {
    pub Bytes: [u8; 8],
    pub Int8: i8,
    pub UInt8: u8,
    pub Int16: i16,
    pub UInt16: u16,
    pub Int32: i32,
    pub UInt32: u32,
    pub Int64: i64,
    pub UInt64: u64,
    pub Float32: f32,
    pub Float64: f64,
}
impl ::core::marker::Copy for DML_SCALAR_UNION {}
impl ::core::clone::Clone for DML_SCALAR_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DML_SCALAR_UNION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DML_SCALAR_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SCALE_BIAS {
    pub Scale: f32,
    pub Bias: f32,
}
impl ::core::marker::Copy for DML_SCALE_BIAS {}
impl ::core::clone::Clone for DML_SCALE_BIAS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SCALE_BIAS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SCALE_BIAS").field("Scale", &self.Scale).field("Bias", &self.Bias).finish()
    }
}
impl ::windows::core::TypeKind for DML_SCALE_BIAS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SCALE_BIAS {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale && self.Bias == other.Bias
    }
}
impl ::core::cmp::Eq for DML_SCALE_BIAS {}
impl ::core::default::Default for DML_SCALE_BIAS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SCATTER_ND_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub IndicesTensor: *const DML_TENSOR_DESC,
    pub UpdatesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub InputDimensionCount: u32,
    pub IndicesDimensionCount: u32,
}
impl ::core::marker::Copy for DML_SCATTER_ND_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SCATTER_ND_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SCATTER_ND_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SCATTER_ND_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("UpdatesTensor", &self.UpdatesTensor).field("OutputTensor", &self.OutputTensor).field("InputDimensionCount", &self.InputDimensionCount).field("IndicesDimensionCount", &self.IndicesDimensionCount).finish()
    }
}
impl ::windows::core::TypeKind for DML_SCATTER_ND_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SCATTER_ND_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.UpdatesTensor == other.UpdatesTensor && self.OutputTensor == other.OutputTensor && self.InputDimensionCount == other.InputDimensionCount && self.IndicesDimensionCount == other.IndicesDimensionCount
    }
}
impl ::core::cmp::Eq for DML_SCATTER_ND_OPERATOR_DESC {}
impl ::core::default::Default for DML_SCATTER_ND_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SCATTER_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub IndicesTensor: *const DML_TENSOR_DESC,
    pub UpdatesTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_SCATTER_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SCATTER_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SCATTER_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SCATTER_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("IndicesTensor", &self.IndicesTensor).field("UpdatesTensor", &self.UpdatesTensor).field("OutputTensor", &self.OutputTensor).field("Axis", &self.Axis).finish()
    }
}
impl ::windows::core::TypeKind for DML_SCATTER_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SCATTER_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.IndicesTensor == other.IndicesTensor && self.UpdatesTensor == other.UpdatesTensor && self.OutputTensor == other.OutputTensor && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_SCATTER_OPERATOR_DESC {}
impl ::core::default::Default for DML_SCATTER_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SIZE_2D {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for DML_SIZE_2D {}
impl ::core::clone::Clone for DML_SIZE_2D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SIZE_2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SIZE_2D").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::windows::core::TypeKind for DML_SIZE_2D {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SIZE_2D {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for DML_SIZE_2D {}
impl ::core::default::Default for DML_SIZE_2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SLICE1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub InputWindowOffsets: *const u32,
    pub InputWindowSizes: *const u32,
    pub InputWindowStrides: *const i32,
}
impl ::core::marker::Copy for DML_SLICE1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SLICE1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SLICE1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SLICE1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("InputWindowOffsets", &self.InputWindowOffsets).field("InputWindowSizes", &self.InputWindowSizes).field("InputWindowStrides", &self.InputWindowStrides).finish()
    }
}
impl ::windows::core::TypeKind for DML_SLICE1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SLICE1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.InputWindowOffsets == other.InputWindowOffsets && self.InputWindowSizes == other.InputWindowSizes && self.InputWindowStrides == other.InputWindowStrides
    }
}
impl ::core::cmp::Eq for DML_SLICE1_OPERATOR_DESC {}
impl ::core::default::Default for DML_SLICE1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SLICE_GRAD_OPERATOR_DESC {
    pub InputGradientTensor: *const DML_TENSOR_DESC,
    pub OutputGradientTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub InputWindowOffsets: *const u32,
    pub InputWindowSizes: *const u32,
    pub InputWindowStrides: *const i32,
}
impl ::core::marker::Copy for DML_SLICE_GRAD_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SLICE_GRAD_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SLICE_GRAD_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SLICE_GRAD_OPERATOR_DESC").field("InputGradientTensor", &self.InputGradientTensor).field("OutputGradientTensor", &self.OutputGradientTensor).field("DimensionCount", &self.DimensionCount).field("InputWindowOffsets", &self.InputWindowOffsets).field("InputWindowSizes", &self.InputWindowSizes).field("InputWindowStrides", &self.InputWindowStrides).finish()
    }
}
impl ::windows::core::TypeKind for DML_SLICE_GRAD_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SLICE_GRAD_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputGradientTensor == other.InputGradientTensor && self.OutputGradientTensor == other.OutputGradientTensor && self.DimensionCount == other.DimensionCount && self.InputWindowOffsets == other.InputWindowOffsets && self.InputWindowSizes == other.InputWindowSizes && self.InputWindowStrides == other.InputWindowStrides
    }
}
impl ::core::cmp::Eq for DML_SLICE_GRAD_OPERATOR_DESC {}
impl ::core::default::Default for DML_SLICE_GRAD_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SLICE_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub DimensionCount: u32,
    pub Offsets: *const u32,
    pub Sizes: *const u32,
    pub Strides: *const u32,
}
impl ::core::marker::Copy for DML_SLICE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SLICE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SLICE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SLICE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("DimensionCount", &self.DimensionCount).field("Offsets", &self.Offsets).field("Sizes", &self.Sizes).field("Strides", &self.Strides).finish()
    }
}
impl ::windows::core::TypeKind for DML_SLICE_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SLICE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.DimensionCount == other.DimensionCount && self.Offsets == other.Offsets && self.Sizes == other.Sizes && self.Strides == other.Strides
    }
}
impl ::core::cmp::Eq for DML_SLICE_OPERATOR_DESC {}
impl ::core::default::Default for DML_SLICE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub BlockSize: u32,
    pub Order: DML_DEPTH_SPACE_ORDER,
}
impl ::core::marker::Copy for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SPACE_TO_DEPTH1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).field("Order", &self.Order).finish()
    }
}
impl ::windows::core::TypeKind for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize && self.Order == other.Order
    }
}
impl ::core::cmp::Eq for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {}
impl ::core::default::Default for DML_SPACE_TO_DEPTH1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub BlockSize: u32,
}
impl ::core::marker::Copy for DML_SPACE_TO_DEPTH_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SPACE_TO_DEPTH_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("BlockSize", &self.BlockSize).finish()
    }
}
impl ::windows::core::TypeKind for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.BlockSize == other.BlockSize
    }
}
impl ::core::cmp::Eq for DML_SPACE_TO_DEPTH_OPERATOR_DESC {}
impl ::core::default::Default for DML_SPACE_TO_DEPTH_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_SPLIT_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputCount: u32,
    pub OutputTensors: *const DML_TENSOR_DESC,
    pub Axis: u32,
}
impl ::core::marker::Copy for DML_SPLIT_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_SPLIT_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_SPLIT_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_SPLIT_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputCount", &self.OutputCount).field("OutputTensors", &self.OutputTensors).field("Axis", &self.Axis).finish()
    }
}
impl ::windows::core::TypeKind for DML_SPLIT_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_SPLIT_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputCount == other.OutputCount && self.OutputTensors == other.OutputTensors && self.Axis == other.Axis
    }
}
impl ::core::cmp::Eq for DML_SPLIT_OPERATOR_DESC {}
impl ::core::default::Default for DML_SPLIT_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_TENSOR_DESC {
    pub Type: DML_TENSOR_TYPE,
    pub Desc: *const ::core::ffi::c_void,
}
impl ::core::marker::Copy for DML_TENSOR_DESC {}
impl ::core::clone::Clone for DML_TENSOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_TENSOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_TENSOR_DESC").field("Type", &self.Type).field("Desc", &self.Desc).finish()
    }
}
impl ::windows::core::TypeKind for DML_TENSOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_TENSOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Desc == other.Desc
    }
}
impl ::core::cmp::Eq for DML_TENSOR_DESC {}
impl ::core::default::Default for DML_TENSOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_TILE_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub RepeatsCount: u32,
    pub Repeats: *const u32,
}
impl ::core::marker::Copy for DML_TILE_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_TILE_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_TILE_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_TILE_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("RepeatsCount", &self.RepeatsCount).field("Repeats", &self.Repeats).finish()
    }
}
impl ::windows::core::TypeKind for DML_TILE_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_TILE_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.RepeatsCount == other.RepeatsCount && self.Repeats == other.Repeats
    }
}
impl ::core::cmp::Eq for DML_TILE_OPERATOR_DESC {}
impl ::core::default::Default for DML_TILE_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_TOP_K1_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputValueTensor: *const DML_TENSOR_DESC,
    pub OutputIndexTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
    pub K: u32,
    pub AxisDirection: DML_AXIS_DIRECTION,
}
impl ::core::marker::Copy for DML_TOP_K1_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_TOP_K1_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_TOP_K1_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_TOP_K1_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputValueTensor", &self.OutputValueTensor).field("OutputIndexTensor", &self.OutputIndexTensor).field("Axis", &self.Axis).field("K", &self.K).field("AxisDirection", &self.AxisDirection).finish()
    }
}
impl ::windows::core::TypeKind for DML_TOP_K1_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_TOP_K1_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputValueTensor == other.OutputValueTensor && self.OutputIndexTensor == other.OutputIndexTensor && self.Axis == other.Axis && self.K == other.K && self.AxisDirection == other.AxisDirection
    }
}
impl ::core::cmp::Eq for DML_TOP_K1_OPERATOR_DESC {}
impl ::core::default::Default for DML_TOP_K1_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_TOP_K_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputValueTensor: *const DML_TENSOR_DESC,
    pub OutputIndexTensor: *const DML_TENSOR_DESC,
    pub Axis: u32,
    pub K: u32,
}
impl ::core::marker::Copy for DML_TOP_K_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_TOP_K_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_TOP_K_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_TOP_K_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputValueTensor", &self.OutputValueTensor).field("OutputIndexTensor", &self.OutputIndexTensor).field("Axis", &self.Axis).field("K", &self.K).finish()
    }
}
impl ::windows::core::TypeKind for DML_TOP_K_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_TOP_K_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputValueTensor == other.OutputValueTensor && self.OutputIndexTensor == other.OutputIndexTensor && self.Axis == other.Axis && self.K == other.K
    }
}
impl ::core::cmp::Eq for DML_TOP_K_OPERATOR_DESC {}
impl ::core::default::Default for DML_TOP_K_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_UPSAMPLE_2D_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub ScaleSize: DML_SIZE_2D,
    pub InterpolationMode: DML_INTERPOLATION_MODE,
}
impl ::core::marker::Copy for DML_UPSAMPLE_2D_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_UPSAMPLE_2D_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("ScaleSize", &self.ScaleSize).field("InterpolationMode", &self.InterpolationMode).finish()
    }
}
impl ::windows::core::TypeKind for DML_UPSAMPLE_2D_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.ScaleSize == other.ScaleSize && self.InterpolationMode == other.InterpolationMode
    }
}
impl ::core::cmp::Eq for DML_UPSAMPLE_2D_OPERATOR_DESC {}
impl ::core::default::Default for DML_UPSAMPLE_2D_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_AI_MachineLearning_DirectML\"`*"]
pub struct DML_VALUE_SCALE_2D_OPERATOR_DESC {
    pub InputTensor: *const DML_TENSOR_DESC,
    pub OutputTensor: *const DML_TENSOR_DESC,
    pub Scale: f32,
    pub ChannelCount: u32,
    pub Bias: *const f32,
}
impl ::core::marker::Copy for DML_VALUE_SCALE_2D_OPERATOR_DESC {}
impl ::core::clone::Clone for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DML_VALUE_SCALE_2D_OPERATOR_DESC").field("InputTensor", &self.InputTensor).field("OutputTensor", &self.OutputTensor).field("Scale", &self.Scale).field("ChannelCount", &self.ChannelCount).field("Bias", &self.Bias).finish()
    }
}
impl ::windows::core::TypeKind for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputTensor == other.InputTensor && self.OutputTensor == other.OutputTensor && self.Scale == other.Scale && self.ChannelCount == other.ChannelCount && self.Bias == other.Bias
    }
}
impl ::core::cmp::Eq for DML_VALUE_SCALE_2D_OPERATOR_DESC {}
impl ::core::default::Default for DML_VALUE_SCALE_2D_OPERATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
