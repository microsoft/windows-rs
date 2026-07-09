#[inline]
pub unsafe fn CMP_WaitNoPendingInstallEvents(dwtimeout: u32) -> u32 {
    windows_core::link!("setupapi.dll" "C" fn CMP_WaitNoPendingInstallEvents(dwtimeout : u32) -> u32);
    unsafe { CMP_WaitNoPendingInstallEvents(dwtimeout) }
}
#[inline]
pub unsafe fn CM_Add_Empty_Log_Conf(plclogconf: *mut LOG_CONF, dndevinst: DEVINST, priority: PRIORITY, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Empty_Log_Conf(plclogconf : *mut LOG_CONF, dndevinst : DEVINST, priority : PRIORITY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_Empty_Log_Conf(plclogconf as _, dndevinst, priority, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Add_Empty_Log_Conf_Ex(plclogconf: *mut LOG_CONF, dndevinst: DEVINST, priority: PRIORITY, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Empty_Log_Conf_Ex(plclogconf : *mut LOG_CONF, dndevinst : DEVINST, priority : PRIORITY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Add_Empty_Log_Conf_Ex(plclogconf as _, dndevinst, priority, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Add_IDA<P1>(dndevinst: DEVINST, pszid: P1, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Add_IDA(dndevinst : DEVINST, pszid : windows_core::PCSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_IDA(dndevinst, pszid.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Add_IDW<P1>(dndevinst: DEVINST, pszid: P1, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Add_IDW(dndevinst : DEVINST, pszid : windows_core::PCWSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_IDW(dndevinst, pszid.param().abi(), ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Add_ID_ExA<P1>(dndevinst: DEVINST, pszid: P1, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Add_ID_ExA(dndevinst : DEVINST, pszid : windows_core::PCSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Add_ID_ExA(dndevinst, pszid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Add_ID_ExW<P1>(dndevinst: DEVINST, pszid: P1, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Add_ID_ExW(dndevinst : DEVINST, pszid : windows_core::PCWSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Add_ID_ExW(dndevinst, pszid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Add_Range(ullstartvalue: super::winnt::DWORDLONG, ullendvalue: super::winnt::DWORDLONG, rlh: RANGE_LIST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Range(ullstartvalue : super::winnt::DWORDLONG, ullendvalue : super::winnt::DWORDLONG, rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_Range(ullstartvalue, ullendvalue, rlh, ulflags) }
}
#[inline]
pub unsafe fn CM_Add_Res_Des(prdresdes: Option<*mut RES_DES>, lclogconf: LOG_CONF, resourceid: RESOURCEID, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Res_Des(prdresdes : *mut RES_DES, lclogconf : LOG_CONF, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Add_Res_Des(prdresdes.unwrap_or(core::mem::zeroed()) as _, lclogconf, resourceid, resourcedata, resourcelen, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Add_Res_Des_Ex(prdresdes: Option<*mut RES_DES>, lclogconf: LOG_CONF, resourceid: RESOURCEID, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Res_Des_Ex(prdresdes : *mut RES_DES, lclogconf : LOG_CONF, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Add_Res_Des_Ex(prdresdes.unwrap_or(core::mem::zeroed()) as _, lclogconf, resourceid, resourcedata, resourcelen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Connect_MachineA<P0>(uncservername: P0, phmachine: *mut HMACHINE) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Connect_MachineA(uncservername : windows_core::PCSTR, phmachine : *mut HMACHINE) -> CONFIGRET);
    unsafe { CM_Connect_MachineA(uncservername.param().abi(), phmachine as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Connect_MachineW<P0>(uncservername: P0, phmachine: *mut HMACHINE) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Connect_MachineW(uncservername : windows_core::PCWSTR, phmachine : *mut HMACHINE) -> CONFIGRET);
    unsafe { CM_Connect_MachineW(uncservername.param().abi(), phmachine as _) }
}
#[inline]
pub unsafe fn CM_Create_DevNodeA(pdndevinst: *mut DEVNODE, pdeviceid: *const i8, dnparent: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_DevNodeA(pdndevinst : *mut DEVNODE, pdeviceid : *const i8, dnparent : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Create_DevNodeA(pdndevinst as _, pdeviceid, dnparent, ulflags) }
}
#[inline]
pub unsafe fn CM_Create_DevNodeW(pdndevinst: *mut DEVNODE, pdeviceid: *const u16, dnparent: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_DevNodeW(pdndevinst : *mut DEVNODE, pdeviceid : *const u16, dnparent : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Create_DevNodeW(pdndevinst as _, pdeviceid, dnparent, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Create_DevNode_ExA(pdndevinst: *mut DEVNODE, pdeviceid: *const i8, dnparent: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_DevNode_ExA(pdndevinst : *mut DEVNODE, pdeviceid : *const i8, dnparent : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Create_DevNode_ExA(pdndevinst as _, pdeviceid, dnparent, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Create_DevNode_ExW(pdndevinst: *mut DEVNODE, pdeviceid: *const u16, dnparent: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_DevNode_ExW(pdndevinst : *mut DEVNODE, pdeviceid : *const u16, dnparent : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Create_DevNode_ExW(pdndevinst as _, pdeviceid, dnparent, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Create_Range_List(prlh: *mut RANGE_LIST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_Range_List(prlh : *mut RANGE_LIST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Create_Range_List(prlh as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Delete_Class_Key(classguid: *const windows_core::GUID, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Class_Key(classguid : *const windows_core::GUID, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_Class_Key(classguid, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Delete_Class_Key_Ex(classguid: *const windows_core::GUID, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Class_Key_Ex(classguid : *const windows_core::GUID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Delete_Class_Key_Ex(classguid, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Delete_DevNode_Key(dndevnode: DEVNODE, ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_DevNode_Key(dndevnode : DEVNODE, ulhardwareprofile : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_DevNode_Key(dndevnode, ulhardwareprofile, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Delete_DevNode_Key_Ex(dndevnode: DEVNODE, ulhardwareprofile: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_DevNode_Key_Ex(dndevnode : DEVNODE, ulhardwareprofile : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Delete_DevNode_Key_Ex(dndevnode, ulhardwareprofile, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Delete_Device_Interface_KeyA<P0>(pszdeviceinterface: P0, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_KeyA(pszdeviceinterface : windows_core::PCSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_KeyA(pszdeviceinterface.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Delete_Device_Interface_KeyW<P0>(pszdeviceinterface: P0, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_KeyW(pszdeviceinterface : windows_core::PCWSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_KeyW(pszdeviceinterface.param().abi(), ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_Key_ExA<P0>(pszdeviceinterface: P0, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface : windows_core::PCSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_Key_ExW<P0>(pszdeviceinterface: P0, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface : windows_core::PCWSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Delete_Range(ullstartvalue: super::winnt::DWORDLONG, ullendvalue: super::winnt::DWORDLONG, rlh: RANGE_LIST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Range(ullstartvalue : super::winnt::DWORDLONG, ullendvalue : super::winnt::DWORDLONG, rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Delete_Range(ullstartvalue, ullendvalue, rlh, ulflags) }
}
#[inline]
pub unsafe fn CM_Detect_Resource_Conflict(dndevinst: DEVINST, resourceid: RESOURCEID, resourcedata: *const core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut windows_core::BOOL, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Detect_Resource_Conflict(dndevinst : DEVINST, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, pbconflictdetected : *mut windows_core::BOOL, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Detect_Resource_Conflict(dndevinst, resourceid, resourcedata, resourcelen, pbconflictdetected as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Detect_Resource_Conflict_Ex(dndevinst: DEVINST, resourceid: RESOURCEID, resourcedata: *const core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut windows_core::BOOL, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Detect_Resource_Conflict_Ex(dndevinst : DEVINST, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, pbconflictdetected : *mut windows_core::BOOL, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Detect_Resource_Conflict_Ex(dndevinst, resourceid, resourcedata, resourcelen, pbconflictdetected as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Disable_DevNode(dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Disable_DevNode(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Disable_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Disable_DevNode_Ex(dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Disable_DevNode_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Disable_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Disconnect_Machine(hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Disconnect_Machine(hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Disconnect_Machine(hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Dup_Range_List(rlhold: RANGE_LIST, rlhnew: RANGE_LIST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Dup_Range_List(rlhold : RANGE_LIST, rlhnew : RANGE_LIST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Dup_Range_List(rlhold, rlhnew, ulflags) }
}
#[inline]
pub unsafe fn CM_Enable_DevNode(dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enable_DevNode(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Enable_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Enable_DevNode_Ex(dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enable_DevNode_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Enable_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Enumerate_Classes(ulclassindex: u32, classguid: *mut windows_core::GUID, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_Classes(ulclassindex : u32, classguid : *mut windows_core::GUID, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Enumerate_Classes(ulclassindex, classguid as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Enumerate_Classes_Ex(ulclassindex: u32, classguid: *mut windows_core::GUID, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_Classes_Ex(ulclassindex : u32, classguid : *mut windows_core::GUID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Enumerate_Classes_Ex(ulclassindex, classguid as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Enumerate_EnumeratorsA(ulenumindex: u32, buffer: windows_core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_EnumeratorsA(ulenumindex : u32, buffer : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Enumerate_EnumeratorsA(ulenumindex, core::mem::transmute(buffer), pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Enumerate_EnumeratorsW(ulenumindex: u32, buffer: windows_core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_EnumeratorsW(ulenumindex : u32, buffer : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Enumerate_EnumeratorsW(ulenumindex, core::mem::transmute(buffer), pullength as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Enumerate_Enumerators_ExA(ulenumindex: u32, buffer: windows_core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_Enumerators_ExA(ulenumindex : u32, buffer : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Enumerate_Enumerators_ExA(ulenumindex, core::mem::transmute(buffer), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Enumerate_Enumerators_ExW(ulenumindex: u32, buffer: windows_core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_Enumerators_ExW(ulenumindex : u32, buffer : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Enumerate_Enumerators_ExW(ulenumindex, core::mem::transmute(buffer), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Find_Range(pullstart: *mut super::winnt::DWORDLONG, ullstart: super::winnt::DWORDLONG, ullength: u32, ullalignment: super::winnt::DWORDLONG, ullend: super::winnt::DWORDLONG, rlh: RANGE_LIST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Find_Range(pullstart : *mut super::winnt::DWORDLONG, ullstart : super::winnt::DWORDLONG, ullength : u32, ullalignment : super::winnt::DWORDLONG, ullend : super::winnt::DWORDLONG, rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Find_Range(pullstart as _, ullstart, ullength, ullalignment, ullend, rlh, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_First_Range(rlh: RANGE_LIST, pullstart: *mut super::winnt::DWORDLONG, pullend: *mut super::winnt::DWORDLONG, preelement: *mut RANGE_ELEMENT, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_First_Range(rlh : RANGE_LIST, pullstart : *mut super::winnt::DWORDLONG, pullend : *mut super::winnt::DWORDLONG, preelement : *mut RANGE_ELEMENT, ulflags : u32) -> CONFIGRET);
    unsafe { CM_First_Range(rlh, pullstart as _, pullend as _, preelement as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Free_Log_Conf(lclogconftobefreed: LOG_CONF, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Log_Conf(lclogconftobefreed : LOG_CONF, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Free_Log_Conf(lclogconftobefreed, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Free_Log_Conf_Ex(lclogconftobefreed: LOG_CONF, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Log_Conf_Ex(lclogconftobefreed : LOG_CONF, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Free_Log_Conf_Ex(lclogconftobefreed, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Free_Log_Conf_Handle(lclogconf: LOG_CONF) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Log_Conf_Handle(lclogconf : LOG_CONF) -> CONFIGRET);
    unsafe { CM_Free_Log_Conf_Handle(lclogconf) }
}
#[inline]
pub unsafe fn CM_Free_Range_List(rlh: RANGE_LIST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Range_List(rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Free_Range_List(rlh, ulflags) }
}
#[inline]
pub unsafe fn CM_Free_Res_Des(prdresdes: Option<*mut RES_DES>, rdresdes: RES_DES, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Res_Des(prdresdes : *mut RES_DES, rdresdes : RES_DES, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Free_Res_Des(prdresdes.unwrap_or(core::mem::zeroed()) as _, rdresdes, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Free_Res_Des_Ex(prdresdes: Option<*mut RES_DES>, rdresdes: RES_DES, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Res_Des_Ex(prdresdes : *mut RES_DES, rdresdes : RES_DES, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Free_Res_Des_Ex(prdresdes.unwrap_or(core::mem::zeroed()) as _, rdresdes, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Free_Res_Des_Handle(rdresdes: RES_DES) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Res_Des_Handle(rdresdes : RES_DES) -> CONFIGRET);
    unsafe { CM_Free_Res_Des_Handle(rdresdes) }
}
#[inline]
pub unsafe fn CM_Free_Resource_Conflict_Handle(clconflictlist: CONFLICT_LIST) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Resource_Conflict_Handle(clconflictlist : CONFLICT_LIST) -> CONFIGRET);
    unsafe { CM_Free_Resource_Conflict_Handle(clconflictlist) }
}
#[inline]
pub unsafe fn CM_Get_Child(pdndevinst: *mut DEVNODE, dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Child(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Child(pdndevinst as _, dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Child_Ex(pdndevinst: *mut DEVNODE, dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Child_Ex(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Child_Ex(pdndevinst as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Class_Key_NameA(classguid: *const windows_core::GUID, pszkeyname: Option<windows_core::PSTR>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Key_NameA(classguid : *const windows_core::GUID, pszkeyname : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_NameA(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Class_Key_NameW(classguid: *const windows_core::GUID, pszkeyname: Option<windows_core::PWSTR>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Key_NameW(classguid : *const windows_core::GUID, pszkeyname : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_NameW(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Class_Key_Name_ExA(classguid: *const windows_core::GUID, pszkeyname: Option<windows_core::PSTR>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Key_Name_ExA(classguid : *const windows_core::GUID, pszkeyname : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_Name_ExA(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Class_Key_Name_ExW(classguid: *const windows_core::GUID, pszkeyname: Option<windows_core::PWSTR>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Key_Name_ExW(classguid : *const windows_core::GUID, pszkeyname : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_Name_ExW(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Class_NameA(classguid: *const windows_core::GUID, buffer: Option<windows_core::PSTR>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_NameA(classguid : *const windows_core::GUID, buffer : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_NameA(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Class_NameW(classguid: *const windows_core::GUID, buffer: Option<windows_core::PWSTR>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_NameW(classguid : *const windows_core::GUID, buffer : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_NameW(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Class_Name_ExA(classguid: *const windows_core::GUID, buffer: Option<windows_core::PSTR>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Name_ExA(classguid : *const windows_core::GUID, buffer : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Name_ExA(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Class_Name_ExW(classguid: *const windows_core::GUID, buffer: Option<windows_core::PWSTR>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Name_ExW(classguid : *const windows_core::GUID, buffer : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Name_ExW(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn CM_Get_Class_PropertyW(classguid: *const windows_core::GUID, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: *mut super::devpropdef::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_PropertyW(classguid : *const windows_core::GUID, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_PropertyW(classguid, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Class_Property_ExW(classguid: *const windows_core::GUID, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: *mut super::devpropdef::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_ExW(classguid : *const windows_core::GUID, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Property_ExW(classguid, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn CM_Get_Class_Property_Keys(classguid: *const windows_core::GUID, propertykeyarray: Option<*mut super::devpropdef::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_Keys(classguid : *const windows_core::GUID, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Class_Property_Keys(classguid, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Class_Property_Keys_Ex(classguid: *const windows_core::GUID, propertykeyarray: Option<*mut super::devpropdef::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_Keys_Ex(classguid : *const windows_core::GUID, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Property_Keys_Ex(classguid, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Class_Registry_PropertyA(classguid: *const windows_core::GUID, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Registry_PropertyA(classguid : *const windows_core::GUID, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Registry_PropertyA(classguid, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Class_Registry_PropertyW(classguid: *const windows_core::GUID, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Registry_PropertyW(classguid : *const windows_core::GUID, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Registry_PropertyW(classguid, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Depth(puldepth: *mut u32, dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Depth(puldepth : *mut u32, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Depth(puldepth as _, dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Depth_Ex(puldepth: *mut u32, dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Depth_Ex(puldepth : *mut u32, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Depth_Ex(puldepth as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_PropertyA<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_PropertyA(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_PropertyA(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_PropertyW<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_PropertyW(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCWSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_PropertyW(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_Property_ExA<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_Property_ExA(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_Property_ExA(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_Property_ExW<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_Property_ExW(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCWSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_Property_ExW(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn CM_Get_DevNode_PropertyW(dndevinst: DEVINST, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: *mut super::devpropdef::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_PropertyW(dndevinst : DEVINST, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_PropertyW(dndevinst, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_ExW(dndevinst: DEVINST, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: *mut super::devpropdef::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_ExW(dndevinst : DEVINST, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Property_ExW(dndevinst, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_Keys(dndevinst: DEVINST, propertykeyarray: Option<*mut super::devpropdef::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_Keys(dndevinst : DEVINST, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Property_Keys(dndevinst, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_Keys_Ex(dndevinst: DEVINST, propertykeyarray: Option<*mut super::devpropdef::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_Keys_Ex(dndevinst : DEVINST, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Property_Keys_Ex(dndevinst, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_PropertyA(dndevinst: DEVINST, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_PropertyA(dndevinst : DEVINST, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_PropertyA(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_PropertyW(dndevinst: DEVINST, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_PropertyW(dndevinst : DEVINST, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_PropertyW(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_Property_ExA(dndevinst: DEVINST, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_Property_ExA(dndevinst : DEVINST, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_Property_ExA(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_Property_ExW(dndevinst: DEVINST, ulproperty: u32, pulregdatatype: Option<*mut u32>, buffer: Option<*mut core::ffi::c_void>, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_Property_ExW(dndevinst : DEVINST, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_Property_ExW(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_DevNode_Status(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Status(pulstatus : *mut u32, pulproblemnumber : *mut u32, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Status(pulstatus as _, pulproblemnumber as _, dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_DevNode_Status_Ex(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Status_Ex(pulstatus : *mut u32, pulproblemnumber : *mut u32, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Status_Ex(pulstatus as _, pulproblemnumber as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_IDA(dndevinst: DEVINST, buffer: &mut [u8], ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_IDA(dndevinst : DEVINST, buffer : windows_core::PSTR, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_IDA(dndevinst, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_IDW(dndevinst: DEVINST, buffer: &mut [u16], ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_IDW(dndevinst : DEVINST, buffer : windows_core::PWSTR, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_IDW(dndevinst, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_ID_ExA(dndevinst: DEVINST, buffer: &mut [u8], ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ExA(dndevinst : DEVINST, buffer : windows_core::PSTR, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ExA(dndevinst, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_ID_ExW(dndevinst: DEVINST, buffer: &mut [u16], ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ExW(dndevinst : DEVINST, buffer : windows_core::PWSTR, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ExW(dndevinst, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_ListA<P0>(pszfilter: P0, buffer: &mut [i8], ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ListA(pszfilter : windows_core::PCSTR, buffer : *mut i8, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ListA(pszfilter.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_ListW<P0>(pszfilter: P0, buffer: &mut [u16], ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ListW(pszfilter : windows_core::PCWSTR, buffer : *mut u16, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ListW(pszfilter.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_ExA<P0>(pszfilter: P0, buffer: &mut [i8], ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_ExA(pszfilter : windows_core::PCSTR, buffer : *mut i8, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_ExA(pszfilter.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_ExW<P0>(pszfilter: P0, buffer: &mut [u16], ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_ExW(pszfilter : windows_core::PCWSTR, buffer : *mut u16, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_ExW(pszfilter.param().abi(), core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_List_SizeA<P1>(pullen: *mut u32, pszfilter: P1, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_SizeA(pullen : *mut u32, pszfilter : windows_core::PCSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_SizeA(pullen as _, pszfilter.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_List_SizeW<P1>(pullen: *mut u32, pszfilter: P1, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_SizeW(pullen : *mut u32, pszfilter : windows_core::PCWSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_SizeW(pullen as _, pszfilter.param().abi(), ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_Size_ExA<P1>(pullen: *mut u32, pszfilter: P1, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_Size_ExA(pullen : *mut u32, pszfilter : windows_core::PCSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_Size_ExA(pullen as _, pszfilter.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_Size_ExW<P1>(pullen: *mut u32, pszfilter: P1, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_Size_ExW(pullen : *mut u32, pszfilter : windows_core::PCWSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_Size_ExW(pullen as _, pszfilter.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_ID_Size(pullen: *mut u32, dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_Size(pullen : *mut u32, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_Size(pullen as _, dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_ID_Size_Ex(pullen: *mut u32, dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_Size_Ex(pullen : *mut u32, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_Size_Ex(pullen as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_AliasA<P0>(pszdeviceinterface: P0, aliasinterfaceguid: *const windows_core::GUID, pszaliasdeviceinterface: windows_core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_AliasA(pszdeviceinterface : windows_core::PCSTR, aliasinterfaceguid : *const windows_core::GUID, pszaliasdeviceinterface : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_AliasA(pszdeviceinterface.param().abi(), aliasinterfaceguid, core::mem::transmute(pszaliasdeviceinterface), pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_AliasW<P0>(pszdeviceinterface: P0, aliasinterfaceguid: *const windows_core::GUID, pszaliasdeviceinterface: windows_core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_AliasW(pszdeviceinterface : windows_core::PCWSTR, aliasinterfaceguid : *const windows_core::GUID, pszaliasdeviceinterface : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_AliasW(pszdeviceinterface.param().abi(), aliasinterfaceguid, core::mem::transmute(pszaliasdeviceinterface), pullength as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Alias_ExA<P0>(pszdeviceinterface: P0, aliasinterfaceguid: *const windows_core::GUID, pszaliasdeviceinterface: windows_core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface : windows_core::PCSTR, aliasinterfaceguid : *const windows_core::GUID, pszaliasdeviceinterface : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface.param().abi(), aliasinterfaceguid, core::mem::transmute(pszaliasdeviceinterface), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Alias_ExW<P0>(pszdeviceinterface: P0, aliasinterfaceguid: *const windows_core::GUID, pszaliasdeviceinterface: windows_core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface : windows_core::PCWSTR, aliasinterfaceguid : *const windows_core::GUID, pszaliasdeviceinterface : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface.param().abi(), aliasinterfaceguid, core::mem::transmute(pszaliasdeviceinterface), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_ListA(interfaceclassguid: *const windows_core::GUID, pdeviceid: Option<*const i8>, buffer: &mut [i8], ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_ListA(interfaceclassguid : *const windows_core::GUID, pdeviceid : *const i8, buffer : *mut i8, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_ListA(interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_ListW(interfaceclassguid: *const windows_core::GUID, pdeviceid: Option<*const u16>, buffer: &mut [u16], ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_ListW(interfaceclassguid : *const windows_core::GUID, pdeviceid : *const u16, buffer : *mut u16, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_ListW(interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_ExA(interfaceclassguid: *const windows_core::GUID, pdeviceid: Option<*const i8>, buffer: &mut [i8], ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_ExA(interfaceclassguid : *const windows_core::GUID, pdeviceid : *const i8, buffer : *mut i8, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_ExA(interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_ExW(interfaceclassguid: *const windows_core::GUID, pdeviceid: Option<*const u16>, buffer: &mut [u16], ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_ExW(interfaceclassguid : *const windows_core::GUID, pdeviceid : *const u16, buffer : *mut u16, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_ExW(interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_SizeA(pullen: *mut u32, interfaceclassguid: *const windows_core::GUID, pdeviceid: Option<*const i8>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_SizeA(pullen : *mut u32, interfaceclassguid : *const windows_core::GUID, pdeviceid : *const i8, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_SizeA(pullen as _, interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_SizeW(pullen: *mut u32, interfaceclassguid: *const windows_core::GUID, pdeviceid: Option<*const u16>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_SizeW(pullen : *mut u32, interfaceclassguid : *const windows_core::GUID, pdeviceid : *const u16, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_SizeW(pullen as _, interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_Size_ExA(pullen: *mut u32, interfaceclassguid: *const windows_core::GUID, pdeviceid: Option<*const i8>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_Size_ExA(pullen : *mut u32, interfaceclassguid : *const windows_core::GUID, pdeviceid : *const i8, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_Size_ExA(pullen as _, interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_Size_ExW(pullen: *mut u32, interfaceclassguid: *const windows_core::GUID, pdeviceid: Option<*const u16>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_Size_ExW(pullen : *mut u32, interfaceclassguid : *const windows_core::GUID, pdeviceid : *const u16, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_Size_ExW(pullen as _, interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_PropertyW<P0>(pszdeviceinterface: P0, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: *mut super::devpropdef::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_PropertyW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_PropertyW(pszdeviceinterface.param().abi(), propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_ExW<P0>(pszdeviceinterface: P0, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: *mut super::devpropdef::DEVPROPTYPE, propertybuffer: Option<*mut u8>, propertybuffersize: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_ExW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : *mut super::devpropdef::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Property_ExW(pszdeviceinterface.param().abi(), propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_KeysW<P0>(pszdeviceinterface: P0, propertykeyarray: Option<*mut super::devpropdef::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface : windows_core::PCWSTR, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface.param().abi(), propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_Keys_ExW<P0>(pszdeviceinterface: P0, propertykeyarray: Option<*mut super::devpropdef::DEVPROPKEY>, propertykeycount: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface : windows_core::PCWSTR, propertykeyarray : *mut super::devpropdef::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface.param().abi(), propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_First_Log_Conf(plclogconf: Option<*mut LOG_CONF>, dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_First_Log_Conf(plclogconf : *mut LOG_CONF, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_First_Log_Conf(plclogconf.unwrap_or(core::mem::zeroed()) as _, dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_First_Log_Conf_Ex(plclogconf: Option<*mut LOG_CONF>, dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_First_Log_Conf_Ex(plclogconf : *mut LOG_CONF, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_First_Log_Conf_Ex(plclogconf.unwrap_or(core::mem::zeroed()) as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Global_State(pulstate: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Global_State(pulstate : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Global_State(pulstate as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Global_State_Ex(pulstate: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Global_State_Ex(pulstate : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Global_State_Ex(pulstate as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_HW_Prof_FlagsA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_FlagsA(pdeviceid : *const i8, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_FlagsA(pdeviceid, ulhardwareprofile, pulvalue as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_HW_Prof_FlagsW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_FlagsW(pdeviceid : *const u16, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_FlagsW(pdeviceid, ulhardwareprofile, pulvalue as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_Flags_ExA(pdeviceid : *const i8, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_Flags_ExA(pdeviceid, ulhardwareprofile, pulvalue as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_Flags_ExW(pdeviceid : *const u16, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_Flags_ExW(pdeviceid, ulhardwareprofile, pulvalue as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_InfoA(ulindex: u32, phwprofileinfo: *mut HWPROFILEINFO_A, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_InfoA(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_A, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_InfoA(ulindex, phwprofileinfo as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_InfoW(ulindex: u32, phwprofileinfo: *mut HWPROFILEINFO_W, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_InfoW(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_W, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_InfoW(ulindex, phwprofileinfo as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_Info_ExA(ulindex: u32, phwprofileinfo: *mut HWPROFILEINFO_A, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_Info_ExA(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_A, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_Info_ExA(ulindex, phwprofileinfo as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_Info_ExW(ulindex: u32, phwprofileinfo: *mut HWPROFILEINFO_W, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_Info_ExW(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_W, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_Info_ExW(ulindex, phwprofileinfo as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Log_Conf_Priority(lclogconf: LOG_CONF, ppriority: *mut PRIORITY, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Log_Conf_Priority(lclogconf : LOG_CONF, ppriority : *mut PRIORITY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Log_Conf_Priority(lclogconf, ppriority as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Log_Conf_Priority_Ex(lclogconf: LOG_CONF, ppriority: *mut PRIORITY, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Log_Conf_Priority_Ex(lclogconf : LOG_CONF, ppriority : *mut PRIORITY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Log_Conf_Priority_Ex(lclogconf, ppriority as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Next_Log_Conf(plclogconf: Option<*mut LOG_CONF>, lclogconf: LOG_CONF, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Next_Log_Conf(plclogconf : *mut LOG_CONF, lclogconf : LOG_CONF, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Next_Log_Conf(plclogconf.unwrap_or(core::mem::zeroed()) as _, lclogconf, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Next_Log_Conf_Ex(plclogconf: Option<*mut LOG_CONF>, lclogconf: LOG_CONF, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Next_Log_Conf_Ex(plclogconf : *mut LOG_CONF, lclogconf : LOG_CONF, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Next_Log_Conf_Ex(plclogconf.unwrap_or(core::mem::zeroed()) as _, lclogconf, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Next_Res_Des(prdresdes: *mut RES_DES, rdresdes: RES_DES, forresource: RESOURCEID, presourceid: Option<*mut RESOURCEID>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Next_Res_Des(prdresdes : *mut RES_DES, rdresdes : RES_DES, forresource : RESOURCEID, presourceid : *mut RESOURCEID, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Next_Res_Des(prdresdes as _, rdresdes, forresource, presourceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Next_Res_Des_Ex(prdresdes: *mut RES_DES, rdresdes: RES_DES, forresource: RESOURCEID, presourceid: Option<*mut RESOURCEID>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Next_Res_Des_Ex(prdresdes : *mut RES_DES, rdresdes : RES_DES, forresource : RESOURCEID, presourceid : *mut RESOURCEID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Next_Res_Des_Ex(prdresdes as _, rdresdes, forresource, presourceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Parent(pdndevinst: *mut DEVNODE, dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Parent(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Parent(pdndevinst as _, dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Parent_Ex(pdndevinst: *mut DEVNODE, dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Parent_Ex(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Parent_Ex(pdndevinst as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Res_Des_Data(rdresdes: RES_DES, buffer: *mut core::ffi::c_void, bufferlen: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data(rdresdes : RES_DES, buffer : *mut core::ffi::c_void, bufferlen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data(rdresdes, buffer as _, bufferlen, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Ex(rdresdes: RES_DES, buffer: *mut core::ffi::c_void, bufferlen: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data_Ex(rdresdes : RES_DES, buffer : *mut core::ffi::c_void, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data_Ex(rdresdes, buffer as _, bufferlen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Size(pulsize: *mut u32, rdresdes: RES_DES, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data_Size(pulsize : *mut u32, rdresdes : RES_DES, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data_Size(pulsize as _, rdresdes, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Size_Ex(pulsize: *mut u32, rdresdes: RES_DES, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data_Size_Ex(pulsize : *mut u32, rdresdes : RES_DES, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data_Size_Ex(pulsize as _, rdresdes, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_Count(clconflictlist: CONFLICT_LIST, pulcount: *mut u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Resource_Conflict_Count(clconflictlist : CONFLICT_LIST, pulcount : *mut u32) -> CONFIGRET);
    unsafe { CM_Get_Resource_Conflict_Count(clconflictlist, pulcount as _) }
}
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_DetailsA(clconflictlist: CONFLICT_LIST, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_A) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Resource_Conflict_DetailsA(clconflictlist : CONFLICT_LIST, ulindex : u32, pconflictdetails : *mut CONFLICT_DETAILS_A) -> CONFIGRET);
    unsafe { CM_Get_Resource_Conflict_DetailsA(clconflictlist, ulindex, pconflictdetails as _) }
}
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_DetailsW(clconflictlist: CONFLICT_LIST, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_W) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Resource_Conflict_DetailsW(clconflictlist : CONFLICT_LIST, ulindex : u32, pconflictdetails : *mut CONFLICT_DETAILS_W) -> CONFIGRET);
    unsafe { CM_Get_Resource_Conflict_DetailsW(clconflictlist, ulindex, pconflictdetails as _) }
}
#[inline]
pub unsafe fn CM_Get_Sibling(pdndevinst: *mut DEVNODE, dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Sibling(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Get_Sibling(pdndevinst as _, dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Sibling_Ex(pdndevinst: *mut DEVNODE, dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Sibling_Ex(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Sibling_Ex(pdndevinst as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Get_Version() -> u16 {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Version() -> u16);
    unsafe { CM_Get_Version() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Get_Version_Ex(hmachine: Option<HMACHINE>) -> u16 {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Version_Ex(hmachine : HMACHINE) -> u16);
    unsafe { CM_Get_Version_Ex(hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Intersect_Range_List(rlhold1: RANGE_LIST, rlhold2: RANGE_LIST, rlhnew: RANGE_LIST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Intersect_Range_List(rlhold1 : RANGE_LIST, rlhold2 : RANGE_LIST, rlhnew : RANGE_LIST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Intersect_Range_List(rlhold1, rlhold2, rlhnew, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Invert_Range_List(rlhold: RANGE_LIST, rlhnew: RANGE_LIST, ullmaxvalue: super::winnt::DWORDLONG, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Invert_Range_List(rlhold : RANGE_LIST, rlhnew : RANGE_LIST, ullmaxvalue : super::winnt::DWORDLONG, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Invert_Range_List(rlhold, rlhnew, ullmaxvalue, ulflags) }
}
#[inline]
pub unsafe fn CM_Is_Dock_Station_Present(pbpresent: *mut windows_core::BOOL) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Is_Dock_Station_Present(pbpresent : *mut windows_core::BOOL) -> CONFIGRET);
    unsafe { CM_Is_Dock_Station_Present(pbpresent as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Is_Dock_Station_Present_Ex(pbpresent: *mut windows_core::BOOL, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Is_Dock_Station_Present_Ex(pbpresent : *mut windows_core::BOOL, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Is_Dock_Station_Present_Ex(pbpresent as _, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Is_Version_Available(wversion: u16) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn CM_Is_Version_Available(wversion : u16) -> windows_core::BOOL);
    unsafe { CM_Is_Version_Available(wversion) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Is_Version_Available_Ex(wversion: u16, hmachine: Option<HMACHINE>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn CM_Is_Version_Available_Ex(wversion : u16, hmachine : HMACHINE) -> windows_core::BOOL);
    unsafe { CM_Is_Version_Available_Ex(wversion, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Locate_DevNodeA(pdndevinst: *mut DEVNODE, pdeviceid: Option<*const i8>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Locate_DevNodeA(pdndevinst : *mut DEVNODE, pdeviceid : *const i8, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Locate_DevNodeA(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Locate_DevNodeW(pdndevinst: *mut DEVNODE, pdeviceid: Option<*const u16>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Locate_DevNodeW(pdndevinst : *mut DEVNODE, pdeviceid : *const u16, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Locate_DevNodeW(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Locate_DevNode_ExA(pdndevinst: *mut DEVNODE, pdeviceid: Option<*const i8>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Locate_DevNode_ExA(pdndevinst : *mut DEVNODE, pdeviceid : *const i8, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Locate_DevNode_ExA(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Locate_DevNode_ExW(pdndevinst: *mut DEVNODE, pdeviceid: Option<*const u16>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Locate_DevNode_ExW(pdndevinst : *mut DEVNODE, pdeviceid : *const u16, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Locate_DevNode_ExW(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_MapCrToWin32Err(cmreturncode: CONFIGRET, defaulterr: u32) -> u32 {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_MapCrToWin32Err(cmreturncode : CONFIGRET, defaulterr : u32) -> u32);
    unsafe { CM_MapCrToWin32Err(cmreturncode, defaulterr) }
}
#[inline]
pub unsafe fn CM_Merge_Range_List(rlhold1: RANGE_LIST, rlhold2: RANGE_LIST, rlhnew: RANGE_LIST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Merge_Range_List(rlhold1 : RANGE_LIST, rlhold2 : RANGE_LIST, rlhnew : RANGE_LIST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Merge_Range_List(rlhold1, rlhold2, rlhnew, ulflags) }
}
#[inline]
pub unsafe fn CM_Modify_Res_Des(prdresdes: *mut RES_DES, rdresdes: RES_DES, resourceid: RESOURCEID, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Modify_Res_Des(prdresdes : *mut RES_DES, rdresdes : RES_DES, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Modify_Res_Des(prdresdes as _, rdresdes, resourceid, resourcedata, resourcelen, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Modify_Res_Des_Ex(prdresdes: *mut RES_DES, rdresdes: RES_DES, resourceid: RESOURCEID, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Modify_Res_Des_Ex(prdresdes : *mut RES_DES, rdresdes : RES_DES, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Modify_Res_Des_Ex(prdresdes as _, rdresdes, resourceid, resourcedata, resourcelen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Move_DevNode(dnfromdevinst: DEVINST, dntodevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Move_DevNode(dnfromdevinst : DEVINST, dntodevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Move_DevNode(dnfromdevinst, dntodevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Move_DevNode_Ex(dnfromdevinst: DEVINST, dntodevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Move_DevNode_Ex(dnfromdevinst : DEVINST, dntodevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Move_DevNode_Ex(dnfromdevinst, dntodevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Next_Range(preelement: *mut RANGE_ELEMENT, pullstart: *mut super::winnt::DWORDLONG, pullend: *mut super::winnt::DWORDLONG, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Next_Range(preelement : *mut RANGE_ELEMENT, pullstart : *mut super::winnt::DWORDLONG, pullend : *mut super::winnt::DWORDLONG, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Next_Range(preelement as _, pullstart as _, pullend as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Class_KeyA<P1>(classguid: Option<*const windows_core::GUID>, pszclassname: P1, samdesired: super::winreg::REGSAM, disposition: REGDISPOSITION, phkclass: *mut super::minwindef::HKEY, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Class_KeyA(classguid : *const windows_core::GUID, pszclassname : windows_core::PCSTR, samdesired : super::winreg::REGSAM, disposition : REGDISPOSITION, phkclass : *mut super::minwindef::HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_Class_KeyA(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Class_KeyW<P1>(classguid: Option<*const windows_core::GUID>, pszclassname: P1, samdesired: super::winreg::REGSAM, disposition: REGDISPOSITION, phkclass: *mut super::minwindef::HKEY, ulflags: u32) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Class_KeyW(classguid : *const windows_core::GUID, pszclassname : windows_core::PCWSTR, samdesired : super::winreg::REGSAM, disposition : REGDISPOSITION, phkclass : *mut super::minwindef::HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_Class_KeyW(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Class_Key_ExA<P1>(classguid: Option<*const windows_core::GUID>, pszclassname: P1, samdesired: super::winreg::REGSAM, disposition: REGDISPOSITION, phkclass: *mut super::minwindef::HKEY, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Class_Key_ExA(classguid : *const windows_core::GUID, pszclassname : windows_core::PCSTR, samdesired : super::winreg::REGSAM, disposition : REGDISPOSITION, phkclass : *mut super::minwindef::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_Class_Key_ExA(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Class_Key_ExW<P1>(classguid: Option<*const windows_core::GUID>, pszclassname: P1, samdesired: super::winreg::REGSAM, disposition: REGDISPOSITION, phkclass: *mut super::minwindef::HKEY, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Class_Key_ExW(classguid : *const windows_core::GUID, pszclassname : windows_core::PCWSTR, samdesired : super::winreg::REGSAM, disposition : REGDISPOSITION, phkclass : *mut super::minwindef::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_Class_Key_ExW(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_DevNode_Key(dndevnode: DEVINST, samdesired: super::winreg::REGSAM, ulhardwareprofile: u32, disposition: REGDISPOSITION, phkdevice: *mut super::minwindef::HKEY, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Open_DevNode_Key(dndevnode : DEVINST, samdesired : super::winreg::REGSAM, ulhardwareprofile : u32, disposition : REGDISPOSITION, phkdevice : *mut super::minwindef::HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_DevNode_Key(dndevnode, samdesired, ulhardwareprofile, disposition, phkdevice as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_DevNode_Key_Ex(dndevnode: DEVINST, samdesired: super::winreg::REGSAM, ulhardwareprofile: u32, disposition: REGDISPOSITION, phkdevice: *mut super::minwindef::HKEY, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Open_DevNode_Key_Ex(dndevnode : DEVINST, samdesired : super::winreg::REGSAM, ulhardwareprofile : u32, disposition : REGDISPOSITION, phkdevice : *mut super::minwindef::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_DevNode_Key_Ex(dndevnode, samdesired, ulhardwareprofile, disposition, phkdevice as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Device_Interface_KeyA<P0>(pszdeviceinterface: P0, samdesired: super::winreg::REGSAM, disposition: REGDISPOSITION, phkdeviceinterface: *mut super::minwindef::HKEY, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_KeyA(pszdeviceinterface : windows_core::PCSTR, samdesired : super::winreg::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : *mut super::minwindef::HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_KeyA(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Device_Interface_KeyW<P0>(pszdeviceinterface: P0, samdesired: super::winreg::REGSAM, disposition: REGDISPOSITION, phkdeviceinterface: *mut super::minwindef::HKEY, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_KeyW(pszdeviceinterface : windows_core::PCWSTR, samdesired : super::winreg::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : *mut super::minwindef::HKEY, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_KeyW(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Device_Interface_Key_ExA<P0>(pszdeviceinterface: P0, samdesired: super::winreg::REGSAM, disposition: REGDISPOSITION, phkdeviceinterface: *mut super::minwindef::HKEY, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_Key_ExA(pszdeviceinterface : windows_core::PCSTR, samdesired : super::winreg::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : *mut super::minwindef::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_Key_ExA(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Device_Interface_Key_ExW<P0>(pszdeviceinterface: P0, samdesired: super::winreg::REGSAM, disposition: REGDISPOSITION, phkdeviceinterface: *mut super::minwindef::HKEY, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_Key_ExW(pszdeviceinterface : windows_core::PCWSTR, samdesired : super::winreg::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : *mut super::minwindef::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_Key_ExW(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "cfg")]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTreeA(dnancestor: DEVINST, pvetotype: Option<*mut super::cfg::PNP_VETO_TYPE>, pszvetoname: Option<&mut [u8]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTreeA(dnancestor : DEVINST, pvetotype : *mut super::cfg::PNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTreeA(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(feature = "cfg")]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTreeW(dnancestor: DEVINST, pvetotype: Option<*mut super::cfg::PNP_VETO_TYPE>, pszvetoname: Option<&mut [u16]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTreeW(dnancestor : DEVINST, pvetotype : *mut super::cfg::PNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTreeW(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(all(feature = "cfg", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTree_ExA(dnancestor: DEVINST, pvetotype: Option<*mut super::cfg::PNP_VETO_TYPE>, pszvetoname: Option<&mut [u8]>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTree_ExA(dnancestor : DEVINST, pvetotype : *mut super::cfg::PNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTree_ExA(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "cfg", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTree_ExW(dnancestor: DEVINST, pvetotype: Option<*mut super::cfg::PNP_VETO_TYPE>, pszvetoname: Option<&mut [u16]>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTree_ExW(dnancestor : DEVINST, pvetotype : *mut super::cfg::PNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTree_ExW(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Data(pdata: *mut core::ffi::c_void, datalen: u32, dndevinst: DEVINST, resourceid: RESOURCEID, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Data(pdata : *mut core::ffi::c_void, datalen : u32, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Data(pdata as _, datalen, dndevinst, resourceid, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Data_Ex(pdata: *mut core::ffi::c_void, datalen: u32, dndevinst: DEVINST, resourceid: RESOURCEID, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Data_Ex(pdata : *mut core::ffi::c_void, datalen : u32, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Data_Ex(pdata as _, datalen, dndevinst, resourceid, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Size(pulsize: *mut u32, dndevinst: DEVINST, resourceid: RESOURCEID, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Size(pulsize : *mut u32, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Size(pulsize as _, dndevinst, resourceid, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Size_Ex(pulsize: *mut u32, dndevinst: DEVINST, resourceid: RESOURCEID, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Size_Ex(pulsize : *mut u32, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Size_Ex(pulsize as _, dndevinst, resourceid, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Query_Remove_SubTree(dnancestor: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Remove_SubTree(dnancestor : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Query_Remove_SubTree(dnancestor, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Query_Remove_SubTree_Ex(dnancestor: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Remove_SubTree_Ex(dnancestor : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_Remove_SubTree_Ex(dnancestor, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Query_Resource_Conflict_List(pclconflictlist: *mut CONFLICT_LIST, dndevinst: DEVINST, resourceid: RESOURCEID, resourcedata: *const core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Resource_Conflict_List(pclconflictlist : *mut CONFLICT_LIST, dndevinst : DEVINST, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_Resource_Conflict_List(pclconflictlist as _, dndevinst, resourceid, resourcedata, resourcelen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Reenumerate_DevNode(dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Reenumerate_DevNode(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Reenumerate_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Reenumerate_DevNode_Ex(dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Reenumerate_DevNode_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Reenumerate_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Register_Device_Driver(dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_Driver(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Register_Device_Driver(dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Register_Device_Driver_Ex(dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_Driver_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Register_Device_Driver_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Register_Device_InterfaceA<P2>(dndevinst: DEVINST, interfaceclassguid: *const windows_core::GUID, pszreference: P2, pszdeviceinterface: windows_core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_InterfaceA(dndevinst : DEVINST, interfaceclassguid : *const windows_core::GUID, pszreference : windows_core::PCSTR, pszdeviceinterface : windows_core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Register_Device_InterfaceA(dndevinst, interfaceclassguid, pszreference.param().abi(), core::mem::transmute(pszdeviceinterface), pullength as _, ulflags) }
}
#[inline]
pub unsafe fn CM_Register_Device_InterfaceW<P2>(dndevinst: DEVINST, interfaceclassguid: *const windows_core::GUID, pszreference: P2, pszdeviceinterface: windows_core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_InterfaceW(dndevinst : DEVINST, interfaceclassguid : *const windows_core::GUID, pszreference : windows_core::PCWSTR, pszdeviceinterface : windows_core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Register_Device_InterfaceW(dndevinst, interfaceclassguid, pszreference.param().abi(), core::mem::transmute(pszdeviceinterface), pullength as _, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Register_Device_Interface_ExA<P2>(dndevinst: DEVINST, interfaceclassguid: *const windows_core::GUID, pszreference: P2, pszdeviceinterface: windows_core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_Interface_ExA(dndevinst : DEVINST, interfaceclassguid : *const windows_core::GUID, pszreference : windows_core::PCSTR, pszdeviceinterface : windows_core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Register_Device_Interface_ExA(dndevinst, interfaceclassguid, pszreference.param().abi(), core::mem::transmute(pszdeviceinterface), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Register_Device_Interface_ExW<P2>(dndevinst: DEVINST, interfaceclassguid: *const windows_core::GUID, pszreference: P2, pszdeviceinterface: windows_core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_Interface_ExW(dndevinst : DEVINST, interfaceclassguid : *const windows_core::GUID, pszreference : windows_core::PCWSTR, pszdeviceinterface : windows_core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Register_Device_Interface_ExW(dndevinst, interfaceclassguid, pszreference.param().abi(), core::mem::transmute(pszdeviceinterface), pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Register_Notification(pfilter: *const CM_NOTIFY_FILTER, pcontext: Option<*const core::ffi::c_void>, pcallback: PCM_NOTIFY_CALLBACK, pnotifycontext: *mut HCMNOTIFICATION) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Register_Notification(pfilter : *const CM_NOTIFY_FILTER, pcontext : *const core::ffi::c_void, pcallback : PCM_NOTIFY_CALLBACK, pnotifycontext : *mut HCMNOTIFICATION) -> CONFIGRET);
    unsafe { CM_Register_Notification(pfilter, pcontext.unwrap_or(core::mem::zeroed()) as _, pcallback, pnotifycontext as _) }
}
#[inline]
pub unsafe fn CM_Remove_SubTree(dnancestor: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Remove_SubTree(dnancestor : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Remove_SubTree(dnancestor, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Remove_SubTree_Ex(dnancestor: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Remove_SubTree_Ex(dnancestor : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Remove_SubTree_Ex(dnancestor, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "cfg")]
#[inline]
pub unsafe fn CM_Request_Device_EjectA(dndevinst: DEVINST, pvetotype: Option<*mut super::cfg::PNP_VETO_TYPE>, pszvetoname: Option<&mut [u8]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Device_EjectA(dndevinst : DEVINST, pvetotype : *mut super::cfg::PNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Request_Device_EjectA(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(feature = "cfg")]
#[inline]
pub unsafe fn CM_Request_Device_EjectW(dndevinst: DEVINST, pvetotype: Option<*mut super::cfg::PNP_VETO_TYPE>, pszvetoname: Option<&mut [u16]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Device_EjectW(dndevinst : DEVINST, pvetotype : *mut super::cfg::PNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Request_Device_EjectW(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(all(feature = "cfg", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Request_Device_Eject_ExA(dndevinst: DEVINST, pvetotype: Option<*mut super::cfg::PNP_VETO_TYPE>, pszvetoname: Option<&mut [u8]>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Device_Eject_ExA(dndevinst : DEVINST, pvetotype : *mut super::cfg::PNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Request_Device_Eject_ExA(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "cfg", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Request_Device_Eject_ExW(dndevinst: DEVINST, pvetotype: Option<*mut super::cfg::PNP_VETO_TYPE>, pszvetoname: Option<&mut [u16]>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Device_Eject_ExW(dndevinst : DEVINST, pvetotype : *mut super::cfg::PNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Request_Device_Eject_ExW(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pszvetoname.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszvetoname.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Request_Eject_PC() -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Eject_PC() -> CONFIGRET);
    unsafe { CM_Request_Eject_PC() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Request_Eject_PC_Ex(hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Eject_PC_Ex(hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Request_Eject_PC_Ex(hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Run_Detection(ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Run_Detection(ulflags : u32) -> CONFIGRET);
    unsafe { CM_Run_Detection(ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Run_Detection_Ex(ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Run_Detection_Ex(ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Run_Detection_Ex(ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn CM_Set_Class_PropertyW(classguid: *const windows_core::GUID, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: super::devpropdef::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Class_PropertyW(classguid : *const windows_core::GUID, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_Class_PropertyW(classguid, propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Set_Class_Property_ExW(classguid: *const windows_core::GUID, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: super::devpropdef::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Class_Property_ExW(classguid : *const windows_core::GUID, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_Class_Property_ExW(classguid, propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Set_Class_Registry_PropertyA(classguid: *const windows_core::GUID, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_Class_Registry_PropertyA(classguid : *const windows_core::GUID, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_Class_Registry_PropertyA(classguid, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Set_Class_Registry_PropertyW(classguid: *const windows_core::GUID, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_Class_Registry_PropertyW(classguid : *const windows_core::GUID, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_Class_Registry_PropertyW(classguid, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_DevNode_Problem(dndevinst: DEVINST, ulproblem: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Problem(dndevinst : DEVINST, ulproblem : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Problem(dndevinst, ulproblem, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Set_DevNode_Problem_Ex(dndevinst: DEVINST, ulproblem: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Problem_Ex(dndevinst : DEVINST, ulproblem : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Problem_Ex(dndevinst, ulproblem, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn CM_Set_DevNode_PropertyW(dndevinst: DEVINST, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: super::devpropdef::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_PropertyW(dndevinst : DEVINST, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_DevNode_PropertyW(dndevinst, propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Set_DevNode_Property_ExW(dndevinst: DEVINST, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: super::devpropdef::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Property_ExW(dndevinst : DEVINST, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Property_ExW(dndevinst, propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_PropertyA(dndevinst: DEVINST, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_PropertyA(dndevinst : DEVINST, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_PropertyA(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags) }
}
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_PropertyW(dndevinst: DEVINST, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_PropertyW(dndevinst : DEVINST, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_PropertyW(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_Property_ExA(dndevinst: DEVINST, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_Property_ExA(dndevinst : DEVINST, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_Property_ExA(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_Property_ExW(dndevinst: DEVINST, ulproperty: u32, buffer: Option<*const core::ffi::c_void>, ullength: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_Property_ExW(dndevinst : DEVINST, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_Property_ExW(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "devpropdef")]
#[inline]
pub unsafe fn CM_Set_Device_Interface_PropertyW<P0>(pszdeviceinterface: P0, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: super::devpropdef::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Device_Interface_PropertyW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_Device_Interface_PropertyW(pszdeviceinterface.param().abi(), propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Set_Device_Interface_Property_ExW<P0>(pszdeviceinterface: P0, propertykey: *const super::devpropdef::DEVPROPKEY, propertytype: super::devpropdef::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Device_Interface_Property_ExW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::devpropdef::DEVPROPKEY, propertytype : super::devpropdef::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_Device_Interface_Property_ExW(pszdeviceinterface.param().abi(), propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_HW_Prof(ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof(ulhardwareprofile : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof(ulhardwareprofile, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Set_HW_Prof_Ex(ulhardwareprofile: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_Ex(ulhardwareprofile : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_Ex(ulhardwareprofile, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Set_HW_Prof_FlagsA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_FlagsA(pdeviceid : *const i8, ulconfig : u32, ulvalue : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_FlagsA(pdeviceid, ulconfig, ulvalue, ulflags) }
}
#[inline]
pub unsafe fn CM_Set_HW_Prof_FlagsW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_FlagsW(pdeviceid : *const u16, ulconfig : u32, ulvalue : u32, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_FlagsW(pdeviceid, ulconfig, ulvalue, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Set_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_Flags_ExA(pdeviceid : *const i8, ulconfig : u32, ulvalue : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_Flags_ExA(pdeviceid, ulconfig, ulvalue, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Set_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_Flags_ExW(pdeviceid : *const u16, ulconfig : u32, ulvalue : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_Flags_ExW(pdeviceid, ulconfig, ulvalue, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Setup_DevNode(dndevinst: DEVINST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Setup_DevNode(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Setup_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Setup_DevNode_Ex(dndevinst: DEVINST, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Setup_DevNode_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Setup_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Test_Range_Available(ullstartvalue: super::winnt::DWORDLONG, ullendvalue: super::winnt::DWORDLONG, rlh: RANGE_LIST, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Test_Range_Available(ullstartvalue : super::winnt::DWORDLONG, ullendvalue : super::winnt::DWORDLONG, rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Test_Range_Available(ullstartvalue, ullendvalue, rlh, ulflags) }
}
#[inline]
pub unsafe fn CM_Uninstall_DevNode(dndevinst: DEVNODE, ulflags: u32) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Uninstall_DevNode(dndevinst : DEVNODE, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Uninstall_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Uninstall_DevNode_Ex(dndevinst: DEVNODE, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Uninstall_DevNode_Ex(dndevinst : DEVNODE, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Uninstall_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Unregister_Device_InterfaceA<P0>(pszdeviceinterface: P0, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Unregister_Device_InterfaceA(pszdeviceinterface : windows_core::PCSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Unregister_Device_InterfaceA(pszdeviceinterface.param().abi(), ulflags) }
}
#[inline]
pub unsafe fn CM_Unregister_Device_InterfaceW<P0>(pszdeviceinterface: P0, ulflags: u32) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Unregister_Device_InterfaceW(pszdeviceinterface : windows_core::PCWSTR, ulflags : u32) -> CONFIGRET);
    unsafe { CM_Unregister_Device_InterfaceW(pszdeviceinterface.param().abi(), ulflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Unregister_Device_Interface_ExA<P0>(pszdeviceinterface: P0, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Unregister_Device_Interface_ExA(pszdeviceinterface : windows_core::PCSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Unregister_Device_Interface_ExA(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CM_Unregister_Device_Interface_ExW<P0>(pszdeviceinterface: P0, ulflags: u32, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Unregister_Device_Interface_ExW(pszdeviceinterface : windows_core::PCWSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Unregister_Device_Interface_ExW(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Unregister_Notification(notifycontext: HCMNOTIFICATION) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Unregister_Notification(notifycontext : HCMNOTIFICATION) -> CONFIGRET);
    unsafe { CM_Unregister_Notification(notifycontext) }
}
pub const ALLOC_LOG_CONF: u32 = 2;
pub const BASIC_LOG_CONF: u32 = 0;
pub const BOOT_LOG_CONF: u32 = 3;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct BUSNUMBER_DES {
    pub BUSD_Count: u32,
    pub BUSD_Type: u32,
    pub BUSD_Flags: u32,
    pub BUSD_Alloc_Base: u32,
    pub BUSD_Alloc_End: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct BUSNUMBER_RANGE {
    pub BUSR_Min: u32,
    pub BUSR_Max: u32,
    pub BUSR_nBusNumbers: u32,
    pub BUSR_Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BUSNUMBER_RESOURCE {
    pub BusNumber_Header: BUSNUMBER_DES,
    pub BusNumber_Data: [BUSNUMBER_RANGE; 1],
}
impl Default for BUSNUMBER_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CM_ADD_ID_BITS: u32 = 1;
pub const CM_ADD_ID_COMPATIBLE: u32 = 1;
pub const CM_ADD_ID_HARDWARE: u32 = 0;
pub const CM_ADD_RANGE_ADDIFCONFLICT: u32 = 0;
pub const CM_ADD_RANGE_BITS: u32 = 1;
pub const CM_ADD_RANGE_DONOTADDIFCONFLICT: u32 = 1;
pub const CM_CDFLAGS_DRIVER: u32 = 1;
pub const CM_CDFLAGS_RESERVED: u32 = 4;
pub const CM_CDFLAGS_ROOT_OWNED: u32 = 2;
pub const CM_CDMASK_DESCRIPTION: u32 = 8;
pub const CM_CDMASK_DEVINST: u32 = 1;
pub const CM_CDMASK_FLAGS: u32 = 4;
pub const CM_CDMASK_RESDES: u32 = 2;
pub const CM_CDMASK_VALID: u32 = 15;
pub const CM_CLASS_PROPERTY_BITS: u32 = 1;
pub const CM_CLASS_PROPERTY_INSTALLER: u32 = 0;
pub const CM_CLASS_PROPERTY_INTERFACE: u32 = 1;
pub const CM_CREATE_DEVINST_BITS: u32 = 15;
pub const CM_CREATE_DEVINST_DO_NOT_INSTALL: u32 = 8;
pub const CM_CREATE_DEVINST_GENERATE_ID: u32 = 4;
pub const CM_CREATE_DEVINST_NORMAL: u32 = 0;
pub const CM_CREATE_DEVINST_NO_WAIT_INSTALL: u32 = 1;
pub const CM_CREATE_DEVINST_PHANTOM: u32 = 2;
pub const CM_CREATE_DEVNODE_BITS: u32 = 15;
pub const CM_CREATE_DEVNODE_DO_NOT_INSTALL: u32 = 8;
pub const CM_CREATE_DEVNODE_GENERATE_ID: u32 = 4;
pub const CM_CREATE_DEVNODE_NORMAL: u32 = 0;
pub const CM_CREATE_DEVNODE_NO_WAIT_INSTALL: u32 = 1;
pub const CM_CREATE_DEVNODE_PHANTOM: u32 = 2;
pub const CM_CRP_CHARACTERISTICS: u32 = 28;
pub const CM_CRP_DEVTYPE: u32 = 26;
pub const CM_CRP_EXCLUSIVE: u32 = 27;
pub const CM_CRP_LOWERFILTERS: u32 = 19;
pub const CM_CRP_MAX: u32 = 37;
pub const CM_CRP_MIN: u32 = 1;
pub const CM_CRP_SECURITY: u32 = 24;
pub const CM_CRP_SECURITY_SDS: u32 = 25;
pub const CM_CRP_UPPERFILTERS: u32 = 18;
pub const CM_CUSTOMDEVPROP_BITS: u32 = 1;
pub const CM_CUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1;
pub const CM_DELETE_CLASS_BITS: u32 = 3;
pub const CM_DELETE_CLASS_INTERFACE: u32 = 2;
pub const CM_DELETE_CLASS_ONLY: u32 = 0;
pub const CM_DELETE_CLASS_SUBKEYS: u32 = 1;
pub const CM_DETECT_BITS: u32 = 2147483655;
pub const CM_DETECT_CRASHED: u32 = 2;
pub const CM_DETECT_HWPROF_FIRST_BOOT: u32 = 4;
pub const CM_DETECT_NEW_PROFILE: u32 = 1;
pub const CM_DETECT_RUN: u32 = 2147483648;
pub const CM_DEVCAP_DOCKDEVICE: u32 = 8;
pub const CM_DEVCAP_EJECTSUPPORTED: u32 = 2;
pub const CM_DEVCAP_HARDWAREDISABLED: u32 = 256;
pub const CM_DEVCAP_LOCKSUPPORTED: u32 = 1;
pub const CM_DEVCAP_NONDYNAMIC: u32 = 512;
pub const CM_DEVCAP_RAWDEVICEOK: u32 = 64;
pub const CM_DEVCAP_REMOVABLE: u32 = 4;
pub const CM_DEVCAP_SECUREDEVICE: u32 = 1024;
pub const CM_DEVCAP_SILENTINSTALL: u32 = 32;
pub const CM_DEVCAP_SURPRISEREMOVALOK: u32 = 128;
pub const CM_DEVCAP_UNIQUEID: u32 = 16;
pub const CM_DISABLE_ABSOLUTE: u32 = 1;
pub const CM_DISABLE_BITS: u32 = 15;
pub const CM_DISABLE_HARDWARE: u32 = 2;
pub const CM_DISABLE_PERSIST: u32 = 8;
pub const CM_DISABLE_POLITE: u32 = 0;
pub const CM_DISABLE_UI_NOT_OK: u32 = 4;
pub const CM_DRP_ADDRESS: u32 = 29;
pub const CM_DRP_BASE_CONTAINERID: u32 = 37;
pub const CM_DRP_BUSNUMBER: u32 = 22;
pub const CM_DRP_BUSTYPEGUID: u32 = 20;
pub const CM_DRP_CAPABILITIES: u32 = 16;
pub const CM_DRP_CHARACTERISTICS: u32 = 28;
pub const CM_DRP_CLASS: u32 = 8;
pub const CM_DRP_CLASSGUID: u32 = 9;
pub const CM_DRP_COMPATIBLEIDS: u32 = 3;
pub const CM_DRP_CONFIGFLAGS: u32 = 11;
pub const CM_DRP_DEVICEDESC: u32 = 1;
pub const CM_DRP_DEVICE_POWER_DATA: u32 = 31;
pub const CM_DRP_DEVTYPE: u32 = 26;
pub const CM_DRP_DRIVER: u32 = 10;
pub const CM_DRP_ENUMERATOR_NAME: u32 = 23;
pub const CM_DRP_EXCLUSIVE: u32 = 27;
pub const CM_DRP_FRIENDLYNAME: u32 = 13;
pub const CM_DRP_HARDWAREID: u32 = 2;
pub const CM_DRP_INSTALL_STATE: u32 = 35;
pub const CM_DRP_LEGACYBUSTYPE: u32 = 21;
pub const CM_DRP_LOCATION_INFORMATION: u32 = 14;
pub const CM_DRP_LOCATION_PATHS: u32 = 36;
pub const CM_DRP_LOWERFILTERS: u32 = 19;
pub const CM_DRP_MAX: u32 = 37;
pub const CM_DRP_MFG: u32 = 12;
pub const CM_DRP_MIN: u32 = 1;
pub const CM_DRP_PHYSICAL_DEVICE_OBJECT_NAME: u32 = 15;
pub const CM_DRP_REMOVAL_POLICY: u32 = 32;
pub const CM_DRP_REMOVAL_POLICY_HW_DEFAULT: u32 = 33;
pub const CM_DRP_REMOVAL_POLICY_OVERRIDE: u32 = 34;
pub const CM_DRP_SECURITY: u32 = 24;
pub const CM_DRP_SECURITY_SDS: u32 = 25;
pub const CM_DRP_SERVICE: u32 = 5;
pub const CM_DRP_UI_NUMBER: u32 = 17;
pub const CM_DRP_UI_NUMBER_DESC_FORMAT: u32 = 30;
pub const CM_DRP_UNUSED0: u32 = 4;
pub const CM_DRP_UNUSED1: u32 = 6;
pub const CM_DRP_UNUSED2: u32 = 7;
pub const CM_DRP_UPPERFILTERS: u32 = 18;
pub const CM_ENUMERATE_CLASSES_BITS: u32 = 1;
pub const CM_ENUMERATE_CLASSES_INSTALLER: u32 = 0;
pub const CM_ENUMERATE_CLASSES_INTERFACE: u32 = 1;
pub const CM_GETIDLIST_DONOTGENERATE: u32 = 268435520;
pub const CM_GETIDLIST_FILTER_BITS: u32 = 268436479;
pub const CM_GETIDLIST_FILTER_BUSRELATIONS: u32 = 32;
pub const CM_GETIDLIST_FILTER_CLASS: u32 = 512;
pub const CM_GETIDLIST_FILTER_EJECTRELATIONS: u32 = 4;
pub const CM_GETIDLIST_FILTER_ENUMERATOR: u32 = 1;
pub const CM_GETIDLIST_FILTER_NONE: u32 = 0;
pub const CM_GETIDLIST_FILTER_POWERRELATIONS: u32 = 16;
pub const CM_GETIDLIST_FILTER_PRESENT: u32 = 256;
pub const CM_GETIDLIST_FILTER_REMOVALRELATIONS: u32 = 8;
pub const CM_GETIDLIST_FILTER_SERVICE: u32 = 2;
pub const CM_GETIDLIST_FILTER_TRANSPORTRELATIONS: u32 = 128;
pub const CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES: u32 = 1;
pub const CM_GET_DEVICE_INTERFACE_LIST_BITS: u32 = 1;
pub const CM_GET_DEVICE_INTERFACE_LIST_PRESENT: u32 = 0;
pub const CM_GLOBAL_STATE_CAN_DO_UI: u32 = 1;
pub const CM_GLOBAL_STATE_DETECTION_PENDING: u32 = 16;
pub const CM_GLOBAL_STATE_ON_BIG_STACK: u32 = 2;
pub const CM_GLOBAL_STATE_REBOOT_REQUIRED: u32 = 32;
pub const CM_GLOBAL_STATE_SERVICES_AVAILABLE: u32 = 4;
pub const CM_GLOBAL_STATE_SHUTTING_DOWN: u32 = 8;
pub const CM_HWPI_DOCKED: u32 = 2;
pub const CM_HWPI_NOT_DOCKABLE: u32 = 0;
pub const CM_HWPI_UNDOCKED: u32 = 1;
pub const CM_INSTALL_STATE_FAILED_INSTALL: u32 = 2;
pub const CM_INSTALL_STATE_FINISH_INSTALL: u32 = 3;
pub const CM_INSTALL_STATE_INSTALLED: u32 = 0;
pub const CM_INSTALL_STATE_NEEDS_REINSTALL: u32 = 1;
pub const CM_LOCATE_DEVINST_BITS: u32 = 7;
pub const CM_LOCATE_DEVINST_CANCELREMOVE: u32 = 2;
pub const CM_LOCATE_DEVINST_NORMAL: u32 = 0;
pub const CM_LOCATE_DEVINST_NOVALIDATION: u32 = 4;
pub const CM_LOCATE_DEVINST_PHANTOM: u32 = 1;
pub const CM_LOCATE_DEVNODE_BITS: u32 = 7;
pub const CM_LOCATE_DEVNODE_CANCELREMOVE: u32 = 2;
pub const CM_LOCATE_DEVNODE_NORMAL: u32 = 0;
pub const CM_LOCATE_DEVNODE_NOVALIDATION: u32 = 4;
pub const CM_LOCATE_DEVNODE_PHANTOM: u32 = 1;
pub const CM_NAME_ATTRIBUTE_NAME_RETRIEVED_FROM_DEVICE: u32 = 1;
pub const CM_NAME_ATTRIBUTE_USER_ASSIGNED_NAME: u32 = 2;
pub type CM_NOTIFY_ACTION = i32;
pub const CM_NOTIFY_ACTION_DEVICECUSTOMEVENT: CM_NOTIFY_ACTION = 6;
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEENUMERATED: CM_NOTIFY_ACTION = 7;
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEREMOVED: CM_NOTIFY_ACTION = 9;
pub const CM_NOTIFY_ACTION_DEVICEINSTANCESTARTED: CM_NOTIFY_ACTION = 8;
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEARRIVAL: CM_NOTIFY_ACTION = 0;
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEREMOVAL: CM_NOTIFY_ACTION = 1;
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVE: CM_NOTIFY_ACTION = 2;
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVEFAILED: CM_NOTIFY_ACTION = 3;
pub const CM_NOTIFY_ACTION_DEVICEREMOVECOMPLETE: CM_NOTIFY_ACTION = 5;
pub const CM_NOTIFY_ACTION_DEVICEREMOVEPENDING: CM_NOTIFY_ACTION = 4;
pub const CM_NOTIFY_ACTION_MAX: CM_NOTIFY_ACTION = 10;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_NOTIFY_EVENT_DATA {
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_EVENT_DATA_0,
}
impl Default for CM_NOTIFY_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CM_NOTIFY_EVENT_DATA_0 {
    pub DeviceInterface: CM_NOTIFY_EVENT_DATA_0_0,
    pub DeviceHandle: CM_NOTIFY_EVENT_DATA_0_1,
    pub DeviceInstance: CM_NOTIFY_EVENT_DATA_0_2,
}
impl Default for CM_NOTIFY_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_NOTIFY_EVENT_DATA_0_0 {
    pub ClassGuid: windows_core::GUID,
    pub SymbolicLink: [u16; 1],
}
impl Default for CM_NOTIFY_EVENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_NOTIFY_EVENT_DATA_0_1 {
    pub EventGuid: windows_core::GUID,
    pub NameOffset: i32,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl Default for CM_NOTIFY_EVENT_DATA_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_NOTIFY_EVENT_DATA_0_2 {
    pub InstanceId: [u16; 1],
}
impl Default for CM_NOTIFY_EVENT_DATA_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct CM_NOTIFY_FILTER {
    pub cbSize: u32,
    pub Flags: u32,
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_FILTER_0,
}
#[cfg(feature = "winnt")]
impl Default for CM_NOTIFY_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union CM_NOTIFY_FILTER_0 {
    pub DeviceInterface: CM_NOTIFY_FILTER_0_0,
    pub DeviceHandle: CM_NOTIFY_FILTER_0_1,
    pub DeviceInstance: CM_NOTIFY_FILTER_0_2,
}
#[cfg(feature = "winnt")]
impl Default for CM_NOTIFY_FILTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_NOTIFY_FILTER_0_0 {
    pub ClassGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_NOTIFY_FILTER_0_1 {
    pub hTarget: super::winnt::HANDLE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_NOTIFY_FILTER_0_2 {
    pub InstanceId: [u16; 200],
}
#[cfg(feature = "winnt")]
impl Default for CM_NOTIFY_FILTER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CM_NOTIFY_FILTER_FLAG_ALL_DEVICE_INSTANCES: u32 = 2;
pub const CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES: u32 = 1;
pub type CM_NOTIFY_FILTER_TYPE = i32;
pub const CM_NOTIFY_FILTER_TYPE_DEVICEHANDLE: CM_NOTIFY_FILTER_TYPE = 1;
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINSTANCE: CM_NOTIFY_FILTER_TYPE = 2;
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINTERFACE: CM_NOTIFY_FILTER_TYPE = 0;
pub const CM_NOTIFY_FILTER_TYPE_MAX: CM_NOTIFY_FILTER_TYPE = 3;
pub const CM_NOTIFY_FILTER_VALID_FLAGS: u32 = 3;
pub const CM_OPEN_CLASS_KEY_BITS: u32 = 1;
pub const CM_OPEN_CLASS_KEY_INSTALLER: u32 = 0;
pub const CM_OPEN_CLASS_KEY_INTERFACE: u32 = 1;
pub const CM_QUERY_ARBITRATOR_BITS: u32 = 1;
pub const CM_QUERY_ARBITRATOR_RAW: u32 = 0;
pub const CM_QUERY_ARBITRATOR_TRANSLATED: u32 = 1;
pub const CM_QUERY_REMOVE_BITS: u32 = 1;
pub const CM_QUERY_REMOVE_UI_NOT_OK: u32 = 1;
pub const CM_QUERY_REMOVE_UI_OK: u32 = 0;
pub const CM_REENUMERATE_ASYNCHRONOUS: u32 = 4;
pub const CM_REENUMERATE_BITS: u32 = 7;
pub const CM_REENUMERATE_NORMAL: u32 = 0;
pub const CM_REENUMERATE_RETRY_INSTALLATION: u32 = 2;
pub const CM_REENUMERATE_SYNCHRONOUS: u32 = 1;
pub const CM_REGISTER_DEVICE_DRIVER_BITS: u32 = 3;
pub const CM_REGISTER_DEVICE_DRIVER_DISABLEABLE: u32 = 1;
pub const CM_REGISTER_DEVICE_DRIVER_REMOVABLE: u32 = 2;
pub const CM_REGISTER_DEVICE_DRIVER_STATIC: u32 = 0;
pub const CM_REGISTRY_BITS: u32 = 769;
pub const CM_REGISTRY_CONFIG: u32 = 512;
pub const CM_REGISTRY_HARDWARE: u32 = 0;
pub const CM_REGISTRY_SOFTWARE: u32 = 1;
pub const CM_REGISTRY_USER: u32 = 256;
pub const CM_REMOVAL_POLICY_EXPECT_NO_REMOVAL: u32 = 1;
pub const CM_REMOVAL_POLICY_EXPECT_ORDERLY_REMOVAL: u32 = 2;
pub const CM_REMOVAL_POLICY_EXPECT_SURPRISE_REMOVAL: u32 = 3;
pub const CM_REMOVE_BITS: u32 = 7;
pub const CM_REMOVE_DISABLE: u32 = 4;
pub const CM_REMOVE_NO_RESTART: u32 = 2;
pub const CM_REMOVE_UI_NOT_OK: u32 = 1;
pub const CM_REMOVE_UI_OK: u32 = 0;
pub const CM_RESDES_WIDTH_32: u32 = 1;
pub const CM_RESDES_WIDTH_64: u32 = 2;
pub const CM_RESDES_WIDTH_BITS: u32 = 3;
pub const CM_RESDES_WIDTH_DEFAULT: u32 = 0;
pub const CM_SETUP_BITS: u32 = 15;
pub const CM_SETUP_DEVINST_CONFIG: u32 = 5;
pub const CM_SETUP_DEVINST_CONFIG_CLASS: u32 = 6;
pub const CM_SETUP_DEVINST_CONFIG_EXTENSIONS: u32 = 7;
pub const CM_SETUP_DEVINST_CONFIG_RESET: u32 = 8;
pub const CM_SETUP_DEVINST_READY: u32 = 0;
pub const CM_SETUP_DEVINST_RESET: u32 = 4;
pub const CM_SETUP_DEVNODE_CONFIG: u32 = 5;
pub const CM_SETUP_DEVNODE_CONFIG_CLASS: u32 = 6;
pub const CM_SETUP_DEVNODE_CONFIG_EXTENSIONS: u32 = 7;
pub const CM_SETUP_DEVNODE_CONFIG_RESET: u32 = 8;
pub const CM_SETUP_DEVNODE_READY: u32 = 0;
pub const CM_SETUP_DEVNODE_RESET: u32 = 4;
pub const CM_SETUP_DOWNLOAD: u32 = 1;
pub const CM_SETUP_PROP_CHANGE: u32 = 3;
pub const CM_SETUP_WRITE_LOG_CONFS: u32 = 2;
pub const CM_SET_DEVINST_PROBLEM_BITS: u32 = 1;
pub const CM_SET_DEVINST_PROBLEM_NORMAL: u32 = 0;
pub const CM_SET_DEVINST_PROBLEM_OVERRIDE: u32 = 1;
pub const CM_SET_DEVNODE_PROBLEM_BITS: u32 = 1;
pub const CM_SET_DEVNODE_PROBLEM_NORMAL: u32 = 0;
pub const CM_SET_DEVNODE_PROBLEM_OVERRIDE: u32 = 1;
pub const CM_SET_HW_PROF_FLAGS_BITS: u32 = 1;
pub const CM_SET_HW_PROF_FLAGS_UI_NOT_OK: u32 = 1;
pub const CONFIGMG_VERSION: u32 = 1024;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CONFIGRET(pub RETURN_TYPE);
pub type CONFLICT_DETAILS = CONFLICT_DETAILS_A;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONFLICT_DETAILS_A {
    pub CD_ulSize: u32,
    pub CD_ulMask: u32,
    pub CD_dnDevInst: DEVINST,
    pub CD_rdResDes: RES_DES,
    pub CD_ulFlags: u32,
    pub CD_szDescription: [i8; 260],
}
impl Default for CONFLICT_DETAILS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONFLICT_DETAILS_W {
    pub CD_ulSize: u32,
    pub CD_ulMask: u32,
    pub CD_dnDevInst: DEVINST,
    pub CD_rdResDes: RES_DES,
    pub CD_ulFlags: u32,
    pub CD_szDescription: [u16; 260],
}
impl Default for CONFLICT_DETAILS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CONFLICT_LIST(pub usize);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CONNECTION_DES {
    pub COND_Type: u32,
    pub COND_Flags: u32,
    pub COND_Class: u8,
    pub COND_ClassType: u8,
    pub COND_Reserved1: u8,
    pub COND_Reserved2: u8,
    pub COND_Id: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONNECTION_RESOURCE {
    pub Connection_Header: CONNECTION_DES,
}
pub const CR_ACCESS_DENIED: u32 = 51;
pub const CR_ALREADY_SUCH_DEVINST: u32 = 16;
pub const CR_ALREADY_SUCH_DEVNODE: u32 = 16;
pub const CR_APM_VETOED: u32 = 24;
pub const CR_BUFFER_SMALL: u32 = 26;
pub const CR_CALL_NOT_IMPLEMENTED: u32 = 52;
pub const CR_CANT_SHARE_IRQ: u32 = 43;
pub const CR_CREATE_BLOCKED: u32 = 21;
pub const CR_DEFAULT: u32 = 1;
pub const CR_DEVICE_INTERFACE_ACTIVE: u32 = 54;
pub const CR_DEVICE_NOT_THERE: u32 = 36;
pub const CR_DEVINST_HAS_REQS: u32 = 10;
pub const CR_DEVLOADER_NOT_READY: u32 = 33;
pub const CR_DEVNODE_HAS_REQS: u32 = 10;
pub const CR_DLVXD_NOT_FOUND: u32 = 12;
pub const CR_FAILURE: u32 = 19;
pub const CR_FREE_RESOURCES: u32 = 41;
pub const CR_INVALID_API: u32 = 32;
pub const CR_INVALID_ARBITRATOR: u32 = 8;
pub const CR_INVALID_CONFLICT_LIST: u32 = 57;
pub const CR_INVALID_DATA: u32 = 31;
pub const CR_INVALID_DEVICE_ID: u32 = 30;
pub const CR_INVALID_DEVINST: u32 = 5;
pub const CR_INVALID_DEVNODE: u32 = 5;
pub const CR_INVALID_FLAG: u32 = 4;
pub const CR_INVALID_INDEX: u32 = 58;
pub const CR_INVALID_LOAD_TYPE: u32 = 25;
pub const CR_INVALID_LOG_CONF: u32 = 7;
pub const CR_INVALID_MACHINENAME: u32 = 47;
pub const CR_INVALID_NODELIST: u32 = 9;
pub const CR_INVALID_POINTER: u32 = 3;
pub const CR_INVALID_PRIORITY: u32 = 39;
pub const CR_INVALID_PROPERTY: u32 = 53;
pub const CR_INVALID_RANGE: u32 = 18;
pub const CR_INVALID_RANGE_LIST: u32 = 17;
pub const CR_INVALID_REFERENCE_STRING: u32 = 56;
pub const CR_INVALID_RESOURCEID: u32 = 11;
pub const CR_INVALID_RES_DES: u32 = 6;
pub const CR_INVALID_STRUCTURE_SIZE: u32 = 59;
pub const CR_MACHINE_UNAVAILABLE: u32 = 49;
pub const CR_NEED_RESTART: u32 = 34;
pub const CR_NOT_DISABLEABLE: u32 = 40;
pub const CR_NOT_SYSTEM_VM: u32 = 22;
pub const CR_NO_ARBITRATOR: u32 = 27;
pub const CR_NO_CM_SERVICES: u32 = 50;
pub const CR_NO_DEPENDENT: u32 = 44;
pub const CR_NO_MORE_HW_PROFILES: u32 = 35;
pub const CR_NO_MORE_LOG_CONF: u32 = 14;
pub const CR_NO_MORE_RES_DES: u32 = 15;
pub const CR_NO_REGISTRY_HANDLE: u32 = 28;
pub const CR_NO_SUCH_DEVICE_INTERFACE: u32 = 55;
pub const CR_NO_SUCH_DEVINST: u32 = 13;
pub const CR_NO_SUCH_DEVNODE: u32 = 13;
pub const CR_NO_SUCH_LOGICAL_DEV: u32 = 20;
pub const CR_NO_SUCH_REGISTRY_KEY: u32 = 46;
pub const CR_NO_SUCH_VALUE: u32 = 37;
pub const CR_OUT_OF_MEMORY: u32 = 2;
pub const CR_QUERY_VETOED: u32 = 42;
pub const CR_REGISTRY_ERROR: u32 = 29;
pub const CR_REMOTE_COMM_FAILURE: u32 = 48;
pub const CR_REMOVE_VETOED: u32 = 23;
pub const CR_SAME_RESOURCES: u32 = 45;
pub const CR_SUCCESS: u32 = 0;
pub const CR_WRONG_TYPE: u32 = 38;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CS_DES {
    pub CSD_SignatureLength: u32,
    pub CSD_LegacyDataOffset: u32,
    pub CSD_LegacyDataSize: u32,
    pub CSD_Flags: u32,
    pub CSD_ClassGuid: windows_core::GUID,
    pub CSD_Signature: [u8; 1],
}
impl Default for CS_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CS_RESOURCE {
    pub CS_Header: CS_DES,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DEVINST(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DEVINSTID(pub DEVINSTID_A);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DEVINSTID_A(pub *mut i8);
impl DEVINSTID_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for DEVINSTID_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DEVINSTID_W(pub *mut u16);
impl DEVINSTID_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for DEVINSTID_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DEVNODE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DEVNODEID(pub DEVNODEID_A);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DEVNODEID_A(pub *mut i8);
impl DEVNODEID_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for DEVNODEID_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DEVNODEID_W(pub *mut u16);
impl DEVNODEID_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for DEVNODEID_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVPRIVATE_DES {
    pub PD_Count: u32,
    pub PD_Type: u32,
    pub PD_Data1: u32,
    pub PD_Data2: u32,
    pub PD_Data3: u32,
    pub PD_Flags: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEVPRIVATE_RANGE {
    pub PR_Data1: u32,
    pub PR_Data2: u32,
    pub PR_Data3: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVPRIVATE_RESOURCE {
    pub PRV_Header: DEVPRIVATE_DES,
    pub PRV_Data: [DEVPRIVATE_RANGE; 1],
}
impl Default for DEVPRIVATE_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DMA_DES {
    pub DD_Count: u32,
    pub DD_Type: u32,
    pub DD_Flags: u32,
    pub DD_Alloc_Chan: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DMA_RANGE {
    pub DR_Min: u32,
    pub DR_Max: u32,
    pub DR_Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_RESOURCE {
    pub DMA_Header: DMA_DES,
    pub DMA_Data: [DMA_RANGE; 1],
}
impl Default for DMA_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILTERED_LOG_CONF: u32 = 1;
pub const FORCED_LOG_CONF: u32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCMNOTIFICATION(pub *mut core::ffi::c_void);
impl HCMNOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HCMNOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMACHINE(pub super::winnt::HANDLE);
pub type HWPROFILEINFO = HWPROFILEINFO_A;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct HWPROFILEINFO_A {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [i8; 80],
    pub HWPI_dwFlags: u32,
}
impl Default for HWPROFILEINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct HWPROFILEINFO_W {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [u16; 80],
    pub HWPI_dwFlags: u32,
}
impl Default for HWPROFILEINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IOA_Local: u32 = 255;
pub const IO_ALIAS_10_BIT_DECODE: u32 = 4;
pub const IO_ALIAS_12_BIT_DECODE: u32 = 16;
pub const IO_ALIAS_16_BIT_DECODE: u32 = 0;
pub const IO_ALIAS_POSITIVE_DECODE: u32 = 255;
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct IO_DES {
    pub IOD_Count: u32,
    pub IOD_Type: u32,
    pub IOD_Alloc_Base: super::winnt::DWORDLONG,
    pub IOD_Alloc_End: super::winnt::DWORDLONG,
    pub IOD_DesFlags: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct IO_RANGE {
    pub IOR_Align: super::winnt::DWORDLONG,
    pub IOR_nPorts: u32,
    pub IOR_Min: super::winnt::DWORDLONG,
    pub IOR_Max: super::winnt::DWORDLONG,
    pub IOR_RangeFlags: u32,
    pub IOR_Alias: super::winnt::DWORDLONG,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE {
    pub IO_Header: IO_DES,
    pub IO_Data: [IO_RANGE; 1],
}
#[cfg(feature = "winnt")]
impl Default for IO_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub type IRQ_DES = IRQ_DES_32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IRQ_DES = IRQ_DES_64;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IRQ_DES_32 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: u32,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IRQ_DES_64 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: u32,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IRQ_RANGE {
    pub IRQR_Min: u32,
    pub IRQR_Max: u32,
    pub IRQR_Flags: u32,
}
#[cfg(target_arch = "x86")]
pub type IRQ_RESOURCE = IRQ_RESOURCE_32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IRQ_RESOURCE = IRQ_RESOURCE_64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IRQ_RESOURCE_32 {
    pub IRQ_Header: IRQ_DES_32,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl Default for IRQ_RESOURCE_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IRQ_RESOURCE_64 {
    pub IRQ_Header: IRQ_DES_64,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl Default for IRQ_RESOURCE_64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LOG_CONF(pub usize);
pub const LOG_CONF_BITS: u32 = 7;
pub const MAX_CLASS_NAME_LEN: u32 = 32;
pub const MAX_CONFIG_VALUE: u32 = 9999;
pub const MAX_DEVICE_ID_LEN: u32 = 200;
pub const MAX_DEVNODE_ID_LEN: u32 = 200;
pub const MAX_DMA_CHANNELS: u32 = 7;
pub const MAX_GUID_STRING_LEN: u32 = 39;
pub const MAX_INSTANCE_VALUE: u32 = 9999;
pub const MAX_IO_PORTS: u32 = 20;
pub const MAX_IRQS: u32 = 7;
pub const MAX_MEM_REGISTERS: u32 = 9;
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct MEM_DES {
    pub MD_Count: u32,
    pub MD_Type: u32,
    pub MD_Alloc_Base: super::winnt::DWORDLONG,
    pub MD_Alloc_End: super::winnt::DWORDLONG,
    pub MD_Flags: u32,
    pub MD_Reserved: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct MEM_LARGE_DES {
    pub MLD_Count: u32,
    pub MLD_Type: u32,
    pub MLD_Alloc_Base: super::winnt::DWORDLONG,
    pub MLD_Alloc_End: super::winnt::DWORDLONG,
    pub MLD_Flags: u32,
    pub MLD_Reserved: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct MEM_LARGE_RANGE {
    pub MLR_Align: super::winnt::DWORDLONG,
    pub MLR_nBytes: u64,
    pub MLR_Min: super::winnt::DWORDLONG,
    pub MLR_Max: super::winnt::DWORDLONG,
    pub MLR_Flags: u32,
    pub MLR_Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MEM_LARGE_RESOURCE {
    pub MEM_LARGE_Header: MEM_LARGE_DES,
    pub MEM_LARGE_Data: [MEM_LARGE_RANGE; 1],
}
#[cfg(feature = "winnt")]
impl Default for MEM_LARGE_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct MEM_RANGE {
    pub MR_Align: super::winnt::DWORDLONG,
    pub MR_nBytes: u32,
    pub MR_Min: super::winnt::DWORDLONG,
    pub MR_Max: super::winnt::DWORDLONG,
    pub MR_Flags: u32,
    pub MR_Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct MEM_RESOURCE {
    pub MEM_Header: MEM_DES,
    pub MEM_Data: [MEM_RANGE; 1],
}
#[cfg(feature = "winnt")]
impl Default for MEM_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MFCARD_DES {
    pub PMF_Count: u32,
    pub PMF_Type: u32,
    pub PMF_Flags: u32,
    pub PMF_ConfigOptions: u8,
    pub PMF_IoResourceIndex: u8,
    pub PMF_Reserved: [u8; 2],
    pub PMF_ConfigRegisterBase: u32,
}
impl Default for MFCARD_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFCARD_RESOURCE {
    pub MfCard_Header: MFCARD_DES,
}
pub const NUM_CR_RESULTS: u32 = 60;
pub const NUM_LOG_CONF: u32 = 6;
pub const OVERRIDE_LOG_CONF: u32 = 5;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBUSNUMBER_DES(pub *mut BUSNUMBER_DES);
impl PBUSNUMBER_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBUSNUMBER_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBUSNUMBER_RANGE(pub *mut BUSNUMBER_RANGE);
impl PBUSNUMBER_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBUSNUMBER_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBUSNUMBER_RESOURCE(pub *mut BUSNUMBER_RESOURCE);
impl PBUSNUMBER_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBUSNUMBER_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct PCCARD_DES {
    pub PCD_Count: u32,
    pub PCD_Type: u32,
    pub PCD_Flags: u32,
    pub PCD_ConfigIndex: u8,
    pub PCD_Reserved: [u8; 3],
    pub PCD_MemoryCardBase1: u32,
    pub PCD_MemoryCardBase2: u32,
    pub PCD_MemoryCardBase: [u32; 2],
    pub PCD_MemoryFlags: [u16; 2],
    pub PCD_IoFlags: [u8; 2],
}
impl Default for PCCARD_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCCARD_RESOURCE {
    pub PcCard_Header: PCCARD_DES,
}
pub const PCD_MAX_IO: u32 = 2;
pub const PCD_MAX_MEMORY: u32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCM_NOTIFY_ACTION(pub *mut CM_NOTIFY_ACTION);
impl PCM_NOTIFY_ACTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCM_NOTIFY_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCM_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(hnotify: HCMNOTIFICATION, context: *const core::ffi::c_void, action: CM_NOTIFY_ACTION, eventdata: *const CM_NOTIFY_EVENT_DATA, eventdatasize: u32) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCM_NOTIFY_EVENT_DATA(pub *mut CM_NOTIFY_EVENT_DATA);
impl PCM_NOTIFY_EVENT_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCM_NOTIFY_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCM_NOTIFY_FILTER(pub *mut CM_NOTIFY_FILTER);
#[cfg(feature = "winnt")]
impl PCM_NOTIFY_FILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PCM_NOTIFY_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCM_NOTIFY_FILTER_TYPE(pub *mut CM_NOTIFY_FILTER_TYPE);
impl PCM_NOTIFY_FILTER_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCM_NOTIFY_FILTER_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PCONFLICT_DETAILS(pub PCONFLICT_DETAILS_A);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONFLICT_DETAILS_A(pub *mut CONFLICT_DETAILS_A);
impl PCONFLICT_DETAILS_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCONFLICT_DETAILS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONFLICT_DETAILS_W(pub *mut CONFLICT_DETAILS_W);
impl PCONFLICT_DETAILS_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCONFLICT_DETAILS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONFLICT_LIST(pub *mut CONFLICT_LIST);
impl PCONFLICT_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCONFLICT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONNECTION_DES(pub *mut CONNECTION_DES);
impl PCONNECTION_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCONNECTION_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONNECTION_RESOURCE(pub *mut CONNECTION_RESOURCE);
impl PCONNECTION_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCONNECTION_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCS_DES(pub *mut CS_DES);
impl PCS_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCS_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCS_RESOURCE(pub *mut CS_RESOURCE);
impl PCS_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCS_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDEVINST(pub *mut DEVNODE);
impl PDEVINST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDEVINST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDEVNODE(pub *mut DEVNODE);
impl PDEVNODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDEVNODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDEVPRIVATE_DES(pub *mut DEVPRIVATE_DES);
impl PDEVPRIVATE_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDEVPRIVATE_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDEVPRIVATE_RANGE(pub *mut DEVPRIVATE_RANGE);
impl PDEVPRIVATE_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDEVPRIVATE_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDEVPRIVATE_RESOURCE(pub *mut DEVPRIVATE_RESOURCE);
impl PDEVPRIVATE_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDEVPRIVATE_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDMA_DES(pub *mut DMA_DES);
impl PDMA_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDMA_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDMA_RANGE(pub *mut DMA_RANGE);
impl PDMA_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDMA_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDMA_RESOURCE(pub *mut DMA_RESOURCE);
impl PDMA_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDMA_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHCMNOTIFICATION(pub *mut HCMNOTIFICATION);
impl PHCMNOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHCMNOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHMACHINE(pub *mut HMACHINE);
#[cfg(feature = "winnt")]
impl PHMACHINE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PHMACHINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PHWPROFILEINFO(pub PHWPROFILEINFO_A);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHWPROFILEINFO_A(pub *mut HWPROFILEINFO_A);
impl PHWPROFILEINFO_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHWPROFILEINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHWPROFILEINFO_W(pub *mut HWPROFILEINFO_W);
impl PHWPROFILEINFO_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHWPROFILEINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIO_DES(pub *mut IO_DES);
#[cfg(feature = "winnt")]
impl PIO_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PIO_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIO_RANGE(pub *mut IO_RANGE);
#[cfg(feature = "winnt")]
impl PIO_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PIO_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIO_RESOURCE(pub *mut IO_RESOURCE);
#[cfg(feature = "winnt")]
impl PIO_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PIO_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIRQ_DES(pub PIRQ_DES_32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIRQ_DES(pub PIRQ_DES_64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIRQ_DES_32(pub *mut IRQ_DES_32);
impl PIRQ_DES_32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIRQ_DES_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIRQ_DES_64(pub *mut IRQ_DES_64);
impl PIRQ_DES_64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIRQ_DES_64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIRQ_RANGE(pub *mut IRQ_RANGE);
impl PIRQ_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIRQ_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIRQ_RESOURCE(pub PIRQ_RESOURCE_32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIRQ_RESOURCE(pub PIRQ_RESOURCE_64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIRQ_RESOURCE_32(pub *mut IRQ_RESOURCE_32);
impl PIRQ_RESOURCE_32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIRQ_RESOURCE_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIRQ_RESOURCE_64(pub *mut IRQ_RESOURCE_64);
impl PIRQ_RESOURCE_64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIRQ_RESOURCE_64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLOG_CONF(pub *mut LOG_CONF);
impl PLOG_CONF {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLOG_CONF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_DES(pub *mut MEM_DES);
#[cfg(feature = "winnt")]
impl PMEM_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PMEM_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_LARGE_DES(pub *mut MEM_LARGE_DES);
#[cfg(feature = "winnt")]
impl PMEM_LARGE_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PMEM_LARGE_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_LARGE_RANGE(pub *mut MEM_LARGE_RANGE);
#[cfg(feature = "winnt")]
impl PMEM_LARGE_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PMEM_LARGE_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_LARGE_RESOURCE(pub *mut MEM_LARGE_RESOURCE);
#[cfg(feature = "winnt")]
impl PMEM_LARGE_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PMEM_LARGE_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_RANGE(pub *mut MEM_RANGE);
#[cfg(feature = "winnt")]
impl PMEM_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PMEM_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_RESOURCE(pub *mut MEM_RESOURCE);
#[cfg(feature = "winnt")]
impl PMEM_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PMEM_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMFCARD_DES(pub *mut MFCARD_DES);
impl PMFCARD_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMFCARD_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMFCARD_RESOURCE(pub *mut MFCARD_RESOURCE);
impl PMFCARD_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMFCARD_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCCARD_DES(pub *mut PCCARD_DES);
impl PPCCARD_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCCARD_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCCARD_RESOURCE(pub *mut PCCARD_RESOURCE);
impl PPCCARD_RESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCCARD_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPRIORITY(pub *mut PRIORITY);
impl PPRIORITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPRIORITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRANGE_ELEMENT(pub *mut RANGE_ELEMENT);
impl PRANGE_ELEMENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRANGE_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRANGE_LIST(pub *mut RANGE_LIST);
impl PRANGE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRANGE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRESOURCEID(pub *mut RESOURCEID);
impl PRESOURCEID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRESOURCEID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRES_DES(pub *mut RES_DES);
impl PRES_DES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRES_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PRIORITY(pub u32);
pub const PRIORITY_BIT: u32 = 8;
pub const PRIORITY_EQUAL_FIRST: u32 = 8;
pub const PRIORITY_EQUAL_LAST: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RANGE_ELEMENT(pub usize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RANGE_LIST(pub usize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct REGDISPOSITION(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RESOURCEID(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RES_DES(pub usize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RETURN_TYPE(pub u32);
pub const RegDisposition_Bits: u32 = 1;
pub const RegDisposition_OpenAlways: u32 = 0;
pub const RegDisposition_OpenExisting: u32 = 1;
pub const ResType_All: u32 = 0;
pub const ResType_BusNumber: u32 = 6;
pub const ResType_ClassSpecific: u32 = 65535;
pub const ResType_Connection: u32 = 32772;
pub const ResType_DMA: u32 = 3;
pub const ResType_DevicePrivate: u32 = 32769;
pub const ResType_DoNotUse: u32 = 5;
pub const ResType_IO: u32 = 2;
pub const ResType_IRQ: u32 = 4;
pub const ResType_Ignored_Bit: u32 = 32768;
pub const ResType_MAX: u32 = 7;
pub const ResType_Mem: u32 = 1;
pub const ResType_MemLarge: u32 = 7;
pub const ResType_MfCardConfig: u32 = 32771;
pub const ResType_None: u32 = 0;
pub const ResType_PcCardConfig: u32 = 32770;
pub const ResType_Reserved: u32 = 32768;
pub const fDD_BYTE: u32 = 0;
pub const fDD_BYTE_AND_WORD: u32 = 3;
pub const fDD_BusMaster: u32 = 4;
pub const fDD_DWORD: u32 = 2;
pub const fDD_NoBusMaster: u32 = 0;
pub const fDD_TypeA: u32 = 8;
pub const fDD_TypeB: u32 = 16;
pub const fDD_TypeF: u32 = 24;
pub const fDD_TypeStandard: u32 = 0;
pub const fDD_WORD: u32 = 1;
pub const fIOD_10_BIT_DECODE: u32 = 4;
pub const fIOD_12_BIT_DECODE: u32 = 8;
pub const fIOD_16_BIT_DECODE: u32 = 16;
pub const fIOD_DECODE: u32 = 252;
pub const fIOD_IO: u32 = 1;
pub const fIOD_Memory: u32 = 0;
pub const fIOD_PASSIVE_DECODE: u32 = 64;
pub const fIOD_PORT_BAR: u32 = 256;
pub const fIOD_POSITIVE_DECODE: u32 = 32;
pub const fIOD_PortType: u32 = 1;
pub const fIOD_WINDOW_DECODE: u32 = 128;
pub const fIRQD_Edge: u32 = 2;
pub const fIRQD_Exclusive: u32 = 0;
pub const fIRQD_Level: u32 = 0;
pub const fIRQD_Level_Bit: u32 = 1;
pub const fIRQD_Share: u32 = 1;
pub const fIRQD_Share_Bit: u32 = 0;
pub const fMD_24: u32 = 0;
pub const fMD_32: u32 = 2;
pub const fMD_32_24: u32 = 2;
pub const fMD_Cacheable: u32 = 32;
pub const fMD_CombinedWrite: u32 = 16;
pub const fMD_CombinedWriteAllowed: u32 = 16;
pub const fMD_CombinedWriteDisallowed: u32 = 0;
pub const fMD_MEMORY_BAR: u32 = 128;
pub const fMD_MemoryType: u32 = 1;
pub const fMD_NonCacheable: u32 = 0;
pub const fMD_Pref: u32 = 4;
pub const fMD_PrefetchAllowed: u32 = 4;
pub const fMD_PrefetchDisallowed: u32 = 0;
pub const fMD_Prefetchable: u32 = 4;
pub const fMD_RAM: u32 = 1;
pub const fMD_ROM: u32 = 0;
pub const fMD_ReadAllowed: u32 = 0;
pub const fMD_ReadDisallowed: u32 = 8;
pub const fMD_Readable: u32 = 8;
pub const fMD_WINDOW_DECODE: u32 = 64;
pub const fPCD_ATTRIBUTES_PER_WINDOW: u32 = 32768;
pub const fPCD_IO1_16: u32 = 65536;
pub const fPCD_IO1_SRC_16: u32 = 262144;
pub const fPCD_IO1_WS_16: u32 = 524288;
pub const fPCD_IO1_ZW_8: u32 = 131072;
pub const fPCD_IO2_16: u32 = 1048576;
pub const fPCD_IO2_SRC_16: u32 = 4194304;
pub const fPCD_IO2_WS_16: u32 = 8388608;
pub const fPCD_IO2_ZW_8: u32 = 2097152;
pub const fPCD_IO_16: u32 = 1;
pub const fPCD_IO_8: u32 = 0;
pub const fPCD_IO_SRC_16: u32 = 32;
pub const fPCD_IO_WS_16: u32 = 64;
pub const fPCD_IO_ZW_8: u32 = 16;
pub const fPCD_MEM1_16: u32 = 67108864;
pub const fPCD_MEM1_A: u32 = 4;
pub const fPCD_MEM1_WS_ONE: u32 = 16777216;
pub const fPCD_MEM1_WS_THREE: u32 = 50331648;
pub const fPCD_MEM1_WS_TWO: u32 = 33554432;
pub const fPCD_MEM2_16: u32 = 1073741824;
pub const fPCD_MEM2_A: u32 = 8;
pub const fPCD_MEM2_WS_ONE: u32 = 268435456;
pub const fPCD_MEM2_WS_THREE: u32 = 805306368;
pub const fPCD_MEM2_WS_TWO: u32 = 536870912;
pub const fPCD_MEM_16: u32 = 2;
pub const fPCD_MEM_8: u32 = 0;
pub const fPCD_MEM_A: u32 = 4;
pub const fPCD_MEM_WS_ONE: u32 = 256;
pub const fPCD_MEM_WS_THREE: u32 = 768;
pub const fPCD_MEM_WS_TWO: u32 = 512;
pub const fPMF_AUDIO_ENABLE: u32 = 8;
pub const mDD_BusMaster: u32 = 4;
pub const mDD_Type: u32 = 24;
pub const mDD_Width: u32 = 3;
pub const mIRQD_Edge_Level: u32 = 2;
pub const mIRQD_Share: u32 = 1;
pub const mMD_32_24: u32 = 2;
pub const mMD_Cacheable: u32 = 32;
pub const mMD_CombinedWrite: u32 = 16;
pub const mMD_MemoryType: u32 = 1;
pub const mMD_Prefetchable: u32 = 4;
pub const mMD_Readable: u32 = 8;
pub const mPCD_IO_8_16: u32 = 1;
pub const mPCD_MEM1_WS: u32 = 50331648;
pub const mPCD_MEM2_WS: u32 = 805306368;
pub const mPCD_MEM_8_16: u32 = 2;
pub const mPCD_MEM_A_C: u32 = 12;
pub const mPCD_MEM_WS: u32 = 768;
pub const mPMF_AUDIO_ENABLE: u32 = 8;
