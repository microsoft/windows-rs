#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct DIFsrmClassificationEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl DIFsrmClassificationEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(DIFsrmClassificationEvents, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for DIFsrmClassificationEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DIFsrmClassificationEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DIFsrmClassificationEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DIFsrmClassificationEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIFsrmClassificationEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for DIFsrmClassificationEvents {
    type Vtable = DIFsrmClassificationEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for DIFsrmClassificationEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26942db0_dabf_41d8_bbdd_b129a9f70424);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct DIFsrmClassificationEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmAccessDeniedRemediationClient(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAccessDeniedRemediationClient {
    pub unsafe fn Show(&self, parentwnd: usize, accesspath: &::windows::core::BSTR, errortype: AdrClientErrorType, flags: i32, windowtitle: &::windows::core::BSTR, windowmessage: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Show)(::windows::core::Vtable::as_raw(self), parentwnd, ::core::mem::transmute_copy(accesspath), errortype, flags, ::core::mem::transmute_copy(windowtitle), ::core::mem::transmute_copy(windowmessage), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmAccessDeniedRemediationClient, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmAccessDeniedRemediationClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmAccessDeniedRemediationClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmAccessDeniedRemediationClient {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmAccessDeniedRemediationClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmAccessDeniedRemediationClient").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmAccessDeniedRemediationClient {
    type Vtable = IFsrmAccessDeniedRemediationClient_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmAccessDeniedRemediationClient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40002314_590b_45a5_8e1b_8c05da527e52);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAccessDeniedRemediationClient_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentwnd: usize, accesspath: *mut ::core::ffi::c_void, errortype: AdrClientErrorType, flags: i32, windowtitle: *mut ::core::ffi::c_void, windowmessage: *mut ::core::ffi::c_void, result: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmAction(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAction {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ActionType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RunLimitInterval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRunLimitInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmAction, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmAction {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmAction").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmAction {
    type Vtable = IFsrmAction_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmAction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cd6408a_ae60_463b_9ef1_e117534d69dc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAction_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ActionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: *mut FsrmActionType) -> ::windows::core::HRESULT,
    pub RunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub SetRunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionCommand(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionCommand {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ActionType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RunLimitInterval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRunLimitInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ExecutablePath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExecutablePath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetExecutablePath(&self, executablepath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetExecutablePath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(executablepath)).ok()
    }
    pub unsafe fn Arguments(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Arguments)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetArguments(&self, arguments: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetArguments)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(arguments)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Account)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccount(&self, account: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAccount)(::windows::core::Vtable::as_raw(self), account).ok()
    }
    pub unsafe fn WorkingDirectory(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WorkingDirectory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetWorkingDirectory(&self, workingdirectory: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetWorkingDirectory)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(workingdirectory)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MonitorCommand(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MonitorCommand)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMonitorCommand<P0>(&self, monitorcommand: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetMonitorCommand)(::windows::core::Vtable::as_raw(self), monitorcommand.into()).ok()
    }
    pub unsafe fn KillTimeOut(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).KillTimeOut)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKillTimeOut(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetKillTimeOut)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LogResult(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LogResult)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogResult<P0>(&self, logresults: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetLogResult)(::windows::core::Vtable::as_raw(self), logresults.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmActionCommand, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionCommand {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionCommand").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmActionCommand {
    type Vtable = IFsrmActionCommand_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionCommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12937789_e247_4917_9c20_f3ee9c7ee783);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionCommand_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub ExecutablePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablepath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetExecutablePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executablepath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: *mut FsrmAccountType) -> ::windows::core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: FsrmAccountType) -> ::windows::core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MonitorCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitorcommand: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MonitorCommand: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMonitorCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitorcommand: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMonitorCommand: usize,
    pub KillTimeOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows::core::HRESULT,
    pub SetKillTimeOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LogResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logresults: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LogResult: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLogResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logresults: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLogResult: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionEmail(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEmail {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ActionType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RunLimitInterval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRunLimitInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MailFrom(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailFrom(&self, mailfrom: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailFrom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailfrom)).ok()
    }
    pub unsafe fn MailReplyTo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailReplyTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailReplyTo(&self, mailreplyto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailReplyTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailreplyto)).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailto)).ok()
    }
    pub unsafe fn MailCc(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailCc)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailCc(&self, mailcc: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailCc)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailcc)).ok()
    }
    pub unsafe fn MailBcc(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailBcc)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailBcc(&self, mailbcc: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailBcc)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailbcc)).ok()
    }
    pub unsafe fn MailSubject(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailSubject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailSubject(&self, mailsubject: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailSubject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailsubject)).ok()
    }
    pub unsafe fn MessageText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MessageText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMessageText(&self, messagetext: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMessageText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(messagetext)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmActionEmail, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionEmail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionEmail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionEmail {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionEmail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionEmail").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmActionEmail {
    type Vtable = IFsrmActionEmail_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionEmail {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd646567d_26ae_4caa_9f84_4e0aad207fca);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub MailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MailReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailreplyto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MailCc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailCc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailcc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MailBcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailBcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailbcc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MailSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailsubject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionEmail2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEmail2 {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ActionType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RunLimitInterval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRunLimitInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MailFrom(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MailFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailFrom(&self, mailfrom: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMailFrom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailfrom)).ok()
    }
    pub unsafe fn MailReplyTo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MailReplyTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailReplyTo(&self, mailreplyto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMailReplyTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailreplyto)).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MailTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMailTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailto)).ok()
    }
    pub unsafe fn MailCc(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MailCc)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailCc(&self, mailcc: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMailCc)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailcc)).ok()
    }
    pub unsafe fn MailBcc(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MailBcc)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailBcc(&self, mailbcc: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMailBcc)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailbcc)).ok()
    }
    pub unsafe fn MailSubject(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MailSubject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailSubject(&self, mailsubject: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMailSubject)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailsubject)).ok()
    }
    pub unsafe fn MessageText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MessageText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMessageText(&self, messagetext: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMessageText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(messagetext)).ok()
    }
    pub unsafe fn AttachmentFileListSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AttachmentFileListSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAttachmentFileListSize)(::windows::core::Vtable::as_raw(self), attachmentfilelistsize).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmActionEmail2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction, IFsrmActionEmail);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionEmail2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionEmail2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionEmail2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionEmail2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionEmail2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmActionEmail2 {
    type Vtable = IFsrmActionEmail2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionEmail2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8276702f_2532_4839_89bf_4872609a2ea4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail2_Vtbl {
    pub base__: IFsrmActionEmail_Vtbl,
    pub AttachmentFileListSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentfilelistsize: *mut i32) -> ::windows::core::HRESULT,
    pub SetAttachmentFileListSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentfilelistsize: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionEventLog(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionEventLog {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ActionType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RunLimitInterval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRunLimitInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EventType(&self) -> ::windows::core::Result<FsrmEventType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EventType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEventType(&self, eventtype: FsrmEventType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEventType)(::windows::core::Vtable::as_raw(self), eventtype).ok()
    }
    pub unsafe fn MessageText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MessageText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMessageText(&self, messagetext: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMessageText)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(messagetext)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmActionEventLog, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionEventLog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionEventLog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionEventLog {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionEventLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionEventLog").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmActionEventLog {
    type Vtable = IFsrmActionEventLog_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionEventLog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c8f96c3_5d94_4f37_a4f4_f56ab463546f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEventLog_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtype: *mut FsrmEventType) -> ::windows::core::HRESULT,
    pub SetEventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventtype: FsrmEventType) -> ::windows::core::HRESULT,
    pub MessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMessageText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmActionReport(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmActionReport {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::core::Result<FsrmActionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ActionType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RunLimitInterval)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRunLimitInterval)(::windows::core::Vtable::as_raw(self), minutes).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReportTypes(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReportTypes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetReportTypes(&self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetReportTypes)(::windows::core::Vtable::as_raw(self), reporttypes).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailto)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmActionReport, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmAction);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmActionReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmActionReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmActionReport {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmActionReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmActionReport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmActionReport {
    type Vtable = IFsrmActionReport_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmActionReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dbe63c4_b340_48a0_a5b0_158e07fc567e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionReport_Vtbl {
    pub base__: IFsrmAction_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ReportTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ReportTypes: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetReportTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetReportTypes: usize,
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmAutoApplyQuota(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmAutoApplyQuota {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QuotaLimit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit(&self, quotalimit: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetQuotaLimit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(quotalimit)).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QuotaFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetQuotaFlags)(::windows::core::Vtable::as_raw(self), quotaflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Thresholds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ModifyThreshold)(::windows::core::Vtable::as_raw(self), threshold, newthreshold).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateThresholdAction)(::windows::core::Vtable::as_raw(self), threshold, actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumThresholdActions)(::windows::core::Vtable::as_raw(self), threshold, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserSid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserAccount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SourceTemplateName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MatchesSourceTemplate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ApplyTemplate(&self, quotatemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ApplyTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(quotatemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExcludeFolders(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExcludeFolders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExcludeFolders(&self, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetExcludeFolders)(::windows::core::Vtable::as_raw(self), folders).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommitAndUpdateDerived)(::windows::core::Vtable::as_raw(self), commitoptions, applyoptions, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmAutoApplyQuota, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase, IFsrmQuotaObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmAutoApplyQuota {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmAutoApplyQuota {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmAutoApplyQuota {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmAutoApplyQuota {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmAutoApplyQuota").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmAutoApplyQuota {
    type Vtable = IFsrmAutoApplyQuota_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmAutoApplyQuota {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf82e5729_6aba_4740_bfc7_c7f58f75fb7b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAutoApplyQuota_Vtbl {
    pub base__: IFsrmQuotaObject_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ExcludeFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ExcludeFolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetExcludeFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetExcludeFolders: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassificationManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClassificationReportFormats)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetClassificationReportFormats)(::windows::core::Vtable::as_raw(self), formats).ok()
    }
    pub unsafe fn Logging(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Logging)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLogging)(::windows::core::Vtable::as_raw(self), logging).ok()
    }
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClassificationReportMailTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClassificationReportMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetClassificationReportMailTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailto)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClassificationReportEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassificationReportEnabled<P0>(&self, reportenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetClassificationReportEnabled)(::windows::core::Vtable::as_raw(self), reportenabled.into()).ok()
    }
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClassificationLastReportPathWithoutExtension)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClassificationLastError(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClassificationLastError)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ClassificationRunningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumPropertyDefinitions)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePropertyDefinition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyDefinition(&self, propertyname: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPropertyDefinition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumRules)(::windows::core::Vtable::as_raw(self), ruletype, options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRule)(::windows::core::Vtable::as_raw(self), ruletype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRule(&self, rulename: &::windows::core::BSTR, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRule)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(rulename), ruletype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumModuleDefinitions)(::windows::core::Vtable::as_raw(self), moduletype, options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateModuleDefinition)(::windows::core::Vtable::as_raw(self), moduletype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetModuleDefinition(&self, modulename: &::windows::core::BSTR, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetModuleDefinition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(modulename), moduletype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RunClassification(&self, context: FsrmReportGenerationContext, reserved: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RunClassification)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(reserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WaitForClassificationCompletion)(::windows::core::Vtable::as_raw(self), waitseconds, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CancelClassification(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CancelClassification)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileProperties(&self, filepath: &::windows::core::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumFileProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileProperty(&self, filepath: &::windows::core::BSTR, propertyname: &::windows::core::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), ::core::mem::transmute_copy(propertyname), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileProperty(&self, filepath: &::windows::core::BSTR, propertyname: &::windows::core::BSTR, propertyvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyvalue)).ok()
    }
    pub unsafe fn ClearFileProperty(&self, filepath: &::windows::core::BSTR, property: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ClearFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), ::core::mem::transmute_copy(property)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmClassificationManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassificationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassificationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassificationManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassificationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassificationManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmClassificationManager {
    type Vtable = IFsrmClassificationManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassificationManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2dc89da_ee91_48a0_85d8_cc72a56f7d04);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassificationReportFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassificationReportFormats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetClassificationReportFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetClassificationReportFormats: usize,
    pub Logging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: *mut i32) -> ::windows::core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logging: i32) -> ::windows::core::HRESULT,
    pub ClassificationReportMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetClassificationReportMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ClassificationReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ClassificationReportEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClassificationReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClassificationReportEnabled: usize,
    pub ClassificationLastReportPathWithoutExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastreportpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClassificationLastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClassificationRunningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumPropertyDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, propertydefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumPropertyDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPropertyDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: *mut ::core::ffi::c_void, propertydefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPropertyDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumRules: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulename: *mut ::core::ffi::c_void, ruletype: FsrmRuleType, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumModuleDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumModuleDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateModuleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateModuleDefinition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetModuleDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modulename: *mut ::core::ffi::c_void, moduletype: FsrmPipelineModuleType, moduledefinition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetModuleDefinition: usize,
    pub RunClassification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, reserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForClassificationCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForClassificationCompletion: usize,
    pub CancelClassification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, options: FsrmGetFilePropertyOptions, fileproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, propertyname: *mut ::core::ffi::c_void, options: FsrmGetFilePropertyOptions, property: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileProperty: usize,
    pub SetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, propertyname: *mut ::core::ffi::c_void, propertyvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, property: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassificationManager2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationManager2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClassificationReportFormats)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClassificationReportFormats)(::windows::core::Vtable::as_raw(self), formats).ok()
    }
    pub unsafe fn Logging(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Logging)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLogging)(::windows::core::Vtable::as_raw(self), logging).ok()
    }
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClassificationReportMailTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClassificationReportMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClassificationReportMailTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailto)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClassificationReportEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassificationReportEnabled<P0>(&self, reportenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetClassificationReportEnabled)(::windows::core::Vtable::as_raw(self), reportenabled.into()).ok()
    }
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClassificationLastReportPathWithoutExtension)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClassificationLastError(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClassificationLastError)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClassificationRunningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumPropertyDefinitions)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePropertyDefinition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPropertyDefinition(&self, propertyname: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmPropertyDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyDefinition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(propertyname), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumRules)(::windows::core::Vtable::as_raw(self), ruletype, options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRule)(::windows::core::Vtable::as_raw(self), ruletype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRule(&self, rulename: &::windows::core::BSTR, ruletype: FsrmRuleType) -> ::windows::core::Result<IFsrmRule> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRule)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(rulename), ruletype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumModuleDefinitions)(::windows::core::Vtable::as_raw(self), moduletype, options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateModuleDefinition)(::windows::core::Vtable::as_raw(self), moduletype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetModuleDefinition(&self, modulename: &::windows::core::BSTR, moduletype: FsrmPipelineModuleType) -> ::windows::core::Result<IFsrmPipelineModuleDefinition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetModuleDefinition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(modulename), moduletype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RunClassification(&self, context: FsrmReportGenerationContext, reserved: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RunClassification)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(reserved)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.WaitForClassificationCompletion)(::windows::core::Vtable::as_raw(self), waitseconds, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CancelClassification(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelClassification)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileProperties(&self, filepath: &::windows::core::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumFileProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileProperty(&self, filepath: &::windows::core::BSTR, propertyname: &::windows::core::BSTR, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), ::core::mem::transmute_copy(propertyname), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileProperty(&self, filepath: &::windows::core::BSTR, propertyname: &::windows::core::BSTR, propertyvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), ::core::mem::transmute_copy(propertyname), ::core::mem::transmute_copy(propertyvalue)).ok()
    }
    pub unsafe fn ClearFileProperty(&self, filepath: &::windows::core::BSTR, property: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ClearFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), ::core::mem::transmute_copy(property)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassifyFiles(&self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ClassifyFiles)(::windows::core::Vtable::as_raw(self), filepaths, propertynames, propertyvalues, options).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmClassificationManager2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmClassificationManager);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassificationManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassificationManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassificationManager2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassificationManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassificationManager2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmClassificationManager2 {
    type Vtable = IFsrmClassificationManager2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassificationManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0004c1c9_127e_4765_ba07_6a3147bca112);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager2_Vtbl {
    pub base__: IFsrmClassificationManager_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ClassifyFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ClassifyFiles: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassificationRule(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassificationRule {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn RuleType(&self) -> ::windows::core::Result<FsrmRuleType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RuleType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ModuleDefinitionName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetModuleDefinitionName(&self, moduledefinitionname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetModuleDefinitionName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(moduledefinitionname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NamespaceRoots)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNamespaceRoots)(::windows::core::Vtable::as_raw(self), namespaceroots).ok()
    }
    pub unsafe fn RuleFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RuleFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRuleFlags)(::windows::core::Vtable::as_raw(self), ruleflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParameters)(::windows::core::Vtable::as_raw(self), parameters).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastModified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ExecutionOption(&self) -> ::windows::core::Result<FsrmExecutionOption> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExecutionOption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetExecutionOption(&self, executionoption: FsrmExecutionOption) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetExecutionOption)(::windows::core::Vtable::as_raw(self), executionoption).ok()
    }
    pub unsafe fn PropertyAffected(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyAffected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyAffected(&self, property: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPropertyAffected)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(property)).ok()
    }
    pub unsafe fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetValue(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmClassificationRule, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmRule);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassificationRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassificationRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassificationRule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassificationRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassificationRule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmClassificationRule {
    type Vtable = IFsrmClassificationRule_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassificationRule {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafc052c2_5315_45ab_841b_c6db0e120148);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationRule_Vtbl {
    pub base__: IFsrmRule_Vtbl,
    pub ExecutionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionoption: *mut FsrmExecutionOption) -> ::windows::core::HRESULT,
    pub SetExecutionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, executionoption: FsrmExecutionOption) -> ::windows::core::HRESULT,
    pub PropertyAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPropertyAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassifierModuleDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassifierModuleDefinition {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ModuleClsid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ModuleClsid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetModuleClsid(&self, moduleclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetModuleClsid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(moduleclsid)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Company(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Company)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCompany(&self, company: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCompany)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(company)).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Version)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVersion(&self, version: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(version)).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ModuleType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Enabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnabled)(::windows::core::Vtable::as_raw(self), enabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NeedsFileContent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NeedsFileContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedsFileContent<P0>(&self, needsfilecontent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNeedsFileContent)(::windows::core::Vtable::as_raw(self), needsfilecontent.into()).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Account)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAccount)(::windows::core::Vtable::as_raw(self), retrievalaccount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SupportedExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSupportedExtensions)(::windows::core::Vtable::as_raw(self), supportedextensions).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParameters)(::windows::core::Vtable::as_raw(self), parameters).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesAffected(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertiesAffected)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesAffected(&self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPropertiesAffected)(::windows::core::Vtable::as_raw(self), propertiesaffected).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesUsed(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertiesUsed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesUsed(&self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPropertiesUsed)(::windows::core::Vtable::as_raw(self), propertiesused).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NeedsExplicitValue(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NeedsExplicitValue)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedsExplicitValue<P0>(&self, needsexplicitvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetNeedsExplicitValue)(::windows::core::Vtable::as_raw(self), needsexplicitvalue.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmClassifierModuleDefinition, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmPipelineModuleDefinition);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassifierModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassifierModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassifierModuleDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassifierModuleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassifierModuleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmClassifierModuleDefinition {
    type Vtable = IFsrmClassifierModuleDefinition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassifierModuleDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb36ea26_6318_4b8c_8592_f72dd602e7a5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertiesAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertiesAffected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertiesAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertiesAffected: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertiesUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertiesUsed: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPropertiesUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPropertiesUsed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NeedsExplicitValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsexplicitvalue: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NeedsExplicitValue: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNeedsExplicitValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsexplicitvalue: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNeedsExplicitValue: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmClassifierModuleImplementation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmClassifierModuleImplementation {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnLoad<P0>(&self, moduledefinition: P0) -> ::windows::core::Result<IFsrmPipelineModuleConnector>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmPipelineModuleDefinition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OnLoad)(::windows::core::Vtable::as_raw(self), moduledefinition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnUnload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastModified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UseRulesAndDefinitions<P0, P1>(&self, rules: P0, propertydefinitions: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmCollection>>,
        P1: ::std::convert::Into<::windows::core::InParam<IFsrmCollection>>,
    {
        (::windows::core::Vtable::vtable(self).UseRulesAndDefinitions)(::windows::core::Vtable::as_raw(self), rules.into().abi(), propertydefinitions.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnBeginFile<P0>(&self, propertybag: P0, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmPropertyBag>>,
    {
        (::windows::core::Vtable::vtable(self).OnBeginFile)(::windows::core::Vtable::as_raw(self), propertybag.into().abi(), arrayruleids).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesPropertyValueApply(&self, property: &::windows::core::BSTR, value: &::windows::core::BSTR, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DoesPropertyValueApply)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(property), ::core::mem::transmute_copy(value), applyvalue, ::core::mem::transmute(idrule), ::core::mem::transmute(idpropdef)).ok()
    }
    pub unsafe fn GetPropertyValueToApply(&self, property: &::windows::core::BSTR, value: *mut ::windows::core::BSTR, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPropertyValueToApply)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(property), ::core::mem::transmute(value), ::core::mem::transmute(idrule), ::core::mem::transmute(idpropdef)).ok()
    }
    pub unsafe fn OnEndFile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnEndFile)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmClassifierModuleImplementation, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmPipelineModuleImplementation);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmClassifierModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmClassifierModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmClassifierModuleImplementation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmClassifierModuleImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmClassifierModuleImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmClassifierModuleImplementation {
    type Vtable = IFsrmClassifierModuleImplementation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmClassifierModuleImplementation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c968fc6_6edb_4051_9c18_73b7291ae106);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastModified: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UseRulesAndDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: *mut ::core::ffi::c_void, propertydefinitions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UseRulesAndDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnBeginFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: *mut ::core::ffi::c_void, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnBeginFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesPropertyValueApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, applyvalue: *mut super::super::Foundation::VARIANT_BOOL, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesPropertyValueApply: usize,
    pub GetPropertyValueToApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void, idrule: ::windows::core::GUID, idpropdef: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnEndFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self)._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).get_Item)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<FsrmCollectionState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WaitForCompletion)(::windows::core::Vtable::as_raw(self), waitseconds, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetById(&self, id: ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetById)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(id), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmCollection {
    type Vtable = IFsrmCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf76fbf3b_8ddd_4b42_b05a_cb1c3ff1fee8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCollection_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, item: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut FsrmCollectionState) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForCompletion: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID, entry: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetById: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmCommittableCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmCommittableCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_Item)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<FsrmCollectionState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.WaitForCompletion)(::windows::core::Vtable::as_raw(self), waitseconds, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetById(&self, id: ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetById)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(id), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add(&self, item: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Add)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(item)).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Remove)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn RemoveById(&self, id: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveById)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, options: FsrmCommitOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Commit)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmCommittableCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmCollection, IFsrmMutableCollection);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmCommittableCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmCommittableCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmCommittableCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmCommittableCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmCommittableCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmCommittableCollection {
    type Vtable = IFsrmCommittableCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmCommittableCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96deb3b5_8b91_4a2a_9d93_80a35d8aa847);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCommittableCollection_Vtbl {
    pub base__: IFsrmMutableCollection_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmCommitOptions, results: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Commit: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmDerivedObjectsResult(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmDerivedObjectsResult {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DerivedObjects(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DerivedObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Results(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Results)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmDerivedObjectsResult, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmDerivedObjectsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmDerivedObjectsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmDerivedObjectsResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmDerivedObjectsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmDerivedObjectsResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmDerivedObjectsResult {
    type Vtable = IFsrmDerivedObjectsResult_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmDerivedObjectsResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39322a2d_38ee_4d0d_8095_421a80849a82);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmDerivedObjectsResult_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub DerivedObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, derivedobjects: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DerivedObjects: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, results: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmExportImport(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmExportImport {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportFileGroups(&self, filepath: &::windows::core::BSTR, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ExportFileGroups)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), filegroupnamessafearray, ::core::mem::transmute_copy(remotehost)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportFileGroups(&self, filepath: &::windows::core::BSTR, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ImportFileGroups)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), filegroupnamessafearray, ::core::mem::transmute_copy(remotehost), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportFileScreenTemplates(&self, filepath: &::windows::core::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ExportFileScreenTemplates)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), templatenamessafearray, ::core::mem::transmute_copy(remotehost)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportFileScreenTemplates(&self, filepath: &::windows::core::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ImportFileScreenTemplates)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), templatenamessafearray, ::core::mem::transmute_copy(remotehost), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportQuotaTemplates(&self, filepath: &::windows::core::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ExportQuotaTemplates)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), templatenamessafearray, ::core::mem::transmute_copy(remotehost)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportQuotaTemplates(&self, filepath: &::windows::core::BSTR, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ImportQuotaTemplates)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filepath), templatenamessafearray, ::core::mem::transmute_copy(remotehost), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmExportImport, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmExportImport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmExportImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmExportImport {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmExportImport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmExportImport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmExportImport {
    type Vtable = IFsrmExportImport_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmExportImport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefcb0ab1_16c4_4a79_812c_725614c3306b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmExportImport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: *mut ::core::ffi::c_void, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportFileScreenTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportFileScreenTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportFileScreenTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: *mut ::core::ffi::c_void, templates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportFileScreenTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportQuotaTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportQuotaTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportQuotaTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: *mut ::core::ffi::c_void, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: *mut ::core::ffi::c_void, templates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportQuotaTemplates: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileCondition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileCondition {
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmFileConditionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileCondition, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileCondition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileCondition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileCondition {
    type Vtable = IFsrmFileCondition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70684ffc_691a_4a1a_b922_97752e138cc1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileCondition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileConditionType) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileConditionProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileConditionProperty {
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmFileConditionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PropertyName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyName(&self, newval: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPropertyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(newval)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<FsrmFileSystemPropertyId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, newval: FsrmFileSystemPropertyId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPropertyId)(::windows::core::Vtable::as_raw(self), newval).ok()
    }
    pub unsafe fn Operator(&self) -> ::windows::core::Result<FsrmPropertyConditionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Operator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOperator(&self, newval: FsrmPropertyConditionType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOperator)(::windows::core::Vtable::as_raw(self), newval).ok()
    }
    pub unsafe fn ValueType(&self) -> ::windows::core::Result<FsrmPropertyValueType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ValueType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetValueType(&self, newval: FsrmPropertyValueType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValueType)(::windows::core::Vtable::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, newval: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(newval)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileConditionProperty, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmFileCondition);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileConditionProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileConditionProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileConditionProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileConditionProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileConditionProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileConditionProperty {
    type Vtable = IFsrmFileConditionProperty_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileConditionProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81926775_b981_4479_988f_da171d627360);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileConditionProperty_Vtbl {
    pub base__: IFsrmFileCondition_Vtbl,
    pub PropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PropertyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmFileSystemPropertyId) -> ::windows::core::HRESULT,
    pub SetPropertyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmFileSystemPropertyId) -> ::windows::core::HRESULT,
    pub Operator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub SetOperator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub ValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut FsrmPropertyValueType) -> ::windows::core::HRESULT,
    pub SetValueType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: FsrmPropertyValueType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileGroup(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroup {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Members(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Members)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMembers<P0>(&self, members: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmMutableCollection>>,
    {
        (::windows::core::Vtable::vtable(self).SetMembers)(::windows::core::Vtable::as_raw(self), members.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NonMembers(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NonMembers)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNonMembers<P0>(&self, nonmembers: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmMutableCollection>>,
    {
        (::windows::core::Vtable::vtable(self).SetNonMembers)(::windows::core::Vtable::as_raw(self), nonmembers.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileGroup, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileGroup {
    type Vtable = IFsrmFileGroup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dd04909_0e34_4d55_afaa_89e1f1a1bbb9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroup_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Members: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Members: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, members: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetMembers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NonMembers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNonMembers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonmembers: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNonMembers: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileGroupImported(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroupImported {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Members(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Members)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetMembers<P0>(&self, members: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmMutableCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMembers)(::windows::core::Vtable::as_raw(self), members.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NonMembers(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NonMembers)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNonMembers<P0>(&self, nonmembers: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmMutableCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNonMembers)(::windows::core::Vtable::as_raw(self), nonmembers.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OverwriteOnCommit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOverwriteOnCommit<P0>(&self, overwrite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetOverwriteOnCommit)(::windows::core::Vtable::as_raw(self), overwrite.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileGroupImported, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileGroup);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileGroupImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileGroupImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileGroupImported {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileGroupImported {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileGroupImported").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileGroupImported {
    type Vtable = IFsrmFileGroupImported_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileGroupImported {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad55f10b_5f11_4be7_94ef_d9ee2e470ded);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupImported_Vtbl {
    pub base__: IFsrmFileGroup_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OverwriteOnCommit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOverwriteOnCommit: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileGroupManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileGroupManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileGroup(&self) -> ::windows::core::Result<IFsrmFileGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFileGroup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileGroup(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileGroup)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileGroups(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumFileGroups)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportFileGroups(&self, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExportFileGroups)(::windows::core::Vtable::as_raw(self), filegroupnamesarray, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportFileGroups(&self, serializedfilegroups: &::windows::core::BSTR, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ImportFileGroups)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(serializedfilegroups), filegroupnamesarray, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileGroupManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileGroupManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileGroupManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileGroupManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileGroupManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileGroupManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileGroupManager {
    type Vtable = IFsrmFileGroupManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileGroupManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x426677d5_018c_485c_8a51_20b86d00bdc4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filegroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, filegroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileGroup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filegroupnamesarray: *const super::super::System::Com::VARIANT, serializedfilegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportFileGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedfilegroups: *mut ::core::ffi::c_void, filegroupnamesarray: *const super::super::System::Com::VARIANT, filegroups: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportFileGroups: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileManagementJob(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileManagementJob {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NamespaceRoots)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNamespaceRoots)(::windows::core::Vtable::as_raw(self), namespaceroots).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Enabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnabled)(::windows::core::Vtable::as_raw(self), enabled.into()).ok()
    }
    pub unsafe fn OperationType(&self) -> ::windows::core::Result<FsrmFileManagementType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OperationType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOperationType(&self, operationtype: FsrmFileManagementType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOperationType)(::windows::core::Vtable::as_raw(self), operationtype).ok()
    }
    pub unsafe fn ExpirationDirectory(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExpirationDirectory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetExpirationDirectory(&self, expirationdirectory: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetExpirationDirectory)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(expirationdirectory)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CustomAction(&self) -> ::windows::core::Result<IFsrmActionCommand> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CustomAction)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Notifications(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Notifications)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Logging(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Logging)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLogging(&self, loggingflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLogging)(::windows::core::Vtable::as_raw(self), loggingflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReportEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ReportEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReportEnabled<P0>(&self, reportenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetReportEnabled)(::windows::core::Vtable::as_raw(self), reportenabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Formats)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFormats)(::windows::core::Vtable::as_raw(self), formats).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailto)).ok()
    }
    pub unsafe fn DaysSinceFileCreated(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DaysSinceFileCreated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDaysSinceFileCreated(&self, dayssincecreation: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDaysSinceFileCreated)(::windows::core::Vtable::as_raw(self), dayssincecreation).ok()
    }
    pub unsafe fn DaysSinceFileLastAccessed(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DaysSinceFileLastAccessed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDaysSinceFileLastAccessed(&self, dayssinceaccess: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDaysSinceFileLastAccessed)(::windows::core::Vtable::as_raw(self), dayssinceaccess).ok()
    }
    pub unsafe fn DaysSinceFileLastModified(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DaysSinceFileLastModified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDaysSinceFileLastModified(&self, dayssincemodify: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDaysSinceFileLastModified)(::windows::core::Vtable::as_raw(self), dayssincemodify).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertyConditions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyConditions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FromDate(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FromDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFromDate(&self, fromdate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFromDate)(::windows::core::Vtable::as_raw(self), fromdate).ok()
    }
    pub unsafe fn Task(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Task)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTask(&self, taskname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTask)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(taskname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetParameters)(::windows::core::Vtable::as_raw(self), parameters).ok()
    }
    pub unsafe fn RunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RunningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastError(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastError)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastReportPathWithoutExtension(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastReportPathWithoutExtension)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastRun(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastRun)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn FileNamePattern(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FileNamePattern)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileNamePattern(&self, filenamepattern: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFileNamePattern)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filenamepattern)).ok()
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Run)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WaitForCompletion)(::windows::core::Vtable::as_raw(self), waitseconds, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn AddNotification(&self, days: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddNotification)(::windows::core::Vtable::as_raw(self), days).ok()
    }
    pub unsafe fn DeleteNotification(&self, days: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteNotification)(::windows::core::Vtable::as_raw(self), days).ok()
    }
    pub unsafe fn ModifyNotification(&self, days: i32, newdays: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ModifyNotification)(::windows::core::Vtable::as_raw(self), days, newdays).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateNotificationAction(&self, days: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateNotificationAction)(::windows::core::Vtable::as_raw(self), days, actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumNotificationActions(&self, days: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumNotificationActions)(::windows::core::Vtable::as_raw(self), days, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyCondition(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmPropertyCondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePropertyCondition)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateCustomAction(&self) -> ::windows::core::Result<IFsrmActionCommand> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCustomAction)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileManagementJob, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileManagementJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileManagementJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileManagementJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileManagementJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileManagementJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileManagementJob {
    type Vtable = IFsrmFileManagementJob_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileManagementJob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0770687e_9f36_4d6f_8778_599d188461c9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJob_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    pub OperationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationtype: *mut FsrmFileManagementType) -> ::windows::core::HRESULT,
    pub SetOperationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operationtype: FsrmFileManagementType) -> ::windows::core::HRESULT,
    pub ExpirationDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdirectory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetExpirationDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expirationdirectory: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CustomAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CustomAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Notifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Notifications: usize,
    pub Logging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingflags: *mut i32) -> ::windows::core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, loggingflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReportEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetReportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetReportEnabled: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Formats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Formats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormats: usize,
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DaysSinceFileCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincecreation: *mut i32) -> ::windows::core::HRESULT,
    pub SetDaysSinceFileCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincecreation: i32) -> ::windows::core::HRESULT,
    pub DaysSinceFileLastAccessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssinceaccess: *mut i32) -> ::windows::core::HRESULT,
    pub SetDaysSinceFileLastAccessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssinceaccess: i32) -> ::windows::core::HRESULT,
    pub DaysSinceFileLastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincemodify: *mut i32) -> ::windows::core::HRESULT,
    pub SetDaysSinceFileLastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dayssincemodify: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PropertyConditions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyconditions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PropertyConditions: usize,
    pub FromDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromdate: *mut f64) -> ::windows::core::HRESULT,
    pub SetFromDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fromdate: f64) -> ::windows::core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
    pub RunningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    pub LastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastReportPathWithoutExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT,
    pub FileNamePattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filenamepattern: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFileNamePattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filenamepattern: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForCompletion: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT,
    pub DeleteNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32) -> ::windows::core::HRESULT,
    pub ModifyNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, newdays: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateNotificationAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateNotificationAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumNotificationActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, days: i32, actions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumNotificationActions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, propertycondition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyCondition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateCustomAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateCustomAction: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileManagementJobManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileManagementJobManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ActionVariables)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ActionVariableDescriptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileManagementJobs(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumFileManagementJobs)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileManagementJob(&self) -> ::windows::core::Result<IFsrmFileManagementJob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFileManagementJob)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileManagementJob(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileManagementJob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileManagementJob)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileManagementJobManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileManagementJobManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileManagementJobManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileManagementJobManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileManagementJobManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileManagementJobManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileManagementJobManager {
    type Vtable = IFsrmFileManagementJobManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileManagementJobManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee321ecb_d95e_48e9_907c_c7685a013235);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJobManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileManagementJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filemanagementjobs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileManagementJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileManagementJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filemanagementjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileManagementJob: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileManagementJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, filemanagementjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileManagementJob: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreen(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreen {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BlockedFileGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBlockedFileGroups<P0>(&self, blocklist: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmMutableCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBlockedFileGroups)(::windows::core::Vtable::as_raw(self), blocklist.into().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FileScreenFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileScreenFlags)(::windows::core::Vtable::as_raw(self), filescreenflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateAction)(::windows::core::Vtable::as_raw(self), actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumActions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SourceTemplateName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MatchesSourceTemplate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserSid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserAccount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ApplyTemplate(&self, filescreentemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ApplyTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filescreentemplatename)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileScreen, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileScreenBase);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreen {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreen").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileScreen {
    type Vtable = IFsrmFileScreen_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreen {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f6325d3_ce88_4733_84c1_2d6aefc5ea07);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreen_Vtbl {
    pub base__: IFsrmFileScreenBase_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SourceTemplateName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchesSourceTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchesSourceTemplate: usize,
    pub UserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ApplyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenBase(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenBase {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).BlockedFileGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBlockedFileGroups<P0>(&self, blocklist: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmMutableCollection>>,
    {
        (::windows::core::Vtable::vtable(self).SetBlockedFileGroups)(::windows::core::Vtable::as_raw(self), blocklist.into().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FileScreenFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFileScreenFlags)(::windows::core::Vtable::as_raw(self), filescreenflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAction)(::windows::core::Vtable::as_raw(self), actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumActions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileScreenBase, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenBase {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenBase").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileScreenBase {
    type Vtable = IFsrmFileScreenBase_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenBase {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3637e80_5b22_4a2b_a637_bbb642b41cfc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenBase_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub BlockedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BlockedFileGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBlockedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocklist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBlockedFileGroups: usize,
    pub FileScreenFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: *mut i32) -> ::windows::core::HRESULT,
    pub SetFileScreenFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreenflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumActions: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenException(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenException {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AllowedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AllowedFileGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAllowedFileGroups<P0>(&self, allowlist: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmMutableCollection>>,
    {
        (::windows::core::Vtable::vtable(self).SetAllowedFileGroups)(::windows::core::Vtable::as_raw(self), allowlist.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileScreenException, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenException {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenException").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileScreenException {
    type Vtable = IFsrmFileScreenException_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenException {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbee7ce02_df77_4515_9389_78f01c5afc1a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenException_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AllowedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AllowedFileGroups: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetAllowedFileGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowlist: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetAllowedFileGroups: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ActionVariables)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ActionVariableDescriptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileScreen(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreen> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFileScreen)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileScreen(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreen> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileScreen)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileScreens(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumFileScreens)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileScreenException(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreenException> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFileScreenException)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileScreenException(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreenException> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileScreenException)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumFileScreenExceptions(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumFileScreenExceptions)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateFileScreenCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFileScreenCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileScreenManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileScreenManager {
    type Vtable = IFsrmFileScreenManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff4fa04e_5a94_4bda_a3a0_d5b4d3c52eba);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, filescreen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileScreen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, filescreen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileScreen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileScreens: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreens: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileScreens: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileScreenException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, filescreenexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileScreenException: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileScreenException: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, filescreenexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileScreenException: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumFileScreenExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreenexceptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumFileScreenExceptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateFileScreenCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateFileScreenCollection: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenTemplate(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplate {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BlockedFileGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBlockedFileGroups<P0>(&self, blocklist: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmMutableCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBlockedFileGroups)(::windows::core::Vtable::as_raw(self), blocklist.into().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FileScreenFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileScreenFlags)(::windows::core::Vtable::as_raw(self), filescreenflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateAction)(::windows::core::Vtable::as_raw(self), actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumActions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn CopyTemplate(&self, filescreentemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CopyTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filescreentemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommitAndUpdateDerived)(::windows::core::Vtable::as_raw(self), commitoptions, applyoptions, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileScreenTemplate, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileScreenBase);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenTemplate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenTemplate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileScreenTemplate {
    type Vtable = IFsrmFileScreenTemplate_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenTemplate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x205bebf8_dd93_452a_95a6_32b566b35828);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplate_Vtbl {
    pub base__: IFsrmFileScreenBase_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateImported(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplateImported {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.BlockedFileGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBlockedFileGroups<P0>(&self, blocklist: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmMutableCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBlockedFileGroups)(::windows::core::Vtable::as_raw(self), blocklist.into().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.FileScreenFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFileScreenFlags)(::windows::core::Vtable::as_raw(self), filescreenflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateAction)(::windows::core::Vtable::as_raw(self), actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumActions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumActions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn CopyTemplate(&self, filescreentemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filescreentemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CommitAndUpdateDerived)(::windows::core::Vtable::as_raw(self), commitoptions, applyoptions, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OverwriteOnCommit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOverwriteOnCommit<P0>(&self, overwrite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetOverwriteOnCommit)(::windows::core::Vtable::as_raw(self), overwrite.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileScreenTemplateImported, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmFileScreenBase, IFsrmFileScreenTemplate);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenTemplateImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplateImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenTemplateImported {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenTemplateImported {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenTemplateImported").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileScreenTemplateImported {
    type Vtable = IFsrmFileScreenTemplateImported_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenTemplateImported {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1010359_3e5d_4ecd_9fe4_ef48622fdf30);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateImported_Vtbl {
    pub base__: IFsrmFileScreenTemplate_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OverwriteOnCommit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOverwriteOnCommit: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmFileScreenTemplateManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmFileScreenTemplateManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTemplate(&self) -> ::windows::core::Result<IFsrmFileScreenTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTemplate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTemplate(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmFileScreenTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumTemplates)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportTemplates(&self, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExportTemplates)(::windows::core::Vtable::as_raw(self), filescreentemplatenamesarray, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportTemplates(&self, serializedfilescreentemplates: &::windows::core::BSTR, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ImportTemplates)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(serializedfilescreentemplates), filescreentemplatenamesarray, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmFileScreenTemplateManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmFileScreenTemplateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmFileScreenTemplateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmFileScreenTemplateManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmFileScreenTemplateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmFileScreenTemplateManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmFileScreenTemplateManager {
    type Vtable = IFsrmFileScreenTemplateManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmFileScreenTemplateManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfe36cba_1949_4e74_a14f_f1d580ceaf13);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, filescreentemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, filescreentemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, serializedfilescreentemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedfilescreentemplates: *mut ::core::ffi::c_void, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT, filescreentemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportTemplates: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmMutableCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmMutableCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Item)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<FsrmCollectionState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.WaitForCompletion)(::windows::core::Vtable::as_raw(self), waitseconds, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetById(&self, id: ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetById)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(id), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add(&self, item: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Add)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(item)).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Remove)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn RemoveById(&self, id: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveById)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IFsrmMutableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmMutableCollection, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmCollection);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmMutableCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmMutableCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmMutableCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmMutableCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmMutableCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmMutableCollection {
    type Vtable = IFsrmMutableCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmMutableCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bb617b8_3886_49dc_af82_a6c90fa35dda);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmMutableCollection_Vtbl {
    pub base__: IFsrmCollection_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    pub RemoveById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Clone: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmObject(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmObject {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmObject, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmObject {
    type Vtable = IFsrmObject_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22bcef93_4a3f_4183_89f9_2f8b8a628aee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmObject_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPathMapper(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPathMapper {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSharePathsForLocalPath(&self, localpath: &::windows::core::BSTR) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSharePathsForLocalPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(localpath), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPathMapper, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPathMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPathMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPathMapper {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPathMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPathMapper").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPathMapper {
    type Vtable = IFsrmPathMapper_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPathMapper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f4dbfff_6920_4821_a6c3_b7e94c1fd60c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPathMapper_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSharePathsForLocalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localpath: *mut ::core::ffi::c_void, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSharePathsForLocalPath: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPipelineModuleConnector(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleConnector {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ModuleImplementation(&self) -> ::windows::core::Result<IFsrmPipelineModuleImplementation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ModuleImplementation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ModuleName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ModuleName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HostingUserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HostingUserAccount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn HostingProcessPid(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HostingProcessPid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Bind<P0, P1>(&self, moduledefinition: P0, moduleimplementation: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmPipelineModuleDefinition>>,
        P1: ::std::convert::Into<::windows::core::InParam<IFsrmPipelineModuleImplementation>>,
    {
        (::windows::core::Vtable::vtable(self).Bind)(::windows::core::Vtable::as_raw(self), moduledefinition.into().abi(), moduleimplementation.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPipelineModuleConnector, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPipelineModuleConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPipelineModuleConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPipelineModuleConnector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPipelineModuleConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPipelineModuleConnector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPipelineModuleConnector {
    type Vtable = IFsrmPipelineModuleConnector_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPipelineModuleConnector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc16014f3_9aa1_46b3_b0a7_ab146eb205f2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleConnector_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ModuleImplementation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipelinemoduleimplementation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ModuleImplementation: usize,
    pub ModuleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HostingUserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HostingProcessPid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Bind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: *mut ::core::ffi::c_void, moduleimplementation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Bind: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPipelineModuleDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleDefinition {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ModuleClsid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ModuleClsid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetModuleClsid(&self, moduleclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetModuleClsid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(moduleclsid)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Company(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Company)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCompany(&self, company: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCompany)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(company)).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Version)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVersion(&self, version: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(version)).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ModuleType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Enabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnabled)(::windows::core::Vtable::as_raw(self), enabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NeedsFileContent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NeedsFileContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedsFileContent<P0>(&self, needsfilecontent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetNeedsFileContent)(::windows::core::Vtable::as_raw(self), needsfilecontent.into()).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Account)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAccount)(::windows::core::Vtable::as_raw(self), retrievalaccount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SupportedExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSupportedExtensions)(::windows::core::Vtable::as_raw(self), supportedextensions).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetParameters)(::windows::core::Vtable::as_raw(self), parameters).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPipelineModuleDefinition, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPipelineModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPipelineModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPipelineModuleDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPipelineModuleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPipelineModuleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPipelineModuleDefinition {
    type Vtable = IFsrmPipelineModuleDefinition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPipelineModuleDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x515c1277_2c81_440e_8fcf_367921ed4f59);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleDefinition_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub ModuleClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetModuleClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduleclsid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Company: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompany: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, company: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModuleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduletype: *mut FsrmPipelineModuleType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub NeedsFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NeedsFileContent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNeedsFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, needsfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNeedsFileContent: usize,
    pub Account: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: *mut FsrmAccountType) -> ::windows::core::HRESULT,
    pub SetAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retrievalaccount: FsrmAccountType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SupportedExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupportedExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSupportedExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSupportedExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPipelineModuleImplementation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPipelineModuleImplementation {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnLoad<P0>(&self, moduledefinition: P0) -> ::windows::core::Result<IFsrmPipelineModuleConnector>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmPipelineModuleDefinition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OnLoad)(::windows::core::Vtable::as_raw(self), moduledefinition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).OnUnload)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPipelineModuleImplementation, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPipelineModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPipelineModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPipelineModuleImplementation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPipelineModuleImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPipelineModuleImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPipelineModuleImplementation {
    type Vtable = IFsrmPipelineModuleImplementation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPipelineModuleImplementation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7907906_2b02_4cb5_84a9_fdf54613d6cd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleImplementation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinition: *mut ::core::ffi::c_void, moduleconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnLoad: usize,
    pub OnUnload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmProperty {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sources(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Sources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PropertyFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmProperty, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmProperty {
    type Vtable = IFsrmProperty_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a73fee4_4102_4fcc_9ffb_38614f9ee768);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Sources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Sources: usize,
    pub PropertyFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyBag(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyBag {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RelativePath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RelativePath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VolumeName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RelativeNamespaceRoot)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VolumeIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).VolumeIndex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FileId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FileId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ParentDirectoryId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Size(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Size)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SizeAllocated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreationTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastAccessTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastModificationTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Attributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OwnerSid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OwnerSid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).FilePropertyNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Messages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Messages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PropertyBagFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyBagFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileProperty(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileProperty(&self, name: &::windows::core::BSTR, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn AddMessage(&self, message: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(message)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileStreamInterface)(::windows::core::Vtable::as_raw(self), accessmode, interfacetype, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPropertyBag, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyBag {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyBag {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyBag {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyBag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyBag").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPropertyBag {
    type Vtable = IFsrmPropertyBag_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyBag {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x774589d1_d300_4f7a_9a24_f7b766800250);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RelativePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RelativeNamespaceRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativenamespaceroot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VolumeIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volumeid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FileId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ParentDirectoryId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentdirectoryid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ParentDirectoryId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Size: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SizeAllocated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sizeallocated: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SizeAllocated: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreationTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastAccessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastaccesstime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastAccessTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastModificationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodificationtime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastModificationTime: usize,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut u32) -> ::windows::core::HRESULT,
    pub OwnerSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ownersid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub FilePropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FilePropertyNames: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Messages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Messages: usize,
    pub PropertyBagFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, fileproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFileProperty: usize,
    pub SetFileProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFileStreamInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFileStreamInterface: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyBag2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyBag2 {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RelativePath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RelativePath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VolumeName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VolumeName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RelativeNamespaceRoot)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn VolumeIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VolumeIndex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FileId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FileId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParentDirectoryId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Size(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Size)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SizeAllocated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreationTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastAccessTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LastModificationTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Attributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OwnerSid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OwnerSid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FilePropertyNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Messages(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Messages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PropertyBagFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyBagFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFileProperty(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmProperty> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFileProperty(&self, name: &::windows::core::BSTR, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFileProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn AddMessage(&self, message: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(message)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileStreamInterface)(::windows::core::Vtable::as_raw(self), accessmode, interfacetype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFieldValue(&self, field: FsrmPropertyBagField) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFieldValue)(::windows::core::Vtable::as_raw(self), field, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUntrustedInFileProperties(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUntrustedInFileProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPropertyBag2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmPropertyBag);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyBag2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyBag2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyBag2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyBag2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyBag2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPropertyBag2 {
    type Vtable = IFsrmPropertyBag2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyBag2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e46bdbd_2402_4fed_9c30_9266e6eb2cc9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag2_Vtbl {
    pub base__: IFsrmPropertyBag_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFieldValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, field: FsrmPropertyBagField, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFieldValue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUntrustedInFileProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, props: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUntrustedInFileProperties: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyCondition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyCondition {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmPropertyConditionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyConditionType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetType)(::windows::core::Vtable::as_raw(self), r#type).ok()
    }
    pub unsafe fn Value(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Value)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetValue(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPropertyCondition, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyCondition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyCondition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPropertyCondition {
    type Vtable = IFsrmPropertyCondition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x326af66f_2ac0_4f68_bf8c_4759f054fa29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyCondition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyConditionType) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinition {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmPropertyDefinitionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetType)(::windows::core::Vtable::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleValues(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PossibleValues)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPossibleValues)(::windows::core::Vtable::as_raw(self), possiblevalues).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ValueDescriptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetValueDescriptions)(::windows::core::Vtable::as_raw(self), valuedescriptions).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetParameters)(::windows::core::Vtable::as_raw(self), parameters).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPropertyDefinition, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPropertyDefinition {
    type Vtable = IFsrmPropertyDefinition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xede0150f_e9a3_419c_877c_01fe5d24c5d3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut FsrmPropertyDefinitionType) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: FsrmPropertyDefinitionType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PossibleValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PossibleValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetPossibleValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetPossibleValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetValueDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetValueDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyDefinition2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinition2 {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmPropertyDefinitionType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetType)(::windows::core::Vtable::as_raw(self), r#type).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleValues(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PossibleValues)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPossibleValues)(::windows::core::Vtable::as_raw(self), possiblevalues).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ValueDescriptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetValueDescriptions)(::windows::core::Vtable::as_raw(self), valuedescriptions).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParameters)(::windows::core::Vtable::as_raw(self), parameters).ok()
    }
    pub unsafe fn PropertyDefinitionFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).PropertyDefinitionFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDisplayName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDisplayName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn AppliesTo(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AppliesTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDefinitions(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ValueDefinitions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPropertyDefinition2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmPropertyDefinition);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyDefinition2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyDefinition2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyDefinition2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyDefinition2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyDefinition2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPropertyDefinition2 {
    type Vtable = IFsrmPropertyDefinition2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyDefinition2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47782152_d16c_4229_b4e1_0ddfe308b9f6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition2_Vtbl {
    pub base__: IFsrmPropertyDefinition_Vtbl,
    pub PropertyDefinitionFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinitionflags: *mut i32) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AppliesTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appliesto: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ValueDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuedefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ValueDefinitions: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmPropertyDefinitionValue(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmPropertyDefinitionValue {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DisplayName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UniqueID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UniqueID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmPropertyDefinitionValue, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmPropertyDefinitionValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmPropertyDefinitionValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmPropertyDefinitionValue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmPropertyDefinitionValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmPropertyDefinitionValue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmPropertyDefinitionValue {
    type Vtable = IFsrmPropertyDefinitionValue_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmPropertyDefinitionValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe946d148_bd67_4178_8e22_1c44925ed710);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinitionValue_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UniqueID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uniqueid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuota(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuota {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QuotaLimit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit(&self, quotalimit: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetQuotaLimit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(quotalimit)).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QuotaFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetQuotaFlags)(::windows::core::Vtable::as_raw(self), quotaflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Thresholds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ModifyThreshold)(::windows::core::Vtable::as_raw(self), threshold, newthreshold).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateThresholdAction)(::windows::core::Vtable::as_raw(self), threshold, actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumThresholdActions)(::windows::core::Vtable::as_raw(self), threshold, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserSid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserAccount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SourceTemplateName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MatchesSourceTemplate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ApplyTemplate(&self, quotatemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ApplyTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(quotatemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaUsed(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QuotaUsed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaPeakUsage(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QuotaPeakUsage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QuotaPeakUsageTime(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QuotaPeakUsageTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ResetPeakUsage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ResetPeakUsage)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RefreshUsageProperties(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RefreshUsageProperties)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmQuota, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase, IFsrmQuotaObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuota {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuota {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuota {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuota {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuota").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmQuota {
    type Vtable = IFsrmQuota_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuota {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x377f739d_9647_4b8e_97d2_5ffce6d759cd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuota_Vtbl {
    pub base__: IFsrmQuotaObject_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QuotaUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, used: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QuotaUsed: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QuotaPeakUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peakusage: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QuotaPeakUsage: usize,
    pub QuotaPeakUsageTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peakusagedatetime: *mut f64) -> ::windows::core::HRESULT,
    pub ResetPeakUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RefreshUsageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaBase(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaBase {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QuotaLimit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit(&self, quotalimit: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetQuotaLimit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(quotalimit)).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).QuotaFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetQuotaFlags)(::windows::core::Vtable::as_raw(self), quotaflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Thresholds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ModifyThreshold)(::windows::core::Vtable::as_raw(self), threshold, newthreshold).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateThresholdAction)(::windows::core::Vtable::as_raw(self), threshold, actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumThresholdActions)(::windows::core::Vtable::as_raw(self), threshold, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmQuotaBase, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaBase {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaBase").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmQuotaBase {
    type Vtable = IFsrmQuotaBase_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaBase {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1568a795_3924_4118_b74b_68d8f0fa5daf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaBase_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub QuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    QuotaLimit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetQuotaLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotalimit: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetQuotaLimit: usize,
    pub QuotaFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: *mut i32) -> ::windows::core::HRESULT,
    pub SetQuotaFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotaflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Thresholds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Thresholds: usize,
    pub AddThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub DeleteThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32) -> ::windows::core::HRESULT,
    pub ModifyThreshold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, newthreshold: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateThresholdAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actiontype: FsrmActionType, action: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateThresholdAction: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumThresholdActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: i32, actions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumThresholdActions: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ActionVariables)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ActionVariableDescriptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAutoApplyQuota(&self, quotatemplatename: &::windows::core::BSTR, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateAutoApplyQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(quotatemplatename), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAutoApplyQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAutoApplyQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRestrictiveQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRestrictiveQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumQuotas(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumQuotas)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumAutoApplyQuotas(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumAutoApplyQuotas)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumEffectiveQuotas(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumEffectiveQuotas)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Scan(&self, strpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Scan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateQuotaCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmQuotaManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmQuotaManager {
    type Vtable = IFsrmQuotaManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bb68c7d_19d8_4ffb_809e_be4fc1734014);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariables: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariables: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActionVariableDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActionVariableDescriptions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAutoApplyQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAutoApplyQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAutoApplyQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAutoApplyQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRestrictiveQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, quota: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRestrictiveQuota: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumQuotas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumQuotas: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumAutoApplyQuotas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumAutoApplyQuotas: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumEffectiveQuotas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotas: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumEffectiveQuotas: usize,
    pub Scan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateQuotaCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateQuotaCollection: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaManagerEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaManagerEx {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ActionVariables)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ActionVariableDescriptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAutoApplyQuota(&self, quotatemplatename: &::windows::core::BSTR, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateAutoApplyQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(quotatemplatename), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAutoApplyQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmAutoApplyQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAutoApplyQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRestrictiveQuota(&self, path: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuota> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRestrictiveQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumQuotas(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumQuotas)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumAutoApplyQuotas(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumAutoApplyQuotas)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumEffectiveQuotas(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumEffectiveQuotas)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Scan(&self, strpath: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Scan)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateQuotaCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAffectedByQuota(&self, path: &::windows::core::BSTR, options: FsrmEnumOptions) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsAffectedByQuota)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(path), options, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmQuotaManagerEx, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmQuotaManager);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaManagerEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaManagerEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaManagerEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaManagerEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaManagerEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmQuotaManagerEx {
    type Vtable = IFsrmQuotaManagerEx_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaManagerEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4846cb01_d430_494f_abb4_b1054999fb09);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManagerEx_Vtbl {
    pub base__: IFsrmQuotaManager_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAffectedByQuota: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::core::ffi::c_void, options: FsrmEnumOptions, affected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAffectedByQuota: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaObject(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaObject {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QuotaLimit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit(&self, quotalimit: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetQuotaLimit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(quotalimit)).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QuotaFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetQuotaFlags)(::windows::core::Vtable::as_raw(self), quotaflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Thresholds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ModifyThreshold)(::windows::core::Vtable::as_raw(self), threshold, newthreshold).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateThresholdAction)(::windows::core::Vtable::as_raw(self), threshold, actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumThresholdActions)(::windows::core::Vtable::as_raw(self), threshold, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Path)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserSid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserSid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserAccount(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UserAccount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SourceTemplateName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SourceTemplateName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MatchesSourceTemplate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ApplyTemplate(&self, quotatemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ApplyTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(quotatemplatename)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmQuotaObject, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmQuotaObject {
    type Vtable = IFsrmQuotaObject_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42dc3511_61d5_48ae_b6dc_59fc00c0a8d6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaObject_Vtbl {
    pub base__: IFsrmQuotaBase_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usersid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccount: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SourceTemplateName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchesSourceTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matches: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchesSourceTemplate: usize,
    pub ApplyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaTemplate(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplate {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QuotaLimit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit(&self, quotalimit: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetQuotaLimit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(quotalimit)).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QuotaFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetQuotaFlags)(::windows::core::Vtable::as_raw(self), quotaflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Thresholds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ModifyThreshold)(::windows::core::Vtable::as_raw(self), threshold, newthreshold).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateThresholdAction)(::windows::core::Vtable::as_raw(self), threshold, actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumThresholdActions)(::windows::core::Vtable::as_raw(self), threshold, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn CopyTemplate(&self, quotatemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CopyTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(quotatemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CommitAndUpdateDerived)(::windows::core::Vtable::as_raw(self), commitoptions, applyoptions, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmQuotaTemplate, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaTemplate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaTemplate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmQuotaTemplate {
    type Vtable = IFsrmQuotaTemplate_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaTemplate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2efab31_295e_46bb_b976_e86d58b52e8b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplate_Vtbl {
    pub base__: IFsrmQuotaBase_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatename: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CommitAndUpdateDerived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CommitAndUpdateDerived: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaTemplateImported(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplateImported {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QuotaLimit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetQuotaLimit(&self, quotalimit: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetQuotaLimit)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(quotalimit)).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QuotaFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetQuotaFlags)(::windows::core::Vtable::as_raw(self), quotaflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Thresholds)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteThreshold)(::windows::core::Vtable::as_raw(self), threshold).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ModifyThreshold)(::windows::core::Vtable::as_raw(self), threshold, newthreshold).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::core::Result<IFsrmAction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateThresholdAction)(::windows::core::Vtable::as_raw(self), threshold, actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumThresholdActions)(::windows::core::Vtable::as_raw(self), threshold, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn CopyTemplate(&self, quotatemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(quotatemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::core::Result<IFsrmDerivedObjectsResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CommitAndUpdateDerived)(::windows::core::Vtable::as_raw(self), commitoptions, applyoptions, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).OverwriteOnCommit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOverwriteOnCommit<P0>(&self, overwrite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetOverwriteOnCommit)(::windows::core::Vtable::as_raw(self), overwrite.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmQuotaTemplateImported, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmQuotaBase, IFsrmQuotaTemplate);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaTemplateImported {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaTemplateImported {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaTemplateImported {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaTemplateImported {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaTemplateImported").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmQuotaTemplateImported {
    type Vtable = IFsrmQuotaTemplateImported_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaTemplateImported {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2bf113_a329_44cc_809a_5c00fce8da40);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateImported_Vtbl {
    pub base__: IFsrmQuotaTemplate_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OverwriteOnCommit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOverwriteOnCommit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overwrite: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOverwriteOnCommit: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmQuotaTemplateManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmQuotaTemplateManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTemplate(&self) -> ::windows::core::Result<IFsrmQuotaTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateTemplate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetTemplate(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmQuotaTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTemplate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumTemplates)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExportTemplates(&self, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ExportTemplates)(::windows::core::Vtable::as_raw(self), quotatemplatenamesarray, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ImportTemplates(&self, serializedquotatemplates: &::windows::core::BSTR, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<IFsrmCommittableCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ImportTemplates)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(serializedquotatemplates), quotatemplatenamesarray, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmQuotaTemplateManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmQuotaTemplateManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmQuotaTemplateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmQuotaTemplateManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmQuotaTemplateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmQuotaTemplateManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmQuotaTemplateManager {
    type Vtable = IFsrmQuotaTemplateManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmQuotaTemplateManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4173ac41_172d_4d52_963c_fdc7e415f717);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, quotatemplate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetTemplate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, quotatemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, serializedquotatemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExportTemplates: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ImportTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, serializedquotatemplates: *mut ::core::ffi::c_void, quotatemplatenamesarray: *const super::super::System::Com::VARIANT, quotatemplates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ImportTemplates: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmReport(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReport {
    pub unsafe fn Type(&self) -> ::windows::core::Result<FsrmReportType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn LastGeneratedFileNamePrefix(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastGeneratedFileNamePrefix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFilter(&self, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFilter)(::windows::core::Vtable::as_raw(self), filter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetFilter(&self, filter: FsrmReportFilter, filtervalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFilter)(::windows::core::Vtable::as_raw(self), filter, ::core::mem::transmute(filtervalue)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmReport, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmReport {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmReport {
    type Vtable = IFsrmReport_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8cc81d9_46b8_4fa4_bfa5_4aa9dec9b638);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReport_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: *mut FsrmReportType) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastGeneratedFileNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prefix: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filter: FsrmReportFilter, filtervalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetFilter: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmReportJob(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportJob {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Task(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Task)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTask(&self, taskname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTask)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(taskname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NamespaceRoots)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNamespaceRoots)(::windows::core::Vtable::as_raw(self), namespaceroots).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Formats)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFormats)(::windows::core::Vtable::as_raw(self), formats).ok()
    }
    pub unsafe fn MailTo(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailTo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailTo(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailto)).ok()
    }
    pub unsafe fn RunningStatus(&self) -> ::windows::core::Result<FsrmReportRunningStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RunningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastRun(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastRun)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastError(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastError)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LastGeneratedInDirectory(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastGeneratedInDirectory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumReports(&self) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumReports)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReport(&self, reporttype: FsrmReportType) -> ::windows::core::Result<IFsrmReport> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateReport)(::windows::core::Vtable::as_raw(self), reporttype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Run)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).WaitForCompletion)(::windows::core::Vtable::as_raw(self), waitseconds, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmReportJob, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmReportJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmReportJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmReportJob {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmReportJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReportJob").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmReportJob {
    type Vtable = IFsrmReportJob_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmReportJob {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38e87280_715c_4c7d_a280_ea1651a19fef);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportJob_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Formats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Formats: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetFormats: usize,
    pub MailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RunningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::core::HRESULT,
    pub LastRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastrun: *mut f64) -> ::windows::core::HRESULT,
    pub LastError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lasterror: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LastGeneratedInDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumReports: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reports: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumReports: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, report: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReport: usize,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitseconds: i32, completed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForCompletion: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmReportManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumReportJobs(&self, options: FsrmEnumOptions) -> ::windows::core::Result<IFsrmCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnumReportJobs)(::windows::core::Vtable::as_raw(self), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReportJob(&self) -> ::windows::core::Result<IFsrmReportJob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateReportJob)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetReportJob(&self, taskname: &::windows::core::BSTR) -> ::windows::core::Result<IFsrmReportJob> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReportJob)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(taskname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOutputDirectory(&self, context: FsrmReportGenerationContext) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOutputDirectory)(::windows::core::Vtable::as_raw(self), context, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOutputDirectory(&self, context: FsrmReportGenerationContext, path: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOutputDirectory)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(path)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsFilterValidForReportType(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsFilterValidForReportType)(::windows::core::Vtable::as_raw(self), reporttype, filter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDefaultFilter)(::windows::core::Vtable::as_raw(self), reporttype, filter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultFilter)(::windows::core::Vtable::as_raw(self), reporttype, filter, ::core::mem::transmute(filtervalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetReportSizeLimit(&self, limit: FsrmReportLimit) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetReportSizeLimit)(::windows::core::Vtable::as_raw(self), limit, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetReportSizeLimit(&self, limit: FsrmReportLimit, limitvalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetReportSizeLimit)(::windows::core::Vtable::as_raw(self), limit, ::core::mem::transmute(limitvalue)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmReportManager, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmReportManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmReportManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmReportManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmReportManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReportManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmReportManager {
    type Vtable = IFsrmReportManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmReportManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27b899fe_6ffa_4481_a184_d3daade8a02b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportManager_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumReportJobs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FsrmEnumOptions, reportjobs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumReportJobs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReportJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReportJob: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetReportJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut ::core::ffi::c_void, reportjob: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetReportJob: usize,
    pub GetOutputDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOutputDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: FsrmReportGenerationContext, path: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsFilterValidForReportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsFilterValidForReportType: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetDefaultFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetDefaultFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetDefaultFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetDefaultFilter: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetReportSizeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetReportSizeLimit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetReportSizeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, limit: FsrmReportLimit, limitvalue: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetReportSizeLimit: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmReportScheduler(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmReportScheduler {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VerifyNamespaces(&self, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).VerifyNamespaces)(::windows::core::Vtable::as_raw(self), namespacessafearray).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateScheduleTask(&self, taskname: &::windows::core::BSTR, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).CreateScheduleTask)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(taskname), namespacessafearray, ::core::mem::transmute_copy(serializedtask)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ModifyScheduleTask(&self, taskname: &::windows::core::BSTR, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ModifyScheduleTask)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(taskname), namespacessafearray, ::core::mem::transmute_copy(serializedtask)).ok()
    }
    pub unsafe fn DeleteScheduleTask(&self, taskname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DeleteScheduleTask)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(taskname)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmReportScheduler, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmReportScheduler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmReportScheduler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmReportScheduler {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmReportScheduler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmReportScheduler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmReportScheduler {
    type Vtable = IFsrmReportScheduler_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmReportScheduler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6879caf9_6617_4484_8719_71c3d8645f94);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportScheduler_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub VerifyNamespaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    VerifyNamespaces: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateScheduleTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut ::core::ffi::c_void, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateScheduleTask: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ModifyScheduleTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut ::core::ffi::c_void, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ModifyScheduleTask: usize,
    pub DeleteScheduleTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmRule(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmRule {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn RuleType(&self) -> ::windows::core::Result<FsrmRuleType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RuleType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ModuleDefinitionName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetModuleDefinitionName(&self, moduledefinitionname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetModuleDefinitionName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(moduledefinitionname)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).NamespaceRoots)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetNamespaceRoots)(::windows::core::Vtable::as_raw(self), namespaceroots).ok()
    }
    pub unsafe fn RuleFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).RuleFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRuleFlags)(::windows::core::Vtable::as_raw(self), ruleflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Parameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetParameters)(::windows::core::Vtable::as_raw(self), parameters).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LastModified(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).LastModified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmRule, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmRule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmRule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmRule {
    type Vtable = IFsrmRule_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmRule {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb0df960_16f5_4495_9079_3f9360d831df);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmRule_Vtbl {
    pub base__: IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RuleType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruletype: *mut FsrmRuleType) -> ::windows::core::HRESULT,
    pub ModuleDefinitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetModuleDefinitionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, moduledefinitionname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub NamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NamespaceRoots: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetNamespaceRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetNamespaceRoots: usize,
    pub RuleFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: *mut i32) -> ::windows::core::HRESULT,
    pub SetRuleFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleflags: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parameters: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetParameters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LastModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodified: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LastModified: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmSetting(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmSetting {
    pub unsafe fn SmtpServer(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).SmtpServer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSmtpServer(&self, smtpserver: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSmtpServer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(smtpserver)).ok()
    }
    pub unsafe fn MailFrom(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MailFrom)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMailFrom(&self, mailfrom: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMailFrom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailfrom)).ok()
    }
    pub unsafe fn AdminEmail(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AdminEmail)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAdminEmail(&self, adminemail: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAdminEmail)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(adminemail)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisableCommandLine(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).DisableCommandLine)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisableCommandLine<P0>(&self, disablecommandline: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetDisableCommandLine)(::windows::core::Vtable::as_raw(self), disablecommandline.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableScreeningAudit(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).EnableScreeningAudit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableScreeningAudit<P0>(&self, enablescreeningaudit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetEnableScreeningAudit)(::windows::core::Vtable::as_raw(self), enablescreeningaudit.into()).ok()
    }
    pub unsafe fn EmailTest(&self, mailto: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).EmailTest)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mailto)).ok()
    }
    pub unsafe fn SetActionRunLimitInterval(&self, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetActionRunLimitInterval)(::windows::core::Vtable::as_raw(self), actiontype, delaytimeminutes).ok()
    }
    pub unsafe fn GetActionRunLimitInterval(&self, actiontype: FsrmActionType) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetActionRunLimitInterval)(::windows::core::Vtable::as_raw(self), actiontype, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmSetting, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmSetting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmSetting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmSetting {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmSetting").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmSetting {
    type Vtable = IFsrmSetting_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmSetting {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf411d4fd_14be_4260_8c40_03b7c95e608a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmSetting_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SmtpServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smtpserver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSmtpServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, smtpserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMailFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailfrom: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AdminEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adminemail: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAdminEmail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adminemail: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DisableCommandLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disablecommandline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisableCommandLine: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisableCommandLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disablecommandline: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisableCommandLine: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableScreeningAudit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablescreeningaudit: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableScreeningAudit: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnableScreeningAudit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enablescreeningaudit: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnableScreeningAudit: usize,
    pub EmailTest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mailto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetActionRunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::core::HRESULT,
    pub GetActionRunLimitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmStorageModuleDefinition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmStorageModuleDefinition {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Id)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, description: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(description)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ModuleClsid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ModuleClsid)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetModuleClsid(&self, moduleclsid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetModuleClsid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(moduleclsid)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn Company(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Company)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCompany(&self, company: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCompany)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(company)).ok()
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Version)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVersion(&self, version: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVersion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(version)).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows::core::Result<FsrmPipelineModuleType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ModuleType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Enabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, enabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnabled)(::windows::core::Vtable::as_raw(self), enabled.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NeedsFileContent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NeedsFileContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedsFileContent<P0>(&self, needsfilecontent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNeedsFileContent)(::windows::core::Vtable::as_raw(self), needsfilecontent.into()).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::core::Result<FsrmAccountType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Account)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAccount)(::windows::core::Vtable::as_raw(self), retrievalaccount).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SupportedExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSupportedExtensions)(::windows::core::Vtable::as_raw(self), supportedextensions).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Parameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParameters)(::windows::core::Vtable::as_raw(self), parameters).ok()
    }
    pub unsafe fn Capabilities(&self) -> ::windows::core::Result<FsrmStorageModuleCaps> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Capabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCapabilities(&self, capabilities: FsrmStorageModuleCaps) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCapabilities)(::windows::core::Vtable::as_raw(self), capabilities).ok()
    }
    pub unsafe fn StorageType(&self) -> ::windows::core::Result<FsrmStorageModuleType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).StorageType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStorageType(&self, storagetype: FsrmStorageModuleType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStorageType)(::windows::core::Vtable::as_raw(self), storagetype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdatesFileContent(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).UpdatesFileContent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUpdatesFileContent<P0>(&self, updatesfilecontent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetUpdatesFileContent)(::windows::core::Vtable::as_raw(self), updatesfilecontent.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmStorageModuleDefinition, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmObject, IFsrmPipelineModuleDefinition);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmStorageModuleDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmStorageModuleDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmStorageModuleDefinition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmStorageModuleDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmStorageModuleDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmStorageModuleDefinition {
    type Vtable = IFsrmStorageModuleDefinition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmStorageModuleDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15a81350_497d_4aba_80e9_d4dbcc5521fe);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: *mut FsrmStorageModuleCaps) -> ::windows::core::HRESULT,
    pub SetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: FsrmStorageModuleCaps) -> ::windows::core::HRESULT,
    pub StorageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagetype: *mut FsrmStorageModuleType) -> ::windows::core::HRESULT,
    pub SetStorageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagetype: FsrmStorageModuleType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdatesFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatesfilecontent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdatesFileContent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUpdatesFileContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatesfilecontent: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUpdatesFileContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IFsrmStorageModuleImplementation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IFsrmStorageModuleImplementation {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnLoad<P0>(&self, moduledefinition: P0) -> ::windows::core::Result<IFsrmPipelineModuleConnector>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmPipelineModuleDefinition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OnLoad)(::windows::core::Vtable::as_raw(self), moduledefinition.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnUnload)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UseDefinitions<P0>(&self, propertydefinitions: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmCollection>>,
    {
        (::windows::core::Vtable::vtable(self).UseDefinitions)(::windows::core::Vtable::as_raw(self), propertydefinitions.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadProperties<P0>(&self, propertybag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmPropertyBag>>,
    {
        (::windows::core::Vtable::vtable(self).LoadProperties)(::windows::core::Vtable::as_raw(self), propertybag.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveProperties<P0>(&self, propertybag: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IFsrmPropertyBag>>,
    {
        (::windows::core::Vtable::vtable(self).SaveProperties)(::windows::core::Vtable::as_raw(self), propertybag.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::core::interface_hierarchy!(IFsrmStorageModuleImplementation, ::windows::core::IUnknown, super::super::System::Com::IDispatch, IFsrmPipelineModuleImplementation);
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IFsrmStorageModuleImplementation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IFsrmStorageModuleImplementation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IFsrmStorageModuleImplementation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IFsrmStorageModuleImplementation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFsrmStorageModuleImplementation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Vtable for IFsrmStorageModuleImplementation {
    type Vtable = IFsrmStorageModuleImplementation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IFsrmStorageModuleImplementation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0af4a0da_895a_4e50_8712_a96724bcec64);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub UseDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertydefinitions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UseDefinitions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertybag: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveProperties: usize,
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdSyncTask: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ae64751_b728_4d6b_97a0_b2da2e7d2a3b);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_MAX_EMAILS_SENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200130i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200110i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_PATH_IS_LOCAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200111i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200112i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200253i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_AUTO_QUOTA: ::windows::core::HRESULT = ::windows::core::HRESULT(283419i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CACHE_INVALID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200187i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200186i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_AGGREGATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200201i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200170i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200197i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200132i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200135i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_REMOVE_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200109i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_RENAME_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200198i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_STORE_PROPERTIES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200171i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200143i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200145i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200195i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_CANCELED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200141i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200194i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200136i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200148i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLASSIFICATION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200137i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CLUSTER_NOT_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200210i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200106i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200207i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_DRIVER_NOT_READY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200237i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_DUPLICATE_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200240i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EMAIL_NOT_SENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200228i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ENUM_PROPERTIES_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200173i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_ERROR_NOT_ENABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200133i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200105i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200104i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200103i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FAIL_BATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200247i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_ENCRYPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200156i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200134i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200152i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200153i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200185i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200184i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200193i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200191i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200102i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200192i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200108i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200146i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200190i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200147i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200120i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_OPEN_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200189i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_FILE_SYSTEM_CORRUPT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200225i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INCOMPATIBLE_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200157i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INPROC_MODULE_BLOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200174i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INSECURE_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200233i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INSUFFICIENT_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200236i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_AD_CLAIM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200142i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_COMBINATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200241i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200220i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_EMAIL_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200226i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200223i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_FILENAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200214i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200140i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_IMPORT_VERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200245i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200249i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200248i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_PATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200250i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_REPORT_DESC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200215i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_REPORT_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200216i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200254i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_SMTP_SERVER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200232i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_TEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200246i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_INVALID_USER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200251i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200176i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LEGACY_SCHEDULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200107i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LOADING_DISABLED_MODULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200202i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_LONG_CMDLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200224i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200196i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200200i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_INITIALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200150i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_INVALID_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200151i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200149i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_MODULE_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200101i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NOT_CLUSTER_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200208i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200255i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200239i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NO_EMAIL_ADDRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200131i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_NO_PROPERTY_VALUE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200175i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_OBJECT_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200199i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200243i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200169i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PATH_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200252i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200129i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200155i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200166i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_DELETED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200183i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200138i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200124i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200122i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200123i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200139i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_GENERATION_ERR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200204i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200205i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_TASK_TRIGGER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200203i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200206i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_REQD_PARAM_MISSING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200242i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200126i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200125i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200128i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200127i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SET_PROPERTY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200172i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SHADOW_COPY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200212i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_STORE_NOT_INSTALLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200209i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200119i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_SYNC_TASK_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200144i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200158i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200160i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200167i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200168i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_TEXTREADER_STREAM_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200159i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_UNEXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200234i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200188i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_VOLUME_OFFLINE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200154i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_VOLUME_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200235i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_WMI_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200121i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_E_XML_CORRUPTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147200211i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: ::windows::core::HRESULT = ::windows::core::HRESULT(283398i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_S_PARTIAL_BATCH: ::windows::core::HRESULT = ::windows::core::HRESULT(283396i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FSRM_S_PARTIAL_CLASSIFICATION: ::windows::core::HRESULT = ::windows::core::HRESULT(283397i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccessDeniedRemediationClient: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x100b4fc8_74c1_470f_b1b7_dd7b6bae79bd);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb15c0e47_c391_45b9_95c8_eb596c853f3a);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmDaysNotSpecified: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExportImport: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1482dc37_fae9_4787_9025_8ce4e024ab56);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileGroupManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1363f6_656f_4496_9226_13aecbd7718f);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementJobManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb18f9b2_4c3a_4321_b203_205120cff614);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileScreenManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95941183_db53_4c5f_b37b_7d0921cf9dc7);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileScreenTemplateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x243111df_e474_46aa_a054_eaa33edc292a);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxExcludeFolders: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxNumberThresholds: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMaxThresholdValue: u32 = 250u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMinQuotaLimit: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmMinThresholdValue: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPathMapper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3be42bd_8ac2_409e_bbd8_faf9b6b41feb);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleConnector: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7643375_1eb5_44de_a062_623547d933bc);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90dcab7f_347c_4bfc_b543_540326305fbe);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaTemplateManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97d3d443_251c_4337_81e7_b32e8f4ee65e);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0058ef37_aa66_4c48_bd5b_2fce432ab0c8);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportScheduler: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea25f1b8_1b8d_4290_8ee8_e17c12c2fe20);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmSetting: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf556d708_6d4d_4594_9c61_7dbb0dae2a46);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const MessageSizeLimit: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AdrClientDisplayFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = AdrClientDisplayFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = AdrClientDisplayFlags(2i32);
impl ::core::marker::Copy for AdrClientDisplayFlags {}
impl ::core::clone::Clone for AdrClientDisplayFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrClientDisplayFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdrClientDisplayFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrClientDisplayFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrClientDisplayFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AdrClientErrorType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientErrorType_Unknown: AdrClientErrorType = AdrClientErrorType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = AdrClientErrorType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = AdrClientErrorType(2i32);
impl ::core::marker::Copy for AdrClientErrorType {}
impl ::core::clone::Clone for AdrClientErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrClientErrorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdrClientErrorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrClientErrorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrClientErrorType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AdrClientFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_None: AdrClientFlags = AdrClientFlags(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = AdrClientFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = AdrClientFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = AdrClientFlags(4i32);
impl ::core::marker::Copy for AdrClientFlags {}
impl ::core::clone::Clone for AdrClientFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrClientFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdrClientFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrClientFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrClientFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AdrEmailFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = AdrEmailFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = AdrEmailFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = AdrEmailFlags(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = AdrEmailFlags(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = AdrEmailFlags(16i32);
impl ::core::marker::Copy for AdrEmailFlags {}
impl ::core::clone::Clone for AdrEmailFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AdrEmailFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AdrEmailFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for AdrEmailFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AdrEmailFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmAccountType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_Unknown: FsrmAccountType = FsrmAccountType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_NetworkService: FsrmAccountType = FsrmAccountType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_LocalService: FsrmAccountType = FsrmAccountType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_LocalSystem: FsrmAccountType = FsrmAccountType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_InProc: FsrmAccountType = FsrmAccountType(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_External: FsrmAccountType = FsrmAccountType(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmAccountType_Automatic: FsrmAccountType = FsrmAccountType(500i32);
impl ::core::marker::Copy for FsrmAccountType {}
impl ::core::clone::Clone for FsrmAccountType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmAccountType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmAccountType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmAccountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmAccountType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmActionType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Unknown: FsrmActionType = FsrmActionType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_EventLog: FsrmActionType = FsrmActionType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Email: FsrmActionType = FsrmActionType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Command: FsrmActionType = FsrmActionType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmActionType_Report: FsrmActionType = FsrmActionType(4i32);
impl ::core::marker::Copy for FsrmActionType {}
impl ::core::clone::Clone for FsrmActionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmActionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmActionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmActionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmClassificationLoggingFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(8i32);
impl ::core::marker::Copy for FsrmClassificationLoggingFlags {}
impl ::core::clone::Clone for FsrmClassificationLoggingFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmClassificationLoggingFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmClassificationLoggingFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmClassificationLoggingFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmClassificationLoggingFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmCollectionState(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Fetching: FsrmCollectionState = FsrmCollectionState(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Committing: FsrmCollectionState = FsrmCollectionState(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Complete: FsrmCollectionState = FsrmCollectionState(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = FsrmCollectionState(4i32);
impl ::core::marker::Copy for FsrmCollectionState {}
impl ::core::clone::Clone for FsrmCollectionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmCollectionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmCollectionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmCollectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmCollectionState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmCommitOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCommitOptions_None: FsrmCommitOptions = FsrmCommitOptions(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = FsrmCommitOptions(1i32);
impl ::core::marker::Copy for FsrmCommitOptions {}
impl ::core::clone::Clone for FsrmCommitOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmCommitOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmCommitOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmCommitOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmCommitOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmEnumOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_None: FsrmEnumOptions = FsrmEnumOptions(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = FsrmEnumOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = FsrmEnumOptions(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = FsrmEnumOptions(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = FsrmEnumOptions(8i32);
impl ::core::marker::Copy for FsrmEnumOptions {}
impl ::core::clone::Clone for FsrmEnumOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmEnumOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmEnumOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmEnumOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmEnumOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmEventType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Unknown: FsrmEventType = FsrmEventType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Information: FsrmEventType = FsrmEventType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Warning: FsrmEventType = FsrmEventType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmEventType_Error: FsrmEventType = FsrmEventType(3i32);
impl ::core::marker::Copy for FsrmEventType {}
impl ::core::clone::Clone for FsrmEventType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmEventType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmEventType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmEventType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmExecutionOption(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = FsrmExecutionOption(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = FsrmExecutionOption(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = FsrmExecutionOption(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = FsrmExecutionOption(3i32);
impl ::core::marker::Copy for FsrmExecutionOption {}
impl ::core::clone::Clone for FsrmExecutionOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmExecutionOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmExecutionOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmExecutionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmExecutionOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmFileConditionType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = FsrmFileConditionType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileConditionType_Property: FsrmFileConditionType = FsrmFileConditionType(1i32);
impl ::core::marker::Copy for FsrmFileConditionType {}
impl ::core::clone::Clone for FsrmFileConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileConditionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileConditionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileConditionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileConditionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmFileManagementLoggingFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(4i32);
impl ::core::marker::Copy for FsrmFileManagementLoggingFlags {}
impl ::core::clone::Clone for FsrmFileManagementLoggingFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileManagementLoggingFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileManagementLoggingFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileManagementLoggingFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileManagementLoggingFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmFileManagementType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = FsrmFileManagementType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = FsrmFileManagementType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = FsrmFileManagementType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = FsrmFileManagementType(3i32);
impl ::core::marker::Copy for FsrmFileManagementType {}
impl ::core::clone::Clone for FsrmFileManagementType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileManagementType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileManagementType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileManagementType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileManagementType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmFileScreenFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = FsrmFileScreenFlags(1i32);
impl ::core::marker::Copy for FsrmFileScreenFlags {}
impl ::core::clone::Clone for FsrmFileScreenFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileScreenFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileScreenFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileScreenFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileScreenFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmFileStreamingInterfaceType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(2i32);
impl ::core::marker::Copy for FsrmFileStreamingInterfaceType {}
impl ::core::clone::Clone for FsrmFileStreamingInterfaceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileStreamingInterfaceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileStreamingInterfaceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileStreamingInterfaceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileStreamingInterfaceType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmFileStreamingMode(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = FsrmFileStreamingMode(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = FsrmFileStreamingMode(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = FsrmFileStreamingMode(2i32);
impl ::core::marker::Copy for FsrmFileStreamingMode {}
impl ::core::clone::Clone for FsrmFileStreamingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileStreamingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileStreamingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileStreamingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileStreamingMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmFileSystemPropertyId(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(5i32);
impl ::core::marker::Copy for FsrmFileSystemPropertyId {}
impl ::core::clone::Clone for FsrmFileSystemPropertyId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmFileSystemPropertyId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmFileSystemPropertyId {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmFileSystemPropertyId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmFileSystemPropertyId").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmGetFilePropertyOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(8i32);
impl ::core::marker::Copy for FsrmGetFilePropertyOptions {}
impl ::core::clone::Clone for FsrmGetFilePropertyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmGetFilePropertyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmGetFilePropertyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmGetFilePropertyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmGetFilePropertyOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmPipelineModuleType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = FsrmPipelineModuleType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = FsrmPipelineModuleType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = FsrmPipelineModuleType(2i32);
impl ::core::marker::Copy for FsrmPipelineModuleType {}
impl ::core::clone::Clone for FsrmPipelineModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPipelineModuleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPipelineModuleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPipelineModuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPipelineModuleType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmPropertyBagField(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = FsrmPropertyBagField(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = FsrmPropertyBagField(1i32);
impl ::core::marker::Copy for FsrmPropertyBagField {}
impl ::core::clone::Clone for FsrmPropertyBagField {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyBagField {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyBagField {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyBagField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyBagField").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmPropertyBagFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = FsrmPropertyBagFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(8i32);
impl ::core::marker::Copy for FsrmPropertyBagFlags {}
impl ::core::clone::Clone for FsrmPropertyBagFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyBagFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyBagFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyBagFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyBagFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmPropertyConditionType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = FsrmPropertyConditionType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = FsrmPropertyConditionType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = FsrmPropertyConditionType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = FsrmPropertyConditionType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = FsrmPropertyConditionType(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = FsrmPropertyConditionType(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = FsrmPropertyConditionType(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = FsrmPropertyConditionType(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = FsrmPropertyConditionType(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = FsrmPropertyConditionType(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = FsrmPropertyConditionType(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(12i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = FsrmPropertyConditionType(13i32);
impl ::core::marker::Copy for FsrmPropertyConditionType {}
impl ::core::clone::Clone for FsrmPropertyConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyConditionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyConditionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyConditionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyConditionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmPropertyDefinitionAppliesTo(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(2i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionAppliesTo {}
impl ::core::clone::Clone for FsrmPropertyDefinitionAppliesTo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyDefinitionAppliesTo {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyDefinitionAppliesTo {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyDefinitionAppliesTo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionAppliesTo").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmPropertyDefinitionFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(4i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionFlags {}
impl ::core::clone::Clone for FsrmPropertyDefinitionFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyDefinitionFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyDefinitionFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyDefinitionFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmPropertyDefinitionType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(8i32);
impl ::core::marker::Copy for FsrmPropertyDefinitionType {}
impl ::core::clone::Clone for FsrmPropertyDefinitionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyDefinitionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyDefinitionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyDefinitionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyDefinitionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmPropertyFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = FsrmPropertyFlags(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = FsrmPropertyFlags(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = FsrmPropertyFlags(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = FsrmPropertyFlags(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = FsrmPropertyFlags(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = FsrmPropertyFlags(16i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = FsrmPropertyFlags(32i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = FsrmPropertyFlags(64i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = FsrmPropertyFlags(128i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = FsrmPropertyFlags(256i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = FsrmPropertyFlags(512i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = FsrmPropertyFlags(1024i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = FsrmPropertyFlags(2048i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = FsrmPropertyFlags(4096i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = FsrmPropertyFlags(8192i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = FsrmPropertyFlags(16384i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = FsrmPropertyFlags(32768i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = FsrmPropertyFlags(65536i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = FsrmPropertyFlags(14i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = FsrmPropertyFlags(20480i32);
impl ::core::marker::Copy for FsrmPropertyFlags {}
impl ::core::clone::Clone for FsrmPropertyFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmPropertyValueType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = FsrmPropertyValueType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = FsrmPropertyValueType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = FsrmPropertyValueType(2i32);
impl ::core::marker::Copy for FsrmPropertyValueType {}
impl ::core::clone::Clone for FsrmPropertyValueType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmPropertyValueType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmPropertyValueType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmPropertyValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmPropertyValueType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmQuotaFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = FsrmQuotaFlags(256i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = FsrmQuotaFlags(512i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = FsrmQuotaFlags(65536i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = FsrmQuotaFlags(131072i32);
impl ::core::marker::Copy for FsrmQuotaFlags {}
impl ::core::clone::Clone for FsrmQuotaFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmQuotaFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmQuotaFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmQuotaFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmQuotaFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmReportFilter(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MinSize: FsrmReportFilter = FsrmReportFilter(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = FsrmReportFilter(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = FsrmReportFilter(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = FsrmReportFilter(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = FsrmReportFilter(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_Owners: FsrmReportFilter = FsrmReportFilter(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = FsrmReportFilter(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFilter_Property: FsrmReportFilter = FsrmReportFilter(8i32);
impl ::core::marker::Copy for FsrmReportFilter {}
impl ::core::clone::Clone for FsrmReportFilter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportFilter {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportFilter {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportFilter").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmReportFormat(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Unknown: FsrmReportFormat = FsrmReportFormat(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_DHtml: FsrmReportFormat = FsrmReportFormat(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Html: FsrmReportFormat = FsrmReportFormat(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Txt: FsrmReportFormat = FsrmReportFormat(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Csv: FsrmReportFormat = FsrmReportFormat(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportFormat_Xml: FsrmReportFormat = FsrmReportFormat(5i32);
impl ::core::marker::Copy for FsrmReportFormat {}
impl ::core::clone::Clone for FsrmReportFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmReportGenerationContext(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = FsrmReportGenerationContext(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = FsrmReportGenerationContext(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = FsrmReportGenerationContext(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = FsrmReportGenerationContext(4i32);
impl ::core::marker::Copy for FsrmReportGenerationContext {}
impl ::core::clone::Clone for FsrmReportGenerationContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportGenerationContext {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportGenerationContext {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportGenerationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportGenerationContext").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmReportLimit(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = FsrmReportLimit(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = FsrmReportLimit(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = FsrmReportLimit(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = FsrmReportLimit(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = FsrmReportLimit(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = FsrmReportLimit(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = FsrmReportLimit(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = FsrmReportLimit(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = FsrmReportLimit(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = FsrmReportLimit(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = FsrmReportLimit(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = FsrmReportLimit(12i32);
impl ::core::marker::Copy for FsrmReportLimit {}
impl ::core::clone::Clone for FsrmReportLimit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportLimit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportLimit {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportLimit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportLimit").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmReportRunningStatus(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = FsrmReportRunningStatus(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = FsrmReportRunningStatus(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = FsrmReportRunningStatus(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = FsrmReportRunningStatus(3i32);
impl ::core::marker::Copy for FsrmReportRunningStatus {}
impl ::core::clone::Clone for FsrmReportRunningStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportRunningStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportRunningStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportRunningStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportRunningStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmReportType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_Unknown: FsrmReportType = FsrmReportType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_LargeFiles: FsrmReportType = FsrmReportType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FilesByType: FsrmReportType = FsrmReportType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = FsrmReportType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = FsrmReportType(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_QuotaUsage: FsrmReportType = FsrmReportType(5i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FilesByOwner: FsrmReportType = FsrmReportType(6i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_ExportReport: FsrmReportType = FsrmReportType(7i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_DuplicateFiles: FsrmReportType = FsrmReportType(8i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FileScreenAudit: FsrmReportType = FsrmReportType(9i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FilesByProperty: FsrmReportType = FsrmReportType(10i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_AutomaticClassification: FsrmReportType = FsrmReportType(11i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_Expiration: FsrmReportType = FsrmReportType(12i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmReportType_FoldersByProperty: FsrmReportType = FsrmReportType(13i32);
impl ::core::marker::Copy for FsrmReportType {}
impl ::core::clone::Clone for FsrmReportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmReportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmReportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmReportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmReportType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmRuleFlags(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = FsrmRuleFlags(256i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(1024i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(2048i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = FsrmRuleFlags(4096i32);
impl ::core::marker::Copy for FsrmRuleFlags {}
impl ::core::clone::Clone for FsrmRuleFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmRuleFlags {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmRuleFlags {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmRuleFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmRuleFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmRuleType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleType_Unknown: FsrmRuleType = FsrmRuleType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleType_Classification: FsrmRuleType = FsrmRuleType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmRuleType_Generic: FsrmRuleType = FsrmRuleType(2i32);
impl ::core::marker::Copy for FsrmRuleType {}
impl ::core::clone::Clone for FsrmRuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmRuleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmRuleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmRuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmRuleType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmStorageModuleCaps(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = FsrmStorageModuleCaps(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = FsrmStorageModuleCaps(4i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = FsrmStorageModuleCaps(8i32);
impl ::core::marker::Copy for FsrmStorageModuleCaps {}
impl ::core::clone::Clone for FsrmStorageModuleCaps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmStorageModuleCaps {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmStorageModuleCaps {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmStorageModuleCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmStorageModuleCaps").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmStorageModuleType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = FsrmStorageModuleType(0i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = FsrmStorageModuleType(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = FsrmStorageModuleType(2i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = FsrmStorageModuleType(3i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = FsrmStorageModuleType(100i32);
impl ::core::marker::Copy for FsrmStorageModuleType {}
impl ::core::clone::Clone for FsrmStorageModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmStorageModuleType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmStorageModuleType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmStorageModuleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmStorageModuleType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FsrmTemplateApplyOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_FileServerResourceManager\"`*"]
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(2i32);
impl ::core::marker::Copy for FsrmTemplateApplyOptions {}
impl ::core::clone::Clone for FsrmTemplateApplyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FsrmTemplateApplyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FsrmTemplateApplyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for FsrmTemplateApplyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FsrmTemplateApplyOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
