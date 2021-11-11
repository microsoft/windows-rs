#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn GetTnefStreamCodepage(lpstream: ::windows::runtime::RawPtr, lpulcodepage: *mut u32, lpulsubcodepage: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub fn OpenTnefStream(lpvsupport: *mut ::core::ffi::c_void, lpstream: ::windows::runtime::RawPtr, lpszstreamname: *const i8, ulflags: u32, lpmessage: ::windows::runtime::RawPtr, wkeyval: u16, lpptnef: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub fn OpenTnefStreamEx(lpvsupport: *mut ::core::ffi::c_void, lpstream: ::windows::runtime::RawPtr, lpszstreamname: *const i8, ulflags: u32, lpmessage: ::windows::runtime::RawPtr, wkeyval: u16, lpadressbook: ::windows::runtime::RawPtr, lpptnef: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
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
    pub fn lineCreateAgentSessionA(hline: u32, hagent: u32, lpszagentpin: super::super::Foundation::PSTR, dwworkingaddressid: u32, lpgroupid: *mut ::windows::runtime::GUID, lphagentsession: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineCreateAgentSessionW(hline: u32, hagent: u32, lpszagentpin: super::super::Foundation::PWSTR, dwworkingaddressid: u32, lpgroupid: *mut ::windows::runtime::GUID, lphagentsession: *mut u32) -> i32;
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
    pub fn lineGetQueueListA(hline: u32, lpgroupid: *mut ::windows::runtime::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetQueueListW(hline: u32, lpgroupid: *mut ::windows::runtime::GUID, lpqueuelist: *mut LINEQUEUELIST) -> i32;
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
    pub fn lineInitialize(lphlineapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: ::windows::runtime::RawPtr, lpszappname: super::super::Foundation::PSTR, lpdwnumdevs: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitializeExA(lphlineapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: ::windows::runtime::RawPtr, lpszfriendlyappname: super::super::Foundation::PSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitializeExW(lphlineapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: ::windows::runtime::RawPtr, lpszfriendlyappname: super::super::Foundation::PWSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lplineinitializeexparams: *mut LINEINITIALIZEEXPARAMS) -> i32;
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
    pub fn phoneInitialize(lphphoneapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: ::windows::runtime::RawPtr, lpszappname: super::super::Foundation::PSTR, lpdwnumdevs: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitializeExA(lphphoneapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: ::windows::runtime::RawPtr, lpszfriendlyappname: super::super::Foundation::PSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32;
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitializeExW(lphphoneapp: *mut u32, hinstance: super::super::Foundation::HINSTANCE, lpfncallback: ::windows::runtime::RawPtr, lpszfriendlyappname: super::super::Foundation::PWSTR, lpdwnumdevs: *mut u32, lpdwapiversion: *mut u32, lpphoneinitializeexparams: *mut PHONEINITIALIZEEXPARAMS) -> i32;
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
