pub const CLSID_CTask: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x148bd520_a2ab_11ce_b11f_00aa00530503);
pub const CLSID_CTaskScheduler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x148bd52a_a2ab_11ce_b11f_00aa00530503);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct DAILY {
    pub DaysInterval: u16,
}
impl ::core::marker::Copy for DAILY {}
impl ::core::clone::Clone for DAILY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DAILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DAILY").field("DaysInterval", &self.DaysInterval).finish()
    }
}
unsafe impl ::windows::core::Abi for DAILY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DAILY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DAILY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DAILY {}
impl ::core::default::Default for DAILY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IAction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IAction {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAction> for ::windows::core::IUnknown {
    fn from(value: IAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IAction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAction> for ::windows::core::IUnknown {
    fn from(value: &IAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IAction> for super::Com::IDispatch {
    fn from(value: IAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IAction> for &'a super::Com::IDispatch {
    fn from(value: &'a IAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IAction> for super::Com::IDispatch {
    fn from(value: &IAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IAction {
    type Vtable = IAction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbae54997_48b1_4cbe_9965_d6be263ebea4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IAction_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IActionCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IActionCollection {
    pub unsafe fn Count(&self, pcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<IAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAction>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn XmlText(&self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).XmlText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXmlText<'a, P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetXmlText)(::windows::core::Interface::as_raw(self), text.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create(&self, r#type: TASK_ACTION_TYPE) -> ::windows::core::Result<IAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IAction>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, P0>(&self, index: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), index.into().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Context(&self, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Context)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetContext<'a, P0>(&self, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetContext)(::windows::core::Interface::as_raw(self), context.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IActionCollection> for ::windows::core::IUnknown {
    fn from(value: IActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IActionCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IActionCollection> for ::windows::core::IUnknown {
    fn from(value: &IActionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IActionCollection> for super::Com::IDispatch {
    fn from(value: IActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IActionCollection> for &'a super::Com::IDispatch {
    fn from(value: &'a IActionCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IActionCollection> for super::Com::IDispatch {
    fn from(value: &IActionCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IActionCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IActionCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IActionCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IActionCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActionCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IActionCollection {
    type Vtable = IActionCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02820e19_7b98_4ed2_b2e8_fdccceff619b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IActionCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, ppaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub XmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    XmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXmlText: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TASK_ACTION_TYPE, ppaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Context: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetContext: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IBootTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IBootTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDelay<'a, P0>(&self, delay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDelay)(::windows::core::Interface::as_raw(self), delay.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBootTrigger> for ::windows::core::IUnknown {
    fn from(value: IBootTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBootTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBootTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBootTrigger> for ::windows::core::IUnknown {
    fn from(value: &IBootTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBootTrigger> for super::Com::IDispatch {
    fn from(value: IBootTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBootTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a IBootTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBootTrigger> for super::Com::IDispatch {
    fn from(value: &IBootTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IBootTrigger> for ITrigger {
    fn from(value: IBootTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IBootTrigger> for &'a ITrigger {
    fn from(value: &'a IBootTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IBootTrigger> for ITrigger {
    fn from(value: &IBootTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IBootTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IBootTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IBootTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IBootTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBootTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IBootTrigger {
    type Vtable = IBootTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a9c35da_d357_41f4_bbc1_207ac1b1f3cb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IBootTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IComHandlerAction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IComHandlerAction {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassId(&self, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClassId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pclsid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassId<'a, P0>(&self, clsid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetClassId)(::windows::core::Interface::as_raw(self), clsid.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Data(&self, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Data)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<'a, P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), data.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IComHandlerAction> for ::windows::core::IUnknown {
    fn from(value: IComHandlerAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IComHandlerAction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IComHandlerAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IComHandlerAction> for ::windows::core::IUnknown {
    fn from(value: &IComHandlerAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IComHandlerAction> for super::Com::IDispatch {
    fn from(value: IComHandlerAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IComHandlerAction> for &'a super::Com::IDispatch {
    fn from(value: &'a IComHandlerAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IComHandlerAction> for super::Com::IDispatch {
    fn from(value: &IComHandlerAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IComHandlerAction> for IAction {
    fn from(value: IComHandlerAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IComHandlerAction> for &'a IAction {
    fn from(value: &'a IComHandlerAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IComHandlerAction> for IAction {
    fn from(value: &IComHandlerAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IComHandlerAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IComHandlerAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IComHandlerAction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IComHandlerAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComHandlerAction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IComHandlerAction {
    type Vtable = IComHandlerAction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d2fd252_75c5_4f66_90ba_2a7d8cc3039f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IComHandlerAction_Vtbl {
    pub base__: IAction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClassId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClassId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClassId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Data: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetData: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IDailyTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IDailyTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    pub unsafe fn DaysInterval(&self, pdays: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DaysInterval)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdays)).ok()
    }
    pub unsafe fn SetDaysInterval(&self, days: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDaysInterval)(::windows::core::Interface::as_raw(self), days).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RandomDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRandomDelay<'a, P0>(&self, randomdelay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetRandomDelay)(::windows::core::Interface::as_raw(self), randomdelay.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDailyTrigger> for ::windows::core::IUnknown {
    fn from(value: IDailyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDailyTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDailyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDailyTrigger> for ::windows::core::IUnknown {
    fn from(value: &IDailyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDailyTrigger> for super::Com::IDispatch {
    fn from(value: IDailyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDailyTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a IDailyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDailyTrigger> for super::Com::IDispatch {
    fn from(value: &IDailyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IDailyTrigger> for ITrigger {
    fn from(value: IDailyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IDailyTrigger> for &'a ITrigger {
    fn from(value: &'a IDailyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IDailyTrigger> for ITrigger {
    fn from(value: &IDailyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IDailyTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDailyTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDailyTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDailyTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDailyTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IDailyTrigger {
    type Vtable = IDailyTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x126c5cd8_b288_41d5_8dbf_e491446adc5c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IDailyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT,
    pub SetDaysInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IEmailAction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IEmailAction {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Server(&self, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Server)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pserver)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServer<'a, P0>(&self, server: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetServer)(::windows::core::Interface::as_raw(self), server.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subject(&self, psubject: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Subject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psubject)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubject<'a, P0>(&self, subject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSubject)(::windows::core::Interface::as_raw(self), subject.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn To(&self, pto: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).To)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pto)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTo<'a, P0>(&self, to: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetTo)(::windows::core::Interface::as_raw(self), to.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Cc(&self, pcc: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCc<'a, P0>(&self, cc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetCc)(::windows::core::Interface::as_raw(self), cc.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Bcc(&self, pbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Bcc)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbcc)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBcc<'a, P0>(&self, bcc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetBcc)(::windows::core::Interface::as_raw(self), bcc.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReplyTo(&self, preplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReplyTo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(preplyto)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReplyTo<'a, P0>(&self, replyto: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetReplyTo)(::windows::core::Interface::as_raw(self), replyto.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn From(&self, pfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).From)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfrom)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFrom<'a, P0>(&self, from: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetFrom)(::windows::core::Interface::as_raw(self), from.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HeaderFields(&self) -> ::windows::core::Result<ITaskNamedValueCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HeaderFields)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskNamedValueCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHeaderFields<'a, P0>(&self, pheaderfields: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITaskNamedValueCollection>>,
    {
        (::windows::core::Interface::vtable(self).SetHeaderFields)(::windows::core::Interface::as_raw(self), pheaderfields.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Body(&self, pbody: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Body)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbody)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBody<'a, P0>(&self, body: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetBody)(::windows::core::Interface::as_raw(self), body.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Attachments(&self, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Attachments)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pattachements)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAttachments(&self, pattachements: *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttachments)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pattachements)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEmailAction> for ::windows::core::IUnknown {
    fn from(value: IEmailAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IEmailAction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEmailAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEmailAction> for ::windows::core::IUnknown {
    fn from(value: &IEmailAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEmailAction> for super::Com::IDispatch {
    fn from(value: IEmailAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IEmailAction> for &'a super::Com::IDispatch {
    fn from(value: &'a IEmailAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEmailAction> for super::Com::IDispatch {
    fn from(value: &IEmailAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEmailAction> for IAction {
    fn from(value: IEmailAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IEmailAction> for &'a IAction {
    fn from(value: &'a IEmailAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEmailAction> for IAction {
    fn from(value: &IEmailAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IEmailAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEmailAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEmailAction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEmailAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmailAction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IEmailAction {
    type Vtable = IEmailAction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10f62c64_7e16_4314_a0c2_0c3683f99d40);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAction_Vtbl {
    pub base__: IAction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Server: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Server: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, server: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubject: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    To: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, to: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Cc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Cc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Bcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Bcc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bcc: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBcc: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preplyto: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReplyTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, replyto: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReplyTo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrom: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    From: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFrom: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub HeaderFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppheaderfields: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    HeaderFields: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHeaderFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheaderfields: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHeaderFields: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Body: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, body: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBody: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Attachments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattachements: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attachments: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAttachments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattachements: *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAttachments: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
pub struct IEnumWorkItems(::windows::core::IUnknown);
impl IEnumWorkItems {
    pub unsafe fn Next(&self, celt: u32, rgpwsznames: *mut *mut ::windows::core::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(rgpwsznames), ::core::mem::transmute(pceltfetched))
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumWorkItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWorkItems>(result__)
    }
}
impl ::core::convert::From<IEnumWorkItems> for ::windows::core::IUnknown {
    fn from(value: IEnumWorkItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumWorkItems> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumWorkItems) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumWorkItems> for ::windows::core::IUnknown {
    fn from(value: &IEnumWorkItems) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumWorkItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumWorkItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWorkItems {}
impl ::core::fmt::Debug for IEnumWorkItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWorkItems").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumWorkItems {
    type Vtable = IEnumWorkItems_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x148bd528_a2ab_11ce_b11f_00aa00530503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumWorkItems_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgpwsznames: *mut *mut ::windows::core::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IEventTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IEventTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Subscription(&self, pquery: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Subscription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pquery)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubscription<'a, P0>(&self, query: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSubscription)(::windows::core::Interface::as_raw(self), query.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDelay<'a, P0>(&self, delay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDelay)(::windows::core::Interface::as_raw(self), delay.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueQueries(&self) -> ::windows::core::Result<ITaskNamedValueCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ValueQueries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskNamedValueCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueQueries<'a, P0>(&self, pnamedxpaths: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITaskNamedValueCollection>>,
    {
        (::windows::core::Interface::vtable(self).SetValueQueries)(::windows::core::Interface::as_raw(self), pnamedxpaths.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEventTrigger> for ::windows::core::IUnknown {
    fn from(value: IEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IEventTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEventTrigger> for ::windows::core::IUnknown {
    fn from(value: &IEventTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEventTrigger> for super::Com::IDispatch {
    fn from(value: IEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IEventTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a IEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEventTrigger> for super::Com::IDispatch {
    fn from(value: &IEventTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IEventTrigger> for ITrigger {
    fn from(value: IEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IEventTrigger> for &'a ITrigger {
    fn from(value: &'a IEventTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IEventTrigger> for ITrigger {
    fn from(value: &IEventTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IEventTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEventTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEventTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEventTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IEventTrigger {
    type Vtable = IEventTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45b0167_9653_4eef_b94f_0732ca7af251);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IEventTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Subscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Subscription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubscription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueQueries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnamedxpaths: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueQueries: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetValueQueries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnamedxpaths: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetValueQueries: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IExecAction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IExecAction {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Path)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPath<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetPath)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Arguments(&self, pargument: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Arguments)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pargument)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArguments<'a, P0>(&self, argument: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetArguments)(::windows::core::Interface::as_raw(self), argument.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WorkingDirectory(&self, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WorkingDirectory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pworkingdirectory)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWorkingDirectory<'a, P0>(&self, workingdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetWorkingDirectory)(::windows::core::Interface::as_raw(self), workingdirectory.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IExecAction> for ::windows::core::IUnknown {
    fn from(value: IExecAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IExecAction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IExecAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IExecAction> for ::windows::core::IUnknown {
    fn from(value: &IExecAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IExecAction> for super::Com::IDispatch {
    fn from(value: IExecAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IExecAction> for &'a super::Com::IDispatch {
    fn from(value: &'a IExecAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IExecAction> for super::Com::IDispatch {
    fn from(value: &IExecAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IExecAction> for IAction {
    fn from(value: IExecAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IExecAction> for &'a IAction {
    fn from(value: &'a IExecAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IExecAction> for IAction {
    fn from(value: &IExecAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IExecAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IExecAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IExecAction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IExecAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExecAction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IExecAction {
    type Vtable = IExecAction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c3d624d_fd6b_49a3_b9b7_09cb3cd3f047);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IExecAction_Vtbl {
    pub base__: IAction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pargument: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Arguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, argument: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetArguments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WorkingDirectory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWorkingDirectory: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IExecAction2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IExecAction2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Path)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPath<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPath)(::windows::core::Interface::as_raw(self), path.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Arguments(&self, pargument: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Arguments)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pargument)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArguments<'a, P0>(&self, argument: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetArguments)(::windows::core::Interface::as_raw(self), argument.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WorkingDirectory(&self, pworkingdirectory: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.WorkingDirectory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pworkingdirectory)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWorkingDirectory<'a, P0>(&self, workingdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetWorkingDirectory)(::windows::core::Interface::as_raw(self), workingdirectory.into().abi()).ok()
    }
    pub unsafe fn HideAppWindow(&self, phideappwindow: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HideAppWindow)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phideappwindow)).ok()
    }
    pub unsafe fn SetHideAppWindow(&self, hideappwindow: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHideAppWindow)(::windows::core::Interface::as_raw(self), hideappwindow).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IExecAction2> for ::windows::core::IUnknown {
    fn from(value: IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IExecAction2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IExecAction2> for ::windows::core::IUnknown {
    fn from(value: &IExecAction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IExecAction2> for super::Com::IDispatch {
    fn from(value: IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IExecAction2> for &'a super::Com::IDispatch {
    fn from(value: &'a IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IExecAction2> for super::Com::IDispatch {
    fn from(value: &IExecAction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IExecAction2> for IAction {
    fn from(value: IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IExecAction2> for &'a IAction {
    fn from(value: &'a IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IExecAction2> for IAction {
    fn from(value: &IExecAction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IExecAction2> for IExecAction {
    fn from(value: IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IExecAction2> for &'a IExecAction {
    fn from(value: &'a IExecAction2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IExecAction2> for IExecAction {
    fn from(value: &IExecAction2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IExecAction2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IExecAction2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IExecAction2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IExecAction2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExecAction2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IExecAction2 {
    type Vtable = IExecAction2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2a82542_bda5_4e6b_9143_e2bf4f8987b6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IExecAction2_Vtbl {
    pub base__: IExecAction_Vtbl,
    pub HideAppWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phideappwindow: *mut i16) -> ::windows::core::HRESULT,
    pub SetHideAppWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hideappwindow: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IIdleSettings(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IIdleSettings {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IdleDuration(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IdleDuration)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIdleDuration<'a, P0>(&self, delay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetIdleDuration)(::windows::core::Interface::as_raw(self), delay.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitTimeout(&self, ptimeout: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitTimeout)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimeout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWaitTimeout<'a, P0>(&self, timeout: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetWaitTimeout)(::windows::core::Interface::as_raw(self), timeout.into().abi()).ok()
    }
    pub unsafe fn StopOnIdleEnd(&self, pstop: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopOnIdleEnd)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstop)).ok()
    }
    pub unsafe fn SetStopOnIdleEnd(&self, stop: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStopOnIdleEnd)(::windows::core::Interface::as_raw(self), stop).ok()
    }
    pub unsafe fn RestartOnIdle(&self, prestart: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestartOnIdle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prestart)).ok()
    }
    pub unsafe fn SetRestartOnIdle(&self, restart: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRestartOnIdle)(::windows::core::Interface::as_raw(self), restart).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IIdleSettings> for ::windows::core::IUnknown {
    fn from(value: IIdleSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IIdleSettings> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IIdleSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IIdleSettings> for ::windows::core::IUnknown {
    fn from(value: &IIdleSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IIdleSettings> for super::Com::IDispatch {
    fn from(value: IIdleSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IIdleSettings> for &'a super::Com::IDispatch {
    fn from(value: &'a IIdleSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IIdleSettings> for super::Com::IDispatch {
    fn from(value: &IIdleSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IIdleSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IIdleSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IIdleSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IIdleSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdleSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IIdleSettings {
    type Vtable = IIdleSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84594461_0053_4342_a8fd_088fabf11f32);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIdleSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IdleDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IdleDuration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIdleDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIdleDuration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimeout: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitTimeout: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWaitTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWaitTimeout: usize,
    pub StopOnIdleEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT,
    pub SetStopOnIdleEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT,
    pub RestartOnIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prestart: *mut i16) -> ::windows::core::HRESULT,
    pub SetRestartOnIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restart: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IIdleTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IIdleTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IIdleTrigger> for ::windows::core::IUnknown {
    fn from(value: IIdleTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IIdleTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IIdleTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IIdleTrigger> for ::windows::core::IUnknown {
    fn from(value: &IIdleTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IIdleTrigger> for super::Com::IDispatch {
    fn from(value: IIdleTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IIdleTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a IIdleTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IIdleTrigger> for super::Com::IDispatch {
    fn from(value: &IIdleTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IIdleTrigger> for ITrigger {
    fn from(value: IIdleTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IIdleTrigger> for &'a ITrigger {
    fn from(value: &'a IIdleTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IIdleTrigger> for ITrigger {
    fn from(value: &IIdleTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IIdleTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IIdleTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IIdleTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IIdleTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIdleTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IIdleTrigger {
    type Vtable = IIdleTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd537d2b0_9fb3_4d34_9739_1ff5ce7b1ef3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IIdleTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ILogonTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ILogonTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDelay<'a, P0>(&self, delay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDelay)(::windows::core::Interface::as_raw(self), delay.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserId(&self, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UserId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(puser)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserId<'a, P0>(&self, user: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetUserId)(::windows::core::Interface::as_raw(self), user.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ILogonTrigger> for ::windows::core::IUnknown {
    fn from(value: ILogonTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ILogonTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILogonTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ILogonTrigger> for ::windows::core::IUnknown {
    fn from(value: &ILogonTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ILogonTrigger> for super::Com::IDispatch {
    fn from(value: ILogonTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ILogonTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a ILogonTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ILogonTrigger> for super::Com::IDispatch {
    fn from(value: &ILogonTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ILogonTrigger> for ITrigger {
    fn from(value: ILogonTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ILogonTrigger> for &'a ITrigger {
    fn from(value: &'a ILogonTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ILogonTrigger> for ITrigger {
    fn from(value: &ILogonTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ILogonTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ILogonTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ILogonTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ILogonTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILogonTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ILogonTrigger {
    type Vtable = ILogonTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72dade38_fae4_4b3e_baf4_5d009af02b1c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ILogonTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserId: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMaintenanceSettings(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMaintenanceSettings {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPeriod<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetPeriod)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Period(&self, target: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Period)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(target)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeadline<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDeadline)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Deadline(&self, target: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Deadline)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(target)).ok()
    }
    pub unsafe fn SetExclusive(&self, value: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExclusive)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Exclusive(&self, target: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Exclusive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(target)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMaintenanceSettings> for ::windows::core::IUnknown {
    fn from(value: IMaintenanceSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMaintenanceSettings> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMaintenanceSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMaintenanceSettings> for ::windows::core::IUnknown {
    fn from(value: &IMaintenanceSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMaintenanceSettings> for super::Com::IDispatch {
    fn from(value: IMaintenanceSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMaintenanceSettings> for &'a super::Com::IDispatch {
    fn from(value: &'a IMaintenanceSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMaintenanceSettings> for super::Com::IDispatch {
    fn from(value: &IMaintenanceSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMaintenanceSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMaintenanceSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMaintenanceSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMaintenanceSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMaintenanceSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMaintenanceSettings {
    type Vtable = IMaintenanceSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6024fa8_9652_4adb_a6bf_5cfcd877a7ba);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPeriod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPeriod: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Period: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Period: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDeadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDeadline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Deadline: usize,
    pub SetExclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT,
    pub Exclusive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMonthlyDOWTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMonthlyDOWTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    pub unsafe fn DaysOfWeek(&self, pdays: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DaysOfWeek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdays)).ok()
    }
    pub unsafe fn SetDaysOfWeek(&self, days: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDaysOfWeek)(::windows::core::Interface::as_raw(self), days).ok()
    }
    pub unsafe fn WeeksOfMonth(&self, pweeks: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WeeksOfMonth)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pweeks)).ok()
    }
    pub unsafe fn SetWeeksOfMonth(&self, weeks: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWeeksOfMonth)(::windows::core::Interface::as_raw(self), weeks).ok()
    }
    pub unsafe fn MonthsOfYear(&self, pmonths: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MonthsOfYear)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pmonths)).ok()
    }
    pub unsafe fn SetMonthsOfYear(&self, months: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMonthsOfYear)(::windows::core::Interface::as_raw(self), months).ok()
    }
    pub unsafe fn RunOnLastWeekOfMonth(&self, plastweek: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RunOnLastWeekOfMonth)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plastweek)).ok()
    }
    pub unsafe fn SetRunOnLastWeekOfMonth(&self, lastweek: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRunOnLastWeekOfMonth)(::windows::core::Interface::as_raw(self), lastweek).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RandomDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRandomDelay<'a, P0>(&self, randomdelay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetRandomDelay)(::windows::core::Interface::as_raw(self), randomdelay.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMonthlyDOWTrigger> for ::windows::core::IUnknown {
    fn from(value: IMonthlyDOWTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMonthlyDOWTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMonthlyDOWTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMonthlyDOWTrigger> for ::windows::core::IUnknown {
    fn from(value: &IMonthlyDOWTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMonthlyDOWTrigger> for super::Com::IDispatch {
    fn from(value: IMonthlyDOWTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMonthlyDOWTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a IMonthlyDOWTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMonthlyDOWTrigger> for super::Com::IDispatch {
    fn from(value: &IMonthlyDOWTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMonthlyDOWTrigger> for ITrigger {
    fn from(value: IMonthlyDOWTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMonthlyDOWTrigger> for &'a ITrigger {
    fn from(value: &'a IMonthlyDOWTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMonthlyDOWTrigger> for ITrigger {
    fn from(value: &IMonthlyDOWTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMonthlyDOWTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMonthlyDOWTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMonthlyDOWTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMonthlyDOWTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMonthlyDOWTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMonthlyDOWTrigger {
    type Vtable = IMonthlyDOWTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77d025a3_90fa_43aa_b52e_cda5499b946a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMonthlyDOWTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT,
    pub WeeksOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT,
    pub SetWeeksOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT,
    pub MonthsOfYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT,
    pub SetMonthsOfYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT,
    pub RunOnLastWeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastweek: *mut i16) -> ::windows::core::HRESULT,
    pub SetRunOnLastWeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastweek: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IMonthlyTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IMonthlyTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    pub unsafe fn DaysOfMonth(&self, pdays: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DaysOfMonth)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdays)).ok()
    }
    pub unsafe fn SetDaysOfMonth(&self, days: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDaysOfMonth)(::windows::core::Interface::as_raw(self), days).ok()
    }
    pub unsafe fn MonthsOfYear(&self, pmonths: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MonthsOfYear)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pmonths)).ok()
    }
    pub unsafe fn SetMonthsOfYear(&self, months: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMonthsOfYear)(::windows::core::Interface::as_raw(self), months).ok()
    }
    pub unsafe fn RunOnLastDayOfMonth(&self, plastday: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RunOnLastDayOfMonth)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plastday)).ok()
    }
    pub unsafe fn SetRunOnLastDayOfMonth(&self, lastday: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRunOnLastDayOfMonth)(::windows::core::Interface::as_raw(self), lastday).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RandomDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRandomDelay<'a, P0>(&self, randomdelay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetRandomDelay)(::windows::core::Interface::as_raw(self), randomdelay.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMonthlyTrigger> for ::windows::core::IUnknown {
    fn from(value: IMonthlyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMonthlyTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IMonthlyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMonthlyTrigger> for ::windows::core::IUnknown {
    fn from(value: &IMonthlyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMonthlyTrigger> for super::Com::IDispatch {
    fn from(value: IMonthlyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMonthlyTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a IMonthlyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMonthlyTrigger> for super::Com::IDispatch {
    fn from(value: &IMonthlyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IMonthlyTrigger> for ITrigger {
    fn from(value: IMonthlyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IMonthlyTrigger> for &'a ITrigger {
    fn from(value: &'a IMonthlyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IMonthlyTrigger> for ITrigger {
    fn from(value: &IMonthlyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IMonthlyTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMonthlyTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMonthlyTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMonthlyTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMonthlyTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IMonthlyTrigger {
    type Vtable = IMonthlyTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97c45ef1_6b02_4a1a_9c0e_1ebfba1500ac);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IMonthlyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdays: *mut i32) -> ::windows::core::HRESULT,
    pub SetDaysOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT,
    pub MonthsOfYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmonths: *mut i16) -> ::windows::core::HRESULT,
    pub SetMonthsOfYear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, months: i16) -> ::windows::core::HRESULT,
    pub RunOnLastDayOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastday: *mut i16) -> ::windows::core::HRESULT,
    pub SetRunOnLastDayOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastday: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct INetworkSettings(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl INetworkSettings {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetworkSettings> for ::windows::core::IUnknown {
    fn from(value: INetworkSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a INetworkSettings> for &'a ::windows::core::IUnknown {
    fn from(value: &'a INetworkSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetworkSettings> for ::windows::core::IUnknown {
    fn from(value: &INetworkSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<INetworkSettings> for super::Com::IDispatch {
    fn from(value: INetworkSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a INetworkSettings> for &'a super::Com::IDispatch {
    fn from(value: &'a INetworkSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&INetworkSettings> for super::Com::IDispatch {
    fn from(value: &INetworkSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for INetworkSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for INetworkSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for INetworkSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for INetworkSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetworkSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for INetworkSettings {
    type Vtable = INetworkSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f7dea84_c30b_4245_80b6_00e9f646f1b4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct INetworkSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrincipal(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrincipal {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisplayName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDisplayName)(::windows::core::Interface::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserId(&self, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UserId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(puser)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserId<'a, P0>(&self, user: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetUserId)(::windows::core::Interface::as_raw(self), user.into().abi()).ok()
    }
    pub unsafe fn LogonType(&self, plogon: *mut TASK_LOGON_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LogonType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(plogon)).ok()
    }
    pub unsafe fn SetLogonType(&self, logon: TASK_LOGON_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLogonType)(::windows::core::Interface::as_raw(self), logon).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupId(&self, pgroup: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GroupId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pgroup)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGroupId<'a, P0>(&self, group: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetGroupId)(::windows::core::Interface::as_raw(self), group.into().abi()).ok()
    }
    pub unsafe fn RunLevel(&self, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RunLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prunlevel)).ok()
    }
    pub unsafe fn SetRunLevel(&self, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRunLevel)(::windows::core::Interface::as_raw(self), runlevel).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPrincipal> for ::windows::core::IUnknown {
    fn from(value: IPrincipal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IPrincipal> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrincipal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPrincipal> for ::windows::core::IUnknown {
    fn from(value: &IPrincipal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPrincipal> for super::Com::IDispatch {
    fn from(value: IPrincipal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IPrincipal> for &'a super::Com::IDispatch {
    fn from(value: &'a IPrincipal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPrincipal> for super::Com::IDispatch {
    fn from(value: &IPrincipal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrincipal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrincipal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrincipal {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrincipal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrincipal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrincipal {
    type Vtable = IPrincipal_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd98d51e5_c9b4_496a_a9c1_18980261cf0f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrincipal_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisplayName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserId: usize,
    pub LogonType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plogon: *mut TASK_LOGON_TYPE) -> ::windows::core::HRESULT,
    pub SetLogonType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logon: TASK_LOGON_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgroup: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, group: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGroupId: usize,
    pub RunLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prunlevel: *mut TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT,
    pub SetRunLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runlevel: TASK_RUNLEVEL_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IPrincipal2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IPrincipal2 {
    pub unsafe fn ProcessTokenSidType(&self, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessTokenSidType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprocesstokensidtype)).ok()
    }
    pub unsafe fn SetProcessTokenSidType(&self, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProcessTokenSidType)(::windows::core::Interface::as_raw(self), processtokensidtype).ok()
    }
    pub unsafe fn RequiredPrivilegeCount(&self, pcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequiredPrivilegeCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_RequiredPrivilege(&self, index: i32, pprivilege: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).get_RequiredPrivilege)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(pprivilege)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddRequiredPrivilege<'a, P0>(&self, privilege: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AddRequiredPrivilege)(::windows::core::Interface::as_raw(self), privilege.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPrincipal2> for ::windows::core::IUnknown {
    fn from(value: IPrincipal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IPrincipal2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IPrincipal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPrincipal2> for ::windows::core::IUnknown {
    fn from(value: &IPrincipal2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPrincipal2> for super::Com::IDispatch {
    fn from(value: IPrincipal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IPrincipal2> for &'a super::Com::IDispatch {
    fn from(value: &'a IPrincipal2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPrincipal2> for super::Com::IDispatch {
    fn from(value: &IPrincipal2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IPrincipal2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPrincipal2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPrincipal2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPrincipal2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrincipal2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IPrincipal2 {
    type Vtable = IPrincipal2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x248919ae_e345_4a6d_8aeb_e0d3165c904e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IPrincipal2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ProcessTokenSidType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprocesstokensidtype: *mut TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT,
    pub SetProcessTokenSidType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processtokensidtype: TASK_PROCESSTOKENSID_TYPE) -> ::windows::core::HRESULT,
    pub RequiredPrivilegeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RequiredPrivilege: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pprivilege: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RequiredPrivilege: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddRequiredPrivilege: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, privilege: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddRequiredPrivilege: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
pub struct IProvideTaskPage(::windows::core::IUnknown);
impl IProvideTaskPage {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub unsafe fn GetPage<'a, P0>(&self, tptype: TASKPAGE, fpersistchanges: P0) -> ::windows::core::Result<super::super::UI::Controls::HPROPSHEETPAGE>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPage)(::windows::core::Interface::as_raw(self), tptype, fpersistchanges.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Controls::HPROPSHEETPAGE>(result__)
    }
}
impl ::core::convert::From<IProvideTaskPage> for ::windows::core::IUnknown {
    fn from(value: IProvideTaskPage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProvideTaskPage> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProvideTaskPage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProvideTaskPage> for ::windows::core::IUnknown {
    fn from(value: &IProvideTaskPage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IProvideTaskPage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProvideTaskPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideTaskPage {}
impl ::core::fmt::Debug for IProvideTaskPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideTaskPage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProvideTaskPage {
    type Vtable = IProvideTaskPage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4086658a_cbbb_11cf_b604_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideTaskPage_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub GetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tptype: TASKPAGE, fpersistchanges: super::super::Foundation::BOOL, phpage: *mut super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls")))]
    GetPage: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRegisteredTask(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRegisteredTask {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<TASK_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TASK_STATE>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Run<'a, P0>(&self, params: P0) -> ::windows::core::Result<IRunningTask>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Run)(::windows::core::Interface::as_raw(self), params.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTask>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RunEx<'a, P0, P1>(&self, params: P0, flags: i32, sessionid: i32, user: P1) -> ::windows::core::Result<IRunningTask>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RunEx)(::windows::core::Interface::as_raw(self), params.into().abi(), flags, sessionid, user.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTask>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInstances(&self, flags: i32) -> ::windows::core::Result<IRunningTaskCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInstances)(::windows::core::Interface::as_raw(self), flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTaskCollection>(result__)
    }
    pub unsafe fn LastRunTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastRunTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    pub unsafe fn LastTaskResult(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LastTaskResult)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn NumberOfMissedRuns(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NumberOfMissedRuns)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn NextRunTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NextRunTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<f64>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Definition(&self) -> ::windows::core::Result<ITaskDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Definition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Xml(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Xml)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurityDescriptor)(::windows::core::Interface::as_raw(self), securityinformation, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecurityDescriptor<'a, P0>(&self, sddl: P0, flags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSecurityDescriptor)(::windows::core::Interface::as_raw(self), sddl.into().abi(), flags).ok()
    }
    pub unsafe fn Stop(&self, flags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRunTimes(&self, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRunTimes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pststart), ::core::mem::transmute(pstend), ::core::mem::transmute(pcount), ::core::mem::transmute(pruntimes)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRegisteredTask> for ::windows::core::IUnknown {
    fn from(value: IRegisteredTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRegisteredTask> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRegisteredTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRegisteredTask> for ::windows::core::IUnknown {
    fn from(value: &IRegisteredTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRegisteredTask> for super::Com::IDispatch {
    fn from(value: IRegisteredTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRegisteredTask> for &'a super::Com::IDispatch {
    fn from(value: &'a IRegisteredTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRegisteredTask> for super::Com::IDispatch {
    fn from(value: &IRegisteredTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRegisteredTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRegisteredTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRegisteredTask {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRegisteredTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegisteredTask").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRegisteredTask {
    type Vtable = IRegisteredTask_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c86f320_dee3_4dd1_b972_a303f26b061e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredTask_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Run: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RunEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, params: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, sessionid: i32, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RunEx: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInstances: usize,
    pub LastRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plastruntime: *mut f64) -> ::windows::core::HRESULT,
    pub LastTaskResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plasttaskresult: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfMissedRuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumberofmissedruns: *mut i32) -> ::windows::core::HRESULT,
    pub NextRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnextruntime: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Definition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Definition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Xml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Xml: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurityDescriptor: usize,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRunTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pststart: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u32, pruntimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRunTimes: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRegisteredTaskCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRegisteredTaskCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, P0>(&self, index: P0) -> ::windows::core::Result<IRegisteredTask>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTask>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRegisteredTaskCollection> for ::windows::core::IUnknown {
    fn from(value: IRegisteredTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRegisteredTaskCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRegisteredTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRegisteredTaskCollection> for ::windows::core::IUnknown {
    fn from(value: &IRegisteredTaskCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRegisteredTaskCollection> for super::Com::IDispatch {
    fn from(value: IRegisteredTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRegisteredTaskCollection> for &'a super::Com::IDispatch {
    fn from(value: &'a IRegisteredTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRegisteredTaskCollection> for super::Com::IDispatch {
    fn from(value: &IRegisteredTaskCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRegisteredTaskCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRegisteredTaskCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRegisteredTaskCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRegisteredTaskCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegisteredTaskCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRegisteredTaskCollection {
    type Vtable = IRegisteredTaskCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86627eb4_42a7_41e4_a4d9_ac33a72f2d52);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredTaskCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppregisteredtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRegistrationInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRegistrationInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Description)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdescription)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, P0>(&self, description: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDescription)(::windows::core::Interface::as_raw(self), description.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Author(&self, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Author)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pauthor)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAuthor<'a, P0>(&self, author: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetAuthor)(::windows::core::Interface::as_raw(self), author.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Version)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pversion)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, P0>(&self, version: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetVersion)(::windows::core::Interface::as_raw(self), version.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Date(&self, pdate: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Date)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDate<'a, P0>(&self, date: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDate)(::windows::core::Interface::as_raw(self), date.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Documentation(&self, pdocumentation: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Documentation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdocumentation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDocumentation<'a, P0>(&self, documentation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDocumentation)(::windows::core::Interface::as_raw(self), documentation.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn XmlText(&self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).XmlText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXmlText<'a, P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetXmlText)(::windows::core::Interface::as_raw(self), text.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn URI(&self, puri: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).URI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(puri)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetURI<'a, P0>(&self, uri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetURI)(::windows::core::Interface::as_raw(self), uri.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SecurityDescriptor(&self, psddl: *mut super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SecurityDescriptor)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psddl)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSecurityDescriptor<'a, P0>(&self, sddl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSecurityDescriptor)(::windows::core::Interface::as_raw(self), sddl.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Source(&self, psource: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Source)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psource)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSource<'a, P0>(&self, source: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSource)(::windows::core::Interface::as_raw(self), source.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRegistrationInfo> for ::windows::core::IUnknown {
    fn from(value: IRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRegistrationInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRegistrationInfo> for ::windows::core::IUnknown {
    fn from(value: &IRegistrationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRegistrationInfo> for super::Com::IDispatch {
    fn from(value: IRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRegistrationInfo> for &'a super::Com::IDispatch {
    fn from(value: &'a IRegistrationInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRegistrationInfo> for super::Com::IDispatch {
    fn from(value: &IRegistrationInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRegistrationInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRegistrationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRegistrationInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRegistrationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegistrationInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRegistrationInfo {
    type Vtable = IRegistrationInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x416d8b73_cb41_4ea1_805c_9be9a5ac4a74);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegistrationInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauthor: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Author: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAuthor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, author: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAuthor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Version: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVersion: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Date: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, date: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDate: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Documentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdocumentation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Documentation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDocumentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentation: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDocumentation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub XmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    XmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub URI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    URI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetURI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetURI: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psddl: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SecurityDescriptor: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Source: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSource: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRegistrationTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRegistrationTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDelay<'a, P0>(&self, delay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDelay)(::windows::core::Interface::as_raw(self), delay.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRegistrationTrigger> for ::windows::core::IUnknown {
    fn from(value: IRegistrationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRegistrationTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRegistrationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRegistrationTrigger> for ::windows::core::IUnknown {
    fn from(value: &IRegistrationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRegistrationTrigger> for super::Com::IDispatch {
    fn from(value: IRegistrationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRegistrationTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a IRegistrationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRegistrationTrigger> for super::Com::IDispatch {
    fn from(value: &IRegistrationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRegistrationTrigger> for ITrigger {
    fn from(value: IRegistrationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRegistrationTrigger> for &'a ITrigger {
    fn from(value: &'a IRegistrationTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRegistrationTrigger> for ITrigger {
    fn from(value: &IRegistrationTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRegistrationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRegistrationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRegistrationTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRegistrationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegistrationTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRegistrationTrigger {
    type Vtable = IRegistrationTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c8fec3a_c218_4e0c_b23d_629024db91a2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRegistrationTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRepetitionPattern(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRepetitionPattern {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Interval(&self, pinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Interval)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinterval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInterval<'a, P0>(&self, interval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetInterval)(::windows::core::Interface::as_raw(self), interval.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Duration(&self, pduration: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Duration)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pduration)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDuration<'a, P0>(&self, duration: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDuration)(::windows::core::Interface::as_raw(self), duration.into().abi()).ok()
    }
    pub unsafe fn StopAtDurationEnd(&self, pstop: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopAtDurationEnd)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstop)).ok()
    }
    pub unsafe fn SetStopAtDurationEnd(&self, stop: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStopAtDurationEnd)(::windows::core::Interface::as_raw(self), stop).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRepetitionPattern> for ::windows::core::IUnknown {
    fn from(value: IRepetitionPattern) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRepetitionPattern> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRepetitionPattern) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRepetitionPattern> for ::windows::core::IUnknown {
    fn from(value: &IRepetitionPattern) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRepetitionPattern> for super::Com::IDispatch {
    fn from(value: IRepetitionPattern) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRepetitionPattern> for &'a super::Com::IDispatch {
    fn from(value: &'a IRepetitionPattern) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRepetitionPattern> for super::Com::IDispatch {
    fn from(value: &IRepetitionPattern) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRepetitionPattern {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRepetitionPattern {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRepetitionPattern {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRepetitionPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRepetitionPattern").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRepetitionPattern {
    type Vtable = IRepetitionPattern_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fb9acf1_26be_400e_85b5_294b9c75dfd6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRepetitionPattern_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Interval: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInterval: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduration: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Duration: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDuration: usize,
    pub StopAtDurationEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstop: *mut i16) -> ::windows::core::HRESULT,
    pub SetStopAtDurationEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRunningTask(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRunningTask {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InstanceGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).InstanceGuid)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<TASK_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TASK_STATE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CurrentAction(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CurrentAction)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Refresh)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnginePID(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnginePID)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRunningTask> for ::windows::core::IUnknown {
    fn from(value: IRunningTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRunningTask> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRunningTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRunningTask> for ::windows::core::IUnknown {
    fn from(value: &IRunningTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRunningTask> for super::Com::IDispatch {
    fn from(value: IRunningTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRunningTask> for &'a super::Com::IDispatch {
    fn from(value: &'a IRunningTask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRunningTask> for super::Com::IDispatch {
    fn from(value: &IRunningTask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRunningTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRunningTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRunningTask {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRunningTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunningTask").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRunningTask {
    type Vtable = IRunningTask_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x653758fb_7b9a_4f1e_a471_beeb8e9b834e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRunningTask_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InstanceGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InstanceGuid: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut TASK_STATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CurrentAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CurrentAction: usize,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnginePID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRunningTaskCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRunningTaskCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, P0>(&self, index: P0) -> ::windows::core::Result<IRunningTask>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTask>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRunningTaskCollection> for ::windows::core::IUnknown {
    fn from(value: IRunningTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRunningTaskCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRunningTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRunningTaskCollection> for ::windows::core::IUnknown {
    fn from(value: &IRunningTaskCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRunningTaskCollection> for super::Com::IDispatch {
    fn from(value: IRunningTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRunningTaskCollection> for &'a super::Com::IDispatch {
    fn from(value: &'a IRunningTaskCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRunningTaskCollection> for super::Com::IDispatch {
    fn from(value: &IRunningTaskCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRunningTaskCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRunningTaskCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRunningTaskCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRunningTaskCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRunningTaskCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRunningTaskCollection {
    type Vtable = IRunningTaskCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a67614b_6828_4fec_aa54_6d52e8f1f2db);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRunningTaskCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pprunningtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
pub struct IScheduledWorkItem(::windows::core::IUnknown);
impl IScheduledWorkItem {
    pub unsafe fn CreateTrigger(&self, pinewtrigger: *mut u16, pptrigger: *mut ::core::option::Option<ITaskTrigger>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateTrigger)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinewtrigger), ::core::mem::transmute(pptrigger)).ok()
    }
    pub unsafe fn DeleteTrigger(&self, itrigger: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteTrigger)(::windows::core::Interface::as_raw(self), itrigger).ok()
    }
    pub unsafe fn GetTriggerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTriggerCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetTrigger(&self, itrigger: u16) -> ::windows::core::Result<ITaskTrigger> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTrigger)(::windows::core::Interface::as_raw(self), itrigger, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskTrigger>(result__)
    }
    pub unsafe fn GetTriggerString(&self, itrigger: u16) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTriggerString)(::windows::core::Interface::as_raw(self), itrigger, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRunTimes(&self, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRunTimes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstbegin), ::core::mem::transmute(pstend), ::core::mem::transmute(pcount), ::core::mem::transmute(rgsttasktimes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextRunTime(&self, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextRunTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstnextrun)).ok()
    }
    pub unsafe fn SetIdleWait(&self, widleminutes: u16, wdeadlineminutes: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIdleWait)(::windows::core::Interface::as_raw(self), widleminutes, wdeadlineminutes).ok()
    }
    pub unsafe fn GetIdleWait(&self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIdleWait)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwidleminutes), ::core::mem::transmute(pwdeadlineminutes)).ok()
    }
    pub unsafe fn Run(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Run)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Terminate)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditWorkItem<'a, P0>(&self, hparent: P0, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).EditWorkItem)(::windows::core::Interface::as_raw(self), hparent.into(), dwreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMostRecentRunTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMostRecentRunTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetExitCode(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetExitCode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetComment<'a, P0>(&self, pwszcomment: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetComment)(::windows::core::Interface::as_raw(self), pwszcomment.into()).ok()
    }
    pub unsafe fn GetComment(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetComment)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetCreator<'a, P0>(&self, pwszcreator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetCreator)(::windows::core::Interface::as_raw(self), pwszcreator.into()).ok()
    }
    pub unsafe fn GetCreator(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCreator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetWorkItemData(&self, cbdata: u16, rgbdata: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWorkItemData)(::windows::core::Interface::as_raw(self), cbdata, ::core::mem::transmute(rgbdata)).ok()
    }
    pub unsafe fn GetWorkItemData(&self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWorkItemData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcbdata), ::core::mem::transmute(prgbdata)).ok()
    }
    pub unsafe fn SetErrorRetryCount(&self, wretrycount: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetErrorRetryCount)(::windows::core::Interface::as_raw(self), wretrycount).ok()
    }
    pub unsafe fn GetErrorRetryCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorRetryCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetErrorRetryInterval(&self, wretryinterval: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetErrorRetryInterval)(::windows::core::Interface::as_raw(self), wretryinterval).ok()
    }
    pub unsafe fn GetErrorRetryInterval(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetErrorRetryInterval)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetFlags(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlags)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAccountInformation<'a, P0, P1>(&self, pwszaccountname: P0, pwszpassword: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAccountInformation)(::windows::core::Interface::as_raw(self), pwszaccountname.into(), pwszpassword.into()).ok()
    }
    pub unsafe fn GetAccountInformation(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAccountInformation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<IScheduledWorkItem> for ::windows::core::IUnknown {
    fn from(value: IScheduledWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IScheduledWorkItem> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IScheduledWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IScheduledWorkItem> for ::windows::core::IUnknown {
    fn from(value: &IScheduledWorkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IScheduledWorkItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IScheduledWorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScheduledWorkItem {}
impl ::core::fmt::Debug for IScheduledWorkItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScheduledWorkItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IScheduledWorkItem {
    type Vtable = IScheduledWorkItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6b952f0_a4b1_11d0_997d_00aa006887ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScheduledWorkItem_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinewtrigger: *mut u16, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itrigger: u16) -> ::windows::core::HRESULT,
    pub GetTriggerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT,
    pub GetTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itrigger: u16, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTriggerString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itrigger: u16, ppwsztrigger: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRunTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRunTimes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNextRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNextRunTime: usize,
    pub SetIdleWait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, widleminutes: u16, wdeadlineminutes: u16) -> ::windows::core::HRESULT,
    pub GetIdleWait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EditWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EditWorkItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMostRecentRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstlastrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMostRecentRunTime: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwexitcode: *mut u32) -> ::windows::core::HRESULT,
    pub SetComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszcomment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcreator: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszcreator: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetWorkItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbdata: u16, rgbdata: *const u8) -> ::windows::core::HRESULT,
    pub GetWorkItemData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub SetErrorRetryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wretrycount: u16) -> ::windows::core::HRESULT,
    pub GetErrorRetryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwretrycount: *mut u16) -> ::windows::core::HRESULT,
    pub SetErrorRetryInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wretryinterval: u16) -> ::windows::core::HRESULT,
    pub GetErrorRetryInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwretryinterval: *mut u16) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub SetAccountInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszaccountname: ::windows::core::PCWSTR, pwszpassword: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetAccountInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszaccountname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISessionStateChangeTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISessionStateChangeTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Delay(&self, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Delay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDelay<'a, P0>(&self, delay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDelay)(::windows::core::Interface::as_raw(self), delay.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserId(&self, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UserId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(puser)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserId<'a, P0>(&self, user: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetUserId)(::windows::core::Interface::as_raw(self), user.into().abi()).ok()
    }
    pub unsafe fn StateChange(&self, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StateChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    pub unsafe fn SetStateChange(&self, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStateChange)(::windows::core::Interface::as_raw(self), r#type).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISessionStateChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: ISessionStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISessionStateChangeTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISessionStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISessionStateChangeTrigger> for ::windows::core::IUnknown {
    fn from(value: &ISessionStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISessionStateChangeTrigger> for super::Com::IDispatch {
    fn from(value: ISessionStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISessionStateChangeTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a ISessionStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISessionStateChangeTrigger> for super::Com::IDispatch {
    fn from(value: &ISessionStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISessionStateChangeTrigger> for ITrigger {
    fn from(value: ISessionStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ISessionStateChangeTrigger> for &'a ITrigger {
    fn from(value: &'a ISessionStateChangeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISessionStateChangeTrigger> for ITrigger {
    fn from(value: &ISessionStateChangeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISessionStateChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISessionStateChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISessionStateChangeTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISessionStateChangeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISessionStateChangeTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISessionStateChangeTrigger {
    type Vtable = ISessionStateChangeTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x754da71b_4385_4475_9dd9_598294fa3641);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISessionStateChangeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Delay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, delay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserId: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserId: usize,
    pub StateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT,
    pub SetStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TASK_SESSION_STATE_CHANGE_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IShowMessageAction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IShowMessageAction {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    pub unsafe fn Type(&self, ptype: *mut TASK_ACTION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Title(&self, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Title)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptitle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTitle<'a, P0>(&self, title: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetTitle)(::windows::core::Interface::as_raw(self), title.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageBody(&self, pmessagebody: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MessageBody)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pmessagebody)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageBody<'a, P0>(&self, messagebody: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetMessageBody)(::windows::core::Interface::as_raw(self), messagebody.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IShowMessageAction> for ::windows::core::IUnknown {
    fn from(value: IShowMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IShowMessageAction> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IShowMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IShowMessageAction> for ::windows::core::IUnknown {
    fn from(value: &IShowMessageAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IShowMessageAction> for super::Com::IDispatch {
    fn from(value: IShowMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IShowMessageAction> for &'a super::Com::IDispatch {
    fn from(value: &'a IShowMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IShowMessageAction> for super::Com::IDispatch {
    fn from(value: &IShowMessageAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IShowMessageAction> for IAction {
    fn from(value: IShowMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IShowMessageAction> for &'a IAction {
    fn from(value: &'a IShowMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IShowMessageAction> for IAction {
    fn from(value: &IShowMessageAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IShowMessageAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IShowMessageAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IShowMessageAction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IShowMessageAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShowMessageAction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IShowMessageAction {
    type Vtable = IShowMessageAction_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x505e9e68_af89_46b8_a30f_56162a83d537);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IShowMessageAction_Vtbl {
    pub base__: IAction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptitle: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Title: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTitle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MessageBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmessagebody: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MessageBody: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMessageBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagebody: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMessageBody: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
pub struct ITask(::windows::core::IUnknown);
impl ITask {
    pub unsafe fn CreateTrigger(&self, pinewtrigger: *mut u16, pptrigger: *mut ::core::option::Option<ITaskTrigger>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CreateTrigger)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pinewtrigger), ::core::mem::transmute(pptrigger)).ok()
    }
    pub unsafe fn DeleteTrigger(&self, itrigger: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteTrigger)(::windows::core::Interface::as_raw(self), itrigger).ok()
    }
    pub unsafe fn GetTriggerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTriggerCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn GetTrigger(&self, itrigger: u16) -> ::windows::core::Result<ITaskTrigger> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTrigger)(::windows::core::Interface::as_raw(self), itrigger, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskTrigger>(result__)
    }
    pub unsafe fn GetTriggerString(&self, itrigger: u16) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetTriggerString)(::windows::core::Interface::as_raw(self), itrigger, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRunTimes(&self, pstbegin: *const super::super::Foundation::SYSTEMTIME, pstend: *const super::super::Foundation::SYSTEMTIME, pcount: *mut u16, rgsttasktimes: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRunTimes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstbegin), ::core::mem::transmute(pstend), ::core::mem::transmute(pcount), ::core::mem::transmute(rgsttasktimes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextRunTime(&self, pstnextrun: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNextRunTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstnextrun)).ok()
    }
    pub unsafe fn SetIdleWait(&self, widleminutes: u16, wdeadlineminutes: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetIdleWait)(::windows::core::Interface::as_raw(self), widleminutes, wdeadlineminutes).ok()
    }
    pub unsafe fn GetIdleWait(&self, pwidleminutes: *mut u16, pwdeadlineminutes: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIdleWait)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwidleminutes), ::core::mem::transmute(pwdeadlineminutes)).ok()
    }
    pub unsafe fn Run(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Run)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Terminate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Terminate)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EditWorkItem<'a, P0>(&self, hparent: P0, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.EditWorkItem)(::windows::core::Interface::as_raw(self), hparent.into(), dwreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMostRecentRunTime(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetMostRecentRunTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::SYSTEMTIME>(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn GetExitCode(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetExitCode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetComment<'a, P0>(&self, pwszcomment: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetComment)(::windows::core::Interface::as_raw(self), pwszcomment.into()).ok()
    }
    pub unsafe fn GetComment(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetComment)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetCreator<'a, P0>(&self, pwszcreator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetCreator)(::windows::core::Interface::as_raw(self), pwszcreator.into()).ok()
    }
    pub unsafe fn GetCreator(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCreator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetWorkItemData(&self, cbdata: u16, rgbdata: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetWorkItemData)(::windows::core::Interface::as_raw(self), cbdata, ::core::mem::transmute(rgbdata)).ok()
    }
    pub unsafe fn GetWorkItemData(&self, pcbdata: *mut u16, prgbdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetWorkItemData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcbdata), ::core::mem::transmute(prgbdata)).ok()
    }
    pub unsafe fn SetErrorRetryCount(&self, wretrycount: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetErrorRetryCount)(::windows::core::Interface::as_raw(self), wretrycount).ok()
    }
    pub unsafe fn GetErrorRetryCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetErrorRetryCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetErrorRetryInterval(&self, wretryinterval: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetErrorRetryInterval)(::windows::core::Interface::as_raw(self), wretryinterval).ok()
    }
    pub unsafe fn GetErrorRetryInterval(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetErrorRetryInterval)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u16>(result__)
    }
    pub unsafe fn SetFlags(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFlags)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetAccountInformation<'a, P0, P1>(&self, pwszaccountname: P0, pwszpassword: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetAccountInformation)(::windows::core::Interface::as_raw(self), pwszaccountname.into(), pwszpassword.into()).ok()
    }
    pub unsafe fn GetAccountInformation(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetAccountInformation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetApplicationName<'a, P0>(&self, pwszapplicationname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetApplicationName)(::windows::core::Interface::as_raw(self), pwszapplicationname.into()).ok()
    }
    pub unsafe fn GetApplicationName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetApplicationName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetParameters<'a, P0>(&self, pwszparameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetParameters)(::windows::core::Interface::as_raw(self), pwszparameters.into()).ok()
    }
    pub unsafe fn GetParameters(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetWorkingDirectory<'a, P0>(&self, pwszworkingdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetWorkingDirectory)(::windows::core::Interface::as_raw(self), pwszworkingdirectory.into()).ok()
    }
    pub unsafe fn GetWorkingDirectory(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetWorkingDirectory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn SetPriority(&self, dwpriority: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), dwpriority).ok()
    }
    pub unsafe fn GetPriority(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetTaskFlags(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTaskFlags)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetTaskFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTaskFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMaxRunTime(&self, dwmaxruntimems: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxRunTime)(::windows::core::Interface::as_raw(self), dwmaxruntimems).ok()
    }
    pub unsafe fn GetMaxRunTime(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMaxRunTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<ITask> for ::windows::core::IUnknown {
    fn from(value: ITask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITask> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITask> for ::windows::core::IUnknown {
    fn from(value: &ITask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ITask> for IScheduledWorkItem {
    fn from(value: ITask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITask> for &'a IScheduledWorkItem {
    fn from(value: &'a ITask) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITask> for IScheduledWorkItem {
    fn from(value: &ITask) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITask {}
impl ::core::fmt::Debug for ITask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITask").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITask {
    type Vtable = ITask_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x148bd524_a2ab_11ce_b11f_00aa00530503);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITask_Vtbl {
    pub base__: IScheduledWorkItem_Vtbl,
    pub SetApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszapplicationname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetApplicationName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszapplicationname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszparameters: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszparameters: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszworkingdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszworkingdirectory: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwpriority: u32) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpriority: *mut u32) -> ::windows::core::HRESULT,
    pub SetTaskFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetTaskFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmaxruntimems: u32) -> ::windows::core::HRESULT,
    pub GetMaxRunTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwmaxruntimems: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITaskDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITaskDefinition {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RegistrationInfo(&self) -> ::windows::core::Result<IRegistrationInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RegistrationInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegistrationInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRegistrationInfo<'a, P0>(&self, pregistrationinfo: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRegistrationInfo>>,
    {
        (::windows::core::Interface::vtable(self).SetRegistrationInfo)(::windows::core::Interface::as_raw(self), pregistrationinfo.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Triggers(&self) -> ::windows::core::Result<ITriggerCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Triggers)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITriggerCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTriggers<'a, P0>(&self, ptriggers: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITriggerCollection>>,
    {
        (::windows::core::Interface::vtable(self).SetTriggers)(::windows::core::Interface::as_raw(self), ptriggers.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Settings(&self) -> ::windows::core::Result<ITaskSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Settings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSettings<'a, P0>(&self, psettings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ITaskSettings>>,
    {
        (::windows::core::Interface::vtable(self).SetSettings)(::windows::core::Interface::as_raw(self), psettings.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Data(&self, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Data)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdata)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetData<'a, P0>(&self, data: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), data.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Principal(&self) -> ::windows::core::Result<IPrincipal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Principal)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IPrincipal>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPrincipal<'a, P0>(&self, pprincipal: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IPrincipal>>,
    {
        (::windows::core::Interface::vtable(self).SetPrincipal)(::windows::core::Interface::as_raw(self), pprincipal.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Actions(&self) -> ::windows::core::Result<IActionCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Actions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IActionCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetActions<'a, P0>(&self, pactions: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IActionCollection>>,
    {
        (::windows::core::Interface::vtable(self).SetActions)(::windows::core::Interface::as_raw(self), pactions.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn XmlText(&self, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).XmlText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pxml)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXmlText<'a, P0>(&self, xml: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetXmlText)(::windows::core::Interface::as_raw(self), xml.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskDefinition> for ::windows::core::IUnknown {
    fn from(value: ITaskDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskDefinition> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskDefinition> for ::windows::core::IUnknown {
    fn from(value: &ITaskDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskDefinition> for super::Com::IDispatch {
    fn from(value: ITaskDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskDefinition> for &'a super::Com::IDispatch {
    fn from(value: &'a ITaskDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskDefinition> for super::Com::IDispatch {
    fn from(value: &ITaskDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITaskDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITaskDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITaskDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITaskDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITaskDefinition {
    type Vtable = ITaskDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5bc8fc5_536d_4f77_b852_fbc1356fdeb6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskDefinition_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RegistrationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppregistrationinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RegistrationInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRegistrationInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pregistrationinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRegistrationInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Triggers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptriggers: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Triggers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTriggers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptriggers: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTriggers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSettings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Data: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Principal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprincipal: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Principal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPrincipal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprincipal: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPrincipal: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppactions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Actions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pactions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetActions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub XmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    XmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXmlText: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITaskFolder(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITaskFolder {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Path)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFolder<'a, P0>(&self, path: P0) -> ::windows::core::Result<ITaskFolder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFolder)(::windows::core::Interface::as_raw(self), path.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolder>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFolders(&self, flags: i32) -> ::windows::core::Result<ITaskFolderCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFolders)(::windows::core::Interface::as_raw(self), flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolderCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateFolder<'a, P0, P1>(&self, subfoldername: P0, sddl: P1) -> ::windows::core::Result<ITaskFolder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateFolder)(::windows::core::Interface::as_raw(self), subfoldername.into().abi(), sddl.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolder>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteFolder<'a, P0>(&self, subfoldername: P0, flags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).DeleteFolder)(::windows::core::Interface::as_raw(self), subfoldername.into().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetTask<'a, P0>(&self, path: P0) -> ::windows::core::Result<IRegisteredTask>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTask)(::windows::core::Interface::as_raw(self), path.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTask>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTasks(&self, flags: i32) -> ::windows::core::Result<IRegisteredTaskCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTasks)(::windows::core::Interface::as_raw(self), flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTaskCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteTask<'a, P0>(&self, name: P0, flags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).DeleteTask)(::windows::core::Interface::as_raw(self), name.into().abi(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterTask<'a, P0, P1, P2, P3, P4>(&self, path: P0, xmltext: P1, flags: i32, userid: P2, password: P3, logontype: TASK_LOGON_TYPE, sddl: P4) -> ::windows::core::Result<IRegisteredTask>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RegisterTask)(::windows::core::Interface::as_raw(self), path.into().abi(), xmltext.into().abi(), flags, userid.into().abi(), password.into().abi(), logontype, sddl.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTask>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RegisterTaskDefinition<'a, P0, P1, P2, P3, P4>(&self, path: P0, pdefinition: P1, flags: i32, userid: P2, password: P3, logontype: TASK_LOGON_TYPE, sddl: P4) -> ::windows::core::Result<IRegisteredTask>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ITaskDefinition>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
        P4: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RegisterTaskDefinition)(::windows::core::Interface::as_raw(self), path.into().abi(), pdefinition.into().abi(), flags, userid.into().abi(), password.into().abi(), logontype, sddl.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredTask>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecurityDescriptor(&self, securityinformation: i32) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSecurityDescriptor)(::windows::core::Interface::as_raw(self), securityinformation, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSecurityDescriptor<'a, P0>(&self, sddl: P0, flags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetSecurityDescriptor)(::windows::core::Interface::as_raw(self), sddl.into().abi(), flags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskFolder> for ::windows::core::IUnknown {
    fn from(value: ITaskFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskFolder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskFolder> for ::windows::core::IUnknown {
    fn from(value: &ITaskFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskFolder> for super::Com::IDispatch {
    fn from(value: ITaskFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskFolder> for &'a super::Com::IDispatch {
    fn from(value: &'a ITaskFolder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskFolder> for super::Com::IDispatch {
    fn from(value: &ITaskFolder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITaskFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITaskFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITaskFolder {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITaskFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskFolder").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITaskFolder {
    type Vtable = ITaskFolder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cfac062_a080_4c15_9a88_aa7c2af80dfc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskFolder_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Path: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFolder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, ppfolders: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFolders: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateFolder: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subfoldername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteFolder: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetTask: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pptasks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTasks: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, xmltext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RegisterTaskDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdefinition: *mut ::core::ffi::c_void, flags: i32, userid: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>, logontype: TASK_LOGON_TYPE, sddl: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RegisterTaskDefinition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securityinformation: i32, psddl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sddl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSecurityDescriptor: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITaskFolderCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITaskFolderCollection {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, P0>(&self, index: P0) -> ::windows::core::Result<ITaskFolder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolder>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskFolderCollection> for ::windows::core::IUnknown {
    fn from(value: ITaskFolderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskFolderCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskFolderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskFolderCollection> for ::windows::core::IUnknown {
    fn from(value: &ITaskFolderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskFolderCollection> for super::Com::IDispatch {
    fn from(value: ITaskFolderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskFolderCollection> for &'a super::Com::IDispatch {
    fn from(value: &'a ITaskFolderCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskFolderCollection> for super::Com::IDispatch {
    fn from(value: &ITaskFolderCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITaskFolderCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITaskFolderCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITaskFolderCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITaskFolderCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskFolderCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITaskFolderCollection {
    type Vtable = ITaskFolderCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79184a66_8664_423f_97f1_637356a5d812);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskFolderCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
pub struct ITaskHandler(::windows::core::IUnknown);
impl ITaskHandler {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<'a, P0, P1>(&self, phandlerservices: P0, data: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self), phandlerservices.into().abi(), data.into().abi()).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::HRESULT>(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ITaskHandler> for ::windows::core::IUnknown {
    fn from(value: ITaskHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITaskHandler> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskHandler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskHandler> for ::windows::core::IUnknown {
    fn from(value: &ITaskHandler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITaskHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskHandler {}
impl ::core::fmt::Debug for ITaskHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITaskHandler {
    type Vtable = ITaskHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x839d7762_5121_4009_9234_4f0d19394f04);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandlerservices: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
pub struct ITaskHandlerStatus(::windows::core::IUnknown);
impl ITaskHandlerStatus {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateStatus<'a, P0>(&self, percentcomplete: i16, statusmessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).UpdateStatus)(::windows::core::Interface::as_raw(self), percentcomplete, statusmessage.into().abi()).ok()
    }
    pub unsafe fn TaskCompleted(&self, taskerrcode: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TaskCompleted)(::windows::core::Interface::as_raw(self), taskerrcode).ok()
    }
}
impl ::core::convert::From<ITaskHandlerStatus> for ::windows::core::IUnknown {
    fn from(value: ITaskHandlerStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITaskHandlerStatus> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskHandlerStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskHandlerStatus> for ::windows::core::IUnknown {
    fn from(value: &ITaskHandlerStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITaskHandlerStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskHandlerStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskHandlerStatus {}
impl ::core::fmt::Debug for ITaskHandlerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskHandlerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITaskHandlerStatus {
    type Vtable = ITaskHandlerStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeaec7a8f_27a0_4ddc_8675_14726a01a38a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskHandlerStatus_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, percentcomplete: i16, statusmessage: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateStatus: usize,
    pub TaskCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskerrcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITaskNamedValueCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITaskNamedValueCollection {
    pub unsafe fn Count(&self, pcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<ITaskNamedValuePair> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskNamedValuePair>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Create<'a, P0, P1>(&self, name: P0, value: P1) -> ::windows::core::Result<ITaskNamedValuePair>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), name.into().abi(), value.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskNamedValuePair>(result__)
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskNamedValueCollection> for ::windows::core::IUnknown {
    fn from(value: ITaskNamedValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskNamedValueCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskNamedValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskNamedValueCollection> for ::windows::core::IUnknown {
    fn from(value: &ITaskNamedValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskNamedValueCollection> for super::Com::IDispatch {
    fn from(value: ITaskNamedValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskNamedValueCollection> for &'a super::Com::IDispatch {
    fn from(value: &'a ITaskNamedValueCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskNamedValueCollection> for super::Com::IDispatch {
    fn from(value: &ITaskNamedValueCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITaskNamedValueCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITaskNamedValueCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITaskNamedValueCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITaskNamedValueCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskNamedValueCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITaskNamedValueCollection {
    type Vtable = ITaskNamedValueCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4ef826b_63c3_46e4_a504_ef69e4f7ea4d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskNamedValueCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pppair: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pppair: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Create: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITaskNamedValuePair(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITaskNamedValuePair {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pname)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self, pvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), value.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskNamedValuePair> for ::windows::core::IUnknown {
    fn from(value: ITaskNamedValuePair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskNamedValuePair> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskNamedValuePair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskNamedValuePair> for ::windows::core::IUnknown {
    fn from(value: &ITaskNamedValuePair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskNamedValuePair> for super::Com::IDispatch {
    fn from(value: ITaskNamedValuePair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskNamedValuePair> for &'a super::Com::IDispatch {
    fn from(value: &'a ITaskNamedValuePair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskNamedValuePair> for super::Com::IDispatch {
    fn from(value: &ITaskNamedValuePair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITaskNamedValuePair {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITaskNamedValuePair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITaskNamedValuePair {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITaskNamedValuePair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskNamedValuePair").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITaskNamedValuePair {
    type Vtable = ITaskNamedValuePair_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39038068_2b46_4afd_8662_7bb6f868d221);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskNamedValuePair_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Value: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
pub struct ITaskScheduler(::windows::core::IUnknown);
impl ITaskScheduler {
    pub unsafe fn SetTargetComputer<'a, P0>(&self, pwszcomputer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetTargetComputer)(::windows::core::Interface::as_raw(self), pwszcomputer.into()).ok()
    }
    pub unsafe fn GetTargetComputer(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTargetComputer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn Enum(&self) -> ::windows::core::Result<IEnumWorkItems> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Enum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumWorkItems>(result__)
    }
    pub unsafe fn Activate<'a, P0>(&self, pwszname: P0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), pwszname.into(), ::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn Delete<'a, P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Delete)(::windows::core::Interface::as_raw(self), pwszname.into()).ok()
    }
    pub unsafe fn NewWorkItem<'a, P0>(&self, pwsztaskname: P0, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NewWorkItem)(::windows::core::Interface::as_raw(self), pwsztaskname.into(), ::core::mem::transmute(rclsid), ::core::mem::transmute(riid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn AddWorkItem<'a, P0, P1>(&self, pwsztaskname: P0, pworkitem: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IScheduledWorkItem>>,
    {
        (::windows::core::Interface::vtable(self).AddWorkItem)(::windows::core::Interface::as_raw(self), pwsztaskname.into(), pworkitem.into().abi()).ok()
    }
    pub unsafe fn IsOfType<'a, P0>(&self, pwszname: P0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsOfType)(::windows::core::Interface::as_raw(self), pwszname.into(), ::core::mem::transmute(riid)).ok()
    }
}
impl ::core::convert::From<ITaskScheduler> for ::windows::core::IUnknown {
    fn from(value: ITaskScheduler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITaskScheduler> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskScheduler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskScheduler> for ::windows::core::IUnknown {
    fn from(value: &ITaskScheduler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITaskScheduler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskScheduler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskScheduler {}
impl ::core::fmt::Debug for ITaskScheduler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskScheduler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITaskScheduler {
    type Vtable = ITaskScheduler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x148bd527_a2ab_11ce_b11f_00aa00530503);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskScheduler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetTargetComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomputer: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetTargetComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszcomputer: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Enum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumworkitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub NewWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztaskname: ::windows::core::PCWSTR, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztaskname: ::windows::core::PCWSTR, pworkitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsOfType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITaskService(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITaskService {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetFolder<'a, P0>(&self, path: P0) -> ::windows::core::Result<ITaskFolder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFolder)(::windows::core::Interface::as_raw(self), path.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskFolder>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRunningTasks(&self, flags: i32) -> ::windows::core::Result<IRunningTaskCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRunningTasks)(::windows::core::Interface::as_raw(self), flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRunningTaskCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NewTask(&self, flags: u32) -> ::windows::core::Result<ITaskDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NewTask)(::windows::core::Interface::as_raw(self), flags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITaskDefinition>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Connect<'a, P0, P1, P2, P3>(&self, servername: P0, user: P1, domain: P2, password: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), servername.into().abi(), user.into().abi(), domain.into().abi(), password.into().abi()).ok()
    }
    pub unsafe fn Connected(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Connected)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TargetServer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).TargetServer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectedUser(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConnectedUser)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectedDomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConnectedDomain)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn HighestVersion(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).HighestVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskService> for ::windows::core::IUnknown {
    fn from(value: ITaskService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskService> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskService> for ::windows::core::IUnknown {
    fn from(value: &ITaskService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskService> for super::Com::IDispatch {
    fn from(value: ITaskService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskService> for &'a super::Com::IDispatch {
    fn from(value: &'a ITaskService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskService> for super::Com::IDispatch {
    fn from(value: &ITaskService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITaskService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITaskService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITaskService {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITaskService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskService").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITaskService {
    type Vtable = ITaskService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2faba4c7_4da9_4013_9697_20cc3fd40f85);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskService_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfolder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetFolder: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRunningTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: i32, pprunningtasks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRunningTasks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NewTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: u32, ppdefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NewTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servername: ::core::mem::ManuallyDrop<super::Com::VARIANT>, user: ::core::mem::ManuallyDrop<super::Com::VARIANT>, domain: ::core::mem::ManuallyDrop<super::Com::VARIANT>, password: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Connect: usize,
    pub Connected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnected: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TargetServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserver: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TargetServer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectedUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectedUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectedDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectedDomain: usize,
    pub HighestVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITaskSettings(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITaskSettings {
    pub unsafe fn AllowDemandStart(&self, pallowdemandstart: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllowDemandStart)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pallowdemandstart)).ok()
    }
    pub unsafe fn SetAllowDemandStart(&self, allowdemandstart: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowDemandStart)(::windows::core::Interface::as_raw(self), allowdemandstart).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestartInterval(&self, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestartInterval)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prestartinterval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestartInterval<'a, P0>(&self, restartinterval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetRestartInterval)(::windows::core::Interface::as_raw(self), restartinterval.into().abi()).ok()
    }
    pub unsafe fn RestartCount(&self, prestartcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestartCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prestartcount)).ok()
    }
    pub unsafe fn SetRestartCount(&self, restartcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRestartCount)(::windows::core::Interface::as_raw(self), restartcount).ok()
    }
    pub unsafe fn MultipleInstances(&self, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MultipleInstances)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppolicy)).ok()
    }
    pub unsafe fn SetMultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMultipleInstances)(::windows::core::Interface::as_raw(self), policy).ok()
    }
    pub unsafe fn StopIfGoingOnBatteries(&self, pstopifonbatteries: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopIfGoingOnBatteries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstopifonbatteries)).ok()
    }
    pub unsafe fn SetStopIfGoingOnBatteries(&self, stopifonbatteries: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStopIfGoingOnBatteries)(::windows::core::Interface::as_raw(self), stopifonbatteries).ok()
    }
    pub unsafe fn DisallowStartIfOnBatteries(&self, pdisallowstart: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisallowStartIfOnBatteries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdisallowstart)).ok()
    }
    pub unsafe fn SetDisallowStartIfOnBatteries(&self, disallowstart: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisallowStartIfOnBatteries)(::windows::core::Interface::as_raw(self), disallowstart).ok()
    }
    pub unsafe fn AllowHardTerminate(&self, pallowhardterminate: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AllowHardTerminate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pallowhardterminate)).ok()
    }
    pub unsafe fn SetAllowHardTerminate(&self, allowhardterminate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowHardTerminate)(::windows::core::Interface::as_raw(self), allowhardterminate).ok()
    }
    pub unsafe fn StartWhenAvailable(&self, pstartwhenavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartWhenAvailable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstartwhenavailable)).ok()
    }
    pub unsafe fn SetStartWhenAvailable(&self, startwhenavailable: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStartWhenAvailable)(::windows::core::Interface::as_raw(self), startwhenavailable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn XmlText(&self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).XmlText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXmlText<'a, P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetXmlText)(::windows::core::Interface::as_raw(self), text.into().abi()).ok()
    }
    pub unsafe fn RunOnlyIfNetworkAvailable(&self, prunonlyifnetworkavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RunOnlyIfNetworkAvailable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prunonlyifnetworkavailable)).ok()
    }
    pub unsafe fn SetRunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRunOnlyIfNetworkAvailable)(::windows::core::Interface::as_raw(self), runonlyifnetworkavailable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pexecutiontimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, executiontimelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), executiontimelimit.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteExpiredTaskAfter(&self, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteExpiredTaskAfter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pexpirationdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeleteExpiredTaskAfter<'a, P0>(&self, expirationdelay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDeleteExpiredTaskAfter)(::windows::core::Interface::as_raw(self), expirationdelay.into().abi()).ok()
    }
    pub unsafe fn Priority(&self, ppriority: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Priority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppriority)).ok()
    }
    pub unsafe fn SetPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn Compatibility(&self, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Compatibility)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcompatlevel)).ok()
    }
    pub unsafe fn SetCompatibility(&self, compatlevel: TASK_COMPATIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCompatibility)(::windows::core::Interface::as_raw(self), compatlevel).ok()
    }
    pub unsafe fn Hidden(&self, phidden: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Hidden)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phidden)).ok()
    }
    pub unsafe fn SetHidden(&self, hidden: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetHidden)(::windows::core::Interface::as_raw(self), hidden).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdleSettings(&self) -> ::windows::core::Result<IIdleSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IdleSettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IIdleSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIdleSettings<'a, P0>(&self, pidlesettings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IIdleSettings>>,
    {
        (::windows::core::Interface::vtable(self).SetIdleSettings)(::windows::core::Interface::as_raw(self), pidlesettings.into().abi()).ok()
    }
    pub unsafe fn RunOnlyIfIdle(&self, prunonlyifidle: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RunOnlyIfIdle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prunonlyifidle)).ok()
    }
    pub unsafe fn SetRunOnlyIfIdle(&self, runonlyifidle: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRunOnlyIfIdle)(::windows::core::Interface::as_raw(self), runonlyifidle).ok()
    }
    pub unsafe fn WakeToRun(&self, pwake: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WakeToRun)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwake)).ok()
    }
    pub unsafe fn SetWakeToRun(&self, wake: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWakeToRun)(::windows::core::Interface::as_raw(self), wake).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NetworkSettings(&self) -> ::windows::core::Result<INetworkSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).NetworkSettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetworkSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNetworkSettings<'a, P0>(&self, pnetworksettings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, INetworkSettings>>,
    {
        (::windows::core::Interface::vtable(self).SetNetworkSettings)(::windows::core::Interface::as_raw(self), pnetworksettings.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskSettings> for ::windows::core::IUnknown {
    fn from(value: ITaskSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskSettings> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskSettings> for ::windows::core::IUnknown {
    fn from(value: &ITaskSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskSettings> for super::Com::IDispatch {
    fn from(value: ITaskSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskSettings> for &'a super::Com::IDispatch {
    fn from(value: &'a ITaskSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskSettings> for super::Com::IDispatch {
    fn from(value: &ITaskSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITaskSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITaskSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITaskSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITaskSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITaskSettings {
    type Vtable = ITaskSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fd4711d_2d02_4c8c_87e3_eff699de127e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AllowDemandStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallowdemandstart: *mut i16) -> ::windows::core::HRESULT,
    pub SetAllowDemandStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowdemandstart: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RestartInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestartInterval: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRestartInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restartinterval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRestartInterval: usize,
    pub RestartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prestartcount: *mut i32) -> ::windows::core::HRESULT,
    pub SetRestartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restartcount: i32) -> ::windows::core::HRESULT,
    pub MultipleInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT,
    pub SetMultipleInstances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: TASK_INSTANCES_POLICY) -> ::windows::core::HRESULT,
    pub StopIfGoingOnBatteries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstopifonbatteries: *mut i16) -> ::windows::core::HRESULT,
    pub SetStopIfGoingOnBatteries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stopifonbatteries: i16) -> ::windows::core::HRESULT,
    pub DisallowStartIfOnBatteries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT,
    pub SetDisallowStartIfOnBatteries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT,
    pub AllowHardTerminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallowhardterminate: *mut i16) -> ::windows::core::HRESULT,
    pub SetAllowHardTerminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowhardterminate: i16) -> ::windows::core::HRESULT,
    pub StartWhenAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstartwhenavailable: *mut i16) -> ::windows::core::HRESULT,
    pub SetStartWhenAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startwhenavailable: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub XmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    XmlText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetXmlText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetXmlText: usize,
    pub RunOnlyIfNetworkAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prunonlyifnetworkavailable: *mut i16) -> ::windows::core::HRESULT,
    pub SetRunOnlyIfNetworkAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runonlyifnetworkavailable: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecutionTimeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecutionTimeLimit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExecutionTimeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executiontimelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExecutionTimeLimit: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteExpiredTaskAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteExpiredTaskAfter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDeleteExpiredTaskAfter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDeleteExpiredTaskAfter: usize,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT,
    pub Compatibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows::core::HRESULT,
    pub SetCompatibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compatlevel: TASK_COMPATIBILITY) -> ::windows::core::HRESULT,
    pub Hidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phidden: *mut i16) -> ::windows::core::HRESULT,
    pub SetHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hidden: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub IdleSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidlesettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    IdleSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetIdleSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidlesettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetIdleSettings: usize,
    pub RunOnlyIfIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prunonlyifidle: *mut i16) -> ::windows::core::HRESULT,
    pub SetRunOnlyIfIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runonlyifidle: i16) -> ::windows::core::HRESULT,
    pub WakeToRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwake: *mut i16) -> ::windows::core::HRESULT,
    pub SetWakeToRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wake: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NetworkSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetworksettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NetworkSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNetworkSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnetworksettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNetworkSettings: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITaskSettings2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITaskSettings2 {
    pub unsafe fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisallowStartOnRemoteAppSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdisallowstart)).ok()
    }
    pub unsafe fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisallowStartOnRemoteAppSession)(::windows::core::Interface::as_raw(self), disallowstart).ok()
    }
    pub unsafe fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UseUnifiedSchedulingEngine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(puseunifiedengine)).ok()
    }
    pub unsafe fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseUnifiedSchedulingEngine)(::windows::core::Interface::as_raw(self), useunifiedengine).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskSettings2> for ::windows::core::IUnknown {
    fn from(value: ITaskSettings2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskSettings2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskSettings2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskSettings2> for ::windows::core::IUnknown {
    fn from(value: &ITaskSettings2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskSettings2> for super::Com::IDispatch {
    fn from(value: ITaskSettings2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskSettings2> for &'a super::Com::IDispatch {
    fn from(value: &'a ITaskSettings2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskSettings2> for super::Com::IDispatch {
    fn from(value: &ITaskSettings2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITaskSettings2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITaskSettings2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITaskSettings2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITaskSettings2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskSettings2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITaskSettings2 {
    type Vtable = ITaskSettings2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c05c3f0_6eed_4c05_a15f_ed7d7a98a369);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings2_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITaskSettings3(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITaskSettings3 {
    pub unsafe fn AllowDemandStart(&self, pallowdemandstart: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AllowDemandStart)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pallowdemandstart)).ok()
    }
    pub unsafe fn SetAllowDemandStart(&self, allowdemandstart: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetAllowDemandStart)(::windows::core::Interface::as_raw(self), allowdemandstart).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestartInterval(&self, prestartinterval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RestartInterval)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prestartinterval)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRestartInterval<'a, P0>(&self, restartinterval: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRestartInterval)(::windows::core::Interface::as_raw(self), restartinterval.into().abi()).ok()
    }
    pub unsafe fn RestartCount(&self, prestartcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RestartCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prestartcount)).ok()
    }
    pub unsafe fn SetRestartCount(&self, restartcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRestartCount)(::windows::core::Interface::as_raw(self), restartcount).ok()
    }
    pub unsafe fn MultipleInstances(&self, ppolicy: *mut TASK_INSTANCES_POLICY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.MultipleInstances)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppolicy)).ok()
    }
    pub unsafe fn SetMultipleInstances(&self, policy: TASK_INSTANCES_POLICY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetMultipleInstances)(::windows::core::Interface::as_raw(self), policy).ok()
    }
    pub unsafe fn StopIfGoingOnBatteries(&self, pstopifonbatteries: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StopIfGoingOnBatteries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstopifonbatteries)).ok()
    }
    pub unsafe fn SetStopIfGoingOnBatteries(&self, stopifonbatteries: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetStopIfGoingOnBatteries)(::windows::core::Interface::as_raw(self), stopifonbatteries).ok()
    }
    pub unsafe fn DisallowStartIfOnBatteries(&self, pdisallowstart: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DisallowStartIfOnBatteries)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdisallowstart)).ok()
    }
    pub unsafe fn SetDisallowStartIfOnBatteries(&self, disallowstart: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetDisallowStartIfOnBatteries)(::windows::core::Interface::as_raw(self), disallowstart).ok()
    }
    pub unsafe fn AllowHardTerminate(&self, pallowhardterminate: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AllowHardTerminate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pallowhardterminate)).ok()
    }
    pub unsafe fn SetAllowHardTerminate(&self, allowhardterminate: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetAllowHardTerminate)(::windows::core::Interface::as_raw(self), allowhardterminate).ok()
    }
    pub unsafe fn StartWhenAvailable(&self, pstartwhenavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartWhenAvailable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstartwhenavailable)).ok()
    }
    pub unsafe fn SetStartWhenAvailable(&self, startwhenavailable: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetStartWhenAvailable)(::windows::core::Interface::as_raw(self), startwhenavailable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn XmlText(&self, ptext: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.XmlText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptext)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXmlText<'a, P0>(&self, text: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetXmlText)(::windows::core::Interface::as_raw(self), text.into().abi()).ok()
    }
    pub unsafe fn RunOnlyIfNetworkAvailable(&self, prunonlyifnetworkavailable: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RunOnlyIfNetworkAvailable)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prunonlyifnetworkavailable)).ok()
    }
    pub unsafe fn SetRunOnlyIfNetworkAvailable(&self, runonlyifnetworkavailable: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRunOnlyIfNetworkAvailable)(::windows::core::Interface::as_raw(self), runonlyifnetworkavailable).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, pexecutiontimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pexecutiontimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, executiontimelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), executiontimelimit.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteExpiredTaskAfter(&self, pexpirationdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteExpiredTaskAfter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pexpirationdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeleteExpiredTaskAfter<'a, P0>(&self, expirationdelay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDeleteExpiredTaskAfter)(::windows::core::Interface::as_raw(self), expirationdelay.into().abi()).ok()
    }
    pub unsafe fn Priority(&self, ppriority: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Priority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppriority)).ok()
    }
    pub unsafe fn SetPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPriority)(::windows::core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn Compatibility(&self, pcompatlevel: *mut TASK_COMPATIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Compatibility)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcompatlevel)).ok()
    }
    pub unsafe fn SetCompatibility(&self, compatlevel: TASK_COMPATIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetCompatibility)(::windows::core::Interface::as_raw(self), compatlevel).ok()
    }
    pub unsafe fn Hidden(&self, phidden: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Hidden)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phidden)).ok()
    }
    pub unsafe fn SetHidden(&self, hidden: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetHidden)(::windows::core::Interface::as_raw(self), hidden).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IdleSettings(&self) -> ::windows::core::Result<IIdleSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.IdleSettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IIdleSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIdleSettings<'a, P0>(&self, pidlesettings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IIdleSettings>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetIdleSettings)(::windows::core::Interface::as_raw(self), pidlesettings.into().abi()).ok()
    }
    pub unsafe fn RunOnlyIfIdle(&self, prunonlyifidle: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RunOnlyIfIdle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prunonlyifidle)).ok()
    }
    pub unsafe fn SetRunOnlyIfIdle(&self, runonlyifidle: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRunOnlyIfIdle)(::windows::core::Interface::as_raw(self), runonlyifidle).ok()
    }
    pub unsafe fn WakeToRun(&self, pwake: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.WakeToRun)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwake)).ok()
    }
    pub unsafe fn SetWakeToRun(&self, wake: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetWakeToRun)(::windows::core::Interface::as_raw(self), wake).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NetworkSettings(&self) -> ::windows::core::Result<INetworkSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.NetworkSettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<INetworkSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNetworkSettings<'a, P0>(&self, pnetworksettings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, INetworkSettings>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNetworkSettings)(::windows::core::Interface::as_raw(self), pnetworksettings.into().abi()).ok()
    }
    pub unsafe fn DisallowStartOnRemoteAppSession(&self, pdisallowstart: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisallowStartOnRemoteAppSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdisallowstart)).ok()
    }
    pub unsafe fn SetDisallowStartOnRemoteAppSession(&self, disallowstart: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisallowStartOnRemoteAppSession)(::windows::core::Interface::as_raw(self), disallowstart).ok()
    }
    pub unsafe fn UseUnifiedSchedulingEngine(&self, puseunifiedengine: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UseUnifiedSchedulingEngine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(puseunifiedengine)).ok()
    }
    pub unsafe fn SetUseUnifiedSchedulingEngine(&self, useunifiedengine: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUseUnifiedSchedulingEngine)(::windows::core::Interface::as_raw(self), useunifiedengine).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MaintenanceSettings(&self) -> ::windows::core::Result<IMaintenanceSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MaintenanceSettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMaintenanceSettings>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMaintenanceSettings<'a, P0>(&self, pmaintenancesettings: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IMaintenanceSettings>>,
    {
        (::windows::core::Interface::vtable(self).SetMaintenanceSettings)(::windows::core::Interface::as_raw(self), pmaintenancesettings.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMaintenanceSettings(&self) -> ::windows::core::Result<IMaintenanceSettings> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateMaintenanceSettings)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IMaintenanceSettings>(result__)
    }
    pub unsafe fn Volatile(&self, pvolatile: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Volatile)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pvolatile)).ok()
    }
    pub unsafe fn SetVolatile(&self, volatile: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVolatile)(::windows::core::Interface::as_raw(self), volatile).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskSettings3> for ::windows::core::IUnknown {
    fn from(value: ITaskSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskSettings3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskSettings3> for ::windows::core::IUnknown {
    fn from(value: &ITaskSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskSettings3> for super::Com::IDispatch {
    fn from(value: ITaskSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskSettings3> for &'a super::Com::IDispatch {
    fn from(value: &'a ITaskSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskSettings3> for super::Com::IDispatch {
    fn from(value: &ITaskSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITaskSettings3> for ITaskSettings {
    fn from(value: ITaskSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITaskSettings3> for &'a ITaskSettings {
    fn from(value: &'a ITaskSettings3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITaskSettings3> for ITaskSettings {
    fn from(value: &ITaskSettings3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITaskSettings3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITaskSettings3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITaskSettings3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITaskSettings3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskSettings3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITaskSettings3 {
    type Vtable = ITaskSettings3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ad9d0d7_0c7f_4ebb_9a5f_d1c648dca528);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITaskSettings3_Vtbl {
    pub base__: ITaskSettings_Vtbl,
    pub DisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdisallowstart: *mut i16) -> ::windows::core::HRESULT,
    pub SetDisallowStartOnRemoteAppSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disallowstart: i16) -> ::windows::core::HRESULT,
    pub UseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puseunifiedengine: *mut i16) -> ::windows::core::HRESULT,
    pub SetUseUnifiedSchedulingEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useunifiedengine: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MaintenanceSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MaintenanceSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMaintenanceSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmaintenancesettings: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMaintenanceSettings: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMaintenanceSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmaintenancesettings: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMaintenanceSettings: usize,
    pub Volatile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvolatile: *mut i16) -> ::windows::core::HRESULT,
    pub SetVolatile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volatile: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
pub struct ITaskTrigger(::windows::core::IUnknown);
impl ITaskTrigger {
    pub unsafe fn SetTrigger(&self, ptrigger: *const TASK_TRIGGER) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTrigger)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptrigger)).ok()
    }
    pub unsafe fn GetTrigger(&self) -> ::windows::core::Result<TASK_TRIGGER> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTrigger)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<TASK_TRIGGER>(result__)
    }
    pub unsafe fn GetTriggerString(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetTriggerString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<ITaskTrigger> for ::windows::core::IUnknown {
    fn from(value: ITaskTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITaskTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskTrigger> for ::windows::core::IUnknown {
    fn from(value: &ITaskTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITaskTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskTrigger {}
impl ::core::fmt::Debug for ITaskTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskTrigger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITaskTrigger {
    type Vtable = ITaskTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x148bd52b_a2ab_11ce_b11f_00aa00530503);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskTrigger_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptrigger: *const TASK_TRIGGER) -> ::windows::core::HRESULT,
    pub GetTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptrigger: *mut TASK_TRIGGER) -> ::windows::core::HRESULT,
    pub GetTriggerString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwsztrigger: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
pub struct ITaskVariables(::windows::core::IUnknown);
impl ITaskVariables {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInput(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInput)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutput<'a, P0>(&self, input: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetOutput)(::windows::core::Interface::as_raw(self), input.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<ITaskVariables> for ::windows::core::IUnknown {
    fn from(value: ITaskVariables) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ITaskVariables> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITaskVariables) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITaskVariables> for ::windows::core::IUnknown {
    fn from(value: &ITaskVariables) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ITaskVariables {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITaskVariables {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITaskVariables {}
impl ::core::fmt::Debug for ITaskVariables {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITaskVariables").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITaskVariables {
    type Vtable = ITaskVariables_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e4c9351_d966_4b8b_bb87_ceba68bb0107);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITaskVariables_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetContext: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITimeTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITimeTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RandomDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRandomDelay<'a, P0>(&self, randomdelay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetRandomDelay)(::windows::core::Interface::as_raw(self), randomdelay.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITimeTrigger> for ::windows::core::IUnknown {
    fn from(value: ITimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITimeTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITimeTrigger> for ::windows::core::IUnknown {
    fn from(value: &ITimeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITimeTrigger> for super::Com::IDispatch {
    fn from(value: ITimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITimeTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a ITimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITimeTrigger> for super::Com::IDispatch {
    fn from(value: &ITimeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITimeTrigger> for ITrigger {
    fn from(value: ITimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITimeTrigger> for &'a ITrigger {
    fn from(value: &'a ITimeTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITimeTrigger> for ITrigger {
    fn from(value: &ITimeTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITimeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITimeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITimeTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITimeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimeTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITimeTrigger {
    type Vtable = ITimeTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb45747e0_eba7_4276_9f29_85c5bb300006);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITrigger> for ::windows::core::IUnknown {
    fn from(value: ITrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITrigger> for ::windows::core::IUnknown {
    fn from(value: &ITrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITrigger> for super::Com::IDispatch {
    fn from(value: ITrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a ITrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITrigger> for super::Com::IDispatch {
    fn from(value: &ITrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITrigger {
    type Vtable = ITrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09941815_ea89_4b5b_89e0_2a773801fac3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITrigger_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Id: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Repetition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprepeat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Repetition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetRepetition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prepeat: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetRepetition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ExecutionTimeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ExecutionTimeLimit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetExecutionTimeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timelimit: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetExecutionTimeLimit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartBoundary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStartBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStartBoundary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EndBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndBoundary: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEndBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, end: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEndBoundary: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, penabled: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITriggerCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITriggerCollection {
    pub unsafe fn Count(&self, pcount: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcount)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<ITrigger> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), index, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITrigger>(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Create(&self, r#type: TASK_TRIGGER_TYPE2) -> ::windows::core::Result<ITrigger> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), r#type, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ITrigger>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Remove<'a, P0>(&self, index: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), index.into().abi()).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITriggerCollection> for ::windows::core::IUnknown {
    fn from(value: ITriggerCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITriggerCollection> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ITriggerCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITriggerCollection> for ::windows::core::IUnknown {
    fn from(value: &ITriggerCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITriggerCollection> for super::Com::IDispatch {
    fn from(value: ITriggerCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a ITriggerCollection> for &'a super::Com::IDispatch {
    fn from(value: &'a ITriggerCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITriggerCollection> for super::Com::IDispatch {
    fn from(value: &ITriggerCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITriggerCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITriggerCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITriggerCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITriggerCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITriggerCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITriggerCollection {
    type Vtable = ITriggerCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85df5081_1b24_4f32_878a_d9d14df4cb77);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITriggerCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: TASK_TRIGGER_TYPE2, pptrigger: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Create: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Remove: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWeeklyTrigger(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWeeklyTrigger {
    pub unsafe fn Type(&self, ptype: *mut TASK_TRIGGER_TYPE2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Type)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Id(&self, pid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<'a, P0>(&self, id: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), id.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Repetition(&self) -> ::windows::core::Result<IRepetitionPattern> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Repetition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRepetitionPattern>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetRepetition<'a, P0>(&self, prepeat: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRepetitionPattern>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRepetition)(::windows::core::Interface::as_raw(self), prepeat.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutionTimeLimit(&self, ptimelimit: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExecutionTimeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptimelimit)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutionTimeLimit<'a, P0>(&self, timelimit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetExecutionTimeLimit)(::windows::core::Interface::as_raw(self), timelimit.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartBoundary(&self, pstart: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.StartBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstart)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStartBoundary<'a, P0>(&self, start: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStartBoundary)(::windows::core::Interface::as_raw(self), start.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndBoundary(&self, pend: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.EndBoundary)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEndBoundary<'a, P0>(&self, end: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.SetEndBoundary)(::windows::core::Interface::as_raw(self), end.into().abi()).ok()
    }
    pub unsafe fn Enabled(&self, penabled: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(penabled)).ok()
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetEnabled)(::windows::core::Interface::as_raw(self), enabled).ok()
    }
    pub unsafe fn DaysOfWeek(&self, pdays: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DaysOfWeek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdays)).ok()
    }
    pub unsafe fn SetDaysOfWeek(&self, days: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDaysOfWeek)(::windows::core::Interface::as_raw(self), days).ok()
    }
    pub unsafe fn WeeksInterval(&self, pweeks: *mut i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WeeksInterval)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pweeks)).ok()
    }
    pub unsafe fn SetWeeksInterval(&self, weeks: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWeeksInterval)(::windows::core::Interface::as_raw(self), weeks).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RandomDelay(&self, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RandomDelay)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prandomdelay)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRandomDelay<'a, P0>(&self, randomdelay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetRandomDelay)(::windows::core::Interface::as_raw(self), randomdelay.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWeeklyTrigger> for ::windows::core::IUnknown {
    fn from(value: IWeeklyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IWeeklyTrigger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWeeklyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWeeklyTrigger> for ::windows::core::IUnknown {
    fn from(value: &IWeeklyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWeeklyTrigger> for super::Com::IDispatch {
    fn from(value: IWeeklyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IWeeklyTrigger> for &'a super::Com::IDispatch {
    fn from(value: &'a IWeeklyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWeeklyTrigger> for super::Com::IDispatch {
    fn from(value: &IWeeklyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IWeeklyTrigger> for ITrigger {
    fn from(value: IWeeklyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IWeeklyTrigger> for &'a ITrigger {
    fn from(value: &'a IWeeklyTrigger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IWeeklyTrigger> for ITrigger {
    fn from(value: &IWeeklyTrigger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWeeklyTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWeeklyTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWeeklyTrigger {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWeeklyTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWeeklyTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IWeeklyTrigger {
    type Vtable = IWeeklyTrigger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5038fc98_82ff_436d_8728_a512a57c9dc1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWeeklyTrigger_Vtbl {
    pub base__: ITrigger_Vtbl,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdays: *mut i16) -> ::windows::core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i16) -> ::windows::core::HRESULT,
    pub WeeksInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pweeks: *mut i16) -> ::windows::core::HRESULT,
    pub SetWeeksInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weeks: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prandomdelay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RandomDelay: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRandomDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, randomdelay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRandomDelay: usize,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct MONTHLYDATE {
    pub rgfDays: u32,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDATE {}
impl ::core::clone::Clone for MONTHLYDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MONTHLYDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONTHLYDATE").field("rgfDays", &self.rgfDays).field("rgfMonths", &self.rgfMonths).finish()
    }
}
unsafe impl ::windows::core::Abi for MONTHLYDATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MONTHLYDATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONTHLYDATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for MONTHLYDATE {}
impl ::core::default::Default for MONTHLYDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct MONTHLYDOW {
    pub wWhichWeek: u16,
    pub rgfDaysOfTheWeek: u16,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDOW {}
impl ::core::clone::Clone for MONTHLYDOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MONTHLYDOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONTHLYDOW").field("wWhichWeek", &self.wWhichWeek).field("rgfDaysOfTheWeek", &self.rgfDaysOfTheWeek).field("rgfMonths", &self.rgfMonths).finish()
    }
}
unsafe impl ::windows::core::Abi for MONTHLYDOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MONTHLYDOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MONTHLYDOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for MONTHLYDOW {}
impl ::core::default::Default for MONTHLYDOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASKPAGE(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASKPAGE_TASK: TASKPAGE = TASKPAGE(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASKPAGE_SCHEDULE: TASKPAGE = TASKPAGE(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASKPAGE_SETTINGS: TASKPAGE = TASKPAGE(2i32);
impl ::core::marker::Copy for TASKPAGE {}
impl ::core::clone::Clone for TASKPAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASKPAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASKPAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASKPAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASKPAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_ACTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_EXEC: TASK_ACTION_TYPE = TASK_ACTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_COM_HANDLER: TASK_ACTION_TYPE = TASK_ACTION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_SEND_EMAIL: TASK_ACTION_TYPE = TASK_ACTION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ACTION_SHOW_MESSAGE: TASK_ACTION_TYPE = TASK_ACTION_TYPE(7i32);
impl ::core::marker::Copy for TASK_ACTION_TYPE {}
impl ::core::clone::Clone for TASK_ACTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_ACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_ACTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_ACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_ACTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_APRIL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_AUGUST: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_COMPATIBILITY(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_AT: TASK_COMPATIBILITY = TASK_COMPATIBILITY(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V1: TASK_COMPATIBILITY = TASK_COMPATIBILITY(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2: TASK_COMPATIBILITY = TASK_COMPATIBILITY(2i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_1: TASK_COMPATIBILITY = TASK_COMPATIBILITY(3i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_2: TASK_COMPATIBILITY = TASK_COMPATIBILITY(4i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_3: TASK_COMPATIBILITY = TASK_COMPATIBILITY(5i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_COMPATIBILITY_V2_4: TASK_COMPATIBILITY = TASK_COMPATIBILITY(6i32);
impl ::core::marker::Copy for TASK_COMPATIBILITY {}
impl ::core::clone::Clone for TASK_COMPATIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_COMPATIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_COMPATIBILITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_COMPATIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_COMPATIBILITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_CREATION(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_VALIDATE_ONLY: TASK_CREATION = TASK_CREATION(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CREATE: TASK_CREATION = TASK_CREATION(2i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_UPDATE: TASK_CREATION = TASK_CREATION(4i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CREATE_OR_UPDATE: TASK_CREATION = TASK_CREATION(6i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_DISABLE: TASK_CREATION = TASK_CREATION(8i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_DONT_ADD_PRINCIPAL_ACE: TASK_CREATION = TASK_CREATION(16i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_IGNORE_REGISTRATION_TRIGGERS: TASK_CREATION = TASK_CREATION(32i32);
impl ::core::marker::Copy for TASK_CREATION {}
impl ::core::clone::Clone for TASK_CREATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_CREATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_CREATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_CREATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_CREATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_DECEMBER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_ENUM_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_ENUM_HIDDEN: TASK_ENUM_FLAGS = TASK_ENUM_FLAGS(1i32);
impl ::core::marker::Copy for TASK_ENUM_FLAGS {}
impl ::core::clone::Clone for TASK_ENUM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_ENUM_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FEBRUARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FIRST_WEEK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_DELETE_WHEN_DONE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_DONT_START_IF_ON_BATTERIES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_HIDDEN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_INTERACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_KILL_IF_GOING_ON_BATTERIES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_KILL_ON_IDLE_END: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RESTART_ON_IDLE_RESUME: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RUN_IF_CONNECTED_TO_INTERNET: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RUN_ONLY_IF_DOCKED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_RUN_ONLY_IF_LOGGED_ON: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_START_ONLY_IF_IDLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FLAG_SYSTEM_REQUIRED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FOURTH_WEEK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_FRIDAY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_INSTANCES_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_PARALLEL: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_QUEUE: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_IGNORE_NEW: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(2i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_INSTANCES_STOP_EXISTING: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(3i32);
impl ::core::marker::Copy for TASK_INSTANCES_POLICY {}
impl ::core::clone::Clone for TASK_INSTANCES_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_INSTANCES_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_INSTANCES_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_INSTANCES_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_INSTANCES_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_JANUARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_JULY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_JUNE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LAST_WEEK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_LOGON_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_NONE: TASK_LOGON_TYPE = TASK_LOGON_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_PASSWORD: TASK_LOGON_TYPE = TASK_LOGON_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_S4U: TASK_LOGON_TYPE = TASK_LOGON_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_INTERACTIVE_TOKEN: TASK_LOGON_TYPE = TASK_LOGON_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_GROUP: TASK_LOGON_TYPE = TASK_LOGON_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_SERVICE_ACCOUNT: TASK_LOGON_TYPE = TASK_LOGON_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD: TASK_LOGON_TYPE = TASK_LOGON_TYPE(6i32);
impl ::core::marker::Copy for TASK_LOGON_TYPE {}
impl ::core::clone::Clone for TASK_LOGON_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_LOGON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_LOGON_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_LOGON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_LOGON_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MARCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MAX_RUN_TIMES: u32 = 1440u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MAY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_MONDAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_NOVEMBER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_OCTOBER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_PROCESSTOKENSID_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_PROCESSTOKENSID_NONE: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_PROCESSTOKENSID_UNRESTRICTED: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_PROCESSTOKENSID_DEFAULT: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(2i32);
impl ::core::marker::Copy for TASK_PROCESSTOKENSID_TYPE {}
impl ::core::clone::Clone for TASK_PROCESSTOKENSID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_PROCESSTOKENSID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_PROCESSTOKENSID_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_PROCESSTOKENSID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_PROCESSTOKENSID_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_RUNLEVEL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUNLEVEL_LUA: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUNLEVEL_HIGHEST: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(1i32);
impl ::core::marker::Copy for TASK_RUNLEVEL_TYPE {}
impl ::core::clone::Clone for TASK_RUNLEVEL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_RUNLEVEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_RUNLEVEL_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_RUNLEVEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_RUNLEVEL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_RUN_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_NO_FLAGS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_AS_SELF: TASK_RUN_FLAGS = TASK_RUN_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_IGNORE_CONSTRAINTS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_USE_SESSION_ID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_RUN_USER_SID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(8i32);
impl ::core::marker::Copy for TASK_RUN_FLAGS {}
impl ::core::clone::Clone for TASK_RUN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_RUN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_RUN_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_RUN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_RUN_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SATURDAY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SECOND_WEEK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SEPTEMBER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_SESSION_STATE_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CONSOLE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_CONSOLE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_REMOTE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_REMOTE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SESSION_LOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SESSION_UNLOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(8i32);
impl ::core::marker::Copy for TASK_SESSION_STATE_CHANGE_TYPE {}
impl ::core::clone::Clone for TASK_SESSION_STATE_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_SESSION_STATE_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_SESSION_STATE_CHANGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_SESSION_STATE_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_SESSION_STATE_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_UNKNOWN: TASK_STATE = TASK_STATE(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_DISABLED: TASK_STATE = TASK_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_QUEUED: TASK_STATE = TASK_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_READY: TASK_STATE = TASK_STATE(3i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_STATE_RUNNING: TASK_STATE = TASK_STATE(4i32);
impl ::core::marker::Copy for TASK_STATE {}
impl ::core::clone::Clone for TASK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_SUNDAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_THIRD_WEEK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_THURSDAY: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct TASK_TRIGGER {
    pub cbTriggerSize: u16,
    pub Reserved1: u16,
    pub wBeginYear: u16,
    pub wBeginMonth: u16,
    pub wBeginDay: u16,
    pub wEndYear: u16,
    pub wEndMonth: u16,
    pub wEndDay: u16,
    pub wStartHour: u16,
    pub wStartMinute: u16,
    pub MinutesDuration: u32,
    pub MinutesInterval: u32,
    pub rgFlags: u32,
    pub TriggerType: TASK_TRIGGER_TYPE,
    pub Type: TRIGGER_TYPE_UNION,
    pub Reserved2: u16,
    pub wRandomMinutesInterval: u16,
}
impl ::core::marker::Copy for TASK_TRIGGER {}
impl ::core::clone::Clone for TASK_TRIGGER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TASK_TRIGGER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TASK_TRIGGER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TASK_TRIGGER>()) == 0 }
    }
}
impl ::core::cmp::Eq for TASK_TRIGGER {}
impl ::core::default::Default for TASK_TRIGGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_FLAG_DISABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_FLAG_HAS_END_DATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_FLAG_KILL_AT_DURATION_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_TRIGGER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_ONCE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_DAILY: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_MONTHLYDATE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TIME_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_EVENT_TRIGGER_ON_IDLE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_EVENT_TRIGGER_AT_SYSTEMSTART: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_EVENT_TRIGGER_AT_LOGON: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(7i32);
impl ::core::marker::Copy for TASK_TRIGGER_TYPE {}
impl ::core::clone::Clone for TASK_TRIGGER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_TRIGGER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_TRIGGER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_TRIGGER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_TRIGGER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TASK_TRIGGER_TYPE2(pub i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_EVENT: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(0i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_TIME: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(1i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_DAILY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(2i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(3i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_MONTHLY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(4i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(5i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_IDLE: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(6i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_REGISTRATION: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(7i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_BOOT: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(8i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_LOGON: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(9i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_SESSION_STATE_CHANGE: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(11i32);
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TRIGGER_CUSTOM_TRIGGER_01: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(12i32);
impl ::core::marker::Copy for TASK_TRIGGER_TYPE2 {}
impl ::core::clone::Clone for TASK_TRIGGER_TYPE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TASK_TRIGGER_TYPE2 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TASK_TRIGGER_TYPE2 {
    type Abi = Self;
}
impl ::core::fmt::Debug for TASK_TRIGGER_TYPE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TASK_TRIGGER_TYPE2").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_TUESDAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub const TASK_WEDNESDAY: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub union TRIGGER_TYPE_UNION {
    pub Daily: DAILY,
    pub Weekly: WEEKLY,
    pub MonthlyDate: MONTHLYDATE,
    pub MonthlyDOW: MONTHLYDOW,
}
impl ::core::marker::Copy for TRIGGER_TYPE_UNION {}
impl ::core::clone::Clone for TRIGGER_TYPE_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TRIGGER_TYPE_UNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TRIGGER_TYPE_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TRIGGER_TYPE_UNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TRIGGER_TYPE_UNION {}
impl ::core::default::Default for TRIGGER_TYPE_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const TaskHandlerPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2a69db7_da2c_4352_9066_86fee6dacac9);
pub const TaskHandlerStatusPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f15266d_d7ba_48f0_93c1_e6895f6fe5ac);
pub const TaskScheduler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f87369f_a4e5_4cfc_bd3e_73e6154572dd);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_TaskScheduler\"`*"]
pub struct WEEKLY {
    pub WeeksInterval: u16,
    pub rgfDaysOfTheWeek: u16,
}
impl ::core::marker::Copy for WEEKLY {}
impl ::core::clone::Clone for WEEKLY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WEEKLY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WEEKLY").field("WeeksInterval", &self.WeeksInterval).field("rgfDaysOfTheWeek", &self.rgfDaysOfTheWeek).finish()
    }
}
unsafe impl ::windows::core::Abi for WEEKLY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WEEKLY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WEEKLY>()) == 0 }
    }
}
impl ::core::cmp::Eq for WEEKLY {}
impl ::core::default::Default for WEEKLY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
