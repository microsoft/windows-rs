#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn GetTnefStreamCodepage(lpstream: super::super::System::Com::IStream, lpulcodepage: *mut u32, lpulsubcodepage: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub fn OpenTnefStream(lpvsupport: *mut ::core::ffi::c_void, lpstream: super::super::System::Com::IStream, lpszstreamname: *const i8, ulflags: u32, lpmessage: super::super::System::AddressBook::IMessage, wkeyval: u16, lpptnef: *mut ITnef) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub fn OpenTnefStreamEx(lpvsupport: *mut ::core::ffi::c_void, lpstream: super::super::System::Com::IStream, lpszstreamname: *const i8, ulflags: u32, lpmessage: super::super::System::AddressBook::IMessage, wkeyval: u16, lpadressbook: super::super::System::AddressBook::IAddrBook, lpptnef: *mut ITnef) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAccept(hcall: u32, lpsuseruserinfo: super::super::Foundation::PSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAddProvider(lpszproviderfilename: super::super::Foundation::PSTR, hwndowner: super::super::Foundation::HWND, lpdwpermanentproviderid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAddProviderA(lpszproviderfilename: super::super::Foundation::PSTR, hwndowner: super::super::Foundation::HWND, lpdwpermanentproviderid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAddProviderW(lpszproviderfilename: super::super::Foundation::PWSTR, hwndowner: super::super::Foundation::HWND, lpdwpermanentproviderid: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineAddToConference(hconfcall: u32, hconsultcall: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineAgentSpecific(hline: u32, dwaddressid: u32, dwagentextensionidindex: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAnswer(hcall: u32, lpsuseruserinfo: super::super::Foundation::PSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineBlindTransfer(hcall: u32, lpszdestaddress: super::super::Foundation::PSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineBlindTransferA(hcall: u32, lpszdestaddress: super::super::Foundation::PSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineBlindTransferW(hcall: u32, lpszdestaddressw: super::super::Foundation::PWSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineClose(hline: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineCompleteCall(hcall: u32, lpdwcompletionid: *mut u32, dwcompletionmode: u32, dwmessageid: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineCompleteTransfer(hcall: u32, hconsultcall: u32, lphconfcall: *mut u32, dwtransfermode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialog(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogA(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogEdit(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: super::super::Foundation::PSTR, lpdeviceconfigin: *const ::core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogEditA(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: super::super::Foundation::PSTR, lpdeviceconfigin: *const ::core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogEditW(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: super::super::Foundation::PWSTR, lpdeviceconfigin: *const ::core::ffi::c_void, dwsize: u32, lpdeviceconfigout: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogW(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigProvider(hwndowner: super::super::Foundation::HWND, dwpermanentproviderid: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineCreateAgentA(hline: u32, lpszagentid: super::super::Foundation::PSTR, lpszagentpin: super::super::Foundation::PSTR, lphagent: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineCreateAgentSessionA(hline: u32, hagent: u32, lpszagentpin: super::super::Foundation::PSTR, dwworkingaddressid: u32, lpgroupid: *mut ::windows_sys::core::GUID, lphagentsession: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineCreateAgentSessionW(hline: u32, hagent: u32, lpszagentpin: super::super::Foundation::PWSTR, dwworkingaddressid: u32, lpgroupid: *mut ::windows_sys::core::GUID, lphagentsession: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineCreateAgentW(hline: u32, lpszagentid: super::super::Foundation::PWSTR, lpszagentpin: super::super::Foundation::PWSTR, lphagent: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineDeallocateCall(hcall: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineDevSpecific(hline: u32, dwaddressid: u32, hcall: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineDevSpecificFeature(hline: u32, dwfeature: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineDial(hcall: u32, lpszdestaddress: super::super::Foundation::PSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineDialA(hcall: u32, lpszdestaddress: super::super::Foundation::PSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineDialW(hcall: u32, lpszdestaddress: super::super::Foundation::PWSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineDrop(hcall: u32, lpsuseruserinfo: super::super::Foundation::PSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineForward(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineForwardA(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineForwardW(hline: u32, balladdresses: u32, dwaddressid: u32, lpforwardlist: *const LINEFORWARDLIST, dwnumringsnoanswer: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGatherDigits(hcall: u32, dwdigitmodes: u32, lpsdigits: super::super::Foundation::PSTR, dwnumdigits: u32, lpszterminationdigits: super::super::Foundation::PSTR, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGatherDigitsA(hcall: u32, dwdigitmodes: u32, lpsdigits: super::super::Foundation::PSTR, dwnumdigits: u32, lpszterminationdigits: super::super::Foundation::PSTR, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGatherDigitsW(hcall: u32, dwdigitmodes: u32, lpsdigits: super::super::Foundation::PWSTR, dwnumdigits: u32, lpszterminationdigits: super::super::Foundation::PWSTR, dwfirstdigittimeout: u32, dwinterdigittimeout: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGenerateDigits(hcall: u32, dwdigitmode: u32, lpszdigits: super::super::Foundation::PSTR, dwduration: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGenerateDigitsA(hcall: u32, dwdigitmode: u32, lpszdigits: super::super::Foundation::PSTR, dwduration: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGenerateDigitsW(hcall: u32, dwdigitmode: u32, lpszdigits: super::super::Foundation::PWSTR, dwduration: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGenerateTone(hcall: u32, dwtonemode: u32, dwduration: u32, dwnumtones: u32, lptones: *const LINEGENERATETONE) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressCaps(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressCapsA(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressCapsW(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwapiversion: u32, dwextversion: u32, lpaddresscaps: *mut LINEADDRESSCAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAddressID(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: super::super::Foundation::PSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAddressIDA(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: super::super::Foundation::PSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAddressIDW(hline: u32, lpdwaddressid: *mut u32, dwaddressmode: u32, lpsaddress: super::super::Foundation::PWSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressStatus(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressStatusA(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressStatusW(hline: u32, dwaddressid: u32, lpaddressstatus: *mut LINEADDRESSSTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentActivityListA(hline: u32, dwaddressid: u32, lpagentactivitylist: *mut LINEAGENTACTIVITYLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentActivityListW(hline: u32, dwaddressid: u32, lpagentactivitylist: *mut LINEAGENTACTIVITYLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentCapsA(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwappapiversion: u32, lpagentcaps: *mut LINEAGENTCAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentCapsW(hlineapp: u32, dwdeviceid: u32, dwaddressid: u32, dwappapiversion: u32, lpagentcaps: *mut LINEAGENTCAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentGroupListA(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentGroupListW(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn lineGetAgentInfo(hline: u32, hagent: u32, lpagentinfo: *mut LINEAGENTINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn lineGetAgentSessionInfo(hline: u32, hagentsession: u32, lpagentsessioninfo: *mut LINEAGENTSESSIONINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentSessionList(hline: u32, hagent: u32, lpagentsessionlist: *mut LINEAGENTSESSIONLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentStatusA(hline: u32, dwaddressid: u32, lpagentstatus: *mut LINEAGENTSTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentStatusW(hline: u32, dwaddressid: u32, lpagentstatus: *mut LINEAGENTSTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAppPriority(lpszappfilename: super::super::Foundation::PSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAppPriorityA(lpszappfilename: super::super::Foundation::PSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAppPriorityW(lpszappfilename: super::super::Foundation::PWSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpextensionname: *mut VARSTRING, lpdwpriority: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCallInfo(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCallInfoA(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCallInfoW(hcall: u32, lpcallinfo: *mut LINECALLINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetCallStatus(hcall: u32, lpcallstatus: *mut LINECALLSTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetConfRelatedCalls(hcall: u32, lpcalllist: *mut LINECALLLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCountry(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCountryA(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCountryW(dwcountryid: u32, dwapiversion: u32, lplinecountrylist: *mut LINECOUNTRYLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetDevCaps(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetDevCapsA(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetDevCapsW(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lplinedevcaps: *mut LINEDEVCAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetDevConfig(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetDevConfigA(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetDevConfigW(dwdeviceid: u32, lpdeviceconfig: *mut VARSTRING, lpszdeviceclass: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetGroupListA(hline: u32, lpgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetGroupListW(hline: u32, lpgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetID(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIDA(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIDW(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIcon(dwdeviceid: u32, lpszdeviceclass: super::super::Foundation::PSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIconA(dwdeviceid: u32, lpszdeviceclass: super::super::Foundation::PSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIconW(dwdeviceid: u32, lpszdeviceclass: super::super::Foundation::PWSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetLineDevStatus(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetLineDevStatusA(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetLineDevStatusW(hline: u32, lplinedevstatus: *mut LINEDEVSTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetMessage(hlineapp: u32, lpmessage: *mut LINEMESSAGE, dwtimeout: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetNewCalls(hline: u32, dwaddressid: u32, dwselect: u32, lpcalllist: *mut LINECALLLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetNumRings(hline: u32, dwaddressid: u32, lpdwnumrings: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetProviderList(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetProviderListA(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetProviderListW(dwapiversion: u32, lpproviderlist: *mut LINEPROVIDERLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetProxyStatus(hlineapp: u32, dwdeviceid: u32, dwappapiversion: u32, lplineproxyreqestlist: *mut LINEPROXYREQUESTLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetQueueInfo(hline: u32, dwqueueid: u32, lplinequeueinfo: *mut LINEQUEUEINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetQueueListA(hline: u32, lpgroupid: *mut ::windows_sys::core::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetQueueListW(hline: u32, lpgroupid: *mut ::windows_sys::core::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetRequest(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetRequestA(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetRequestW(hlineapp: u32, dwrequestmode: u32, lprequestbuffer: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetStatusMessages(hline: u32, lpdwlinestates: *mut u32, lpdwaddressstates: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetTranslateCaps(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetTranslateCapsA(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetTranslateCapsW(hlineapp: u32, dwapiversion: u32, lptranslatecaps: *mut LINETRANSLATECAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineHandoff(hcall: u32, lpszfilename: super::super::Foundation::PSTR, dwmediamode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineHandoffA(hcall: u32, lpszfilename: super::super::Foundation::PSTR, dwmediamode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineHandoffW(hcall: u32, lpszfilename: super::super::Foundation::PWSTR, dwmediamode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineHold(hcall: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitialize(lphlineapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: LINECALLBACK, lpszappname: super::super::Foundation::PSTR, lpdwnumdevs: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitializeExA(lphlineapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: LINECALLBACK, lpszfriendlyappname: super::super::Foundation::PSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitializeExW(lphlineapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: LINECALLBACK, lpszfriendlyappname: super::super::Foundation::PWSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineMakeCall(hline: u32, lphcall: *mut u32, lpszdestaddress: super::super::Foundation::PSTR, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineMakeCallA(hline: u32, lphcall: *mut u32, lpszdestaddress: super::super::Foundation::PSTR, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineMakeCallW(hline: u32, lphcall: *mut u32, lpszdestaddress: super::super::Foundation::PWSTR, dwcountrycode: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineMonitorDigits(hcall: u32, dwdigitmodes: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineMonitorMedia(hcall: u32, dwmediamodes: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineMonitorTones(hcall: u32, lptonelist: *const LINEMONITORTONE, dwnumentries: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineNegotiateAPIVersion(hlineapp: u32, dwdeviceid: u32, dwapilowversion: u32, dwapihighversion: u32, lpdwapiversion: *mut u32, lpextensionid: *mut LINEEXTENSIONID) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineNegotiateExtVersion(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextlowversion: u32, dwexthighversion: u32, lpdwextversion: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineOpen(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineOpenA(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineOpenW(hlineapp: u32, dwdeviceid: u32, lphline: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivileges: u32, dwmediamodes: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn linePark(hcall: u32, dwparkmode: u32, lpszdiraddress: super::super::Foundation::PSTR, lpnondiraddress: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineParkA(hcall: u32, dwparkmode: u32, lpszdiraddress: super::super::Foundation::PSTR, lpnondiraddress: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineParkW(hcall: u32, dwparkmode: u32, lpszdiraddress: super::super::Foundation::PWSTR, lpnondiraddress: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn linePickup(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: super::super::Foundation::PSTR, lpszgroupid: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn linePickupA(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: super::super::Foundation::PSTR, lpszgroupid: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn linePickupW(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: super::super::Foundation::PWSTR, lpszgroupid: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn linePrepareAddToConference(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn linePrepareAddToConferenceA(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn linePrepareAddToConferenceW(hconfcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineProxyMessage(hline: u32, hcall: u32, dwmsg: u32, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn lineProxyResponse(hline: u32, lpproxyrequest: *mut LINEPROXYREQUEST, dwresult: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineRedirect(hcall: u32, lpszdestaddress: super::super::Foundation::PSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineRedirectA(hcall: u32, lpszdestaddress: super::super::Foundation::PSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineRedirectW(hcall: u32, lpszdestaddress: super::super::Foundation::PWSTR, dwcountrycode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineRegisterRequestRecipient(hlineapp: u32, dwregistrationinstance: u32, dwrequestmode: u32, benable: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineReleaseUserUserInfo(hcall: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineRemoveFromConference(hcall: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineRemoveProvider(dwpermanentproviderid: u32, hwndowner: super::super::Foundation::HWND) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSecureCall(hcall: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSendUserUserInfo(hcall: u32, lpsuseruserinfo: super::super::Foundation::PSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentActivity(hline: u32, dwaddressid: u32, dwactivityid: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentGroup(hline: u32, dwaddressid: u32, lpagentgrouplist: *mut LINEAGENTGROUPLIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentMeasurementPeriod(hline: u32, hagent: u32, dwmeasurementperiod: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentSessionState(hline: u32, hagentsession: u32, dwagentsessionstate: u32, dwnextagentsessionstate: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentState(hline: u32, dwaddressid: u32, dwagentstate: u32, dwnextagentstate: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentStateEx(hline: u32, hagent: u32, dwagentstate: u32, dwnextagentstate: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetAppPriority(lpszappfilename: super::super::Foundation::PSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: super::super::Foundation::PSTR, dwpriority: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetAppPriorityA(lpszappfilename: super::super::Foundation::PSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: super::super::Foundation::PSTR, dwpriority: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetAppPriorityW(lpszappfilename: super::super::Foundation::PWSTR, dwmediamode: u32, lpextensionid: *mut LINEEXTENSIONID, dwrequestmode: u32, lpszextensionname: super::super::Foundation::PWSTR, dwpriority: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAppSpecific(hcall: u32, dwappspecific: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallData(hcall: u32, lpcalldata: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallParams(hcall: u32, dwbearermode: u32, dwminrate: u32, dwmaxrate: u32, lpdialparams: *const LINEDIALPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallPrivilege(hcall: u32, dwcallprivilege: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallQualityOfService(hcall: u32, lpsendingflowspec: *mut ::core::ffi::c_void, dwsendingflowspecsize: u32, lpreceivingflowspec: *mut ::core::ffi::c_void, dwreceivingflowspecsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallTreatment(hcall: u32, dwtreatment: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCurrentLocation(hlineapp: u32, dwlocation: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetDevConfig(dwdeviceid: u32, lpdeviceconfig: *const ::core::ffi::c_void, dwsize: u32, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetDevConfigA(dwdeviceid: u32, lpdeviceconfig: *const ::core::ffi::c_void, dwsize: u32, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetDevConfigW(dwdeviceid: u32, lpdeviceconfig: *const ::core::ffi::c_void, dwsize: u32, lpszdeviceclass: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetLineDevStatus(hline: u32, dwstatustochange: u32, fstatus: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetMediaControl(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, lpdigitlist: *const LINEMEDIACONTROLDIGIT, dwdigitnumentries: u32, lpmedialist: *const LINEMEDIACONTROLMEDIA, dwmedianumentries: u32, lptonelist: *const LINEMEDIACONTROLTONE, dwtonenumentries: u32, lpcallstatelist: *const LINEMEDIACONTROLCALLSTATE, dwcallstatenumentries: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetMediaMode(hcall: u32, dwmediamodes: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetNumRings(hline: u32, dwaddressid: u32, dwnumrings: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetQueueMeasurementPeriod(hline: u32, dwqueueid: u32, dwmeasurementperiod: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetStatusMessages(hline: u32, dwlinestates: u32, dwaddressstates: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetTerminal(hline: u32, dwaddressid: u32, hcall: u32, dwselect: u32, dwterminalmodes: u32, dwterminalid: u32, benable: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetTollList(hlineapp: u32, dwdeviceid: u32, lpszaddressin: super::super::Foundation::PSTR, dwtolllistoption: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetTollListA(hlineapp: u32, dwdeviceid: u32, lpszaddressin: super::super::Foundation::PSTR, dwtolllistoption: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetTollListW(hlineapp: u32, dwdeviceid: u32, lpszaddressinw: super::super::Foundation::PWSTR, dwtolllistoption: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupConference(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupConferenceA(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupConferenceW(hcall: u32, hline: u32, lphconfcall: *mut u32, lphconsultcall: *mut u32, dwnumparties: u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupTransfer(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupTransferA(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupTransferW(hcall: u32, lphconsultcall: *mut u32, lpcallparams: *const LINECALLPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineShutdown(hlineapp: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSwapHold(hactivecall: u32, hheldcall: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateAddress(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: super::super::Foundation::PSTR, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateAddressA(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: super::super::Foundation::PSTR, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateAddressW(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, lpszaddressin: super::super::Foundation::PWSTR, dwcard: u32, dwtranslateoptions: u32, lptranslateoutput: *mut LINETRANSLATEOUTPUT) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateDialog(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: super::super::Foundation::HWND, lpszaddressin: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateDialogA(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: super::super::Foundation::HWND, lpszaddressin: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateDialogW(hlineapp: u32, dwdeviceid: u32, dwapiversion: u32, hwndowner: super::super::Foundation::HWND, lpszaddressin: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineUncompleteCall(hline: u32, dwcompletionid: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineUnhold(hcall: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineUnpark(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineUnparkA(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineUnparkW(hline: u32, dwaddressid: u32, lphcall: *mut u32, lpszdestaddress: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneClose(hphone: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneConfigDialog(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneConfigDialogA(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneConfigDialogW(dwdeviceid: u32, hwndowner: super::super::Foundation::HWND, lpszdeviceclass: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneDevSpecific(hphone: u32, lpparams: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetButtonInfo(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetButtonInfoA(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetButtonInfoW(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *mut PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetData(hphone: u32, dwdataid: u32, lpdata: *mut ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetDevCaps(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetDevCapsA(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetDevCapsW(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextversion: u32, lpphonecaps: *mut PHONECAPS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetDisplay(hphone: u32, lpdisplay: *mut VARSTRING) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetGain(hphone: u32, dwhookswitchdev: u32, lpdwgain: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetHookSwitch(hphone: u32, lpdwhookswitchdevs: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetID(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIDA(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIDW(hphone: u32, lpdeviceid: *mut VARSTRING, lpszdeviceclass: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIcon(dwdeviceid: u32, lpszdeviceclass: super::super::Foundation::PSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIconA(dwdeviceid: u32, lpszdeviceclass: super::super::Foundation::PSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIconW(dwdeviceid: u32, lpszdeviceclass: super::super::Foundation::PWSTR, lphicon: *mut isize) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetLamp(hphone: u32, dwbuttonlampid: u32, lpdwlampmode: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetMessage(hphoneapp: u32, lpmessage: *mut PHONEMESSAGE, dwtimeout: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetRing(hphone: u32, lpdwringmode: *mut u32, lpdwvolume: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetStatus(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetStatusA(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetStatusMessages(hphone: u32, lpdwphonestates: *mut u32, lpdwbuttonmodes: *mut u32, lpdwbuttonstates: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetStatusW(hphone: u32, lpphonestatus: *mut PHONESTATUS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetVolume(hphone: u32, dwhookswitchdev: u32, lpdwvolume: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitialize(lphphoneapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: PHONECALLBACK, lpszappname: super::super::Foundation::PSTR, lpdwnumdevs: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitializeExA(lphphoneapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: PHONECALLBACK, lpszfriendlyappname: super::super::Foundation::PSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitializeExW(lphphoneapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: PHONECALLBACK, lpszfriendlyappname: super::super::Foundation::PWSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneNegotiateAPIVersion(hphoneapp: u32, dwdeviceid: u32, dwapilowversion: u32, dwapihighversion: u32, lpdwapiversion: *mut u32, lpextensionid: *mut PHONEEXTENSIONID) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneNegotiateExtVersion(hphoneapp: u32, dwdeviceid: u32, dwapiversion: u32, dwextlowversion: u32, dwexthighversion: u32, lpdwextversion: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneOpen(hphoneapp: u32, dwdeviceid: u32, lphphone: *mut u32, dwapiversion: u32, dwextversion: u32, dwcallbackinstance: usize, dwprivilege: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetButtonInfo(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetButtonInfoA(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetButtonInfoW(hphone: u32, dwbuttonlampid: u32, lpbuttoninfo: *const PHONEBUTTONINFO) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetData(hphone: u32, dwdataid: u32, lpdata: *const ::core::ffi::c_void, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneSetDisplay(hphone: u32, dwrow: u32, dwcolumn: u32, lpsdisplay: super::super::Foundation::PSTR, dwsize: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetGain(hphone: u32, dwhookswitchdev: u32, dwgain: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetHookSwitch(hphone: u32, dwhookswitchdevs: u32, dwhookswitchmode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetLamp(hphone: u32, dwbuttonlampid: u32, dwlampmode: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetRing(hphone: u32, dwringmode: u32, dwvolume: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetStatusMessages(hphone: u32, dwphonestates: u32, dwbuttonmodes: u32, dwbuttonstates: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetVolume(hphone: u32, dwhookswitchdev: u32, dwvolume: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneShutdown(hphoneapp: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiGetLocationInfo(lpszcountrycode: super::super::Foundation::PSTR, lpszcitycode: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiGetLocationInfoA(lpszcountrycode: super::super::Foundation::PSTR, lpszcitycode: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiGetLocationInfoW(lpszcountrycodew: super::super::Foundation::PWSTR, lpszcitycodew: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestDrop(hwnd: super::super::Foundation::HWND, wrequestid: super::super::Foundation::WPARAM) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMakeCall(lpszdestaddress: super::super::Foundation::PSTR, lpszappname: super::super::Foundation::PSTR, lpszcalledparty: super::super::Foundation::PSTR, lpszcomment: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMakeCallA(lpszdestaddress: super::super::Foundation::PSTR, lpszappname: super::super::Foundation::PSTR, lpszcalledparty: super::super::Foundation::PSTR, lpszcomment: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMakeCallW(lpszdestaddress: super::super::Foundation::PWSTR, lpszappname: super::super::Foundation::PWSTR, lpszcalledparty: super::super::Foundation::PWSTR, lpszcomment: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMediaCall(hwnd: super::super::Foundation::HWND, wrequestid: super::super::Foundation::WPARAM, lpszdeviceclass: super::super::Foundation::PSTR, lpdeviceid: super::super::Foundation::PSTR, dwsize: u32, dwsecure: u32, lpszdestaddress: super::super::Foundation::PSTR, lpszappname: super::super::Foundation::PSTR, lpszcalledparty: super::super::Foundation::PSTR, lpszcomment: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMediaCallA(hwnd: super::super::Foundation::HWND, wrequestid: super::super::Foundation::WPARAM, lpszdeviceclass: super::super::Foundation::PSTR, lpdeviceid: super::super::Foundation::PSTR, dwsize: u32, dwsecure: u32, lpszdestaddress: super::super::Foundation::PSTR, lpszappname: super::super::Foundation::PSTR, lpszcalledparty: super::super::Foundation::PSTR, lpszcomment: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMediaCallW(hwnd: super::super::Foundation::HWND, wrequestid: super::super::Foundation::WPARAM, lpszdeviceclass: super::super::Foundation::PWSTR, lpdeviceid: super::super::Foundation::PWSTR, dwsize: u32, dwsecure: u32, lpszdestaddress: super::super::Foundation::PWSTR, lpszappname: super::super::Foundation::PWSTR, lpszcalledparty: super::super::Foundation::PWSTR, lpszcomment: super::super::Foundation::PWSTR) -> i32;
}
pub struct ACDGROUP_EVENT(i32);
pub struct ACDQUEUE_EVENT(i32);
pub struct ADDRESS_CAPABILITY(i32);
pub struct ADDRESS_CAPABILITY_STRING(i32);
pub struct ADDRESS_EVENT(i32);
pub struct ADDRESS_STATE(i32);
pub struct AGENTHANDLER_EVENT(i32);
pub struct AGENT_EVENT(i32);
pub struct AGENT_SESSION_EVENT(i32);
pub struct AGENT_SESSION_STATE(i32);
pub struct AGENT_STATE(i32);
pub struct ASYNC_COMPLETION(i32);
pub struct CALLHUB_EVENT(i32);
pub struct CALLHUB_STATE(i32);
pub struct CALLINFOCHANGE_CAUSE(i32);
pub struct CALLINFO_BUFFER(i32);
pub struct CALLINFO_LONG(i32);
pub struct CALLINFO_STRING(i32);
pub struct CALL_MEDIA_EVENT(i32);
pub struct CALL_MEDIA_EVENT_CAUSE(i32);
pub struct CALL_NOTIFICATION_EVENT(i32);
pub struct CALL_PRIVILEGE(i32);
pub struct CALL_STATE(i32);
pub struct CALL_STATE_EVENT_CAUSE(i32);
pub struct DIRECTORY_OBJECT_TYPE(i32);
pub struct DIRECTORY_TYPE(i32);
pub struct DISCONNECT_CODE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const DISPIDMASK: u32 = 65535u32;
pub struct DispatchMapper(i32);
pub struct FINISH_MODE(i32);
pub struct FT_STATE_EVENT_CAUSE(i32);
pub struct FULLDUPLEX_SUPPORT(i32);
pub struct HDRVCALL__(i32);
pub struct HDRVDIALOGINSTANCE__(i32);
pub struct HDRVLINE__(i32);
pub struct HDRVMSPLINE__(i32);
pub struct HDRVPHONE__(i32);
pub struct HPROVIDER__(i32);
pub struct HTAPICALL__(i32);
pub struct HTAPILINE__(i32);
pub struct HTAPIPHONE__(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPADDRESS: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPADDRESSCAPABILITIES: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPADDRESSTRANSLATION: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPAGGREGATEDMSPADDRESSOBJ: u32 = 393216u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPAGGREGATEDMSPCALLOBJ: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPAPC: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPBASICCALLCONTROL: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPCALLINFO: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPDIRECTORY: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPDIROBJCONFERENCE: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPDIROBJECT: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPDIROBJUSER: u32 = 196608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPFILETRACK: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPILSCONFIG: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPLEGACYADDRESSMEDIACONTROL: u32 = 327680u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPLEGACYCALLMEDIACONTROL: u32 = 196608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPMEDIACONTROL: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPMEDIAPLAYBACK: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPMEDIARECORD: u32 = 196608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPMEDIASUPPORT: u32 = 196608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPMULTITRACK: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPPHONE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPTAPI: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const IDISPTAPICALLCENTER: u32 = 131072u32;
pub struct IEnumACDGroup(i32);
pub struct IEnumAddress(i32);
pub struct IEnumAgent(i32);
pub struct IEnumAgentHandler(i32);
pub struct IEnumAgentSession(i32);
pub struct IEnumBstr(i32);
pub struct IEnumCall(i32);
pub struct IEnumCallHub(i32);
pub struct IEnumCallingCard(i32);
pub struct IEnumDialableAddrs(i32);
pub struct IEnumDirectory(i32);
pub struct IEnumDirectoryObject(i32);
pub struct IEnumLocation(i32);
pub struct IEnumMcastScope(i32);
pub struct IEnumPhone(i32);
pub struct IEnumPluggableSuperclassInfo(i32);
pub struct IEnumPluggableTerminalClassInfo(i32);
pub struct IEnumQueue(i32);
pub struct IEnumStream(i32);
pub struct IEnumSubStream(i32);
pub struct IEnumTerminal(i32);
pub struct IEnumTerminalClass(i32);
pub struct IMcastAddressAllocation(i32);
pub struct IMcastLeaseInfo(i32);
pub struct IMcastScope(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const INITIALIZE_NEGOTIATION: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const INTERFACEMASK: u32 = 16711680u32;
pub struct ITACDGroup(i32);
pub struct ITACDGroupEvent(i32);
pub struct ITAMMediaFormat(i32);
pub struct ITASRTerminalEvent(i32);
pub struct ITAddress(i32);
pub struct ITAddress2(i32);
pub struct ITAddressCapabilities(i32);
pub struct ITAddressDeviceSpecificEvent(i32);
pub struct ITAddressEvent(i32);
pub struct ITAddressTranslation(i32);
pub struct ITAddressTranslationInfo(i32);
pub struct ITAgent(i32);
pub struct ITAgentEvent(i32);
pub struct ITAgentHandler(i32);
pub struct ITAgentHandlerEvent(i32);
pub struct ITAgentSession(i32);
pub struct ITAgentSessionEvent(i32);
pub struct ITAllocatorProperties(i32);
pub struct ITAutomatedPhoneControl(i32);
pub struct ITBasicAudioTerminal(i32);
pub struct ITBasicCallControl(i32);
pub struct ITBasicCallControl2(i32);
pub struct ITCallHub(i32);
pub struct ITCallHubEvent(i32);
pub struct ITCallInfo(i32);
pub struct ITCallInfo2(i32);
pub struct ITCallInfoChangeEvent(i32);
pub struct ITCallMediaEvent(i32);
pub struct ITCallNotificationEvent(i32);
pub struct ITCallStateEvent(i32);
pub struct ITCallingCard(i32);
pub struct ITCollection(i32);
pub struct ITCollection2(i32);
pub struct ITCustomTone(i32);
pub struct ITDetectTone(i32);
pub struct ITDigitDetectionEvent(i32);
pub struct ITDigitGenerationEvent(i32);
pub struct ITDigitsGatheredEvent(i32);
pub struct ITDirectory(i32);
pub struct ITDirectoryObject(i32);
pub struct ITDirectoryObjectConference(i32);
pub struct ITDirectoryObjectUser(i32);
pub struct ITDispatchMapper(i32);
pub struct ITFileTerminalEvent(i32);
pub struct ITFileTrack(i32);
pub struct ITForwardInformation(i32);
pub struct ITForwardInformation2(i32);
pub struct ITILSConfig(i32);
pub struct ITLegacyAddressMediaControl(i32);
pub struct ITLegacyAddressMediaControl2(i32);
pub struct ITLegacyCallMediaControl(i32);
pub struct ITLegacyCallMediaControl2(i32);
pub struct ITLegacyWaveSupport(i32);
pub struct ITLocationInfo(i32);
pub struct ITMSPAddress(i32);
pub struct ITMediaControl(i32);
pub struct ITMediaPlayback(i32);
pub struct ITMediaRecord(i32);
pub struct ITMediaSupport(i32);
pub struct ITMultiTrackTerminal(i32);
pub struct ITPhone(i32);
pub struct ITPhoneDeviceSpecificEvent(i32);
pub struct ITPhoneEvent(i32);
pub struct ITPluggableTerminalClassInfo(i32);
pub struct ITPluggableTerminalEventSink(i32);
pub struct ITPluggableTerminalEventSinkRegistration(i32);
pub struct ITPluggableTerminalSuperclassInfo(i32);
pub struct ITPrivateEvent(i32);
pub struct ITQOSEvent(i32);
pub struct ITQueue(i32);
pub struct ITQueueEvent(i32);
pub struct ITRendezvous(i32);
pub struct ITRequest(i32);
pub struct ITRequestEvent(i32);
pub struct ITScriptableAudioFormat(i32);
pub struct ITStaticAudioTerminal(i32);
pub struct ITStream(i32);
pub struct ITStreamControl(i32);
pub struct ITSubStream(i32);
pub struct ITSubStreamControl(i32);
pub struct ITTAPI(i32);
pub struct ITTAPI2(i32);
pub struct ITTAPICallCenter(i32);
pub struct ITTAPIDispatchEventNotification(i32);
pub struct ITTAPIEventNotification(i32);
pub struct ITTAPIObjectEvent(i32);
pub struct ITTAPIObjectEvent2(i32);
pub struct ITTTSTerminalEvent(i32);
pub struct ITTerminal(i32);
pub struct ITTerminalSupport(i32);
pub struct ITTerminalSupport2(i32);
pub struct ITToneDetectionEvent(i32);
pub struct ITToneTerminalEvent(i32);
pub struct ITnef(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LAST_LINEMEDIAMODE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LAST_LINEREQUESTMODE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_ACCEPTTOALERT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_ACDGROUP: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_AUTORECONNECT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_BLOCKIDDEFAULT: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_BLOCKIDOVERRIDE: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_COMPLETIONID: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_CONFDROP: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_CONFERENCEHELD: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_CONFERENCEMAKE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_DESTOFFHOOK: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_DIALED: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_FWDBUSYNAADDR: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_FWDCONSULT: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_FWDINTEXTADDR: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_FWDNUMRINGS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_FWDSTATUSVALID: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_HOLDMAKESNEW: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_NOEXTERNALCALLS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_NOINTERNALCALLS: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_NOPSTNADDRESSTRANSLATION: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_ORIGOFFHOOK: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_PARTIALDIAL: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_PICKUPCALLWAIT: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_PICKUPGROUPID: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_PREDICTIVEDIALER: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_QUEUE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_ROUTEPOINT: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_SECURE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_SETCALLINGID: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_SETUPCONFNULL: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_TRANSFERHELD: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRCAPFLAGS_TRANSFERMAKE: u32 = 8192u32;
pub struct LINEADDRESSCAPS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSMODE_ADDRESSID: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSMODE_DIALABLEADDR: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSHARING_BRIDGEDEXCL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSHARING_BRIDGEDNEW: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSHARING_BRIDGEDSHARED: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSHARING_MONITORED: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSHARING_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSTATE_CAPSCHANGE: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSTATE_DEVSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSTATE_FORWARD: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSTATE_INUSEMANY: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSTATE_INUSEONE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSTATE_INUSEZERO: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSTATE_NUMCALLS: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSSTATE_TERMINALS: u32 = 128u32;
pub struct LINEADDRESSSTATUS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSTYPE_DOMAINNAME: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSTYPE_EMAILNAME: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSTYPE_IPADDRESS: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSTYPE_PHONENUMBER: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRESSTYPE_SDP: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_FORWARD: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_FORWARDDND: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_FORWARDFWD: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_MAKECALL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_PICKUP: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_PICKUPDIRECT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_PICKUPGROUP: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_PICKUPHELD: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_PICKUPWAITING: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_SETMEDIACONTROL: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_SETTERMINAL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_SETUPCONF: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_UNCOMPLETECALL: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEADDRFEATURE_UNPARK: u32 = 128u32;
pub struct LINEAGENTACTIVITYENTRY(i32);
pub struct LINEAGENTACTIVITYLIST(i32);
pub struct LINEAGENTCAPS(i32);
pub struct LINEAGENTENTRY(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTFEATURE_AGENTSPECIFIC: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTFEATURE_GETAGENTACTIVITYLIST: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTFEATURE_GETAGENTGROUP: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTFEATURE_SETAGENTACTIVITY: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTFEATURE_SETAGENTGROUP: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTFEATURE_SETAGENTSTATE: u32 = 2u32;
pub struct LINEAGENTGROUPENTRY(i32);
pub struct LINEAGENTGROUPLIST(i32);
pub struct LINEAGENTINFO(i32);
pub struct LINEAGENTLIST(i32);
pub struct LINEAGENTSESSIONENTRY(i32);
pub struct LINEAGENTSESSIONINFO(i32);
pub struct LINEAGENTSESSIONLIST(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSESSIONSTATE_BUSYONCALL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSESSIONSTATE_BUSYWRAPUP: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSESSIONSTATE_ENDED: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSESSIONSTATE_NOTREADY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSESSIONSTATE_READY: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSESSIONSTATE_RELEASED: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSESSIONSTATUS_NEWSESSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSESSIONSTATUS_STATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSESSIONSTATUS_UPDATEINFO: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATEEX_BUSYACD: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATEEX_BUSYINCOMING: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATEEX_BUSYOUTGOING: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATEEX_NOTREADY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATEEX_READY: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATEEX_RELEASED: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATEEX_UNKNOWN: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_BUSYACD: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_BUSYINCOMING: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_BUSYOTHER: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_BUSYOUTBOUND: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_LOGGEDOFF: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_NOTREADY: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_READY: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_UNAVAIL: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_UNKNOWN: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATE_WORKINGAFTERCALL: u32 = 128u32;
pub struct LINEAGENTSTATUS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUSEX_NEWAGENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUSEX_STATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUSEX_UPDATEINFO: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUS_ACTIVITY: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUS_ACTIVITYLIST: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUS_CAPSCHANGE: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUS_GROUP: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUS_GROUPLIST: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUS_NEXTSTATE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUS_STATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUS_VALIDNEXTSTATES: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEAGENTSTATUS_VALIDSTATES: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEANSWERMODE_DROP: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEANSWERMODE_HOLD: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEANSWERMODE_NONE: u32 = 1u32;
pub struct LINEAPPINFO(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBEARERMODE_ALTSPEECHDATA: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBEARERMODE_DATA: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBEARERMODE_MULTIUSE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBEARERMODE_NONCALLSIGNALING: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBEARERMODE_PASSTHROUGH: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBEARERMODE_RESTRICTEDDATA: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBEARERMODE_SPEECH: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBEARERMODE_VOICE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBUSYMODE_STATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBUSYMODE_TRUNK: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBUSYMODE_UNAVAIL: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEBUSYMODE_UNKNOWN: u32 = 4u32;
pub struct LINECALLBACK(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLCOMPLCOND_BUSY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLCOMPLCOND_NOANSWER: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLCOMPLMODE_CALLBACK: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLCOMPLMODE_CAMPON: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLCOMPLMODE_INTRUDE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLCOMPLMODE_MESSAGE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_COMPLCALLBACK: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_COMPLCAMPON: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_COMPLINTRUDE: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_COMPLMESSAGE: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_NOHOLDCONFERENCE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_ONESTEPTRANSFER: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_PARKDIRECT: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_PARKNONDIRECT: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_TRANSFERCONF: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE2_TRANSFERNORM: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_ACCEPT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_ADDTOCONF: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_ANSWER: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_BLINDTRANSFER: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_COMPLETECALL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_COMPLETETRANSF: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_DIAL: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_DROP: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_GATHERDIGITS: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_GENERATEDIGITS: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_GENERATETONE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_HOLD: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_MONITORDIGITS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_MONITORMEDIA: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_MONITORTONES: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_PARK: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_PREPAREADDCONF: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_REDIRECT: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_RELEASEUSERUSERINFO: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_REMOVEFROMCONF: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SECURECALL: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SENDUSERUSER: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SETCALLDATA: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SETCALLPARAMS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SETMEDIACONTROL: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SETQOS: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SETTERMINAL: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SETTREATMENT: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SETUPCONF: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SETUPTRANSFER: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_SWAPHOLD: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLFEATURE_UNHOLD: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLHUBTRACKING_ALLCALLS: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLHUBTRACKING_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLHUBTRACKING_PROVIDERLEVEL: u32 = 1u32;
pub struct LINECALLINFO(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_APPSPECIFIC: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_BEARERMODE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_CALLDATA: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_CALLEDID: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_CALLERID: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_CALLID: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_CHARGINGINFO: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_COMPLETIONID: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_CONNECTEDID: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_DEVSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_DIALPARAMS: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_DISPLAY: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_HIGHLEVELCOMP: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_LOWLEVELCOMP: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_MEDIAMODE: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_MONITORMODES: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_NUMMONITORS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_NUMOWNERDECR: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_NUMOWNERINCR: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_ORIGIN: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_QOS: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_RATE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_REASON: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_REDIRECTINGID: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_REDIRECTIONID: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_RELATEDCALLID: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_TERMINAL: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_TREATMENT: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_TRUNK: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLINFOSTATE_USERUSERINFO: u32 = 2097152u32;
pub struct LINECALLLIST(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLORIGIN_CONFERENCE: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLORIGIN_EXTERNAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLORIGIN_INBOUND: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLORIGIN_INTERNAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLORIGIN_OUTBOUND: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLORIGIN_UNAVAIL: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLORIGIN_UNKNOWN: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARAMFLAGS_BLOCKID: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARAMFLAGS_DESTOFFHOOK: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARAMFLAGS_IDLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARAMFLAGS_NOHOLDCONFERENCE: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARAMFLAGS_ONESTEPTRANSFER: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARAMFLAGS_ORIGOFFHOOK: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARAMFLAGS_PREDICTIVEDIAL: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARAMFLAGS_SECURE: u32 = 1u32;
pub struct LINECALLPARAMS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARTYID_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARTYID_BLOCKED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARTYID_NAME: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARTYID_OUTOFAREA: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARTYID_PARTIAL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARTYID_UNAVAIL: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPARTYID_UNKNOWN: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPRIVILEGE_MONITOR: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPRIVILEGE_NONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLPRIVILEGE_OWNER: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_CALLCOMPLETION: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_CAMPEDON: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_DIRECT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_FWDBUSY: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_FWDNOANSWER: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_FWDUNCOND: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_INTRUDE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_PARKED: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_PICKUP: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_REDIRECT: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_REMINDER: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_ROUTEREQUEST: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_TRANSFER: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_UNAVAIL: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_UNKNOWN: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLREASON_UNPARK: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSELECT_ADDRESS: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSELECT_CALL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSELECT_CALLID: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSELECT_DEVICEID: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSELECT_LINE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_ACCEPTED: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_BUSY: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_CONFERENCED: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_CONNECTED: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_DIALING: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_DIALTONE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_DISCONNECTED: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_IDLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_OFFERING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_ONHOLD: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_ONHOLDPENDCONF: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_ONHOLDPENDTRANSFER: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_PROCEEDING: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_RINGBACK: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_SPECIALINFO: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLSTATE_UNKNOWN: u32 = 32768u32;
pub struct LINECALLSTATUS(i32);
pub struct LINECALLTREATMENTENTRY(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLTREATMENT_BUSY: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLTREATMENT_MUSIC: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLTREATMENT_RINGBACK: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECALLTREATMENT_SILENCE: u32 = 1u32;
pub struct LINECARDENTRY(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECARDOPTION_HIDDEN: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECARDOPTION_PREDEFINED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECONNECTEDMODE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECONNECTEDMODE_ACTIVEHELD: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECONNECTEDMODE_CONFIRMED: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECONNECTEDMODE_INACTIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINECONNECTEDMODE_INACTIVEHELD: u32 = 8u32;
pub struct LINECOUNTRYENTRY(i32);
pub struct LINECOUNTRYLIST(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_CALLHUB: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_CALLHUBTRACKING: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_CLOSEDROP: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_CROSSADDRCONF: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_DIALBILLING: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_DIALDIALTONE: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_DIALQUIET: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_HIGHLEVCOMP: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_LOCAL: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_LOWLEVCOMP: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_MEDIACONTROL: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_MSP: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_MULTIPLEADDR: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVCAPFLAGS_PRIVATEOBJECTS: u32 = 4096u32;
pub struct LINEDEVCAPS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_BATTERY: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_CAPSCHANGE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_CLOSE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_COMPLCANCEL: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_CONFIGCHANGE: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_CONNECTED: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_DEVSPECIFIC: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_DISCONNECTED: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_INSERVICE: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_LOCK: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_MAINTENANCE: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_MSGWAITOFF: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_MSGWAITON: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_NUMCALLS: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_NUMCOMPLETIONS: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_OPEN: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_OUTOFSERVICE: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_REINIT: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_REMOVED: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_RINGING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_ROAMMODE: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_SIGNAL: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_TERMINALS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATE_TRANSLATECHANGE: u32 = 4194304u32;
pub struct LINEDEVSTATUS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATUSFLAGS_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATUSFLAGS_INSERVICE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATUSFLAGS_LOCKED: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDEVSTATUSFLAGS_MSGWAIT: u32 = 2u32;
pub struct LINEDIALPARAMS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDIALTONEMODE_EXTERNAL: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDIALTONEMODE_INTERNAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDIALTONEMODE_NORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDIALTONEMODE_SPECIAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDIALTONEMODE_UNAVAIL: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDIALTONEMODE_UNKNOWN: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDIGITMODE_DTMF: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDIGITMODE_DTMFEND: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDIGITMODE_PULSE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_BADADDRESS: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_BLOCKED: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_BUSY: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_CANCELLED: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_CONGESTION: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_DESTINATIONBARRED: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_DONOTDISTURB: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_FDNRESTRICT: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_FORWARDED: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_INCOMPATIBLE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_NOANSWER: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_NODIALTONE: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_NORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_NUMBERCHANGED: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_OUTOFORDER: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_PICKUP: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_QOSUNAVAIL: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_REJECT: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_TEMPFAILURE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_UNAVAIL: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEDISCONNECTMODE_UNREACHABLE: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEEQOSINFO_ADMISSIONFAILURE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEEQOSINFO_GENERICERROR: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEEQOSINFO_NOQOS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEEQOSINFO_POLICYFAILURE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_ADDRESSBLOCKED: u32 = 2147483731u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_ALLOCATED: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_BADDEVICEID: u32 = 2147483650u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_BEARERMODEUNAVAIL: u32 = 2147483651u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_BILLINGREJECTED: u32 = 2147483732u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_CALLUNAVAIL: u32 = 2147483653u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_COMPLETIONOVERRUN: u32 = 2147483654u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_CONFERENCEFULL: u32 = 2147483655u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_DIALBILLING: u32 = 2147483656u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_DIALDIALTONE: u32 = 2147483657u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_DIALPROMPT: u32 = 2147483658u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_DIALQUIET: u32 = 2147483659u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_DIALVOICEDETECT: u32 = 2147483740u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_DISCONNECTED: u32 = 2147483744u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INCOMPATIBLEAPIVERSION: u32 = 2147483660u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INCOMPATIBLEEXTVERSION: u32 = 2147483661u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INIFILECORRUPT: u32 = 2147483662u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INUSE: u32 = 2147483663u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALADDRESS: u32 = 2147483664u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALADDRESSID: u32 = 2147483665u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALADDRESSMODE: u32 = 2147483666u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALADDRESSSTATE: u32 = 2147483667u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALADDRESSTYPE: u32 = 2147483742u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALAGENTACTIVITY: u32 = 2147483739u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALAGENTGROUP: u32 = 2147483736u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALAGENTID: u32 = 2147483735u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALAGENTSESSIONSTATE: u32 = 2147483743u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALAGENTSTATE: u32 = 2147483738u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALAPPHANDLE: u32 = 2147483668u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALAPPNAME: u32 = 2147483669u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALBEARERMODE: u32 = 2147483670u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCALLCOMPLMODE: u32 = 2147483671u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCALLHANDLE: u32 = 2147483672u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCALLPARAMS: u32 = 2147483673u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCALLPRIVILEGE: u32 = 2147483674u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCALLSELECT: u32 = 2147483675u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCALLSTATE: u32 = 2147483676u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCALLSTATELIST: u32 = 2147483677u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCARD: u32 = 2147483678u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCOMPLETIONID: u32 = 2147483679u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCONFCALLHANDLE: u32 = 2147483680u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCONSULTCALLHANDLE: u32 = 2147483681u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALCOUNTRYCODE: u32 = 2147483682u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALDEVICECLASS: u32 = 2147483683u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALDEVICEHANDLE: u32 = 2147483684u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALDIALPARAMS: u32 = 2147483685u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALDIGITLIST: u32 = 2147483686u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALDIGITMODE: u32 = 2147483687u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALDIGITS: u32 = 2147483688u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALEXTVERSION: u32 = 2147483689u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALFEATURE: u32 = 2147483733u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALGROUPID: u32 = 2147483690u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALLINEHANDLE: u32 = 2147483691u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALLINESTATE: u32 = 2147483692u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALLOCATION: u32 = 2147483693u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALMEDIALIST: u32 = 2147483694u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALMEDIAMODE: u32 = 2147483695u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALMESSAGEID: u32 = 2147483696u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALPARAM: u32 = 2147483698u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALPARKID: u32 = 2147483699u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALPARKMODE: u32 = 2147483700u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALPASSWORD: u32 = 2147483737u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALPOINTER: u32 = 2147483701u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALPRIVSELECT: u32 = 2147483702u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALRATE: u32 = 2147483703u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALREQUESTMODE: u32 = 2147483704u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALTERMINALID: u32 = 2147483705u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALTERMINALMODE: u32 = 2147483706u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALTIMEOUT: u32 = 2147483707u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALTONE: u32 = 2147483708u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALTONELIST: u32 = 2147483709u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALTONEMODE: u32 = 2147483710u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_INVALTRANSFERMODE: u32 = 2147483711u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_LINEMAPPERFAILED: u32 = 2147483712u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_NOCONFERENCE: u32 = 2147483713u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_NODEVICE: u32 = 2147483714u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_NODRIVER: u32 = 2147483715u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_NOMEM: u32 = 2147483716u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_NOMULTIPLEINSTANCE: u32 = 2147483734u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_NOREQUEST: u32 = 2147483717u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_NOTOWNER: u32 = 2147483718u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_NOTREGISTERED: u32 = 2147483719u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_OPERATIONFAILED: u32 = 2147483720u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_OPERATIONUNAVAIL: u32 = 2147483721u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_RATEUNAVAIL: u32 = 2147483722u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_REINIT: u32 = 2147483730u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_REQUESTOVERRUN: u32 = 2147483724u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_RESOURCEUNAVAIL: u32 = 2147483723u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_SERVICE_NOT_RUNNING: u32 = 2147483745u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_STRUCTURETOOSMALL: u32 = 2147483725u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_TARGETNOTFOUND: u32 = 2147483726u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_TARGETSELF: u32 = 2147483727u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_UNINITIALIZED: u32 = 2147483728u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_USERCANCELLED: u32 = 2147483741u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEERR_USERUSERINFOTOOBIG: u32 = 2147483729u32;
pub struct LINEEVENT(i32);
pub struct LINEEXTENSIONID(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFEATURE_DEVSPECIFIC: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFEATURE_DEVSPECIFICFEAT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFEATURE_FORWARD: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFEATURE_FORWARDDND: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFEATURE_FORWARDFWD: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFEATURE_MAKECALL: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFEATURE_SETDEVSTATUS: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFEATURE_SETMEDIACONTROL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFEATURE_SETTERMINAL: u32 = 32u32;
pub struct LINEFORWARD(i32);
pub struct LINEFORWARDLIST(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_BUSY: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_BUSYEXTERNAL: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_BUSYINTERNAL: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_BUSYNA: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_BUSYNAEXTERNAL: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_BUSYNAINTERNAL: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_BUSYNASPECIFIC: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_BUSYSPECIFIC: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_NOANSW: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_NOANSWEXTERNAL: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_NOANSWINTERNAL: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_NOANSWSPECIFIC: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_UNAVAIL: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_UNCOND: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_UNCONDEXTERNAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_UNCONDINTERNAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_UNCONDSPECIFIC: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEFORWARDMODE_UNKNOWN: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEGATHERTERM_BUFFERFULL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEGATHERTERM_CANCEL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEGATHERTERM_FIRSTTIMEOUT: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEGATHERTERM_INTERTIMEOUT: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEGATHERTERM_TERMDIGIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEGENERATETERM_CANCEL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEGENERATETERM_DONE: u32 = 1u32;
pub struct LINEGENERATETONE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEGROUPSTATUS_GROUPREMOVED: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEGROUPSTATUS_NEWGROUP: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEINITIALIZEEXOPTION_CALLHUBTRACKING: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEINITIALIZEEXOPTION_USECOMPLETIONPORT: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEINITIALIZEEXOPTION_USEEVENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEINITIALIZEEXOPTION_USEHIDDENWINDOW: u32 = 1u32;
pub struct LINEINITIALIZEEXPARAMS(i32);
pub struct LINELOCATIONENTRY(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINELOCATIONOPTION_PULSEDIAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMAPPER: u32 = 4294967295u32;
pub struct LINEMEDIACONTROLCALLSTATE(i32);
pub struct LINEMEDIACONTROLDIGIT(i32);
pub struct LINEMEDIACONTROLMEDIA(i32);
pub struct LINEMEDIACONTROLTONE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_NONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_PAUSE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_RATEDOWN: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_RATENORMAL: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_RATEUP: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_RESET: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_RESUME: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_START: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_VOLUMEDOWN: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_VOLUMENORMAL: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIACONTROL_VOLUMEUP: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_ADSI: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_AUTOMATEDVOICE: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_DATAMODEM: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_DIGITALDATA: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_G3FAX: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_G4FAX: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_INTERACTIVEVOICE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_MIXED: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_TDD: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_TELETEX: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_TELEX: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_UNKNOWN: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_VIDEO: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_VIDEOTEX: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEMEDIAMODE_VOICEVIEW: u32 = 16384u32;
pub struct LINEMESSAGE(i32);
pub struct LINEMONITORTONE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEOFFERINGMODE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEOFFERINGMODE_INACTIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEOPENOPTION_PROXY: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEOPENOPTION_SINGLEADDRESS: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPARKMODE_DIRECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPARKMODE_NONDIRECTED: u32 = 2u32;
pub struct LINEPROVIDERENTRY(i32);
pub struct LINEPROVIDERLIST(i32);
pub struct LINEPROXYREQUEST(i32);
pub struct LINEPROXYREQUESTLIST(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_AGENTSPECIFIC: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_CREATEAGENT: u32 = 9u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_CREATEAGENTSESSION: u32 = 12u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETAGENTACTIVITYLIST: u32 = 7u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETAGENTCAPS: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETAGENTGROUPLIST: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETAGENTINFO: u32 = 11u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETAGENTSESSIONINFO: u32 = 15u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETAGENTSESSIONLIST: u32 = 13u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETAGENTSTATUS: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETGROUPLIST: u32 = 19u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETQUEUEINFO: u32 = 18u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_GETQUEUELIST: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_SETAGENTACTIVITY: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_SETAGENTGROUP: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_SETAGENTMEASUREMENTPERIOD: u32 = 10u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_SETAGENTSESSIONSTATE: u32 = 14u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_SETAGENTSTATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_SETAGENTSTATEEX: u32 = 20u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYREQUEST_SETQUEUEMEASUREMENTPERIOD: u32 = 17u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYSTATUS_ALLOPENFORACD: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYSTATUS_CLOSE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEPROXYSTATUS_OPEN: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEQOSREQUESTTYPE_SERVICELEVEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEQOSSERVICELEVEL_BESTEFFORT: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEQOSSERVICELEVEL_IFAVAILABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEQOSSERVICELEVEL_NEEDED: u32 = 1u32;
pub struct LINEQUEUEENTRY(i32);
pub struct LINEQUEUEINFO(i32);
pub struct LINEQUEUELIST(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEQUEUESTATUS_NEWQUEUE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEQUEUESTATUS_QUEUEREMOVED: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEQUEUESTATUS_UPDATEINFO: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEREMOVEFROMCONF_ANY: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEREMOVEFROMCONF_LAST: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEREMOVEFROMCONF_NONE: u32 = 1u32;
pub struct LINEREQMAKECALL(i32);
pub struct LINEREQMEDIACALL(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEREQUESTMODE_DROP: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEREQUESTMODE_MAKECALL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEREQUESTMODE_MEDIACALL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEROAMMODE_HOME: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEROAMMODE_ROAMA: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEROAMMODE_ROAMB: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEROAMMODE_UNAVAIL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINEROAMMODE_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINESPECIALINFO_CUSTIRREG: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINESPECIALINFO_NOCIRCUIT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINESPECIALINFO_REORDER: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINESPECIALINFO_UNAVAIL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINESPECIALINFO_UNKNOWN: u32 = 8u32;
pub struct LINETERMCAPS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMDEV_HEADSET: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMDEV_PHONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMDEV_SPEAKER: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMMODE_BUTTONS: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMMODE_DISPLAY: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMMODE_HOOKSWITCH: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMMODE_LAMPS: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMMODE_MEDIABIDIRECT: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMMODE_MEDIAFROMLINE: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMMODE_MEDIATOLINE: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMMODE_RINGER: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMSHARING_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMSHARING_SHAREDCONF: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETERMSHARING_SHAREDEXCL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETOLLLISTOPTION_ADD: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETOLLLISTOPTION_REMOVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETONEMODE_BEEP: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETONEMODE_BILLING: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETONEMODE_BUSY: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETONEMODE_CUSTOM: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETONEMODE_RINGBACK: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSFERMODE_CONFERENCE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSFERMODE_TRANSFER: u32 = 1u32;
pub struct LINETRANSLATECAPS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATEOPTION_CANCELCALLWAITING: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATEOPTION_CARDOVERRIDE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATEOPTION_FORCELD: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATEOPTION_FORCELOCAL: u32 = 4u32;
pub struct LINETRANSLATEOUTPUT(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_CANONICAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_DIALBILLING: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_DIALDIALTONE: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_DIALPROMPT: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_DIALQUIET: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_INTERNATIONAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_INTOLLLIST: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_LOCAL: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_LONGDISTANCE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_NOTINTOLLLIST: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_NOTRANSLATION: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETRANSLATERESULT_VOICEDETECT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINETSPIOPTION_NONREENTRANT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_ADDRESSSTATE: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_AGENTSESSIONSTATUS: i32 = 27i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_AGENTSPECIFIC: i32 = 21i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_AGENTSTATUS: i32 = 22i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_AGENTSTATUSEX: i32 = 29i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_APPNEWCALL: i32 = 23i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_APPNEWCALLHUB: i32 = 32i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_CALLHUBCLOSE: i32 = 33i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_CALLINFO: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_CALLSTATE: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_CLOSE: i32 = 3i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_CREATE: i32 = 19i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_DEVSPECIFIC: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_DEVSPECIFICEX: i32 = 34i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_DEVSPECIFICFEATURE: i32 = 5i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_GATHERDIGITS: i32 = 6i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_GENERATE: i32 = 7i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_GROUPSTATUS: i32 = 30i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_LINEDEVSTATE: i32 = 8i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_MONITORDIGITS: i32 = 9i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_MONITORMEDIA: i32 = 10i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_MONITORTONE: i32 = 11i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_PROXYREQUEST: i32 = 24i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_PROXYSTATUS: i32 = 31i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_QUEUESTATUS: i32 = 28i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_REMOVE: i32 = 25i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_REPLY: i32 = 12i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const LINE_REQUEST: i32 = 13i32;
pub struct LPGETTNEFSTREAMCODEPAGE(i32);
pub struct LPOPENTNEFSTREAM(i32);
pub struct LPOPENTNEFSTREAMEX(i32);
pub struct MSP_ADDRESS_EVENT(i32);
pub struct MSP_CALL_EVENT(i32);
pub struct MSP_CALL_EVENT_CAUSE(i32);
pub struct MSP_EVENT(i32);
pub struct MSP_EVENT_INFO(i32);
pub struct McastAddressAllocation(i32);
pub struct NSID(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_ABBREVDIAL: u32 = 11u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_BRIDGEDAPP: u32 = 28u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_BUSY: u32 = 29u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_CALLAPP: u32 = 30u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_CALLID: u32 = 34u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_CAMPON: u32 = 43u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_CONFERENCE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_CONNECT: u32 = 7u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_COVER: u32 = 33u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_DATAOFF: u32 = 25u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_DATAON: u32 = 24u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_DATETIME: u32 = 31u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_DIRECTORY: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_DISCONNECT: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_DONOTDISTURB: u32 = 26u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_DROP: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_FLASH: u32 = 23u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_FORWARD: u32 = 12u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_HOLD: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_INTERCOM: u32 = 27u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_LASTNUM: u32 = 35u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_MSGINDICATOR: u32 = 38u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_MSGWAITOFF: u32 = 9u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_MSGWAITON: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_MUTE: u32 = 18u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_NIGHTSRV: u32 = 36u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_NONE: u32 = 46u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_PARK: u32 = 15u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_PICKUP: u32 = 13u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_QUEUECALL: u32 = 45u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_RECALL: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_REDIRECT: u32 = 17u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_REJECT: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_REPDIAL: u32 = 39u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_RINGAGAIN: u32 = 14u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_SAVEREPEAT: u32 = 44u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_SELECTRING: u32 = 10u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_SEND: u32 = 47u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_SENDCALLS: u32 = 37u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_SETREPDIAL: u32 = 40u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_SPEAKEROFF: u32 = 22u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_SPEAKERON: u32 = 21u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_STATIONSPEED: u32 = 42u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_SYSTEMSPEED: u32 = 41u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_TRANSFER: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_VOLUMEDOWN: u32 = 20u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONFUNCTION_VOLUMEUP: u32 = 19u32;
pub struct PHONEBUTTONINFO(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONMODE_CALL: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONMODE_DISPLAY: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONMODE_DUMMY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONMODE_FEATURE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONMODE_KEYPAD: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONMODE_LOCAL: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONSTATE_DOWN: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONSTATE_UNAVAIL: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONSTATE_UNKNOWN: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEBUTTONSTATE_UP: u32 = 1u32;
pub struct PHONECALLBACK(i32);
pub struct PHONECAPS(i32);
pub struct PHONECAPS_BUFFER(i32);
pub struct PHONECAPS_LONG(i32);
pub struct PHONECAPS_STRING(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_ALLOCATED: u32 = 2415919105u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_BADDEVICEID: u32 = 2415919106u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_DISCONNECTED: u32 = 2415919140u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INCOMPATIBLEAPIVERSION: u32 = 2415919107u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INCOMPATIBLEEXTVERSION: u32 = 2415919108u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INIFILECORRUPT: u32 = 2415919109u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INUSE: u32 = 2415919110u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALAPPHANDLE: u32 = 2415919111u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALAPPNAME: u32 = 2415919112u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALBUTTONLAMPID: u32 = 2415919113u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALBUTTONMODE: u32 = 2415919114u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALBUTTONSTATE: u32 = 2415919115u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALDATAID: u32 = 2415919116u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALDEVICECLASS: u32 = 2415919117u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALEXTVERSION: u32 = 2415919118u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALHOOKSWITCHDEV: u32 = 2415919119u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALHOOKSWITCHMODE: u32 = 2415919120u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALLAMPMODE: u32 = 2415919121u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALPARAM: u32 = 2415919122u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALPHONEHANDLE: u32 = 2415919123u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALPHONESTATE: u32 = 2415919124u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALPOINTER: u32 = 2415919125u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALPRIVILEGE: u32 = 2415919126u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_INVALRINGMODE: u32 = 2415919127u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_NODEVICE: u32 = 2415919128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_NODRIVER: u32 = 2415919129u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_NOMEM: u32 = 2415919130u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_NOTOWNER: u32 = 2415919131u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_OPERATIONFAILED: u32 = 2415919132u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_OPERATIONUNAVAIL: u32 = 2415919133u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_REINIT: u32 = 2415919139u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_REQUESTOVERRUN: u32 = 2415919136u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_RESOURCEUNAVAIL: u32 = 2415919135u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_SERVICE_NOT_RUNNING: u32 = 2415919141u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_STRUCTURETOOSMALL: u32 = 2415919137u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEERR_UNINITIALIZED: u32 = 2415919138u32;
pub struct PHONEEVENT(i32);
pub struct PHONEEXTENSIONID(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GENERICPHONE: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETBUTTONINFO: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETDATA: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETDISPLAY: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETGAINHANDSET: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETGAINHEADSET: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETGAINSPEAKER: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETHOOKSWITCHHANDSET: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETHOOKSWITCHHEADSET: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETHOOKSWITCHSPEAKER: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETLAMP: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETRING: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETVOLUMEHANDSET: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETVOLUMEHEADSET: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_GETVOLUMESPEAKER: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETBUTTONINFO: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETDATA: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETDISPLAY: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETGAINHANDSET: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETGAINHEADSET: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETGAINSPEAKER: u32 = 262144u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETHOOKSWITCHHANDSET: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETHOOKSWITCHHEADSET: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETHOOKSWITCHSPEAKER: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETLAMP: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETRING: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETVOLUMEHANDSET: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETVOLUMEHEADSET: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEFEATURE_SETVOLUMESPEAKER: u32 = 67108864u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEHOOKSWITCHDEV_HANDSET: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEHOOKSWITCHDEV_HEADSET: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEHOOKSWITCHDEV_SPEAKER: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEHOOKSWITCHMODE_MIC: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEHOOKSWITCHMODE_MICSPEAKER: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEHOOKSWITCHMODE_ONHOOK: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEHOOKSWITCHMODE_SPEAKER: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEHOOKSWITCHMODE_UNKNOWN: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEINITIALIZEEXOPTION_USECOMPLETIONPORT: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEINITIALIZEEXOPTION_USEEVENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEINITIALIZEEXOPTION_USEHIDDENWINDOW: u32 = 1u32;
pub struct PHONEINITIALIZEEXPARAMS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONELAMPMODE_BROKENFLUTTER: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONELAMPMODE_DUMMY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONELAMPMODE_FLASH: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONELAMPMODE_FLUTTER: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONELAMPMODE_OFF: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONELAMPMODE_STEADY: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONELAMPMODE_UNKNOWN: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONELAMPMODE_WINK: u32 = 8u32;
pub struct PHONEMESSAGE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEPRIVILEGE_MONITOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONEPRIVILEGE_OWNER: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_CAPSCHANGE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_CONNECTED: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_DEVSPECIFIC: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_DISCONNECTED: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_DISPLAY: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_HANDSETGAIN: u32 = 2048u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_HANDSETHOOKSWITCH: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_HANDSETVOLUME: u32 = 1024u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_HEADSETGAIN: u32 = 131072u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_HEADSETHOOKSWITCH: u32 = 32768u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_HEADSETVOLUME: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_LAMP: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_MONITORS: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_OTHER: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_OWNER: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_REINIT: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_REMOVED: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_RESUME: u32 = 524288u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_RINGMODE: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_RINGVOLUME: u32 = 256u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_SPEAKERGAIN: u32 = 16384u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_SPEAKERHOOKSWITCH: u32 = 4096u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_SPEAKERVOLUME: u32 = 8192u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATE_SUSPEND: u32 = 262144u32;
pub struct PHONESTATUS(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATUSFLAGS_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONESTATUSFLAGS_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONE_BUTTON: i32 = 14i32;
pub struct PHONE_BUTTON_FUNCTION(i32);
pub struct PHONE_BUTTON_MODE(i32);
pub struct PHONE_BUTTON_STATE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONE_CLOSE: i32 = 15i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONE_CREATE: i32 = 20i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONE_DEVSPECIFIC: i32 = 16i32;
pub struct PHONE_EVENT(i32);
pub struct PHONE_HOOK_SWITCH_DEVICE(i32);
pub struct PHONE_HOOK_SWITCH_STATE(i32);
pub struct PHONE_LAMP_MODE(i32);
pub struct PHONE_PRIVILEGE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONE_REMOVE: i32 = 26i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONE_REPLY: i32 = 17i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PHONE_STATE: i32 = 18i32;
pub struct PHONE_TONE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PRIVATEOBJECT_ADDRESS: u32 = 6u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PRIVATEOBJECT_CALL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PRIVATEOBJECT_CALLID: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PRIVATEOBJECT_LINE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PRIVATEOBJECT_NONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const PRIVATEOBJECT_PHONE: u32 = 5u32;
pub struct QOS_EVENT(i32);
pub struct QOS_SERVICE_LEVEL(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const RENDBIND_AUTHENTICATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const RENDBIND_DEFAULTCREDENTIALS: u32 = 14u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const RENDBIND_DEFAULTDOMAINNAME: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const RENDBIND_DEFAULTPASSWORD: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const RENDBIND_DEFAULTUSERNAME: u32 = 4u32;
pub struct RND_ADVERTISING_SCOPE(i32);
pub struct Rendezvous(i32);
pub struct RequestMakeCall(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRINGFORMAT_ASCII: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRINGFORMAT_BINARY: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRINGFORMAT_DBCS: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRINGFORMAT_UNICODE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRM_CONFIGURED: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRM_INITIAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRM_PAUSED: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRM_RUNNING: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRM_STOPPED: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const STRM_TERMINALSELECTED: u32 = 1u32;
pub struct STnefProblem(i32);
pub struct STnefProblemArray(i32);
pub struct TAPI(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_CONNECTED: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_DESTBUSY: i32 = -11i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_DESTNOANSWER: i32 = -12i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_DESTUNAVAIL: i32 = -13i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_DEVICECLASSUNAVAIL: i32 = -8i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_DEVICEIDUNAVAIL: i32 = -9i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_DEVICEINUSE: i32 = -10i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_DROPPED: i32 = -1i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_INVALDESTADDRESS: i32 = -4i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_INVALDEVICECLASS: i32 = -6i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_INVALDEVICEID: i32 = -7i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_INVALPOINTER: i32 = -18i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_INVALWINDOWHANDLE: i32 = -5i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_MMCWRITELOCKED: i32 = -20i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_NOREQUESTRECIPIENT: i32 = -2i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_NOTADMIN: i32 = -19i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_PROVIDERALREADYINSTALLED: i32 = -21i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_REQUESTCANCELLED: i32 = -17i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_REQUESTFAILED: i32 = -16i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_REQUESTQUEUEFULL: i32 = -3i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_SCP_ALREADY_EXISTS: i32 = -22i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_SCP_DOES_NOT_EXIST: i32 = -23i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_UNKNOWNREQUESTID: i32 = -15i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIERR_UNKNOWNWINHANDLE: i32 = -14i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMAXAPPNAMESIZE: i32 = 40i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMAXCALLEDPARTYSIZE: i32 = 40i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMAXCOMMENTSIZE: i32 = 80i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMAXDESTADDRESSSIZE: i32 = 80i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMAXDEVICECLASSSIZE: i32 = 40i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMAXDEVICEIDSIZE: i32 = 40i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMEDIATYPE_AUDIO: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMEDIATYPE_DATAMODEM: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMEDIATYPE_G3FAX: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMEDIATYPE_MULTITRACK: u32 = 65536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPIMEDIATYPE_VIDEO: u32 = 32768u32;
pub struct TAPIOBJECT_EVENT(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_CURRENT_VERSION: u32 = 131074u32;
pub struct TAPI_CUSTOMTONE(i32);
pub struct TAPI_DETECTTONE(i32);
pub struct TAPI_EVENT(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_ADDRESSBLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221462i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_ALLOCATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221498i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_BILLINGREJECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221461i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLCENTER_GROUP_REMOVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221435i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTACTIVITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221428i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTGROUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221431i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221432i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLCENTER_INVALAGENTSTATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221429i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLCENTER_INVALPASSWORD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221430i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLCENTER_NO_AGENT_ID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221433i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLCENTER_QUEUE_REMOVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221434i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLNOTSELECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221420i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CALLUNAVAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221497i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_COMPLETIONOVERRUN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221496i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_CONFERENCEFULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221495i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_DESTBUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221452i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_DESTNOANSWER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221451i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_DESTUNAVAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221450i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_DIALMODIFIERNOTSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221494i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_DROPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221455i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INUSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221493i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221492i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALADDRESSSTATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221491i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALADDRESSTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221423i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALBUTTONLAMPID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221459i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALBUTTONSTATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221458i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALCALLPARAMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221490i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALCALLPRIVILEGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221489i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALCALLSTATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221488i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALCARD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221487i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALCOMPLETIONID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221486i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALCOUNTRYCODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221485i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALDATAID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221457i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALDEVICECLASS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221484i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALDIALPARAMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221483i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALDIGITS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221482i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALFEATURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221460i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALGROUPID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221481i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALHOOKSWITCHDEV: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221456i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALIDDIRECTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221446i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALIDMEDIATYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221500i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALIDSTREAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221437i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALIDSTREAMSTATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221417i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALIDTERMINAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221445i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALIDTERMINALCLASS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221444i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALLIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221474i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALLOCATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221480i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALMESSAGEID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221479i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALMODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221473i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALPARKID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221478i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALPRIVILEGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221447i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALRATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221477i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALTIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221476i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_INVALTONE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221475i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_MAXSTREAMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221442i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_MAXTERMINALS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221438i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOCONFERENCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221472i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NODEVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221471i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NODRIVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221443i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOEVENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221424i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOFORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221418i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOITEMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221502i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOREQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221470i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOREQUESTRECIPIENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221454i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOTENOUGHMEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221503i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOTERMINALSELECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221441i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOTOWNER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221469i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOTREGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221468i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOTSTOPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221439i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOTSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221501i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221415i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_OPERATIONFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221499i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_PEER_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221425i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_PHONENOTOPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221421i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_REGISTRY_SETTING_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221427i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_REINIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221463i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_REQUESTCANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221448i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_REQUESTFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221449i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_REQUESTOVERRUN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221467i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_REQUESTQUEUEFULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221453i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_RESOURCEUNAVAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221422i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_SERVICE_NOT_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221414i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_TARGETNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221466i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_TARGETSELF: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221465i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_TERMINALINUSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221440i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_TERMINAL_PEER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221426i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221436i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_USERUSERINFOTOOBIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221464i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_WRONGEVENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221419i32 as _);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_E_WRONG_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221416i32 as _);
pub struct TAPI_GATHERTERM(i32);
pub struct TAPI_OBJECT_TYPE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TAPI_REPLY: u32 = 1123u32;
pub struct TAPI_TONEMODE(i32);
pub struct TERMINAL_DIRECTION(i32);
pub struct TERMINAL_MEDIA_STATE(i32);
pub struct TERMINAL_STATE(i32);
pub struct TERMINAL_TYPE(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEACCEPT: u32 = 500u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEADDTOCONFERENCE: u32 = 501u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEANSWER: u32 = 502u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEBLINDTRANSFER: u32 = 503u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINECLOSE: u32 = 504u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINECLOSECALL: u32 = 505u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINECLOSEMSPINSTANCE: u32 = 609u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINECOMPLETECALL: u32 = 506u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINECOMPLETETRANSFER: u32 = 507u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINECONDITIONALMEDIADETECTION: u32 = 508u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINECONFIGDIALOG: u32 = 509u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINECONFIGDIALOGEDIT: u32 = 601u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINECREATEMSPINSTANCE: u32 = 608u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEDEVSPECIFIC: u32 = 510u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEDEVSPECIFICFEATURE: u32 = 511u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEDIAL: u32 = 512u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEDROP: u32 = 513u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEDROPNOOWNER: u32 = 597u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEDROPONCLOSE: u32 = 596u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEFORWARD: u32 = 514u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGATHERDIGITS: u32 = 515u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGENERATEDIGITS: u32 = 516u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGENERATETONE: u32 = 517u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETADDRESSCAPS: u32 = 518u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETADDRESSID: u32 = 519u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETADDRESSSTATUS: u32 = 520u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETCALLADDRESSID: u32 = 521u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETCALLHUBTRACKING: u32 = 604u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETCALLID: u32 = 603u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETCALLINFO: u32 = 522u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETCALLSTATUS: u32 = 523u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETDEVCAPS: u32 = 524u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETDEVCONFIG: u32 = 525u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETEXTENSIONID: u32 = 526u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETICON: u32 = 527u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETID: u32 = 528u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETLINEDEVSTATUS: u32 = 529u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEGETNUMADDRESSIDS: u32 = 530u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEHOLD: u32 = 531u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEMAKECALL: u32 = 532u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEMONITORDIGITS: u32 = 533u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEMONITORMEDIA: u32 = 534u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEMONITORTONES: u32 = 535u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEMSPIDENTIFY: u32 = 607u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINENEGOTIATEEXTVERSION: u32 = 536u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINENEGOTIATETSPIVERSION: u32 = 537u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEOPEN: u32 = 538u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEPARK: u32 = 539u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEPICKUP: u32 = 540u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEPREPAREADDTOCONFERENCE: u32 = 541u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINERECEIVEMSPDATA: u32 = 606u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEREDIRECT: u32 = 542u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINERELEASEUSERUSERINFO: u32 = 602u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEREMOVEFROMCONFERENCE: u32 = 543u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESECURECALL: u32 = 544u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESELECTEXTVERSION: u32 = 545u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESENDUSERUSERINFO: u32 = 546u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETAPPSPECIFIC: u32 = 547u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETCALLHUBTRACKING: u32 = 605u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETCALLPARAMS: u32 = 548u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETCURRENTLOCATION: u32 = 600u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETDEFAULTMEDIADETECTION: u32 = 549u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETDEVCONFIG: u32 = 550u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETMEDIACONTROL: u32 = 551u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETMEDIAMODE: u32 = 552u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETSTATUSMESSAGES: u32 = 553u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETTERMINAL: u32 = 554u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETUPCONFERENCE: u32 = 555u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESETUPTRANSFER: u32 = 556u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINESWAPHOLD: u32 = 557u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEUNCOMPLETECALL: u32 = 558u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEUNHOLD: u32 = 559u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_LINEUNPARK: u32 = 560u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_MESSAGE_BASE: u32 = 500u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONECLOSE: u32 = 561u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONECONFIGDIALOG: u32 = 562u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEDEVSPECIFIC: u32 = 563u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETBUTTONINFO: u32 = 564u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETDATA: u32 = 565u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETDEVCAPS: u32 = 566u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETDISPLAY: u32 = 567u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETEXTENSIONID: u32 = 568u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETGAIN: u32 = 569u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETHOOKSWITCH: u32 = 570u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETICON: u32 = 571u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETID: u32 = 572u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETLAMP: u32 = 573u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETRING: u32 = 574u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETSTATUS: u32 = 575u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEGETVOLUME: u32 = 576u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONENEGOTIATEEXTVERSION: u32 = 577u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONENEGOTIATETSPIVERSION: u32 = 578u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONEOPEN: u32 = 579u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESELECTEXTVERSION: u32 = 580u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESETBUTTONINFO: u32 = 581u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESETDATA: u32 = 582u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESETDISPLAY: u32 = 583u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESETGAIN: u32 = 584u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESETHOOKSWITCH: u32 = 585u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESETLAMP: u32 = 586u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESETRING: u32 = 587u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESETSTATUSMESSAGES: u32 = 588u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PHONESETVOLUME: u32 = 589u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PROC_BASE: u32 = 500u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PROVIDERCONFIG: u32 = 590u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PROVIDERCREATELINEDEVICE: u32 = 598u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PROVIDERCREATEPHONEDEVICE: u32 = 599u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PROVIDERENUMDEVICES: u32 = 595u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PROVIDERINIT: u32 = 591u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PROVIDERINSTALL: u32 = 592u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PROVIDERREMOVE: u32 = 593u32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TSPI_PROVIDERSHUTDOWN: u32 = 594u32;
pub struct TUISPICREATEDIALOGINSTANCEPARAMS(i32);
pub struct TUISPIDLLCALLBACK(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TUISPIDLL_OBJECT_DIALOGINSTANCE: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TUISPIDLL_OBJECT_LINEID: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TUISPIDLL_OBJECT_PHONEID: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const TUISPIDLL_OBJECT_PROVIDERID: i32 = 3i32;
pub struct VARSTRING(i32);
pub struct _ADDR_ALIAS(i32);
pub struct _dtr(i32);
pub struct _renddata(i32);
pub struct _trp(i32);
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const atypFile: i32 = 1i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const atypMax: i32 = 4i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const atypNull: i32 = 0i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const atypOle: i32 = 2i32;
#[doc = "*Required features: `Win32_Devices_Tapi`*"]
pub const atypPicture: i32 = 3i32;
pub struct linereqmakecallW_tag(i32);
pub struct linereqmediacallW_tag(i32);
