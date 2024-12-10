#[inline]
pub unsafe fn SnmpCancelMsg(session: isize, reqid: i32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCancelMsg(session : isize, reqid : i32) -> u32);
    SnmpCancelMsg(core::mem::transmute(session), core::mem::transmute(reqid))
}
#[inline]
pub unsafe fn SnmpCleanup() -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCleanup() -> u32);
    SnmpCleanup()
}
#[inline]
pub unsafe fn SnmpCleanupEx() -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCleanupEx() -> u32);
    SnmpCleanupEx()
}
#[inline]
pub unsafe fn SnmpClose(session: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpClose(session : isize) -> u32);
    SnmpClose(core::mem::transmute(session))
}
#[inline]
pub unsafe fn SnmpContextToStr(context: isize, string: *mut smiOCTETS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpContextToStr(context : isize, string : *mut smiOCTETS) -> u32);
    SnmpContextToStr(core::mem::transmute(context), core::mem::transmute(string))
}
#[inline]
pub unsafe fn SnmpCountVbl(vbl: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCountVbl(vbl : isize) -> u32);
    SnmpCountVbl(core::mem::transmute(vbl))
}
#[inline]
pub unsafe fn SnmpCreatePdu(session: isize, pdu_type: SNMP_PDU_TYPE, request_id: i32, error_status: i32, error_index: i32, varbindlist: isize) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCreatePdu(session : isize, pdu_type : SNMP_PDU_TYPE, request_id : i32, error_status : i32, error_index : i32, varbindlist : isize) -> isize);
    SnmpCreatePdu(core::mem::transmute(session), core::mem::transmute(pdu_type), core::mem::transmute(request_id), core::mem::transmute(error_status), core::mem::transmute(error_index), core::mem::transmute(varbindlist))
}
#[inline]
pub unsafe fn SnmpCreateSession(hwnd: super::super::Foundation::HWND, wmsg: u32, fcallback: SNMPAPI_CALLBACK, lpclientdata: *mut core::ffi::c_void) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCreateSession(hwnd : super::super::Foundation:: HWND, wmsg : u32, fcallback : SNMPAPI_CALLBACK, lpclientdata : *mut core::ffi::c_void) -> isize);
    SnmpCreateSession(core::mem::transmute(hwnd), core::mem::transmute(wmsg), core::mem::transmute(fcallback), core::mem::transmute(lpclientdata))
}
#[inline]
pub unsafe fn SnmpCreateVbl(session: isize, name: *mut smiOID, value: *mut smiVALUE) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpCreateVbl(session : isize, name : *mut smiOID, value : *mut smiVALUE) -> isize);
    SnmpCreateVbl(core::mem::transmute(session), core::mem::transmute(name), core::mem::transmute(value))
}
#[inline]
pub unsafe fn SnmpDecodeMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize, msgbufdesc: *mut smiOCTETS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpDecodeMsg(session : isize, srcentity : *mut isize, dstentity : *mut isize, context : *mut isize, pdu : *mut isize, msgbufdesc : *mut smiOCTETS) -> u32);
    SnmpDecodeMsg(core::mem::transmute(session), core::mem::transmute(srcentity), core::mem::transmute(dstentity), core::mem::transmute(context), core::mem::transmute(pdu), core::mem::transmute(msgbufdesc))
}
#[inline]
pub unsafe fn SnmpDeleteVb(vbl: isize, index: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpDeleteVb(vbl : isize, index : u32) -> u32);
    SnmpDeleteVb(core::mem::transmute(vbl), core::mem::transmute(index))
}
#[inline]
pub unsafe fn SnmpDuplicatePdu(session: isize, pdu: isize) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpDuplicatePdu(session : isize, pdu : isize) -> isize);
    SnmpDuplicatePdu(core::mem::transmute(session), core::mem::transmute(pdu))
}
#[inline]
pub unsafe fn SnmpDuplicateVbl(session: isize, vbl: isize) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpDuplicateVbl(session : isize, vbl : isize) -> isize);
    SnmpDuplicateVbl(core::mem::transmute(session), core::mem::transmute(vbl))
}
#[inline]
pub unsafe fn SnmpEncodeMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize, msgbufdesc: *mut smiOCTETS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpEncodeMsg(session : isize, srcentity : isize, dstentity : isize, context : isize, pdu : isize, msgbufdesc : *mut smiOCTETS) -> u32);
    SnmpEncodeMsg(core::mem::transmute(session), core::mem::transmute(srcentity), core::mem::transmute(dstentity), core::mem::transmute(context), core::mem::transmute(pdu), core::mem::transmute(msgbufdesc))
}
#[inline]
pub unsafe fn SnmpEntityToStr(entity: isize, string: &mut [u8]) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpEntityToStr(entity : isize, size : u32, string : windows_core::PSTR) -> u32);
    SnmpEntityToStr(core::mem::transmute(entity), string.len().try_into().unwrap(), core::mem::transmute(string.as_ptr()))
}
#[inline]
pub unsafe fn SnmpFreeContext(context: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreeContext(context : isize) -> u32);
    SnmpFreeContext(core::mem::transmute(context))
}
#[inline]
pub unsafe fn SnmpFreeDescriptor(syntax: u32, descriptor: *mut smiOCTETS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreeDescriptor(syntax : u32, descriptor : *mut smiOCTETS) -> u32);
    SnmpFreeDescriptor(core::mem::transmute(syntax), core::mem::transmute(descriptor))
}
#[inline]
pub unsafe fn SnmpFreeEntity(entity: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreeEntity(entity : isize) -> u32);
    SnmpFreeEntity(core::mem::transmute(entity))
}
#[inline]
pub unsafe fn SnmpFreePdu(pdu: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreePdu(pdu : isize) -> u32);
    SnmpFreePdu(core::mem::transmute(pdu))
}
#[inline]
pub unsafe fn SnmpFreeVbl(vbl: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpFreeVbl(vbl : isize) -> u32);
    SnmpFreeVbl(core::mem::transmute(vbl))
}
#[inline]
pub unsafe fn SnmpGetLastError(session: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetLastError(session : isize) -> u32);
    SnmpGetLastError(core::mem::transmute(session))
}
#[inline]
pub unsafe fn SnmpGetPduData(pdu: isize, pdu_type: *mut SNMP_PDU_TYPE, request_id: *mut i32, error_status: *mut SNMP_ERROR, error_index: *mut i32, varbindlist: *mut isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetPduData(pdu : isize, pdu_type : *mut SNMP_PDU_TYPE, request_id : *mut i32, error_status : *mut SNMP_ERROR, error_index : *mut i32, varbindlist : *mut isize) -> u32);
    SnmpGetPduData(core::mem::transmute(pdu), core::mem::transmute(pdu_type), core::mem::transmute(request_id), core::mem::transmute(error_status), core::mem::transmute(error_index), core::mem::transmute(varbindlist))
}
#[inline]
pub unsafe fn SnmpGetRetransmitMode(nretransmitmode: *mut SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetRetransmitMode(nretransmitmode : *mut SNMP_STATUS) -> u32);
    SnmpGetRetransmitMode(core::mem::transmute(nretransmitmode))
}
#[inline]
pub unsafe fn SnmpGetRetry(hentity: isize, npolicyretry: *mut u32, nactualretry: *mut u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetRetry(hentity : isize, npolicyretry : *mut u32, nactualretry : *mut u32) -> u32);
    SnmpGetRetry(core::mem::transmute(hentity), core::mem::transmute(npolicyretry), core::mem::transmute(nactualretry))
}
#[inline]
pub unsafe fn SnmpGetTimeout(hentity: isize, npolicytimeout: *mut u32, nactualtimeout: *mut u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetTimeout(hentity : isize, npolicytimeout : *mut u32, nactualtimeout : *mut u32) -> u32);
    SnmpGetTimeout(core::mem::transmute(hentity), core::mem::transmute(npolicytimeout), core::mem::transmute(nactualtimeout))
}
#[inline]
pub unsafe fn SnmpGetTranslateMode(ntranslatemode: *mut SNMP_API_TRANSLATE_MODE) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetTranslateMode(ntranslatemode : *mut SNMP_API_TRANSLATE_MODE) -> u32);
    SnmpGetTranslateMode(core::mem::transmute(ntranslatemode))
}
#[inline]
pub unsafe fn SnmpGetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetVb(vbl : isize, index : u32, name : *mut smiOID, value : *mut smiVALUE) -> u32);
    SnmpGetVb(core::mem::transmute(vbl), core::mem::transmute(index), core::mem::transmute(name), core::mem::transmute(value))
}
#[inline]
pub unsafe fn SnmpGetVendorInfo(vendorinfo: *mut smiVENDORINFO) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpGetVendorInfo(vendorinfo : *mut smiVENDORINFO) -> u32);
    SnmpGetVendorInfo(core::mem::transmute(vendorinfo))
}
#[inline]
pub unsafe fn SnmpListen(hentity: isize, lstatus: SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpListen(hentity : isize, lstatus : SNMP_STATUS) -> u32);
    SnmpListen(core::mem::transmute(hentity), core::mem::transmute(lstatus))
}
#[inline]
pub unsafe fn SnmpListenEx(hentity: isize, lstatus: u32, nuseentityaddr: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpListenEx(hentity : isize, lstatus : u32, nuseentityaddr : u32) -> u32);
    SnmpListenEx(core::mem::transmute(hentity), core::mem::transmute(lstatus), core::mem::transmute(nuseentityaddr))
}
#[inline]
pub unsafe fn SnmpMgrClose(session: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrClose(session : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SnmpMgrClose(core::mem::transmute(session))
}
#[inline]
pub unsafe fn SnmpMgrCtl(session: *mut core::ffi::c_void, dwctlcode: u32, lpvinbuffer: *mut core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrCtl(session : *mut core::ffi::c_void, dwctlcode : u32, lpvinbuffer : *mut core::ffi::c_void, cbinbuffer : u32, lpvoutbuffer : *mut core::ffi::c_void, cboutbuffer : u32, lpcbbytesreturned : *mut u32) -> super::super::Foundation:: BOOL);
    SnmpMgrCtl(core::mem::transmute(session), core::mem::transmute(dwctlcode), core::mem::transmute(lpvinbuffer), core::mem::transmute(cbinbuffer), core::mem::transmute(lpvoutbuffer), core::mem::transmute(cboutbuffer), core::mem::transmute(lpcbbytesreturned)).ok()
}
#[inline]
pub unsafe fn SnmpMgrGetTrap(enterprise: *mut AsnObjectIdentifier, ipaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrGetTrap(enterprise : *mut AsnObjectIdentifier, ipaddress : *mut AsnOctetString, generictrap : *mut SNMP_GENERICTRAP, specifictrap : *mut i32, timestamp : *mut u32, variablebindings : *mut SnmpVarBindList) -> super::super::Foundation:: BOOL);
    SnmpMgrGetTrap(core::mem::transmute(enterprise), core::mem::transmute(ipaddress), core::mem::transmute(generictrap), core::mem::transmute(specifictrap), core::mem::transmute(timestamp), core::mem::transmute(variablebindings))
}
#[inline]
pub unsafe fn SnmpMgrGetTrapEx(enterprise: *mut AsnObjectIdentifier, agentaddress: *mut AsnOctetString, sourceaddress: *mut AsnOctetString, generictrap: *mut SNMP_GENERICTRAP, specifictrap: *mut i32, community: *mut AsnOctetString, timestamp: *mut u32, variablebindings: *mut SnmpVarBindList) -> super::super::Foundation::BOOL {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrGetTrapEx(enterprise : *mut AsnObjectIdentifier, agentaddress : *mut AsnOctetString, sourceaddress : *mut AsnOctetString, generictrap : *mut SNMP_GENERICTRAP, specifictrap : *mut i32, community : *mut AsnOctetString, timestamp : *mut u32, variablebindings : *mut SnmpVarBindList) -> super::super::Foundation:: BOOL);
    SnmpMgrGetTrapEx(core::mem::transmute(enterprise), core::mem::transmute(agentaddress), core::mem::transmute(sourceaddress), core::mem::transmute(generictrap), core::mem::transmute(specifictrap), core::mem::transmute(community), core::mem::transmute(timestamp), core::mem::transmute(variablebindings))
}
#[inline]
pub unsafe fn SnmpMgrOidToStr(oid: *mut AsnObjectIdentifier, string: Option<*mut windows_core::PSTR>) -> super::super::Foundation::BOOL {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrOidToStr(oid : *mut AsnObjectIdentifier, string : *mut windows_core::PSTR) -> super::super::Foundation:: BOOL);
    SnmpMgrOidToStr(core::mem::transmute(oid), core::mem::transmute(string.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn SnmpMgrOpen<P0, P1>(lpagentaddress: P0, lpagentcommunity: P1, ntimeout: i32, nretries: i32) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrOpen(lpagentaddress : windows_core::PCSTR, lpagentcommunity : windows_core::PCSTR, ntimeout : i32, nretries : i32) -> *mut core::ffi::c_void);
    SnmpMgrOpen(lpagentaddress.param().abi(), lpagentcommunity.param().abi(), core::mem::transmute(ntimeout), core::mem::transmute(nretries))
}
#[inline]
pub unsafe fn SnmpMgrRequest(session: *mut core::ffi::c_void, requesttype: u8, variablebindings: *mut SnmpVarBindList, errorstatus: *mut SNMP_ERROR_STATUS, errorindex: *mut i32) -> i32 {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrRequest(session : *mut core::ffi::c_void, requesttype : u8, variablebindings : *mut SnmpVarBindList, errorstatus : *mut SNMP_ERROR_STATUS, errorindex : *mut i32) -> i32);
    SnmpMgrRequest(core::mem::transmute(session), core::mem::transmute(requesttype), core::mem::transmute(variablebindings), core::mem::transmute(errorstatus), core::mem::transmute(errorindex))
}
#[inline]
pub unsafe fn SnmpMgrStrToOid<P0>(string: P0, oid: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrStrToOid(string : windows_core::PCSTR, oid : *mut AsnObjectIdentifier) -> super::super::Foundation:: BOOL);
    SnmpMgrStrToOid(string.param().abi(), core::mem::transmute(oid))
}
#[inline]
pub unsafe fn SnmpMgrTrapListen(phtrapavailable: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("mgmtapi.dll" "system" fn SnmpMgrTrapListen(phtrapavailable : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SnmpMgrTrapListen(core::mem::transmute(phtrapavailable)).ok()
}
#[inline]
pub unsafe fn SnmpOidCompare(xoid: *mut smiOID, yoid: *mut smiOID, maxlen: u32, result: *mut i32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpOidCompare(xoid : *mut smiOID, yoid : *mut smiOID, maxlen : u32, result : *mut i32) -> u32);
    SnmpOidCompare(core::mem::transmute(xoid), core::mem::transmute(yoid), core::mem::transmute(maxlen), core::mem::transmute(result))
}
#[inline]
pub unsafe fn SnmpOidCopy(srcoid: *mut smiOID, dstoid: *mut smiOID) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpOidCopy(srcoid : *mut smiOID, dstoid : *mut smiOID) -> u32);
    SnmpOidCopy(core::mem::transmute(srcoid), core::mem::transmute(dstoid))
}
#[inline]
pub unsafe fn SnmpOidToStr(srcoid: *const smiOID, string: &mut [u8]) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpOidToStr(srcoid : *const smiOID, size : u32, string : windows_core::PSTR) -> u32);
    SnmpOidToStr(core::mem::transmute(srcoid), string.len().try_into().unwrap(), core::mem::transmute(string.as_ptr()))
}
#[inline]
pub unsafe fn SnmpOpen(hwnd: super::super::Foundation::HWND, wmsg: u32) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpOpen(hwnd : super::super::Foundation:: HWND, wmsg : u32) -> isize);
    SnmpOpen(core::mem::transmute(hwnd), core::mem::transmute(wmsg))
}
#[inline]
pub unsafe fn SnmpRecvMsg(session: isize, srcentity: *mut isize, dstentity: *mut isize, context: *mut isize, pdu: *mut isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpRecvMsg(session : isize, srcentity : *mut isize, dstentity : *mut isize, context : *mut isize, pdu : *mut isize) -> u32);
    SnmpRecvMsg(core::mem::transmute(session), core::mem::transmute(srcentity), core::mem::transmute(dstentity), core::mem::transmute(context), core::mem::transmute(pdu))
}
#[inline]
pub unsafe fn SnmpRegister(session: isize, srcentity: isize, dstentity: isize, context: isize, notification: *mut smiOID, state: SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpRegister(session : isize, srcentity : isize, dstentity : isize, context : isize, notification : *mut smiOID, state : SNMP_STATUS) -> u32);
    SnmpRegister(core::mem::transmute(session), core::mem::transmute(srcentity), core::mem::transmute(dstentity), core::mem::transmute(context), core::mem::transmute(notification), core::mem::transmute(state))
}
#[inline]
pub unsafe fn SnmpSendMsg(session: isize, srcentity: isize, dstentity: isize, context: isize, pdu: isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSendMsg(session : isize, srcentity : isize, dstentity : isize, context : isize, pdu : isize) -> u32);
    SnmpSendMsg(core::mem::transmute(session), core::mem::transmute(srcentity), core::mem::transmute(dstentity), core::mem::transmute(context), core::mem::transmute(pdu))
}
#[inline]
pub unsafe fn SnmpSetPduData(pdu: isize, pdu_type: *const i32, request_id: *const i32, non_repeaters: *const i32, max_repetitions: *const i32, varbindlist: *const isize) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetPduData(pdu : isize, pdu_type : *const i32, request_id : *const i32, non_repeaters : *const i32, max_repetitions : *const i32, varbindlist : *const isize) -> u32);
    SnmpSetPduData(core::mem::transmute(pdu), core::mem::transmute(pdu_type), core::mem::transmute(request_id), core::mem::transmute(non_repeaters), core::mem::transmute(max_repetitions), core::mem::transmute(varbindlist))
}
#[inline]
pub unsafe fn SnmpSetPort(hentity: isize, nport: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetPort(hentity : isize, nport : u32) -> u32);
    SnmpSetPort(core::mem::transmute(hentity), core::mem::transmute(nport))
}
#[inline]
pub unsafe fn SnmpSetRetransmitMode(nretransmitmode: SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetRetransmitMode(nretransmitmode : SNMP_STATUS) -> u32);
    SnmpSetRetransmitMode(core::mem::transmute(nretransmitmode))
}
#[inline]
pub unsafe fn SnmpSetRetry(hentity: isize, npolicyretry: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetRetry(hentity : isize, npolicyretry : u32) -> u32);
    SnmpSetRetry(core::mem::transmute(hentity), core::mem::transmute(npolicyretry))
}
#[inline]
pub unsafe fn SnmpSetTimeout(hentity: isize, npolicytimeout: u32) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetTimeout(hentity : isize, npolicytimeout : u32) -> u32);
    SnmpSetTimeout(core::mem::transmute(hentity), core::mem::transmute(npolicytimeout))
}
#[inline]
pub unsafe fn SnmpSetTranslateMode(ntranslatemode: SNMP_API_TRANSLATE_MODE) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetTranslateMode(ntranslatemode : SNMP_API_TRANSLATE_MODE) -> u32);
    SnmpSetTranslateMode(core::mem::transmute(ntranslatemode))
}
#[inline]
pub unsafe fn SnmpSetVb(vbl: isize, index: u32, name: *mut smiOID, value: *mut smiVALUE) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpSetVb(vbl : isize, index : u32, name : *mut smiOID, value : *mut smiVALUE) -> u32);
    SnmpSetVb(core::mem::transmute(vbl), core::mem::transmute(index), core::mem::transmute(name), core::mem::transmute(value))
}
#[inline]
pub unsafe fn SnmpStartup(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStartup(nmajorversion : *mut u32, nminorversion : *mut u32, nlevel : *mut u32, ntranslatemode : *mut SNMP_API_TRANSLATE_MODE, nretransmitmode : *mut SNMP_STATUS) -> u32);
    SnmpStartup(core::mem::transmute(nmajorversion), core::mem::transmute(nminorversion), core::mem::transmute(nlevel), core::mem::transmute(ntranslatemode), core::mem::transmute(nretransmitmode))
}
#[inline]
pub unsafe fn SnmpStartupEx(nmajorversion: *mut u32, nminorversion: *mut u32, nlevel: *mut u32, ntranslatemode: *mut SNMP_API_TRANSLATE_MODE, nretransmitmode: *mut SNMP_STATUS) -> u32 {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStartupEx(nmajorversion : *mut u32, nminorversion : *mut u32, nlevel : *mut u32, ntranslatemode : *mut SNMP_API_TRANSLATE_MODE, nretransmitmode : *mut SNMP_STATUS) -> u32);
    SnmpStartupEx(core::mem::transmute(nmajorversion), core::mem::transmute(nminorversion), core::mem::transmute(nlevel), core::mem::transmute(ntranslatemode), core::mem::transmute(nretransmitmode))
}
#[inline]
pub unsafe fn SnmpStrToContext(session: isize, string: *mut smiOCTETS) -> isize {
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStrToContext(session : isize, string : *mut smiOCTETS) -> isize);
    SnmpStrToContext(core::mem::transmute(session), core::mem::transmute(string))
}
#[inline]
pub unsafe fn SnmpStrToEntity<P1>(session: isize, string: P1) -> isize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStrToEntity(session : isize, string : windows_core::PCSTR) -> isize);
    SnmpStrToEntity(core::mem::transmute(session), string.param().abi())
}
#[inline]
pub unsafe fn SnmpStrToOid<P0>(string: P0, dstoid: *mut smiOID) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("wsnmp32.dll" "system" fn SnmpStrToOid(string : windows_core::PCSTR, dstoid : *mut smiOID) -> u32);
    SnmpStrToOid(string.param().abi(), core::mem::transmute(dstoid))
}
#[inline]
pub unsafe fn SnmpSvcGetUptime() -> u32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpSvcGetUptime() -> u32);
    SnmpSvcGetUptime()
}
#[inline]
pub unsafe fn SnmpSvcSetLogLevel(nloglevel: SNMP_LOG) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpSvcSetLogLevel(nloglevel : SNMP_LOG));
    SnmpSvcSetLogLevel(core::mem::transmute(nloglevel))
}
#[inline]
pub unsafe fn SnmpSvcSetLogType(nlogtype: SNMP_OUTPUT_LOG_TYPE) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpSvcSetLogType(nlogtype : i32));
    SnmpSvcSetLogType(nlogtype.0 as _)
}
#[inline]
pub unsafe fn SnmpUtilAsnAnyCpy(panydst: *mut AsnAny, panysrc: *mut AsnAny) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilAsnAnyCpy(panydst : *mut AsnAny, panysrc : *mut AsnAny) -> i32);
    SnmpUtilAsnAnyCpy(core::mem::transmute(panydst), core::mem::transmute(panysrc))
}
#[inline]
pub unsafe fn SnmpUtilAsnAnyFree(pany: *mut AsnAny) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilAsnAnyFree(pany : *mut AsnAny));
    SnmpUtilAsnAnyFree(core::mem::transmute(pany))
}
#[inline]
pub unsafe fn SnmpUtilDbgPrint<P1>(nloglevel: SNMP_LOG, szformat: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("snmpapi.dll" "cdecl" fn SnmpUtilDbgPrint(nloglevel : SNMP_LOG, szformat : windows_core::PCSTR));
    SnmpUtilDbgPrint(core::mem::transmute(nloglevel), szformat.param().abi())
}
#[inline]
pub unsafe fn SnmpUtilIdsToA(ids: *mut u32, idlength: u32) -> windows_core::PSTR {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilIdsToA(ids : *mut u32, idlength : u32) -> windows_core::PSTR);
    SnmpUtilIdsToA(core::mem::transmute(ids), core::mem::transmute(idlength))
}
#[inline]
pub unsafe fn SnmpUtilMemAlloc(nbytes: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilMemAlloc(nbytes : u32) -> *mut core::ffi::c_void);
    SnmpUtilMemAlloc(core::mem::transmute(nbytes))
}
#[inline]
pub unsafe fn SnmpUtilMemFree(pmem: *mut core::ffi::c_void) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilMemFree(pmem : *mut core::ffi::c_void));
    SnmpUtilMemFree(core::mem::transmute(pmem))
}
#[inline]
pub unsafe fn SnmpUtilMemReAlloc(pmem: *mut core::ffi::c_void, nbytes: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilMemReAlloc(pmem : *mut core::ffi::c_void, nbytes : u32) -> *mut core::ffi::c_void);
    SnmpUtilMemReAlloc(core::mem::transmute(pmem), core::mem::transmute(nbytes))
}
#[inline]
pub unsafe fn SnmpUtilOctetsCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOctetsCmp(poctets1 : *mut AsnOctetString, poctets2 : *mut AsnOctetString) -> i32);
    SnmpUtilOctetsCmp(core::mem::transmute(poctets1), core::mem::transmute(poctets2))
}
#[inline]
pub unsafe fn SnmpUtilOctetsCpy(poctetsdst: *mut AsnOctetString, poctetssrc: *mut AsnOctetString) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOctetsCpy(poctetsdst : *mut AsnOctetString, poctetssrc : *mut AsnOctetString) -> i32);
    SnmpUtilOctetsCpy(core::mem::transmute(poctetsdst), core::mem::transmute(poctetssrc))
}
#[inline]
pub unsafe fn SnmpUtilOctetsFree(poctets: *mut AsnOctetString) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOctetsFree(poctets : *mut AsnOctetString));
    SnmpUtilOctetsFree(core::mem::transmute(poctets))
}
#[inline]
pub unsafe fn SnmpUtilOctetsNCmp(poctets1: *mut AsnOctetString, poctets2: *mut AsnOctetString, nchars: u32) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOctetsNCmp(poctets1 : *mut AsnOctetString, poctets2 : *mut AsnOctetString, nchars : u32) -> i32);
    SnmpUtilOctetsNCmp(core::mem::transmute(poctets1), core::mem::transmute(poctets2), core::mem::transmute(nchars))
}
#[inline]
pub unsafe fn SnmpUtilOidAppend(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidAppend(poiddst : *mut AsnObjectIdentifier, poidsrc : *mut AsnObjectIdentifier) -> i32);
    SnmpUtilOidAppend(core::mem::transmute(poiddst), core::mem::transmute(poidsrc))
}
#[inline]
pub unsafe fn SnmpUtilOidCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidCmp(poid1 : *mut AsnObjectIdentifier, poid2 : *mut AsnObjectIdentifier) -> i32);
    SnmpUtilOidCmp(core::mem::transmute(poid1), core::mem::transmute(poid2))
}
#[inline]
pub unsafe fn SnmpUtilOidCpy(poiddst: *mut AsnObjectIdentifier, poidsrc: *mut AsnObjectIdentifier) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidCpy(poiddst : *mut AsnObjectIdentifier, poidsrc : *mut AsnObjectIdentifier) -> i32);
    SnmpUtilOidCpy(core::mem::transmute(poiddst), core::mem::transmute(poidsrc))
}
#[inline]
pub unsafe fn SnmpUtilOidFree(poid: *mut AsnObjectIdentifier) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidFree(poid : *mut AsnObjectIdentifier));
    SnmpUtilOidFree(core::mem::transmute(poid))
}
#[inline]
pub unsafe fn SnmpUtilOidNCmp(poid1: *mut AsnObjectIdentifier, poid2: *mut AsnObjectIdentifier, nsubids: u32) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidNCmp(poid1 : *mut AsnObjectIdentifier, poid2 : *mut AsnObjectIdentifier, nsubids : u32) -> i32);
    SnmpUtilOidNCmp(core::mem::transmute(poid1), core::mem::transmute(poid2), core::mem::transmute(nsubids))
}
#[inline]
pub unsafe fn SnmpUtilOidToA(oid: *mut AsnObjectIdentifier) -> windows_core::PSTR {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilOidToA(oid : *mut AsnObjectIdentifier) -> windows_core::PSTR);
    SnmpUtilOidToA(core::mem::transmute(oid))
}
#[inline]
pub unsafe fn SnmpUtilPrintAsnAny(pany: *mut AsnAny) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilPrintAsnAny(pany : *mut AsnAny));
    SnmpUtilPrintAsnAny(core::mem::transmute(pany))
}
#[inline]
pub unsafe fn SnmpUtilPrintOid(oid: *mut AsnObjectIdentifier) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilPrintOid(oid : *mut AsnObjectIdentifier));
    SnmpUtilPrintOid(core::mem::transmute(oid))
}
#[inline]
pub unsafe fn SnmpUtilVarBindCpy(pvbdst: *mut SnmpVarBind, pvbsrc: *mut SnmpVarBind) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilVarBindCpy(pvbdst : *mut SnmpVarBind, pvbsrc : *mut SnmpVarBind) -> i32);
    SnmpUtilVarBindCpy(core::mem::transmute(pvbdst), core::mem::transmute(pvbsrc))
}
#[inline]
pub unsafe fn SnmpUtilVarBindFree(pvb: *mut SnmpVarBind) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilVarBindFree(pvb : *mut SnmpVarBind));
    SnmpUtilVarBindFree(core::mem::transmute(pvb))
}
#[inline]
pub unsafe fn SnmpUtilVarBindListCpy(pvbldst: *mut SnmpVarBindList, pvblsrc: *mut SnmpVarBindList) -> i32 {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilVarBindListCpy(pvbldst : *mut SnmpVarBindList, pvblsrc : *mut SnmpVarBindList) -> i32);
    SnmpUtilVarBindListCpy(core::mem::transmute(pvbldst), core::mem::transmute(pvblsrc))
}
#[inline]
pub unsafe fn SnmpUtilVarBindListFree(pvbl: *mut SnmpVarBindList) {
    windows_targets::link!("snmpapi.dll" "system" fn SnmpUtilVarBindListFree(pvbl : *mut SnmpVarBindList));
    SnmpUtilVarBindListFree(core::mem::transmute(pvbl))
}
pub const ASN_APPLICATION: u32 = 64u32;
pub const ASN_CONSTRUCTOR: u32 = 32u32;
pub const ASN_CONTEXT: u32 = 128u32;
pub const ASN_CONTEXTSPECIFIC: u32 = 128u32;
pub const ASN_PRIMATIVE: u32 = 0u32;
pub const ASN_PRIMITIVE: u32 = 0u32;
pub const ASN_PRIVATE: u32 = 192u32;
pub const ASN_UNIVERSAL: u32 = 0u32;
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct AsnAny {
    pub asnType: u8,
    pub asnValue: AsnAny_0,
}
impl Default for AsnAny {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union AsnAny_0 {
    pub number: i32,
    pub unsigned32: u32,
    pub counter64: u64,
    pub string: AsnOctetString,
    pub bits: AsnOctetString,
    pub object: AsnObjectIdentifier,
    pub sequence: AsnOctetString,
    pub address: AsnOctetString,
    pub counter: u32,
    pub gauge: u32,
    pub ticks: u32,
    pub arbitrary: AsnOctetString,
}
impl Default for AsnAny_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsnObjectIdentifier {
    pub idLength: u32,
    pub ids: *mut u32,
}
#[cfg(target_arch = "x86")]
impl Default for AsnObjectIdentifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct AsnObjectIdentifier {
    pub idLength: u32,
    pub ids: *mut u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for AsnObjectIdentifier {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsnOctetString {
    pub stream: *mut u8,
    pub length: u32,
    pub dynamic: super::super::Foundation::BOOL,
}
#[cfg(target_arch = "x86")]
impl Default for AsnOctetString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct AsnOctetString {
    pub stream: *mut u8,
    pub length: u32,
    pub dynamic: super::super::Foundation::BOOL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for AsnOctetString {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEFAULT_SNMPTRAP_PORT_IPX: u32 = 36880u32;
pub const DEFAULT_SNMPTRAP_PORT_UDP: u32 = 162u32;
pub const DEFAULT_SNMP_PORT_IPX: u32 = 36879u32;
pub const DEFAULT_SNMP_PORT_UDP: u32 = 161u32;
pub const MAXOBJIDSIZE: u32 = 128u32;
pub const MAXOBJIDSTRSIZE: u32 = 1408u32;
pub const MAXVENDORINFO: u32 = 32u32;
pub const MGMCTL_SETAGENTPORT: u32 = 1u32;
pub type PFNSNMPCLEANUPEX = Option<unsafe extern "system" fn() -> u32>;
pub type PFNSNMPEXTENSIONCLOSE = Option<unsafe extern "system" fn()>;
pub type PFNSNMPEXTENSIONINIT = Option<unsafe extern "system" fn(dwuptimereference: u32, phsubagenttrapevent: *mut super::super::Foundation::HANDLE, pfirstsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONINITEX = Option<unsafe extern "system" fn(pnextsupportedregion: *mut AsnObjectIdentifier) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONMONITOR = Option<unsafe extern "system" fn(pagentmgmtdata: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONQUERY = Option<unsafe extern "system" fn(bpdutype: u8, pvarbindlist: *mut SnmpVarBindList, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONQUERYEX = Option<unsafe extern "system" fn(nrequesttype: u32, ntransactionid: u32, pvarbindlist: *mut SnmpVarBindList, pcontextinfo: *mut AsnOctetString, perrorstatus: *mut i32, perrorindex: *mut i32) -> super::super::Foundation::BOOL>;
pub type PFNSNMPEXTENSIONTRAP = Option<unsafe extern "system" fn(penterpriseoid: *mut AsnObjectIdentifier, pgenerictrapid: *mut i32, pspecifictrapid: *mut i32, ptimestamp: *mut u32, pvarbindlist: *mut SnmpVarBindList) -> super::super::Foundation::BOOL>;
pub type PFNSNMPSTARTUPEX = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u32, param2: *mut u32, param3: *mut u32, param4: *mut u32) -> u32>;
pub const SNMPAPI_ALLOC_ERROR: u32 = 2u32;
pub type SNMPAPI_CALLBACK = Option<unsafe extern "system" fn(hsession: isize, hwnd: super::super::Foundation::HWND, wmsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lpclientdata: *mut core::ffi::c_void) -> u32>;
pub const SNMPAPI_CONTEXT_INVALID: u32 = 3u32;
pub const SNMPAPI_CONTEXT_UNKNOWN: u32 = 4u32;
pub const SNMPAPI_ENTITY_INVALID: u32 = 5u32;
pub const SNMPAPI_ENTITY_UNKNOWN: u32 = 6u32;
pub const SNMPAPI_ERROR: u32 = 0u32;
pub const SNMPAPI_FAILURE: u32 = 0u32;
pub const SNMPAPI_HWND_INVALID: u32 = 20u32;
pub const SNMPAPI_INDEX_INVALID: u32 = 7u32;
pub const SNMPAPI_M2M_SUPPORT: u32 = 3u32;
pub const SNMPAPI_MESSAGE_INVALID: u32 = 19u32;
pub const SNMPAPI_MODE_INVALID: u32 = 16u32;
pub const SNMPAPI_NOERROR: u32 = 1u32;
pub const SNMPAPI_NOOP: u32 = 8u32;
pub const SNMPAPI_NOT_INITIALIZED: u32 = 18u32;
pub const SNMPAPI_NO_SUPPORT: u32 = 0u32;
pub const SNMPAPI_OFF: SNMP_STATUS = SNMP_STATUS(0u32);
pub const SNMPAPI_OID_INVALID: u32 = 9u32;
pub const SNMPAPI_ON: SNMP_STATUS = SNMP_STATUS(1u32);
pub const SNMPAPI_OPERATION_INVALID: u32 = 10u32;
pub const SNMPAPI_OTHER_ERROR: u32 = 99u32;
pub const SNMPAPI_OUTPUT_TRUNCATED: u32 = 11u32;
pub const SNMPAPI_PDU_INVALID: u32 = 12u32;
pub const SNMPAPI_SESSION_INVALID: u32 = 13u32;
pub const SNMPAPI_SIZE_INVALID: u32 = 17u32;
pub const SNMPAPI_SUCCESS: u32 = 1u32;
pub const SNMPAPI_SYNTAX_INVALID: u32 = 14u32;
pub const SNMPAPI_TL_INVALID_PARAM: u32 = 106u32;
pub const SNMPAPI_TL_IN_USE: u32 = 107u32;
pub const SNMPAPI_TL_NOT_AVAILABLE: u32 = 102u32;
pub const SNMPAPI_TL_NOT_INITIALIZED: u32 = 100u32;
pub const SNMPAPI_TL_NOT_SUPPORTED: u32 = 101u32;
pub const SNMPAPI_TL_OTHER: u32 = 199u32;
pub const SNMPAPI_TL_PDU_TOO_BIG: u32 = 109u32;
pub const SNMPAPI_TL_RESOURCE_ERROR: u32 = 103u32;
pub const SNMPAPI_TL_SRC_INVALID: u32 = 105u32;
pub const SNMPAPI_TL_TIMEOUT: u32 = 108u32;
pub const SNMPAPI_TL_UNDELIVERABLE: u32 = 104u32;
pub const SNMPAPI_TRANSLATED: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(0u32);
pub const SNMPAPI_UNTRANSLATED_V1: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(1u32);
pub const SNMPAPI_UNTRANSLATED_V2: SNMP_API_TRANSLATE_MODE = SNMP_API_TRANSLATE_MODE(2u32);
pub const SNMPAPI_V1_SUPPORT: u32 = 1u32;
pub const SNMPAPI_V2_SUPPORT: u32 = 2u32;
pub const SNMPAPI_VBL_INVALID: u32 = 15u32;
pub const SNMPLISTEN_ALL_ADDR: u32 = 1u32;
pub const SNMPLISTEN_USEENTITY_ADDR: u32 = 0u32;
pub const SNMP_ACCESS_NONE: u32 = 0u32;
pub const SNMP_ACCESS_NOTIFY: u32 = 1u32;
pub const SNMP_ACCESS_READ_CREATE: u32 = 4u32;
pub const SNMP_ACCESS_READ_ONLY: u32 = 2u32;
pub const SNMP_ACCESS_READ_WRITE: u32 = 3u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SNMP_API_TRANSLATE_MODE(pub u32);
pub const SNMP_AUTHAPI_INVALID_MSG_TYPE: u32 = 31u32;
pub const SNMP_AUTHAPI_INVALID_VERSION: u32 = 30u32;
pub const SNMP_AUTHAPI_TRIV_AUTH_FAILED: u32 = 32u32;
pub const SNMP_BERAPI_INVALID_LENGTH: u32 = 10u32;
pub const SNMP_BERAPI_INVALID_OBJELEM: u32 = 14u32;
pub const SNMP_BERAPI_INVALID_TAG: u32 = 11u32;
pub const SNMP_BERAPI_OVERFLOW: u32 = 12u32;
pub const SNMP_BERAPI_SHORT_BUFFER: u32 = 13u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SNMP_ERROR(pub u32);
pub const SNMP_ERRORSTATUS_AUTHORIZATIONERROR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(16u32);
pub const SNMP_ERRORSTATUS_BADVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(3u32);
pub const SNMP_ERRORSTATUS_COMMITFAILED: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(14u32);
pub const SNMP_ERRORSTATUS_GENERR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(5u32);
pub const SNMP_ERRORSTATUS_INCONSISTENTNAME: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(18u32);
pub const SNMP_ERRORSTATUS_INCONSISTENTVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(12u32);
pub const SNMP_ERRORSTATUS_NOACCESS: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(6u32);
pub const SNMP_ERRORSTATUS_NOCREATION: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(11u32);
pub const SNMP_ERRORSTATUS_NOERROR: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(0u32);
pub const SNMP_ERRORSTATUS_NOSUCHNAME: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(2u32);
pub const SNMP_ERRORSTATUS_NOTWRITABLE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(17u32);
pub const SNMP_ERRORSTATUS_READONLY: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(4u32);
pub const SNMP_ERRORSTATUS_RESOURCEUNAVAILABLE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(13u32);
pub const SNMP_ERRORSTATUS_TOOBIG: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(1u32);
pub const SNMP_ERRORSTATUS_UNDOFAILED: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(15u32);
pub const SNMP_ERRORSTATUS_WRONGENCODING: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(9u32);
pub const SNMP_ERRORSTATUS_WRONGLENGTH: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(8u32);
pub const SNMP_ERRORSTATUS_WRONGTYPE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(7u32);
pub const SNMP_ERRORSTATUS_WRONGVALUE: SNMP_ERROR_STATUS = SNMP_ERROR_STATUS(10u32);
pub const SNMP_ERROR_AUTHORIZATIONERROR: SNMP_ERROR = SNMP_ERROR(16u32);
pub const SNMP_ERROR_BADVALUE: SNMP_ERROR = SNMP_ERROR(3u32);
pub const SNMP_ERROR_COMMITFAILED: SNMP_ERROR = SNMP_ERROR(14u32);
pub const SNMP_ERROR_GENERR: SNMP_ERROR = SNMP_ERROR(5u32);
pub const SNMP_ERROR_INCONSISTENTNAME: SNMP_ERROR = SNMP_ERROR(18u32);
pub const SNMP_ERROR_INCONSISTENTVALUE: SNMP_ERROR = SNMP_ERROR(12u32);
pub const SNMP_ERROR_NOACCESS: SNMP_ERROR = SNMP_ERROR(6u32);
pub const SNMP_ERROR_NOCREATION: SNMP_ERROR = SNMP_ERROR(11u32);
pub const SNMP_ERROR_NOERROR: SNMP_ERROR = SNMP_ERROR(0u32);
pub const SNMP_ERROR_NOSUCHNAME: SNMP_ERROR = SNMP_ERROR(2u32);
pub const SNMP_ERROR_NOTWRITABLE: SNMP_ERROR = SNMP_ERROR(17u32);
pub const SNMP_ERROR_READONLY: SNMP_ERROR = SNMP_ERROR(4u32);
pub const SNMP_ERROR_RESOURCEUNAVAILABLE: SNMP_ERROR = SNMP_ERROR(13u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SNMP_ERROR_STATUS(pub u32);
pub const SNMP_ERROR_TOOBIG: SNMP_ERROR = SNMP_ERROR(1u32);
pub const SNMP_ERROR_UNDOFAILED: SNMP_ERROR = SNMP_ERROR(15u32);
pub const SNMP_ERROR_WRONGENCODING: SNMP_ERROR = SNMP_ERROR(9u32);
pub const SNMP_ERROR_WRONGLENGTH: SNMP_ERROR = SNMP_ERROR(8u32);
pub const SNMP_ERROR_WRONGTYPE: SNMP_ERROR = SNMP_ERROR(7u32);
pub const SNMP_ERROR_WRONGVALUE: SNMP_ERROR = SNMP_ERROR(10u32);
pub const SNMP_EXTENSION_GET: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(160u32);
pub const SNMP_EXTENSION_GET_NEXT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(161u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SNMP_EXTENSION_REQUEST_TYPE(pub u32);
pub const SNMP_EXTENSION_SET_CLEANUP: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(226u32);
pub const SNMP_EXTENSION_SET_COMMIT: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(163u32);
pub const SNMP_EXTENSION_SET_TEST: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(224u32);
pub const SNMP_EXTENSION_SET_UNDO: SNMP_EXTENSION_REQUEST_TYPE = SNMP_EXTENSION_REQUEST_TYPE(225u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SNMP_GENERICTRAP(pub u32);
pub const SNMP_GENERICTRAP_AUTHFAILURE: SNMP_GENERICTRAP = SNMP_GENERICTRAP(4u32);
pub const SNMP_GENERICTRAP_COLDSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(0u32);
pub const SNMP_GENERICTRAP_EGPNEIGHLOSS: SNMP_GENERICTRAP = SNMP_GENERICTRAP(5u32);
pub const SNMP_GENERICTRAP_ENTERSPECIFIC: SNMP_GENERICTRAP = SNMP_GENERICTRAP(6u32);
pub const SNMP_GENERICTRAP_LINKDOWN: SNMP_GENERICTRAP = SNMP_GENERICTRAP(2u32);
pub const SNMP_GENERICTRAP_LINKUP: SNMP_GENERICTRAP = SNMP_GENERICTRAP(3u32);
pub const SNMP_GENERICTRAP_WARMSTART: SNMP_GENERICTRAP = SNMP_GENERICTRAP(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SNMP_LOG(pub i32);
pub const SNMP_LOG_ERROR: SNMP_LOG = SNMP_LOG(2i32);
pub const SNMP_LOG_FATAL: SNMP_LOG = SNMP_LOG(1i32);
pub const SNMP_LOG_SILENT: SNMP_LOG = SNMP_LOG(0i32);
pub const SNMP_LOG_TRACE: SNMP_LOG = SNMP_LOG(4i32);
pub const SNMP_LOG_VERBOSE: SNMP_LOG = SNMP_LOG(5i32);
pub const SNMP_LOG_WARNING: SNMP_LOG = SNMP_LOG(3i32);
pub const SNMP_MAX_OID_LEN: u32 = 128u32;
pub const SNMP_MEM_ALLOC_ERROR: u32 = 1u32;
pub const SNMP_MGMTAPI_AGAIN: u32 = 45u32;
pub const SNMP_MGMTAPI_INVALID_BUFFER: u32 = 48u32;
pub const SNMP_MGMTAPI_INVALID_CTL: u32 = 46u32;
pub const SNMP_MGMTAPI_INVALID_SESSION: u32 = 47u32;
pub const SNMP_MGMTAPI_NOTRAPS: u32 = 44u32;
pub const SNMP_MGMTAPI_SELECT_FDERRORS: u32 = 41u32;
pub const SNMP_MGMTAPI_TIMEOUT: u32 = 40u32;
pub const SNMP_MGMTAPI_TRAP_DUPINIT: u32 = 43u32;
pub const SNMP_MGMTAPI_TRAP_ERRORS: u32 = 42u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SNMP_OUTPUT_LOG_TYPE(pub u32);
pub const SNMP_OUTPUT_TO_CONSOLE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(1u32);
pub const SNMP_OUTPUT_TO_DEBUGGER: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(8u32);
pub const SNMP_OUTPUT_TO_EVENTLOG: u32 = 4u32;
pub const SNMP_OUTPUT_TO_LOGFILE: SNMP_OUTPUT_LOG_TYPE = SNMP_OUTPUT_LOG_TYPE(2u32);
pub const SNMP_PDUAPI_INVALID_ES: u32 = 21u32;
pub const SNMP_PDUAPI_INVALID_GT: u32 = 22u32;
pub const SNMP_PDUAPI_UNRECOGNIZED_PDU: u32 = 20u32;
pub const SNMP_PDU_GET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(160u32);
pub const SNMP_PDU_GETBULK: SNMP_PDU_TYPE = SNMP_PDU_TYPE(165u32);
pub const SNMP_PDU_GETNEXT: SNMP_PDU_TYPE = SNMP_PDU_TYPE(161u32);
pub const SNMP_PDU_RESPONSE: SNMP_PDU_TYPE = SNMP_PDU_TYPE(162u32);
pub const SNMP_PDU_SET: SNMP_PDU_TYPE = SNMP_PDU_TYPE(163u32);
pub const SNMP_PDU_TRAP: SNMP_PDU_TYPE = SNMP_PDU_TYPE(167u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SNMP_PDU_TYPE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SNMP_STATUS(pub u32);
pub const SNMP_TRAP_AUTHFAIL: u32 = 4u32;
pub const SNMP_TRAP_COLDSTART: u32 = 0u32;
pub const SNMP_TRAP_EGPNEIGHBORLOSS: u32 = 5u32;
pub const SNMP_TRAP_ENTERPRISESPECIFIC: u32 = 6u32;
pub const SNMP_TRAP_LINKDOWN: u32 = 2u32;
pub const SNMP_TRAP_LINKUP: u32 = 3u32;
pub const SNMP_TRAP_WARMSTART: u32 = 1u32;
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct SnmpVarBind {
    pub name: AsnObjectIdentifier,
    pub value: AsnAny,
}
impl Default for SnmpVarBind {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SnmpVarBindList {
    pub list: *mut SnmpVarBind,
    pub len: u32,
}
#[cfg(target_arch = "x86")]
impl Default for SnmpVarBindList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct SnmpVarBindList {
    pub list: *mut SnmpVarBind,
    pub len: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SnmpVarBindList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct smiCNTR64 {
    pub hipart: u32,
    pub lopart: u32,
}
impl Default for smiCNTR64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct smiOCTETS {
    pub len: u32,
    pub ptr: *mut u8,
}
impl Default for smiOCTETS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct smiOID {
    pub len: u32,
    pub ptr: *mut u32,
}
impl Default for smiOID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct smiVALUE {
    pub syntax: u32,
    pub value: smiVALUE_0,
}
impl Default for smiVALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union smiVALUE_0 {
    pub sNumber: i32,
    pub uNumber: u32,
    pub hNumber: smiCNTR64,
    pub string: smiOCTETS,
    pub oid: smiOID,
    pub empty: u8,
}
impl Default for smiVALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct smiVENDORINFO {
    pub vendorName: [i8; 64],
    pub vendorContact: [i8; 64],
    pub vendorVersionId: [i8; 32],
    pub vendorVersionDate: [i8; 32],
    pub vendorEnterprise: u32,
}
impl Default for smiVENDORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
