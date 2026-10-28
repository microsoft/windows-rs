#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CMP_WaitNoPendingInstallEvents(dwtimeout: super::DWORD) -> super::DWORD {
    windows_core::link!("setupapi.dll" "system" fn CMP_WaitNoPendingInstallEvents(dwtimeout : super::DWORD) -> super::DWORD);
    unsafe { CMP_WaitNoPendingInstallEvents(dwtimeout) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Add_Empty_Log_Conf(plclogconf: PLOG_CONF, dndevinst: DEVINST, priority: PRIORITY, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Empty_Log_Conf(plclogconf : PLOG_CONF, dndevinst : DEVINST, priority : PRIORITY, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Add_Empty_Log_Conf(plclogconf as _, dndevinst, priority, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Add_Empty_Log_Conf_Ex(plclogconf: PLOG_CONF, dndevinst: DEVINST, priority: PRIORITY, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Empty_Log_Conf_Ex(plclogconf : PLOG_CONF, dndevinst : DEVINST, priority : PRIORITY, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Add_Empty_Log_Conf_Ex(plclogconf as _, dndevinst, priority, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Add_IDA<P1>(dndevinst: DEVINST, pszid: P1, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Add_IDA(dndevinst : DEVINST, pszid : windows_core::PCSTR, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Add_IDA(dndevinst, pszid.param().abi(), ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Add_IDW<P1>(dndevinst: DEVINST, pszid: P1, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Add_IDW(dndevinst : DEVINST, pszid : windows_core::PCWSTR, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Add_IDW(dndevinst, pszid.param().abi(), ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Add_ID_ExA<P1>(dndevinst: DEVINST, pszid: P1, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Add_ID_ExA(dndevinst : DEVINST, pszid : windows_core::PCSTR, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Add_ID_ExA(dndevinst, pszid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Add_ID_ExW<P1>(dndevinst: DEVINST, pszid: P1, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Add_ID_ExW(dndevinst : DEVINST, pszid : windows_core::PCWSTR, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Add_ID_ExW(dndevinst, pszid.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Add_Range(ullstartvalue: super::DWORDLONG, ullendvalue: super::DWORDLONG, rlh: RANGE_LIST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Range(ullstartvalue : super::DWORDLONG, ullendvalue : super::DWORDLONG, rlh : RANGE_LIST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Add_Range(ullstartvalue, ullendvalue, rlh, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Add_Res_Des(prdresdes: Option<PRES_DES>, lclogconf: LOG_CONF, resourceid: RESOURCEID, resourcedata: PCVOID, resourcelen: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Res_Des(prdresdes : PRES_DES, lclogconf : LOG_CONF, resourceid : RESOURCEID, resourcedata : PCVOID, resourcelen : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Add_Res_Des(prdresdes.unwrap_or(core::mem::zeroed()) as _, lclogconf, resourceid, resourcedata, resourcelen, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Add_Res_Des_Ex(prdresdes: Option<PRES_DES>, lclogconf: LOG_CONF, resourceid: RESOURCEID, resourcedata: PCVOID, resourcelen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Add_Res_Des_Ex(prdresdes : PRES_DES, lclogconf : LOG_CONF, resourceid : RESOURCEID, resourcedata : PCVOID, resourcelen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Add_Res_Des_Ex(prdresdes.unwrap_or(core::mem::zeroed()) as _, lclogconf, resourceid, resourcedata, resourcelen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Connect_MachineA<P0>(uncservername: P0, phmachine: PHMACHINE) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Connect_MachineA(uncservername : windows_core::PCSTR, phmachine : PHMACHINE) -> CONFIGRET);
    unsafe { CM_Connect_MachineA(uncservername.param().abi(), phmachine as _) }
}
#[inline]
pub unsafe fn CM_Connect_MachineW<P0>(uncservername: P0, phmachine: PHMACHINE) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Connect_MachineW(uncservername : windows_core::PCWSTR, phmachine : PHMACHINE) -> CONFIGRET);
    unsafe { CM_Connect_MachineW(uncservername.param().abi(), phmachine as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Create_DevInstA(pdndevinst: PDEVINST, pdeviceid: DEVINSTID_A, dnparent: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Create_DevNodeA" fn CM_Create_DevInstA(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_A, dnparent : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Create_DevInstA(pdndevinst as _, pdeviceid, dnparent, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Create_DevInstW(pdndevinst: PDEVINST, pdeviceid: DEVINSTID_W, dnparent: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Create_DevNodeW" fn CM_Create_DevInstW(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_W, dnparent : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Create_DevInstW(pdndevinst as _, pdeviceid, dnparent, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Create_DevInst_ExA(pdndevinst: PDEVINST, pdeviceid: DEVINSTID_A, dnparent: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Create_DevNode_ExA" fn CM_Create_DevInst_ExA(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_A, dnparent : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Create_DevInst_ExA(pdndevinst as _, pdeviceid, dnparent, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Create_DevInst_ExW(pdndevinst: PDEVINST, pdeviceid: DEVINSTID_W, dnparent: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Create_DevNode_ExW" fn CM_Create_DevInst_ExW(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_W, dnparent : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Create_DevInst_ExW(pdndevinst as _, pdeviceid, dnparent, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Create_DevNodeA(pdndevinst: PDEVINST, pdeviceid: DEVINSTID_A, dnparent: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_DevNodeA(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_A, dnparent : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Create_DevNodeA(pdndevinst as _, pdeviceid, dnparent, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Create_DevNodeW(pdndevinst: PDEVINST, pdeviceid: DEVINSTID_W, dnparent: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_DevNodeW(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_W, dnparent : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Create_DevNodeW(pdndevinst as _, pdeviceid, dnparent, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Create_DevNode_ExA(pdndevinst: PDEVINST, pdeviceid: DEVINSTID_A, dnparent: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_DevNode_ExA(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_A, dnparent : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Create_DevNode_ExA(pdndevinst as _, pdeviceid, dnparent, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Create_DevNode_ExW(pdndevinst: PDEVINST, pdeviceid: DEVINSTID_W, dnparent: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_DevNode_ExW(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_W, dnparent : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Create_DevNode_ExW(pdndevinst as _, pdeviceid, dnparent, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Create_Range_List(prlh: PRANGE_LIST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Create_Range_List(prlh : PRANGE_LIST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Create_Range_List(prlh as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Delete_Class_Key(classguid: super::LPGUID, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Class_Key(classguid : super::LPGUID, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Delete_Class_Key(classguid, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Delete_Class_Key_Ex(classguid: super::LPGUID, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Class_Key_Ex(classguid : super::LPGUID, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Delete_Class_Key_Ex(classguid, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Delete_DevInst_Key(dndevnode: DEVNODE, ulhardwareprofile: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Delete_DevNode_Key" fn CM_Delete_DevInst_Key(dndevnode : DEVNODE, ulhardwareprofile : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Delete_DevInst_Key(dndevnode, ulhardwareprofile, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Delete_DevInst_Key_Ex(dndevnode: DEVNODE, ulhardwareprofile: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Delete_DevNode_Key_Ex" fn CM_Delete_DevInst_Key_Ex(dndevnode : DEVNODE, ulhardwareprofile : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Delete_DevInst_Key_Ex(dndevnode, ulhardwareprofile, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Delete_DevNode_Key(dndevnode: DEVNODE, ulhardwareprofile: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_DevNode_Key(dndevnode : DEVNODE, ulhardwareprofile : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Delete_DevNode_Key(dndevnode, ulhardwareprofile, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Delete_DevNode_Key_Ex(dndevnode: DEVNODE, ulhardwareprofile: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_DevNode_Key_Ex(dndevnode : DEVNODE, ulhardwareprofile : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Delete_DevNode_Key_Ex(dndevnode, ulhardwareprofile, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_KeyA<P0>(pszdeviceinterface: P0, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_KeyA(pszdeviceinterface : windows_core::PCSTR, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_KeyA(pszdeviceinterface.param().abi(), ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_KeyW<P0>(pszdeviceinterface: P0, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_KeyW(pszdeviceinterface : windows_core::PCWSTR, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_KeyW(pszdeviceinterface.param().abi(), ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_Key_ExA<P0>(pszdeviceinterface: P0, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface : windows_core::PCSTR, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Delete_Device_Interface_Key_ExW<P0>(pszdeviceinterface: P0, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface : windows_core::PCWSTR, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Delete_Range(ullstartvalue: super::DWORDLONG, ullendvalue: super::DWORDLONG, rlh: RANGE_LIST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Delete_Range(ullstartvalue : super::DWORDLONG, ullendvalue : super::DWORDLONG, rlh : RANGE_LIST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Delete_Range(ullstartvalue, ullendvalue, rlh, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Detect_Resource_Conflict(dndevinst: DEVINST, resourceid: RESOURCEID, resourcedata: PCVOID, resourcelen: super::ULONG, pbconflictdetected: super::PBOOL, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Detect_Resource_Conflict(dndevinst : DEVINST, resourceid : RESOURCEID, resourcedata : PCVOID, resourcelen : super::ULONG, pbconflictdetected : super::PBOOL, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Detect_Resource_Conflict(dndevinst, resourceid, resourcedata, resourcelen, pbconflictdetected as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Detect_Resource_Conflict_Ex(dndevinst: DEVINST, resourceid: RESOURCEID, resourcedata: PCVOID, resourcelen: super::ULONG, pbconflictdetected: super::PBOOL, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Detect_Resource_Conflict_Ex(dndevinst : DEVINST, resourceid : RESOURCEID, resourcedata : PCVOID, resourcelen : super::ULONG, pbconflictdetected : super::PBOOL, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Detect_Resource_Conflict_Ex(dndevinst, resourceid, resourcedata, resourcelen, pbconflictdetected as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Disable_DevInst(dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Disable_DevNode" fn CM_Disable_DevInst(dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Disable_DevInst(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Disable_DevInst_Ex(dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Disable_DevNode_Ex" fn CM_Disable_DevInst_Ex(dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Disable_DevInst_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Disable_DevNode(dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Disable_DevNode(dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Disable_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Disable_DevNode_Ex(dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Disable_DevNode_Ex(dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Disable_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Disconnect_Machine(hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Disconnect_Machine(hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Disconnect_Machine(hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Dup_Range_List(rlhold: RANGE_LIST, rlhnew: RANGE_LIST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Dup_Range_List(rlhold : RANGE_LIST, rlhnew : RANGE_LIST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Dup_Range_List(rlhold, rlhnew, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Enable_DevInst(dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Enable_DevNode" fn CM_Enable_DevInst(dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Enable_DevInst(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Enable_DevInst_Ex(dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Enable_DevNode_Ex" fn CM_Enable_DevInst_Ex(dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Enable_DevInst_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Enable_DevNode(dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enable_DevNode(dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Enable_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Enable_DevNode_Ex(dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enable_DevNode_Ex(dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Enable_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Enumerate_Classes(ulclassindex: super::ULONG, classguid: super::LPGUID, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_Classes(ulclassindex : super::ULONG, classguid : super::LPGUID, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Enumerate_Classes(ulclassindex, classguid as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Enumerate_Classes_Ex(ulclassindex: super::ULONG, classguid: super::LPGUID, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_Classes_Ex(ulclassindex : super::ULONG, classguid : super::LPGUID, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Enumerate_Classes_Ex(ulclassindex, classguid as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Enumerate_EnumeratorsA(ulenumindex: super::ULONG, buffer: windows_core::PSTR, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_EnumeratorsA(ulenumindex : super::ULONG, buffer : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Enumerate_EnumeratorsA(ulenumindex, buffer, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Enumerate_EnumeratorsW(ulenumindex: super::ULONG, buffer: windows_core::PWSTR, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_EnumeratorsW(ulenumindex : super::ULONG, buffer : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Enumerate_EnumeratorsW(ulenumindex, buffer, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Enumerate_Enumerators_ExA(ulenumindex: super::ULONG, buffer: windows_core::PSTR, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_Enumerators_ExA(ulenumindex : super::ULONG, buffer : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Enumerate_Enumerators_ExA(ulenumindex, buffer, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Enumerate_Enumerators_ExW(ulenumindex: super::ULONG, buffer: windows_core::PWSTR, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Enumerate_Enumerators_ExW(ulenumindex : super::ULONG, buffer : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Enumerate_Enumerators_ExW(ulenumindex, buffer, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Find_Range(pullstart: super::PDWORDLONG, ullstart: super::DWORDLONG, ullength: super::ULONG, ullalignment: super::DWORDLONG, ullend: super::DWORDLONG, rlh: RANGE_LIST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Find_Range(pullstart : super::PDWORDLONG, ullstart : super::DWORDLONG, ullength : super::ULONG, ullalignment : super::DWORDLONG, ullend : super::DWORDLONG, rlh : RANGE_LIST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Find_Range(pullstart as _, ullstart, ullength, ullalignment, ullend, rlh, ulflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_First_Range(rlh: RANGE_LIST, pullstart: super::PDWORDLONG, pullend: super::PDWORDLONG, preelement: PRANGE_ELEMENT, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_First_Range(rlh : RANGE_LIST, pullstart : super::PDWORDLONG, pullend : super::PDWORDLONG, preelement : PRANGE_ELEMENT, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_First_Range(rlh, pullstart as _, pullend as _, preelement as _, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Free_Log_Conf(lclogconftobefreed: LOG_CONF, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Log_Conf(lclogconftobefreed : LOG_CONF, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Free_Log_Conf(lclogconftobefreed, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Free_Log_Conf_Ex(lclogconftobefreed: LOG_CONF, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Log_Conf_Ex(lclogconftobefreed : LOG_CONF, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Free_Log_Conf_Ex(lclogconftobefreed, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Free_Log_Conf_Handle(lclogconf: LOG_CONF) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Log_Conf_Handle(lclogconf : LOG_CONF) -> CONFIGRET);
    unsafe { CM_Free_Log_Conf_Handle(lclogconf) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Free_Range_List(rlh: RANGE_LIST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Range_List(rlh : RANGE_LIST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Free_Range_List(rlh, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Free_Res_Des(prdresdes: Option<PRES_DES>, rdresdes: RES_DES, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Res_Des(prdresdes : PRES_DES, rdresdes : RES_DES, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Free_Res_Des(prdresdes.unwrap_or(core::mem::zeroed()) as _, rdresdes, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Free_Res_Des_Ex(prdresdes: Option<PRES_DES>, rdresdes: RES_DES, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Free_Res_Des_Ex(prdresdes : PRES_DES, rdresdes : RES_DES, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
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
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Child(pdndevinst: PDEVINST, dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Child(pdndevinst : PDEVINST, dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Child(pdndevinst as _, dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Child_Ex(pdndevinst: PDEVINST, dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Child_Ex(pdndevinst : PDEVINST, dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Child_Ex(pdndevinst as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_Key_NameA(classguid: super::LPGUID, pszkeyname: Option<windows_core::PSTR>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Key_NameA(classguid : super::LPGUID, pszkeyname : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_NameA(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_Key_NameW(classguid: super::LPGUID, pszkeyname: Option<windows_core::PWSTR>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Key_NameW(classguid : super::LPGUID, pszkeyname : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_NameW(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_Key_Name_ExA(classguid: super::LPGUID, pszkeyname: Option<windows_core::PSTR>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Key_Name_ExA(classguid : super::LPGUID, pszkeyname : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_Name_ExA(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_Key_Name_ExW(classguid: super::LPGUID, pszkeyname: Option<windows_core::PWSTR>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Key_Name_ExW(classguid : super::LPGUID, pszkeyname : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Key_Name_ExW(classguid, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_NameA(classguid: super::LPGUID, buffer: Option<windows_core::PSTR>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_NameA(classguid : super::LPGUID, buffer : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Class_NameA(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_NameW(classguid: super::LPGUID, buffer: Option<windows_core::PWSTR>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_NameW(classguid : super::LPGUID, buffer : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Class_NameW(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_Name_ExA(classguid: super::LPGUID, buffer: Option<windows_core::PSTR>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Name_ExA(classguid : super::LPGUID, buffer : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Name_ExA(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_Name_ExW(classguid: super::LPGUID, buffer: Option<windows_core::PWSTR>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Name_ExW(classguid : super::LPGUID, buffer : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Name_ExW(classguid, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "devpropdef", feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_PropertyW(classguid: super::LPCGUID, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<super::PBYTE>, propertybuffersize: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_PropertyW(classguid : super::LPCGUID, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Class_PropertyW(classguid, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_Property_ExW(classguid: super::LPCGUID, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<super::PBYTE>, propertybuffersize: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_ExW(classguid : super::LPCGUID, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Property_ExW(classguid, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "devpropdef", feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_Property_Keys(classguid: super::LPCGUID, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_Keys(classguid : super::LPCGUID, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Class_Property_Keys(classguid, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Class_Property_Keys_Ex(classguid: super::LPCGUID, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_Keys_Ex(classguid : super::LPCGUID, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Property_Keys_Ex(classguid, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Class_Registry_PropertyA(classguid: super::LPGUID, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Registry_PropertyA(classguid : super::LPGUID, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Registry_PropertyA(classguid, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Class_Registry_PropertyW(classguid: super::LPGUID, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Class_Registry_PropertyW(classguid : super::LPGUID, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Class_Registry_PropertyW(classguid, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Depth(puldepth: super::PULONG, dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Depth(puldepth : super::PULONG, dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Depth(puldepth as _, dndevinst, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Depth_Ex(puldepth: super::PULONG, dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Depth_Ex(puldepth : super::PULONG, dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Depth_Ex(puldepth as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Custom_PropertyA<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Custom_PropertyA" fn CM_Get_DevInst_Custom_PropertyA(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCSTR, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Custom_PropertyA(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Custom_PropertyW<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Custom_PropertyW" fn CM_Get_DevInst_Custom_PropertyW(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCWSTR, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Custom_PropertyW(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Custom_Property_ExA<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Custom_Property_ExA" fn CM_Get_DevInst_Custom_Property_ExA(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCSTR, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Custom_Property_ExA(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Custom_Property_ExW<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Custom_Property_ExW" fn CM_Get_DevInst_Custom_Property_ExW(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCWSTR, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Custom_Property_ExW(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Registry_PropertyA(dndevinst: DEVINST, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Registry_PropertyA" fn CM_Get_DevInst_Registry_PropertyA(dndevinst : DEVINST, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Registry_PropertyA(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Registry_PropertyW(dndevinst: DEVINST, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Registry_PropertyW" fn CM_Get_DevInst_Registry_PropertyW(dndevinst : DEVINST, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Registry_PropertyW(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Registry_Property_ExA(dndevinst: DEVINST, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Registry_Property_ExA" fn CM_Get_DevInst_Registry_Property_ExA(dndevinst : DEVINST, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Registry_Property_ExA(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Registry_Property_ExW(dndevinst: DEVINST, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Registry_Property_ExW" fn CM_Get_DevInst_Registry_Property_ExW(dndevinst : DEVINST, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Registry_Property_ExW(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Status(pulstatus: super::PULONG, pulproblemnumber: super::PULONG, dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Status" fn CM_Get_DevInst_Status(pulstatus : super::PULONG, pulproblemnumber : super::PULONG, dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Status(pulstatus as _, pulproblemnumber as _, dndevinst, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_DevInst_Status_Ex(pulstatus: super::PULONG, pulproblemnumber: super::PULONG, dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Get_DevNode_Status_Ex" fn CM_Get_DevInst_Status_Ex(pulstatus : super::PULONG, pulproblemnumber : super::PULONG, dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevInst_Status_Ex(pulstatus as _, pulproblemnumber as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_PropertyA<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_PropertyA(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCSTR, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_PropertyA(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_PropertyW<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_PropertyW(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCWSTR, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_PropertyW(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_Property_ExA<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_Property_ExA(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCSTR, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_Property_ExA(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Custom_Property_ExW<P1>(dndevinst: DEVINST, pszcustompropertyname: P1, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_Property_ExW(dndevinst : DEVINST, pszcustompropertyname : windows_core::PCWSTR, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Custom_Property_ExW(dndevinst, pszcustompropertyname.param().abi(), pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "devpropdef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_DevNode_PropertyW(dndevinst: DEVINST, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<super::PBYTE>, propertybuffersize: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_PropertyW(dndevinst : DEVINST, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevNode_PropertyW(dndevinst, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_ExW(dndevinst: DEVINST, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<super::PBYTE>, propertybuffersize: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_ExW(dndevinst : DEVINST, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Property_ExW(dndevinst, propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "devpropdef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_Keys(dndevinst: DEVINST, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_Keys(dndevinst : DEVINST, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Property_Keys(dndevinst, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Property_Keys_Ex(dndevinst: DEVINST, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_Keys_Ex(dndevinst : DEVINST, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Property_Keys_Ex(dndevinst, propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_PropertyA(dndevinst: DEVINST, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_PropertyA(dndevinst : DEVINST, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_PropertyA(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_PropertyW(dndevinst: DEVINST, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_PropertyW(dndevinst : DEVINST, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_PropertyW(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_Property_ExA(dndevinst: DEVINST, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_Property_ExA(dndevinst : DEVINST, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_Property_ExA(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Registry_Property_ExW(dndevinst: DEVINST, ulproperty: super::ULONG, pulregdatatype: Option<super::PULONG>, buffer: Option<super::PVOID>, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_Property_ExW(dndevinst : DEVINST, ulproperty : super::ULONG, pulregdatatype : super::PULONG, buffer : super::PVOID, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Registry_Property_ExW(dndevinst, ulproperty, pulregdatatype.unwrap_or(core::mem::zeroed()) as _, buffer.unwrap_or(core::mem::zeroed()) as _, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Status(pulstatus: super::PULONG, pulproblemnumber: super::PULONG, dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Status(pulstatus : super::PULONG, pulproblemnumber : super::PULONG, dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Status(pulstatus as _, pulproblemnumber as _, dndevinst, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_DevNode_Status_Ex(pulstatus: super::PULONG, pulproblemnumber: super::PULONG, dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_DevNode_Status_Ex(pulstatus : super::PULONG, pulproblemnumber : super::PULONG, dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_DevNode_Status_Ex(pulstatus as _, pulproblemnumber as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Device_IDA(dndevinst: DEVINST, buffer: windows_core::PSTR, bufferlen: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_IDA(dndevinst : DEVINST, buffer : windows_core::PSTR, bufferlen : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_IDA(dndevinst, buffer, bufferlen, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Device_IDW(dndevinst: DEVINST, buffer: windows_core::PWSTR, bufferlen: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_IDW(dndevinst : DEVINST, buffer : windows_core::PWSTR, bufferlen : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_IDW(dndevinst, buffer, bufferlen, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Device_ID_ExA(dndevinst: DEVINST, buffer: windows_core::PSTR, bufferlen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ExA(dndevinst : DEVINST, buffer : windows_core::PSTR, bufferlen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ExA(dndevinst, buffer, bufferlen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Device_ID_ExW(dndevinst: DEVINST, buffer: windows_core::PWSTR, bufferlen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ExW(dndevinst : DEVINST, buffer : windows_core::PWSTR, bufferlen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ExW(dndevinst, buffer, bufferlen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_ListA<P0>(pszfilter: P0, buffer: super::PZZSTR, bufferlen: super::ULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ListA(pszfilter : windows_core::PCSTR, buffer : super::PZZSTR, bufferlen : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ListA(pszfilter.param().abi(), buffer as _, bufferlen, ulflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_ListW<P0>(pszfilter: P0, buffer: super::PZZWSTR, bufferlen: super::ULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ListW(pszfilter : windows_core::PCWSTR, buffer : super::PZZWSTR, bufferlen : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_ListW(pszfilter.param().abi(), buffer as _, bufferlen, ulflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_ExA<P0>(pszfilter: P0, buffer: super::PZZSTR, bufferlen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_ExA(pszfilter : windows_core::PCSTR, buffer : super::PZZSTR, bufferlen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_ExA(pszfilter.param().abi(), buffer as _, bufferlen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_ExW<P0>(pszfilter: P0, buffer: super::PZZWSTR, bufferlen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_ExW(pszfilter : windows_core::PCWSTR, buffer : super::PZZWSTR, bufferlen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_ExW(pszfilter.param().abi(), buffer as _, bufferlen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_SizeA<P1>(pullen: super::PULONG, pszfilter: P1, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_SizeA(pullen : super::PULONG, pszfilter : windows_core::PCSTR, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_SizeA(pullen as _, pszfilter.param().abi(), ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_SizeW<P1>(pullen: super::PULONG, pszfilter: P1, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_SizeW(pullen : super::PULONG, pszfilter : windows_core::PCWSTR, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_SizeW(pullen as _, pszfilter.param().abi(), ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_Size_ExA<P1>(pullen: super::PULONG, pszfilter: P1, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_Size_ExA(pullen : super::PULONG, pszfilter : windows_core::PCSTR, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_Size_ExA(pullen as _, pszfilter.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_List_Size_ExW<P1>(pullen: super::PULONG, pszfilter: P1, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_Size_ExW(pullen : super::PULONG, pszfilter : windows_core::PCWSTR, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_List_Size_ExW(pullen as _, pszfilter.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_Size(pullen: super::PULONG, dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_Size(pullen : super::PULONG, dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_Size(pullen as _, dndevinst, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_ID_Size_Ex(pullen: super::PULONG, dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_ID_Size_Ex(pullen : super::PULONG, dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_ID_Size_Ex(pullen as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_AliasA<P0>(pszdeviceinterface: P0, aliasinterfaceguid: super::LPGUID, pszaliasdeviceinterface: windows_core::PSTR, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_AliasA(pszdeviceinterface : windows_core::PCSTR, aliasinterfaceguid : super::LPGUID, pszaliasdeviceinterface : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_AliasA(pszdeviceinterface.param().abi(), aliasinterfaceguid, pszaliasdeviceinterface, pullength as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_AliasW<P0>(pszdeviceinterface: P0, aliasinterfaceguid: super::LPGUID, pszaliasdeviceinterface: windows_core::PWSTR, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_AliasW(pszdeviceinterface : windows_core::PCWSTR, aliasinterfaceguid : super::LPGUID, pszaliasdeviceinterface : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_AliasW(pszdeviceinterface.param().abi(), aliasinterfaceguid, pszaliasdeviceinterface, pullength as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Alias_ExA<P0>(pszdeviceinterface: P0, aliasinterfaceguid: super::LPGUID, pszaliasdeviceinterface: windows_core::PSTR, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface : windows_core::PCSTR, aliasinterfaceguid : super::LPGUID, pszaliasdeviceinterface : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface.param().abi(), aliasinterfaceguid, pszaliasdeviceinterface, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Alias_ExW<P0>(pszdeviceinterface: P0, aliasinterfaceguid: super::LPGUID, pszaliasdeviceinterface: windows_core::PWSTR, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface : windows_core::PCWSTR, aliasinterfaceguid : super::LPGUID, pszaliasdeviceinterface : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface.param().abi(), aliasinterfaceguid, pszaliasdeviceinterface, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_ListA(interfaceclassguid: super::LPGUID, pdeviceid: Option<DEVINSTID_A>, buffer: super::PZZSTR, bufferlen: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_ListA(interfaceclassguid : super::LPGUID, pdeviceid : DEVINSTID_A, buffer : super::PZZSTR, bufferlen : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_ListA(interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, buffer as _, bufferlen, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_ListW(interfaceclassguid: super::LPGUID, pdeviceid: Option<DEVINSTID_W>, buffer: super::PZZWSTR, bufferlen: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_ListW(interfaceclassguid : super::LPGUID, pdeviceid : DEVINSTID_W, buffer : super::PZZWSTR, bufferlen : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_ListW(interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, buffer as _, bufferlen, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_ExA(interfaceclassguid: super::LPGUID, pdeviceid: Option<DEVINSTID_A>, buffer: super::PZZSTR, bufferlen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_ExA(interfaceclassguid : super::LPGUID, pdeviceid : DEVINSTID_A, buffer : super::PZZSTR, bufferlen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_ExA(interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, buffer as _, bufferlen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_ExW(interfaceclassguid: super::LPGUID, pdeviceid: Option<DEVINSTID_W>, buffer: super::PZZWSTR, bufferlen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_ExW(interfaceclassguid : super::LPGUID, pdeviceid : DEVINSTID_W, buffer : super::PZZWSTR, bufferlen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_ExW(interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, buffer as _, bufferlen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_SizeA(pullen: super::PULONG, interfaceclassguid: super::LPGUID, pdeviceid: Option<DEVINSTID_A>, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_SizeA(pullen : super::PULONG, interfaceclassguid : super::LPGUID, pdeviceid : DEVINSTID_A, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_SizeA(pullen as _, interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_SizeW(pullen: super::PULONG, interfaceclassguid: super::LPGUID, pdeviceid: Option<DEVINSTID_W>, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_SizeW(pullen : super::PULONG, interfaceclassguid : super::LPGUID, pdeviceid : DEVINSTID_W, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_SizeW(pullen as _, interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_Size_ExA(pullen: super::PULONG, interfaceclassguid: super::LPGUID, pdeviceid: Option<DEVINSTID_A>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_Size_ExA(pullen : super::PULONG, interfaceclassguid : super::LPGUID, pdeviceid : DEVINSTID_A, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_Size_ExA(pullen as _, interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_List_Size_ExW(pullen: super::PULONG, interfaceclassguid: super::LPGUID, pdeviceid: Option<DEVINSTID_W>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_Size_ExW(pullen : super::PULONG, interfaceclassguid : super::LPGUID, pdeviceid : DEVINSTID_W, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_List_Size_ExW(pullen as _, interfaceclassguid, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "devpropdef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_PropertyW<P0>(pszdeviceinterface: P0, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<super::PBYTE>, propertybuffersize: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_PropertyW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_PropertyW(pszdeviceinterface.param().abi(), propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_ExW<P0>(pszdeviceinterface: P0, propertykey: *const super::DEVPROPKEY, propertytype: *mut super::DEVPROPTYPE, propertybuffer: Option<super::PBYTE>, propertybuffersize: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_ExW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Property_ExW(pszdeviceinterface.param().abi(), propertykey, propertytype as _, propertybuffer.unwrap_or(core::mem::zeroed()) as _, propertybuffersize as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "devpropdef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_KeysW<P0>(pszdeviceinterface: P0, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface : windows_core::PCWSTR, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface.param().abi(), propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Device_Interface_Property_Keys_ExW<P0>(pszdeviceinterface: P0, propertykeyarray: Option<*mut super::DEVPROPKEY>, propertykeycount: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface : windows_core::PCWSTR, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface.param().abi(), propertykeyarray.unwrap_or(core::mem::zeroed()) as _, propertykeycount as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_First_Log_Conf(plclogconf: Option<PLOG_CONF>, dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_First_Log_Conf(plclogconf : PLOG_CONF, dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_First_Log_Conf(plclogconf.unwrap_or(core::mem::zeroed()) as _, dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_First_Log_Conf_Ex(plclogconf: Option<PLOG_CONF>, dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_First_Log_Conf_Ex(plclogconf : PLOG_CONF, dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_First_Log_Conf_Ex(plclogconf.unwrap_or(core::mem::zeroed()) as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Global_State(pulstate: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Global_State(pulstate : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Global_State(pulstate as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Global_State_Ex(pulstate: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Global_State_Ex(pulstate : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Global_State_Ex(pulstate as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_HW_Prof_FlagsA(pdeviceid: DEVINSTID_A, ulhardwareprofile: super::ULONG, pulvalue: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_FlagsA(pdeviceid : DEVINSTID_A, ulhardwareprofile : super::ULONG, pulvalue : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_FlagsA(pdeviceid, ulhardwareprofile, pulvalue as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_HW_Prof_FlagsW(pdeviceid: DEVINSTID_W, ulhardwareprofile: super::ULONG, pulvalue: super::PULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_FlagsW(pdeviceid : DEVINSTID_W, ulhardwareprofile : super::ULONG, pulvalue : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_FlagsW(pdeviceid, ulhardwareprofile, pulvalue as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_HW_Prof_Flags_ExA(pdeviceid: DEVINSTID_A, ulhardwareprofile: super::ULONG, pulvalue: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_Flags_ExA(pdeviceid : DEVINSTID_A, ulhardwareprofile : super::ULONG, pulvalue : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_Flags_ExA(pdeviceid, ulhardwareprofile, pulvalue as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_HW_Prof_Flags_ExW(pdeviceid: DEVINSTID_W, ulhardwareprofile: super::ULONG, pulvalue: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_Flags_ExW(pdeviceid : DEVINSTID_W, ulhardwareprofile : super::ULONG, pulvalue : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_HW_Prof_Flags_ExW(pdeviceid, ulhardwareprofile, pulvalue as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_InfoA(ulindex: super::ULONG, phwprofileinfo: PHWPROFILEINFO_A, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_InfoA(ulindex : super::ULONG, phwprofileinfo : PHWPROFILEINFO_A, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_InfoA(ulindex, phwprofileinfo as _, ulflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_InfoW(ulindex: super::ULONG, phwprofileinfo: PHWPROFILEINFO_W, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_InfoW(ulindex : super::ULONG, phwprofileinfo : PHWPROFILEINFO_W, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_InfoW(ulindex, phwprofileinfo as _, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_Info_ExA(ulindex: super::ULONG, phwprofileinfo: PHWPROFILEINFO_A, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_Info_ExA(ulindex : super::ULONG, phwprofileinfo : PHWPROFILEINFO_A, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_Info_ExA(ulindex, phwprofileinfo as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Hardware_Profile_Info_ExW(ulindex: super::ULONG, phwprofileinfo: PHWPROFILEINFO_W, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_Info_ExW(ulindex : super::ULONG, phwprofileinfo : PHWPROFILEINFO_W, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Hardware_Profile_Info_ExW(ulindex, phwprofileinfo as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Log_Conf_Priority(lclogconf: LOG_CONF, ppriority: PPRIORITY, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Log_Conf_Priority(lclogconf : LOG_CONF, ppriority : PPRIORITY, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Log_Conf_Priority(lclogconf, ppriority as _, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Log_Conf_Priority_Ex(lclogconf: LOG_CONF, ppriority: PPRIORITY, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Log_Conf_Priority_Ex(lclogconf : LOG_CONF, ppriority : PPRIORITY, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Log_Conf_Priority_Ex(lclogconf, ppriority as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Next_Log_Conf(plclogconf: Option<PLOG_CONF>, lclogconf: LOG_CONF, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Next_Log_Conf(plclogconf : PLOG_CONF, lclogconf : LOG_CONF, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Next_Log_Conf(plclogconf.unwrap_or(core::mem::zeroed()) as _, lclogconf, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Next_Log_Conf_Ex(plclogconf: Option<PLOG_CONF>, lclogconf: LOG_CONF, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Next_Log_Conf_Ex(plclogconf : PLOG_CONF, lclogconf : LOG_CONF, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Next_Log_Conf_Ex(plclogconf.unwrap_or(core::mem::zeroed()) as _, lclogconf, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Next_Res_Des(prdresdes: PRES_DES, rdresdes: RES_DES, forresource: RESOURCEID, presourceid: Option<PRESOURCEID>, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Next_Res_Des(prdresdes : PRES_DES, rdresdes : RES_DES, forresource : RESOURCEID, presourceid : PRESOURCEID, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Next_Res_Des(prdresdes as _, rdresdes, forresource, presourceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Next_Res_Des_Ex(prdresdes: PRES_DES, rdresdes: RES_DES, forresource: RESOURCEID, presourceid: Option<PRESOURCEID>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Next_Res_Des_Ex(prdresdes : PRES_DES, rdresdes : RES_DES, forresource : RESOURCEID, presourceid : PRESOURCEID, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Next_Res_Des_Ex(prdresdes as _, rdresdes, forresource, presourceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Parent(pdndevinst: PDEVINST, dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Parent(pdndevinst : PDEVINST, dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Parent(pdndevinst as _, dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Parent_Ex(pdndevinst: PDEVINST, dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Parent_Ex(pdndevinst : PDEVINST, dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Parent_Ex(pdndevinst as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data(rdresdes: RES_DES, buffer: super::PVOID, bufferlen: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data(rdresdes : RES_DES, buffer : super::PVOID, bufferlen : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data(rdresdes, buffer as _, bufferlen, ulflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Ex(rdresdes: RES_DES, buffer: super::PVOID, bufferlen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data_Ex(rdresdes : RES_DES, buffer : super::PVOID, bufferlen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data_Ex(rdresdes, buffer as _, bufferlen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Size(pulsize: super::PULONG, rdresdes: RES_DES, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data_Size(pulsize : super::PULONG, rdresdes : RES_DES, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data_Size(pulsize as _, rdresdes, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Get_Res_Des_Data_Size_Ex(pulsize: super::PULONG, rdresdes: RES_DES, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data_Size_Ex(pulsize : super::PULONG, rdresdes : RES_DES, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Res_Des_Data_Size_Ex(pulsize as _, rdresdes, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_Count(clconflictlist: CONFLICT_LIST, pulcount: super::PULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Resource_Conflict_Count(clconflictlist : CONFLICT_LIST, pulcount : super::PULONG) -> CONFIGRET);
    unsafe { CM_Get_Resource_Conflict_Count(clconflictlist, pulcount as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_DetailsA(clconflictlist: CONFLICT_LIST, ulindex: super::ULONG, pconflictdetails: PCONFLICT_DETAILS_A) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Resource_Conflict_DetailsA(clconflictlist : CONFLICT_LIST, ulindex : super::ULONG, pconflictdetails : PCONFLICT_DETAILS_A) -> CONFIGRET);
    unsafe { CM_Get_Resource_Conflict_DetailsA(clconflictlist, ulindex, pconflictdetails as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Get_Resource_Conflict_DetailsW(clconflictlist: CONFLICT_LIST, ulindex: super::ULONG, pconflictdetails: PCONFLICT_DETAILS_W) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Resource_Conflict_DetailsW(clconflictlist : CONFLICT_LIST, ulindex : super::ULONG, pconflictdetails : PCONFLICT_DETAILS_W) -> CONFIGRET);
    unsafe { CM_Get_Resource_Conflict_DetailsW(clconflictlist, ulindex, pconflictdetails as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Sibling(pdndevinst: PDEVINST, dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Sibling(pdndevinst : PDEVINST, dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Get_Sibling(pdndevinst as _, dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Sibling_Ex(pdndevinst: PDEVINST, dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Sibling_Ex(pdndevinst : PDEVINST, dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Get_Sibling_Ex(pdndevinst as _, dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Version() -> super::WORD {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Version() -> super::WORD);
    unsafe { CM_Get_Version() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Get_Version_Ex(hmachine: Option<HMACHINE>) -> super::WORD {
    windows_core::link!("setupapi.dll" "system" fn CM_Get_Version_Ex(hmachine : HMACHINE) -> super::WORD);
    unsafe { CM_Get_Version_Ex(hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Intersect_Range_List(rlhold1: RANGE_LIST, rlhold2: RANGE_LIST, rlhnew: RANGE_LIST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Intersect_Range_List(rlhold1 : RANGE_LIST, rlhold2 : RANGE_LIST, rlhnew : RANGE_LIST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Intersect_Range_List(rlhold1, rlhold2, rlhnew, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Invert_Range_List(rlhold: RANGE_LIST, rlhnew: RANGE_LIST, ullmaxvalue: super::DWORDLONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Invert_Range_List(rlhold : RANGE_LIST, rlhnew : RANGE_LIST, ullmaxvalue : super::DWORDLONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Invert_Range_List(rlhold, rlhnew, ullmaxvalue, ulflags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CM_Is_Dock_Station_Present(pbpresent: super::PBOOL) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Is_Dock_Station_Present(pbpresent : super::PBOOL) -> CONFIGRET);
    unsafe { CM_Is_Dock_Station_Present(pbpresent as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CM_Is_Dock_Station_Present_Ex(pbpresent: super::PBOOL, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Is_Dock_Station_Present_Ex(pbpresent : super::PBOOL, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Is_Dock_Station_Present_Ex(pbpresent as _, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Is_Version_Available(wversion: super::WORD) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn CM_Is_Version_Available(wversion : super::WORD) -> windows_core::BOOL);
    unsafe { CM_Is_Version_Available(wversion) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Is_Version_Available_Ex(wversion: super::WORD, hmachine: Option<HMACHINE>) -> windows_core::BOOL {
    windows_core::link!("setupapi.dll" "system" fn CM_Is_Version_Available_Ex(wversion : super::WORD, hmachine : HMACHINE) -> windows_core::BOOL);
    unsafe { CM_Is_Version_Available_Ex(wversion, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Locate_DevInstA(pdndevinst: PDEVINST, pdeviceid: Option<DEVINSTID_A>, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Locate_DevNodeA" fn CM_Locate_DevInstA(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_A, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Locate_DevInstA(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Locate_DevInstW(pdndevinst: PDEVINST, pdeviceid: Option<DEVINSTID_W>, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Locate_DevNodeW" fn CM_Locate_DevInstW(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_W, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Locate_DevInstW(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Locate_DevInst_ExA(pdndevinst: PDEVINST, pdeviceid: Option<DEVINSTID_A>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Locate_DevNode_ExA" fn CM_Locate_DevInst_ExA(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_A, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Locate_DevInst_ExA(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Locate_DevInst_ExW(pdndevinst: PDEVINST, pdeviceid: Option<DEVINSTID_W>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Locate_DevNode_ExW" fn CM_Locate_DevInst_ExW(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_W, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Locate_DevInst_ExW(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Locate_DevNodeA(pdndevinst: PDEVINST, pdeviceid: Option<DEVINSTID_A>, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Locate_DevNodeA(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_A, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Locate_DevNodeA(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Locate_DevNodeW(pdndevinst: PDEVINST, pdeviceid: Option<DEVINSTID_W>, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Locate_DevNodeW(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_W, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Locate_DevNodeW(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Locate_DevNode_ExA(pdndevinst: PDEVINST, pdeviceid: Option<DEVINSTID_A>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Locate_DevNode_ExA(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_A, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Locate_DevNode_ExA(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Locate_DevNode_ExW(pdndevinst: PDEVINST, pdeviceid: Option<DEVINSTID_W>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Locate_DevNode_ExW(pdndevinst : PDEVINST, pdeviceid : DEVINSTID_W, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Locate_DevNode_ExW(pdndevinst as _, pdeviceid.unwrap_or(core::mem::zeroed()) as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_MapCrToWin32Err(cmreturncode: CONFIGRET, defaulterr: super::DWORD) -> super::DWORD {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_MapCrToWin32Err(cmreturncode : CONFIGRET, defaulterr : super::DWORD) -> super::DWORD);
    unsafe { CM_MapCrToWin32Err(cmreturncode, defaulterr) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Merge_Range_List(rlhold1: RANGE_LIST, rlhold2: RANGE_LIST, rlhnew: RANGE_LIST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Merge_Range_List(rlhold1 : RANGE_LIST, rlhold2 : RANGE_LIST, rlhnew : RANGE_LIST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Merge_Range_List(rlhold1, rlhold2, rlhnew, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Modify_Res_Des(prdresdes: PRES_DES, rdresdes: RES_DES, resourceid: RESOURCEID, resourcedata: PCVOID, resourcelen: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Modify_Res_Des(prdresdes : PRES_DES, rdresdes : RES_DES, resourceid : RESOURCEID, resourcedata : PCVOID, resourcelen : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Modify_Res_Des(prdresdes as _, rdresdes, resourceid, resourcedata, resourcelen, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Modify_Res_Des_Ex(prdresdes: PRES_DES, rdresdes: RES_DES, resourceid: RESOURCEID, resourcedata: PCVOID, resourcelen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Modify_Res_Des_Ex(prdresdes : PRES_DES, rdresdes : RES_DES, resourceid : RESOURCEID, resourcedata : PCVOID, resourcelen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Modify_Res_Des_Ex(prdresdes as _, rdresdes, resourceid, resourcedata, resourcelen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Move_DevInst(dnfromdevinst: DEVINST, dntodevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Move_DevNode" fn CM_Move_DevInst(dnfromdevinst : DEVINST, dntodevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Move_DevInst(dnfromdevinst, dntodevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Move_DevInst_Ex(dnfromdevinst: DEVINST, dntodevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Move_DevNode_Ex" fn CM_Move_DevInst_Ex(dnfromdevinst : DEVINST, dntodevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Move_DevInst_Ex(dnfromdevinst, dntodevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Move_DevNode(dnfromdevinst: DEVINST, dntodevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Move_DevNode(dnfromdevinst : DEVINST, dntodevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Move_DevNode(dnfromdevinst, dntodevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Move_DevNode_Ex(dnfromdevinst: DEVINST, dntodevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Move_DevNode_Ex(dnfromdevinst : DEVINST, dntodevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Move_DevNode_Ex(dnfromdevinst, dntodevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Next_Range(preelement: PRANGE_ELEMENT, pullstart: super::PDWORDLONG, pullend: super::PDWORDLONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Next_Range(preelement : PRANGE_ELEMENT, pullstart : super::PDWORDLONG, pullend : super::PDWORDLONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Next_Range(preelement as _, pullstart as _, pullend as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Class_KeyA<P1>(classguid: Option<super::LPGUID>, pszclassname: P1, samdesired: super::REGSAM, disposition: REGDISPOSITION, phkclass: super::PHKEY, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Class_KeyA(classguid : super::LPGUID, pszclassname : windows_core::PCSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkclass : super::PHKEY, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Open_Class_KeyA(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Class_KeyW<P1>(classguid: Option<super::LPGUID>, pszclassname: P1, samdesired: super::REGSAM, disposition: REGDISPOSITION, phkclass: super::PHKEY, ulflags: super::ULONG) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Class_KeyW(classguid : super::LPGUID, pszclassname : windows_core::PCWSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkclass : super::PHKEY, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Open_Class_KeyW(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Class_Key_ExA<P1>(classguid: Option<super::LPGUID>, pszclassname: P1, samdesired: super::REGSAM, disposition: REGDISPOSITION, phkclass: super::PHKEY, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Class_Key_ExA(classguid : super::LPGUID, pszclassname : windows_core::PCSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkclass : super::PHKEY, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_Class_Key_ExA(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Class_Key_ExW<P1>(classguid: Option<super::LPGUID>, pszclassname: P1, samdesired: super::REGSAM, disposition: REGDISPOSITION, phkclass: super::PHKEY, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Class_Key_ExW(classguid : super::LPGUID, pszclassname : windows_core::PCWSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkclass : super::PHKEY, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_Class_Key_ExW(classguid.unwrap_or(core::mem::zeroed()) as _, pszclassname.param().abi(), samdesired, disposition, phkclass as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_DevInst_Key(dndevnode: DEVINST, samdesired: super::REGSAM, ulhardwareprofile: super::ULONG, disposition: REGDISPOSITION, phkdevice: super::PHKEY, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Open_DevNode_Key" fn CM_Open_DevInst_Key(dndevnode : DEVINST, samdesired : super::REGSAM, ulhardwareprofile : super::ULONG, disposition : REGDISPOSITION, phkdevice : super::PHKEY, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Open_DevInst_Key(dndevnode, samdesired, ulhardwareprofile, disposition, phkdevice as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_DevInst_Key_Ex(dndevnode: DEVINST, samdesired: super::REGSAM, ulhardwareprofile: super::ULONG, disposition: REGDISPOSITION, phkdevice: super::PHKEY, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Open_DevNode_Key_Ex" fn CM_Open_DevInst_Key_Ex(dndevnode : DEVINST, samdesired : super::REGSAM, ulhardwareprofile : super::ULONG, disposition : REGDISPOSITION, phkdevice : super::PHKEY, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_DevInst_Key_Ex(dndevnode, samdesired, ulhardwareprofile, disposition, phkdevice as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_DevNode_Key(dndevnode: DEVINST, samdesired: super::REGSAM, ulhardwareprofile: super::ULONG, disposition: REGDISPOSITION, phkdevice: super::PHKEY, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Open_DevNode_Key(dndevnode : DEVINST, samdesired : super::REGSAM, ulhardwareprofile : super::ULONG, disposition : REGDISPOSITION, phkdevice : super::PHKEY, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Open_DevNode_Key(dndevnode, samdesired, ulhardwareprofile, disposition, phkdevice as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_DevNode_Key_Ex(dndevnode: DEVINST, samdesired: super::REGSAM, ulhardwareprofile: super::ULONG, disposition: REGDISPOSITION, phkdevice: super::PHKEY, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Open_DevNode_Key_Ex(dndevnode : DEVINST, samdesired : super::REGSAM, ulhardwareprofile : super::ULONG, disposition : REGDISPOSITION, phkdevice : super::PHKEY, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_DevNode_Key_Ex(dndevnode, samdesired, ulhardwareprofile, disposition, phkdevice as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Device_Interface_KeyA<P0>(pszdeviceinterface: P0, samdesired: super::REGSAM, disposition: REGDISPOSITION, phkdeviceinterface: super::PHKEY, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_KeyA(pszdeviceinterface : windows_core::PCSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : super::PHKEY, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_KeyA(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Device_Interface_KeyW<P0>(pszdeviceinterface: P0, samdesired: super::REGSAM, disposition: REGDISPOSITION, phkdeviceinterface: super::PHKEY, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_KeyW(pszdeviceinterface : windows_core::PCWSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : super::PHKEY, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_KeyW(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Device_Interface_Key_ExA<P0>(pszdeviceinterface: P0, samdesired: super::REGSAM, disposition: REGDISPOSITION, phkdeviceinterface: super::PHKEY, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_Key_ExA(pszdeviceinterface : windows_core::PCSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : super::PHKEY, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_Key_ExA(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winreg"))]
#[inline]
pub unsafe fn CM_Open_Device_Interface_Key_ExW<P0>(pszdeviceinterface: P0, samdesired: super::REGSAM, disposition: REGDISPOSITION, phkdeviceinterface: super::PHKEY, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_Key_ExW(pszdeviceinterface : windows_core::PCWSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : super::PHKEY, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Open_Device_Interface_Key_ExW(pszdeviceinterface.param().abi(), samdesired, disposition, phkdeviceinterface as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "cfg", feature = "shared"))]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTreeA(dnancestor: DEVINST, pvetotype: Option<super::PPNP_VETO_TYPE>, pszvetoname: Option<windows_core::PSTR>, ulnamelength: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTreeA(dnancestor : DEVINST, pvetotype : super::PPNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTreeA(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, pszvetoname.unwrap_or(core::mem::zeroed()) as _, ulnamelength, ulflags) }
}
#[cfg(all(feature = "cfg", feature = "shared"))]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTreeW(dnancestor: DEVINST, pvetotype: Option<super::PPNP_VETO_TYPE>, pszvetoname: Option<windows_core::PWSTR>, ulnamelength: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTreeW(dnancestor : DEVINST, pvetotype : super::PPNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTreeW(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, pszvetoname.unwrap_or(core::mem::zeroed()) as _, ulnamelength, ulflags) }
}
#[cfg(all(feature = "cfg", feature = "shared"))]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTree_ExA(dnancestor: DEVINST, pvetotype: Option<super::PPNP_VETO_TYPE>, pszvetoname: Option<windows_core::PSTR>, ulnamelength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTree_ExA(dnancestor : DEVINST, pvetotype : super::PPNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTree_ExA(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, pszvetoname.unwrap_or(core::mem::zeroed()) as _, ulnamelength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "cfg", feature = "shared"))]
#[inline]
pub unsafe fn CM_Query_And_Remove_SubTree_ExW(dnancestor: DEVINST, pvetotype: Option<super::PPNP_VETO_TYPE>, pszvetoname: Option<windows_core::PWSTR>, ulnamelength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTree_ExW(dnancestor : DEVINST, pvetotype : super::PPNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_And_Remove_SubTree_ExW(dnancestor, pvetotype.unwrap_or(core::mem::zeroed()) as _, pszvetoname.unwrap_or(core::mem::zeroed()) as _, ulnamelength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Data(pdata: super::PVOID, datalen: super::ULONG, dndevinst: DEVINST, resourceid: RESOURCEID, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Data(pdata : super::PVOID, datalen : super::ULONG, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Data(pdata as _, datalen, dndevinst, resourceid, ulflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Data_Ex(pdata: super::PVOID, datalen: super::ULONG, dndevinst: DEVINST, resourceid: RESOURCEID, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Data_Ex(pdata : super::PVOID, datalen : super::ULONG, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Data_Ex(pdata as _, datalen, dndevinst, resourceid, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Size(pulsize: super::PULONG, dndevinst: DEVINST, resourceid: RESOURCEID, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Size(pulsize : super::PULONG, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Size(pulsize as _, dndevinst, resourceid, ulflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Query_Arbitrator_Free_Size_Ex(pulsize: super::PULONG, dndevinst: DEVINST, resourceid: RESOURCEID, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Size_Ex(pulsize : super::PULONG, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_Arbitrator_Free_Size_Ex(pulsize as _, dndevinst, resourceid, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Query_Remove_SubTree(dnancestor: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Remove_SubTree(dnancestor : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Query_Remove_SubTree(dnancestor, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Query_Remove_SubTree_Ex(dnancestor: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Remove_SubTree_Ex(dnancestor : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_Remove_SubTree_Ex(dnancestor, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Query_Resource_Conflict_List(pclconflictlist: PCONFLICT_LIST, dndevinst: DEVINST, resourceid: RESOURCEID, resourcedata: PCVOID, resourcelen: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Query_Resource_Conflict_List(pclconflictlist : PCONFLICT_LIST, dndevinst : DEVINST, resourceid : RESOURCEID, resourcedata : PCVOID, resourcelen : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Query_Resource_Conflict_List(pclconflictlist as _, dndevinst, resourceid, resourcedata, resourcelen, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Reenumerate_DevInst(dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Reenumerate_DevNode" fn CM_Reenumerate_DevInst(dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Reenumerate_DevInst(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Reenumerate_DevInst_Ex(dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Reenumerate_DevNode_Ex" fn CM_Reenumerate_DevInst_Ex(dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Reenumerate_DevInst_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Reenumerate_DevNode(dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Reenumerate_DevNode(dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Reenumerate_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Reenumerate_DevNode_Ex(dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Reenumerate_DevNode_Ex(dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Reenumerate_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Register_Device_Driver(dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_Driver(dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Register_Device_Driver(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Register_Device_Driver_Ex(dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_Driver_Ex(dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Register_Device_Driver_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Register_Device_InterfaceA<P2>(dndevinst: DEVINST, interfaceclassguid: super::LPGUID, pszreference: P2, pszdeviceinterface: windows_core::PSTR, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_InterfaceA(dndevinst : DEVINST, interfaceclassguid : super::LPGUID, pszreference : windows_core::PCSTR, pszdeviceinterface : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Register_Device_InterfaceA(dndevinst, interfaceclassguid, pszreference.param().abi(), pszdeviceinterface, pullength as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Register_Device_InterfaceW<P2>(dndevinst: DEVINST, interfaceclassguid: super::LPGUID, pszreference: P2, pszdeviceinterface: windows_core::PWSTR, pullength: super::PULONG, ulflags: super::ULONG) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_InterfaceW(dndevinst : DEVINST, interfaceclassguid : super::LPGUID, pszreference : windows_core::PCWSTR, pszdeviceinterface : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Register_Device_InterfaceW(dndevinst, interfaceclassguid, pszreference.param().abi(), pszdeviceinterface, pullength as _, ulflags) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Register_Device_Interface_ExA<P2>(dndevinst: DEVINST, interfaceclassguid: super::LPGUID, pszreference: P2, pszdeviceinterface: windows_core::PSTR, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_Interface_ExA(dndevinst : DEVINST, interfaceclassguid : super::LPGUID, pszreference : windows_core::PCSTR, pszdeviceinterface : windows_core::PSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Register_Device_Interface_ExA(dndevinst, interfaceclassguid, pszreference.param().abi(), pszdeviceinterface, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Register_Device_Interface_ExW<P2>(dndevinst: DEVINST, interfaceclassguid: super::LPGUID, pszreference: P2, pszdeviceinterface: windows_core::PWSTR, pullength: super::PULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Register_Device_Interface_ExW(dndevinst : DEVINST, interfaceclassguid : super::LPGUID, pszreference : windows_core::PCWSTR, pszdeviceinterface : windows_core::PWSTR, pullength : super::PULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Register_Device_Interface_ExW(dndevinst, interfaceclassguid, pszreference.param().abi(), pszdeviceinterface, pullength as _, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CM_Register_Notification(pfilter: PCM_NOTIFY_FILTER, pcontext: Option<super::PVOID>, pcallback: PCM_NOTIFY_CALLBACK, pnotifycontext: PHCMNOTIFICATION) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Register_Notification(pfilter : PCM_NOTIFY_FILTER, pcontext : super::PVOID, pcallback : PCM_NOTIFY_CALLBACK, pnotifycontext : PHCMNOTIFICATION) -> CONFIGRET);
    unsafe { CM_Register_Notification(pfilter, pcontext.unwrap_or(core::mem::zeroed()) as _, pcallback, pnotifycontext as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Remove_SubTree(dnancestor: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Remove_SubTree(dnancestor : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Remove_SubTree(dnancestor, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Remove_SubTree_Ex(dnancestor: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Remove_SubTree_Ex(dnancestor : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Remove_SubTree_Ex(dnancestor, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "cfg", feature = "shared"))]
#[inline]
pub unsafe fn CM_Request_Device_EjectA(dndevinst: DEVINST, pvetotype: Option<super::PPNP_VETO_TYPE>, pszvetoname: Option<windows_core::PSTR>, ulnamelength: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Device_EjectA(dndevinst : DEVINST, pvetotype : super::PPNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Request_Device_EjectA(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, pszvetoname.unwrap_or(core::mem::zeroed()) as _, ulnamelength, ulflags) }
}
#[cfg(all(feature = "cfg", feature = "shared"))]
#[inline]
pub unsafe fn CM_Request_Device_EjectW(dndevinst: DEVINST, pvetotype: Option<super::PPNP_VETO_TYPE>, pszvetoname: Option<windows_core::PWSTR>, ulnamelength: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Device_EjectW(dndevinst : DEVINST, pvetotype : super::PPNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Request_Device_EjectW(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, pszvetoname.unwrap_or(core::mem::zeroed()) as _, ulnamelength, ulflags) }
}
#[cfg(all(feature = "cfg", feature = "shared"))]
#[inline]
pub unsafe fn CM_Request_Device_Eject_ExA(dndevinst: DEVINST, pvetotype: Option<super::PPNP_VETO_TYPE>, pszvetoname: Option<windows_core::PSTR>, ulnamelength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Device_Eject_ExA(dndevinst : DEVINST, pvetotype : super::PPNP_VETO_TYPE, pszvetoname : windows_core::PSTR, ulnamelength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Request_Device_Eject_ExA(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, pszvetoname.unwrap_or(core::mem::zeroed()) as _, ulnamelength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "cfg", feature = "shared"))]
#[inline]
pub unsafe fn CM_Request_Device_Eject_ExW(dndevinst: DEVINST, pvetotype: Option<super::PPNP_VETO_TYPE>, pszvetoname: Option<windows_core::PWSTR>, ulnamelength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Device_Eject_ExW(dndevinst : DEVINST, pvetotype : super::PPNP_VETO_TYPE, pszvetoname : windows_core::PWSTR, ulnamelength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Request_Device_Eject_ExW(dndevinst, pvetotype.unwrap_or(core::mem::zeroed()) as _, pszvetoname.unwrap_or(core::mem::zeroed()) as _, ulnamelength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Request_Eject_PC() -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Eject_PC() -> CONFIGRET);
    unsafe { CM_Request_Eject_PC() }
}
#[inline]
pub unsafe fn CM_Request_Eject_PC_Ex(hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Request_Eject_PC_Ex(hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Request_Eject_PC_Ex(hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Run_Detection(ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Run_Detection(ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Run_Detection(ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Run_Detection_Ex(ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Run_Detection_Ex(ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Run_Detection_Ex(ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "devpropdef", feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Set_Class_PropertyW(classguid: super::LPCGUID, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Class_PropertyW(classguid : super::LPCGUID, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_Class_PropertyW(classguid, propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Set_Class_Property_ExW(classguid: super::LPCGUID, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Class_Property_ExW(classguid : super::LPCGUID, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_Class_Property_ExW(classguid, propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Set_Class_Registry_PropertyA(classguid: super::LPGUID, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_Class_Registry_PropertyA(classguid : super::LPGUID, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_Class_Registry_PropertyA(classguid, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Set_Class_Registry_PropertyW(classguid: super::LPGUID, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_Class_Registry_PropertyW(classguid : super::LPGUID, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_Class_Registry_PropertyW(classguid, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevInst_Problem(dndevinst: DEVINST, ulproblem: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Set_DevNode_Problem" fn CM_Set_DevInst_Problem(dndevinst : DEVINST, ulproblem : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_DevInst_Problem(dndevinst, ulproblem, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevInst_Problem_Ex(dndevinst: DEVINST, ulproblem: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Set_DevNode_Problem_Ex" fn CM_Set_DevInst_Problem_Ex(dndevinst : DEVINST, ulproblem : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevInst_Problem_Ex(dndevinst, ulproblem, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevInst_Registry_PropertyA(dndevinst: DEVINST, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Set_DevNode_Registry_PropertyA" fn CM_Set_DevInst_Registry_PropertyA(dndevinst : DEVINST, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_DevInst_Registry_PropertyA(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevInst_Registry_PropertyW(dndevinst: DEVINST, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Set_DevNode_Registry_PropertyW" fn CM_Set_DevInst_Registry_PropertyW(dndevinst : DEVINST, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_DevInst_Registry_PropertyW(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevInst_Registry_Property_ExA(dndevinst: DEVINST, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Set_DevNode_Registry_Property_ExA" fn CM_Set_DevInst_Registry_Property_ExA(dndevinst : DEVINST, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevInst_Registry_Property_ExA(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevInst_Registry_Property_ExW(dndevinst: DEVINST, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Set_DevNode_Registry_Property_ExW" fn CM_Set_DevInst_Registry_Property_ExW(dndevinst : DEVINST, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevInst_Registry_Property_ExW(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevNode_Problem(dndevinst: DEVINST, ulproblem: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Problem(dndevinst : DEVINST, ulproblem : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Problem(dndevinst, ulproblem, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevNode_Problem_Ex(dndevinst: DEVINST, ulproblem: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Problem_Ex(dndevinst : DEVINST, ulproblem : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Problem_Ex(dndevinst, ulproblem, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "devpropdef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Set_DevNode_PropertyW(dndevinst: DEVINST, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_PropertyW(dndevinst : DEVINST, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_DevNode_PropertyW(dndevinst, propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Set_DevNode_Property_ExW(dndevinst: DEVINST, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Property_ExW(dndevinst : DEVINST, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Property_ExW(dndevinst, propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_PropertyA(dndevinst: DEVINST, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_PropertyA(dndevinst : DEVINST, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_PropertyA(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_PropertyW(dndevinst: DEVINST, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_PropertyW(dndevinst : DEVINST, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_PropertyW(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_Property_ExA(dndevinst: DEVINST, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_Property_ExA(dndevinst : DEVINST, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_Property_ExA(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_DevNode_Registry_Property_ExW(dndevinst: DEVINST, ulproperty: super::ULONG, buffer: Option<PCVOID>, ullength: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_Property_ExW(dndevinst : DEVINST, ulproperty : super::ULONG, buffer : PCVOID, ullength : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_DevNode_Registry_Property_ExW(dndevinst, ulproperty, buffer.unwrap_or(core::mem::zeroed()) as _, ullength, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "devpropdef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Set_Device_Interface_PropertyW<P0>(pszdeviceinterface: P0, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Device_Interface_PropertyW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_Device_Interface_PropertyW(pszdeviceinterface.param().abi(), propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags) }
}
#[cfg(all(feature = "devpropdef", feature = "shared"))]
#[inline]
pub unsafe fn CM_Set_Device_Interface_Property_ExW<P0>(pszdeviceinterface: P0, propertykey: *const super::DEVPROPKEY, propertytype: super::DEVPROPTYPE, propertybuffer: Option<&[u8]>, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Set_Device_Interface_Property_ExW(pszdeviceinterface : windows_core::PCWSTR, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : super::PBYTE, propertybuffersize : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_Device_Interface_Property_ExW(pszdeviceinterface.param().abi(), propertykey, propertytype, core::mem::transmute(propertybuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), propertybuffer.map_or(0, |slice| slice.len().try_into().unwrap()), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_HW_Prof(ulhardwareprofile: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof(ulhardwareprofile : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof(ulhardwareprofile, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_HW_Prof_Ex(ulhardwareprofile: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_Ex(ulhardwareprofile : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_Ex(ulhardwareprofile, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_HW_Prof_FlagsA(pdeviceid: DEVINSTID_A, ulconfig: super::ULONG, ulvalue: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_FlagsA(pdeviceid : DEVINSTID_A, ulconfig : super::ULONG, ulvalue : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_FlagsA(pdeviceid, ulconfig, ulvalue, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_HW_Prof_FlagsW(pdeviceid: DEVINSTID_W, ulconfig: super::ULONG, ulvalue: super::ULONG, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_FlagsW(pdeviceid : DEVINSTID_W, ulconfig : super::ULONG, ulvalue : super::ULONG, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_FlagsW(pdeviceid, ulconfig, ulvalue, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_HW_Prof_Flags_ExA(pdeviceid: DEVINSTID_A, ulconfig: super::ULONG, ulvalue: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_Flags_ExA(pdeviceid : DEVINSTID_A, ulconfig : super::ULONG, ulvalue : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_Flags_ExA(pdeviceid, ulconfig, ulvalue, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Set_HW_Prof_Flags_ExW(pdeviceid: DEVINSTID_W, ulconfig: super::ULONG, ulvalue: super::ULONG, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_Flags_ExW(pdeviceid : DEVINSTID_W, ulconfig : super::ULONG, ulvalue : super::ULONG, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Set_HW_Prof_Flags_ExW(pdeviceid, ulconfig, ulvalue, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Setup_DevInst(dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Setup_DevNode" fn CM_Setup_DevInst(dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Setup_DevInst(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Setup_DevInst_Ex(dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Setup_DevNode_Ex" fn CM_Setup_DevInst_Ex(dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Setup_DevInst_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Setup_DevNode(dndevinst: DEVINST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Setup_DevNode(dndevinst : DEVINST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Setup_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Setup_DevNode_Ex(dndevinst: DEVINST, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Setup_DevNode_Ex(dndevinst : DEVINST, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Setup_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Test_Range_Available(ullstartvalue: super::DWORDLONG, ullendvalue: super::DWORDLONG, rlh: RANGE_LIST, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Test_Range_Available(ullstartvalue : super::DWORDLONG, ullendvalue : super::DWORDLONG, rlh : RANGE_LIST, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Test_Range_Available(ullstartvalue, ullendvalue, rlh, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Uninstall_DevInst(dndevinst: DEVNODE, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Uninstall_DevNode" fn CM_Uninstall_DevInst(dndevinst : DEVNODE, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Uninstall_DevInst(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Uninstall_DevInst_Ex(dndevinst: DEVNODE, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" "CM_Uninstall_DevNode_Ex" fn CM_Uninstall_DevInst_Ex(dndevinst : DEVNODE, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Uninstall_DevInst_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Uninstall_DevNode(dndevinst: DEVNODE, ulflags: super::ULONG) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Uninstall_DevNode(dndevinst : DEVNODE, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Uninstall_DevNode(dndevinst, ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Uninstall_DevNode_Ex(dndevinst: DEVNODE, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET {
    windows_core::link!("setupapi.dll" "system" fn CM_Uninstall_DevNode_Ex(dndevinst : DEVNODE, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Uninstall_DevNode_Ex(dndevinst, ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Unregister_Device_InterfaceA<P0>(pszdeviceinterface: P0, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Unregister_Device_InterfaceA(pszdeviceinterface : windows_core::PCSTR, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Unregister_Device_InterfaceA(pszdeviceinterface.param().abi(), ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Unregister_Device_InterfaceW<P0>(pszdeviceinterface: P0, ulflags: super::ULONG) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Unregister_Device_InterfaceW(pszdeviceinterface : windows_core::PCWSTR, ulflags : super::ULONG) -> CONFIGRET);
    unsafe { CM_Unregister_Device_InterfaceW(pszdeviceinterface.param().abi(), ulflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Unregister_Device_Interface_ExA<P0>(pszdeviceinterface: P0, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Unregister_Device_Interface_ExA(pszdeviceinterface : windows_core::PCSTR, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Unregister_Device_Interface_ExA(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_Unregister_Device_Interface_ExW<P0>(pszdeviceinterface: P0, ulflags: super::ULONG, hmachine: Option<HMACHINE>) -> CONFIGRET
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("setupapi.dll" "system" fn CM_Unregister_Device_Interface_ExW(pszdeviceinterface : windows_core::PCWSTR, ulflags : super::ULONG, hmachine : HMACHINE) -> CONFIGRET);
    unsafe { CM_Unregister_Device_Interface_ExW(pszdeviceinterface.param().abi(), ulflags, hmachine.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CM_Unregister_Notification(notifycontext: HCMNOTIFICATION) -> CONFIGRET {
    windows_core::link!("cfgmgr32.dll" "system" fn CM_Unregister_Notification(notifycontext : HCMNOTIFICATION) -> CONFIGRET);
    unsafe { CM_Unregister_Notification(notifycontext) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CM_WaitNoPendingInstallEvents(dwtimeout: super::DWORD) -> super::DWORD {
    windows_core::link!("setupapi.dll" "system" "CMP_WaitNoPendingInstallEvents" fn CM_WaitNoPendingInstallEvents(dwtimeout : super::DWORD) -> super::DWORD);
    unsafe { CM_WaitNoPendingInstallEvents(dwtimeout) }
}
pub const ALLOC_LOG_CONF: i32 = 2;
pub const BASIC_LOG_CONF: i32 = 0;
pub const BOOT_LOG_CONF: i32 = 3;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BUSNUMBER_DES {
    pub BUSD_Count: super::DWORD,
    pub BUSD_Type: super::DWORD,
    pub BUSD_Flags: super::DWORD,
    pub BUSD_Alloc_Base: super::ULONG,
    pub BUSD_Alloc_End: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BUSNUMBER_RANGE {
    pub BUSR_Min: super::ULONG,
    pub BUSR_Max: super::ULONG,
    pub BUSR_nBusNumbers: super::ULONG,
    pub BUSR_Flags: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct BUSNUMBER_RESOURCE {
    pub BusNumber_Header: BUSNUMBER_DES,
    pub BusNumber_Data: [BUSNUMBER_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for BUSNUMBER_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const BusNumberType_Range: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const BusNumberType_Range: u64 = 16;
pub const CM_ADD_ID_BITS: i32 = 1;
pub const CM_ADD_ID_COMPATIBLE: i32 = 1;
pub const CM_ADD_ID_HARDWARE: i32 = 0;
pub const CM_ADD_RANGE_ADDIFCONFLICT: i32 = 0;
pub const CM_ADD_RANGE_BITS: i32 = 1;
pub const CM_ADD_RANGE_DONOTADDIFCONFLICT: i32 = 1;
pub const CM_CDFLAGS_DRIVER: i32 = 1;
pub const CM_CDFLAGS_RESERVED: i32 = 4;
pub const CM_CDFLAGS_ROOT_OWNED: i32 = 2;
pub const CM_CDMASK_DESCRIPTION: i32 = 8;
pub const CM_CDMASK_DEVINST: i32 = 1;
pub const CM_CDMASK_FLAGS: i32 = 4;
pub const CM_CDMASK_RESDES: i32 = 2;
pub const CM_CDMASK_VALID: i32 = 15;
pub const CM_CLASS_PROPERTY_BITS: i32 = 1;
pub const CM_CLASS_PROPERTY_INSTALLER: i32 = 0;
pub const CM_CLASS_PROPERTY_INTERFACE: i32 = 1;
pub const CM_CREATE_DEVINST_BITS: i32 = 15;
pub const CM_CREATE_DEVINST_DO_NOT_INSTALL: i32 = 8;
pub const CM_CREATE_DEVINST_GENERATE_ID: i32 = 4;
pub const CM_CREATE_DEVINST_NORMAL: i32 = 0;
pub const CM_CREATE_DEVINST_NO_WAIT_INSTALL: i32 = 1;
pub const CM_CREATE_DEVINST_PHANTOM: i32 = 2;
pub const CM_CREATE_DEVNODE_BITS: i32 = 15;
pub const CM_CREATE_DEVNODE_DO_NOT_INSTALL: i32 = 8;
pub const CM_CREATE_DEVNODE_GENERATE_ID: i32 = 4;
pub const CM_CREATE_DEVNODE_NORMAL: i32 = 0;
pub const CM_CREATE_DEVNODE_NO_WAIT_INSTALL: i32 = 1;
pub const CM_CREATE_DEVNODE_PHANTOM: i32 = 2;
pub const CM_CRP_CHARACTERISTICS: i32 = 28;
pub const CM_CRP_DEVTYPE: i32 = 26;
pub const CM_CRP_EXCLUSIVE: i32 = 27;
pub const CM_CRP_LOWERFILTERS: i32 = 19;
pub const CM_CRP_MAX: i32 = 37;
pub const CM_CRP_MIN: i32 = 1;
pub const CM_CRP_SECURITY: i32 = 24;
pub const CM_CRP_SECURITY_SDS: i32 = 25;
pub const CM_CRP_UPPERFILTERS: i32 = 18;
pub const CM_CUSTOMDEVPROP_BITS: i32 = 1;
pub const CM_CUSTOMDEVPROP_MERGE_MULTISZ: i32 = 1;
pub const CM_DELETE_CLASS_BITS: i32 = 3;
pub const CM_DELETE_CLASS_INTERFACE: i32 = 2;
pub const CM_DELETE_CLASS_ONLY: i32 = 0;
pub const CM_DELETE_CLASS_SUBKEYS: i32 = 1;
pub const CM_DETECT_BITS: u32 = 2147483655;
pub const CM_DETECT_CRASHED: i32 = 2;
pub const CM_DETECT_HWPROF_FIRST_BOOT: i32 = 4;
pub const CM_DETECT_NEW_PROFILE: i32 = 1;
pub const CM_DETECT_RUN: u32 = 2147483648;
pub const CM_DEVCAP_DOCKDEVICE: i32 = 8;
pub const CM_DEVCAP_EJECTSUPPORTED: i32 = 2;
pub const CM_DEVCAP_HARDWAREDISABLED: i32 = 256;
pub const CM_DEVCAP_LOCKSUPPORTED: i32 = 1;
pub const CM_DEVCAP_NONDYNAMIC: i32 = 512;
pub const CM_DEVCAP_RAWDEVICEOK: i32 = 64;
pub const CM_DEVCAP_REMOVABLE: i32 = 4;
pub const CM_DEVCAP_SECUREDEVICE: i32 = 1024;
pub const CM_DEVCAP_SILENTINSTALL: i32 = 32;
pub const CM_DEVCAP_SURPRISEREMOVALOK: i32 = 128;
pub const CM_DEVCAP_UNIQUEID: i32 = 16;
pub const CM_DISABLE_ABSOLUTE: i32 = 1;
pub const CM_DISABLE_BITS: i32 = 15;
pub const CM_DISABLE_HARDWARE: i32 = 2;
pub const CM_DISABLE_PERSIST: i32 = 8;
pub const CM_DISABLE_POLITE: i32 = 0;
pub const CM_DISABLE_UI_NOT_OK: i32 = 4;
pub const CM_DRP_ADDRESS: i32 = 29;
pub const CM_DRP_BASE_CONTAINERID: i32 = 37;
pub const CM_DRP_BUSNUMBER: i32 = 22;
pub const CM_DRP_BUSTYPEGUID: i32 = 20;
pub const CM_DRP_CAPABILITIES: i32 = 16;
pub const CM_DRP_CHARACTERISTICS: i32 = 28;
pub const CM_DRP_CLASS: i32 = 8;
pub const CM_DRP_CLASSGUID: i32 = 9;
pub const CM_DRP_COMPATIBLEIDS: i32 = 3;
pub const CM_DRP_CONFIGFLAGS: i32 = 11;
pub const CM_DRP_DEVICEDESC: i32 = 1;
pub const CM_DRP_DEVICE_POWER_DATA: i32 = 31;
pub const CM_DRP_DEVTYPE: i32 = 26;
pub const CM_DRP_DRIVER: i32 = 10;
pub const CM_DRP_ENUMERATOR_NAME: i32 = 23;
pub const CM_DRP_EXCLUSIVE: i32 = 27;
pub const CM_DRP_FRIENDLYNAME: i32 = 13;
pub const CM_DRP_HARDWAREID: i32 = 2;
pub const CM_DRP_INSTALL_STATE: i32 = 35;
pub const CM_DRP_LEGACYBUSTYPE: i32 = 21;
pub const CM_DRP_LOCATION_INFORMATION: i32 = 14;
pub const CM_DRP_LOCATION_PATHS: i32 = 36;
pub const CM_DRP_LOWERFILTERS: i32 = 19;
pub const CM_DRP_MAX: i32 = 37;
pub const CM_DRP_MFG: i32 = 12;
pub const CM_DRP_MIN: i32 = 1;
pub const CM_DRP_PHYSICAL_DEVICE_OBJECT_NAME: i32 = 15;
pub const CM_DRP_REMOVAL_POLICY: i32 = 32;
pub const CM_DRP_REMOVAL_POLICY_HW_DEFAULT: i32 = 33;
pub const CM_DRP_REMOVAL_POLICY_OVERRIDE: i32 = 34;
pub const CM_DRP_SECURITY: i32 = 24;
pub const CM_DRP_SECURITY_SDS: i32 = 25;
pub const CM_DRP_SERVICE: i32 = 5;
pub const CM_DRP_UI_NUMBER: i32 = 17;
pub const CM_DRP_UI_NUMBER_DESC_FORMAT: i32 = 30;
pub const CM_DRP_UNUSED0: i32 = 4;
pub const CM_DRP_UNUSED1: i32 = 6;
pub const CM_DRP_UNUSED2: i32 = 7;
pub const CM_DRP_UPPERFILTERS: i32 = 18;
pub const CM_ENUMERATE_CLASSES_BITS: i32 = 1;
pub const CM_ENUMERATE_CLASSES_INSTALLER: i32 = 0;
pub const CM_ENUMERATE_CLASSES_INTERFACE: i32 = 1;
pub const CM_GETIDLIST_DONOTGENERATE: i32 = 268435520;
pub const CM_GETIDLIST_FILTER_BITS: i32 = 268436479;
pub const CM_GETIDLIST_FILTER_BUSRELATIONS: i32 = 32;
pub const CM_GETIDLIST_FILTER_CLASS: i32 = 512;
pub const CM_GETIDLIST_FILTER_EJECTRELATIONS: i32 = 4;
pub const CM_GETIDLIST_FILTER_ENUMERATOR: i32 = 1;
pub const CM_GETIDLIST_FILTER_NONE: i32 = 0;
pub const CM_GETIDLIST_FILTER_POWERRELATIONS: i32 = 16;
pub const CM_GETIDLIST_FILTER_PRESENT: i32 = 256;
pub const CM_GETIDLIST_FILTER_REMOVALRELATIONS: i32 = 8;
pub const CM_GETIDLIST_FILTER_SERVICE: i32 = 2;
pub const CM_GETIDLIST_FILTER_TRANSPORTRELATIONS: i32 = 128;
pub const CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES: i32 = 1;
pub const CM_GET_DEVICE_INTERFACE_LIST_BITS: i32 = 1;
pub const CM_GET_DEVICE_INTERFACE_LIST_PRESENT: i32 = 0;
pub const CM_GLOBAL_STATE_CAN_DO_UI: i32 = 1;
pub const CM_GLOBAL_STATE_DETECTION_PENDING: i32 = 16;
pub const CM_GLOBAL_STATE_ON_BIG_STACK: i32 = 2;
pub const CM_GLOBAL_STATE_REBOOT_REQUIRED: i32 = 32;
pub const CM_GLOBAL_STATE_SERVICES_AVAILABLE: i32 = 4;
pub const CM_GLOBAL_STATE_SHUTTING_DOWN: i32 = 8;
pub const CM_HWPI_DOCKED: i32 = 2;
pub const CM_HWPI_NOT_DOCKABLE: i32 = 0;
pub const CM_HWPI_UNDOCKED: i32 = 1;
pub const CM_INSTALL_STATE_FAILED_INSTALL: i32 = 2;
pub const CM_INSTALL_STATE_FINISH_INSTALL: i32 = 3;
pub const CM_INSTALL_STATE_INSTALLED: i32 = 0;
pub const CM_INSTALL_STATE_NEEDS_REINSTALL: i32 = 1;
pub const CM_LOCATE_DEVINST_BITS: i32 = 7;
pub const CM_LOCATE_DEVINST_CANCELREMOVE: i32 = 2;
pub const CM_LOCATE_DEVINST_NORMAL: i32 = 0;
pub const CM_LOCATE_DEVINST_NOVALIDATION: i32 = 4;
pub const CM_LOCATE_DEVINST_PHANTOM: i32 = 1;
pub const CM_LOCATE_DEVNODE_BITS: i32 = 7;
pub const CM_LOCATE_DEVNODE_CANCELREMOVE: i32 = 2;
pub const CM_LOCATE_DEVNODE_NORMAL: i32 = 0;
pub const CM_LOCATE_DEVNODE_NOVALIDATION: i32 = 4;
pub const CM_LOCATE_DEVNODE_PHANTOM: i32 = 1;
pub const CM_NAME_ATTRIBUTE_NAME_RETRIEVED_FROM_DEVICE: i32 = 1;
pub const CM_NAME_ATTRIBUTE_USER_ASSIGNED_NAME: i32 = 2;
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CM_NOTIFY_EVENT_DATA {
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: super::DWORD,
    pub u: CM_NOTIFY_EVENT_DATA_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CM_NOTIFY_EVENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union CM_NOTIFY_EVENT_DATA_0 {
    pub DeviceInterface: CM_NOTIFY_EVENT_DATA_0_0,
    pub DeviceHandle: CM_NOTIFY_EVENT_DATA_0_1,
    pub DeviceInstance: CM_NOTIFY_EVENT_DATA_0_2,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CM_NOTIFY_EVENT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CM_NOTIFY_EVENT_DATA_0_0 {
    pub ClassGuid: windows_core::GUID,
    pub SymbolicLink: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CM_NOTIFY_EVENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CM_NOTIFY_EVENT_DATA_0_1 {
    pub EventGuid: windows_core::GUID,
    pub NameOffset: super::LONG,
    pub DataSize: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CM_NOTIFY_EVENT_DATA_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CM_NOTIFY_EVENT_DATA_0_2 {
    pub InstanceId: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CM_NOTIFY_EVENT_DATA_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CM_NOTIFY_FILTER {
    pub cbSize: super::DWORD,
    pub Flags: super::DWORD,
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: super::DWORD,
    pub u: CM_NOTIFY_FILTER_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CM_NOTIFY_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union CM_NOTIFY_FILTER_0 {
    pub DeviceInterface: CM_NOTIFY_FILTER_0_0,
    pub DeviceHandle: CM_NOTIFY_FILTER_0_1,
    pub DeviceInstance: CM_NOTIFY_FILTER_0_2,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CM_NOTIFY_FILTER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_NOTIFY_FILTER_0_0 {
    pub ClassGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CM_NOTIFY_FILTER_0_1 {
    pub hTarget: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CM_NOTIFY_FILTER_0_2 {
    pub InstanceId: [super::WCHAR; 200],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CM_NOTIFY_FILTER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CM_NOTIFY_FILTER_FLAG_ALL_DEVICE_INSTANCES: i32 = 2;
pub const CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES: i32 = 1;
pub type CM_NOTIFY_FILTER_TYPE = i32;
pub const CM_NOTIFY_FILTER_TYPE_DEVICEHANDLE: CM_NOTIFY_FILTER_TYPE = 1;
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINSTANCE: CM_NOTIFY_FILTER_TYPE = 2;
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINTERFACE: CM_NOTIFY_FILTER_TYPE = 0;
pub const CM_NOTIFY_FILTER_TYPE_MAX: CM_NOTIFY_FILTER_TYPE = 3;
pub const CM_NOTIFY_FILTER_VALID_FLAGS: i32 = 3;
pub const CM_OPEN_CLASS_KEY_BITS: i32 = 1;
pub const CM_OPEN_CLASS_KEY_INSTALLER: i32 = 0;
pub const CM_OPEN_CLASS_KEY_INTERFACE: i32 = 1;
pub const CM_QUERY_ARBITRATOR_BITS: i32 = 1;
pub const CM_QUERY_ARBITRATOR_RAW: i32 = 0;
pub const CM_QUERY_ARBITRATOR_TRANSLATED: i32 = 1;
pub const CM_QUERY_REMOVE_BITS: i32 = 1;
pub const CM_QUERY_REMOVE_UI_NOT_OK: i32 = 1;
pub const CM_QUERY_REMOVE_UI_OK: i32 = 0;
pub const CM_REENUMERATE_ASYNCHRONOUS: i32 = 4;
pub const CM_REENUMERATE_BITS: i32 = 7;
pub const CM_REENUMERATE_NORMAL: i32 = 0;
pub const CM_REENUMERATE_RETRY_INSTALLATION: i32 = 2;
pub const CM_REENUMERATE_SYNCHRONOUS: i32 = 1;
pub const CM_REGISTER_DEVICE_DRIVER_BITS: i32 = 3;
pub const CM_REGISTER_DEVICE_DRIVER_DISABLEABLE: i32 = 1;
pub const CM_REGISTER_DEVICE_DRIVER_REMOVABLE: i32 = 2;
pub const CM_REGISTER_DEVICE_DRIVER_STATIC: i32 = 0;
pub const CM_REGISTRY_BITS: i32 = 769;
pub const CM_REGISTRY_CONFIG: i32 = 512;
pub const CM_REGISTRY_HARDWARE: i32 = 0;
pub const CM_REGISTRY_SOFTWARE: i32 = 1;
pub const CM_REGISTRY_USER: i32 = 256;
pub const CM_REMOVAL_POLICY_EXPECT_NO_REMOVAL: i32 = 1;
pub const CM_REMOVAL_POLICY_EXPECT_ORDERLY_REMOVAL: i32 = 2;
pub const CM_REMOVAL_POLICY_EXPECT_SURPRISE_REMOVAL: i32 = 3;
pub const CM_REMOVE_BITS: i32 = 7;
pub const CM_REMOVE_DISABLE: i32 = 4;
pub const CM_REMOVE_NO_RESTART: i32 = 2;
pub const CM_REMOVE_UI_NOT_OK: i32 = 1;
pub const CM_REMOVE_UI_OK: i32 = 0;
pub const CM_RESDES_WIDTH_32: i32 = 1;
pub const CM_RESDES_WIDTH_64: i32 = 2;
pub const CM_RESDES_WIDTH_BITS: i32 = 3;
pub const CM_RESDES_WIDTH_DEFAULT: i32 = 0;
pub const CM_SETUP_BITS: i32 = 15;
pub const CM_SETUP_DEVINST_CONFIG: i32 = 5;
pub const CM_SETUP_DEVINST_CONFIG_CLASS: i32 = 6;
pub const CM_SETUP_DEVINST_CONFIG_EXTENSIONS: i32 = 7;
pub const CM_SETUP_DEVINST_CONFIG_RESET: i32 = 8;
pub const CM_SETUP_DEVINST_READY: i32 = 0;
pub const CM_SETUP_DEVINST_RESET: i32 = 4;
pub const CM_SETUP_DEVNODE_CONFIG: i32 = 5;
pub const CM_SETUP_DEVNODE_CONFIG_CLASS: i32 = 6;
pub const CM_SETUP_DEVNODE_CONFIG_EXTENSIONS: i32 = 7;
pub const CM_SETUP_DEVNODE_CONFIG_RESET: i32 = 8;
pub const CM_SETUP_DEVNODE_READY: i32 = 0;
pub const CM_SETUP_DEVNODE_RESET: i32 = 4;
pub const CM_SETUP_DOWNLOAD: i32 = 1;
pub const CM_SETUP_PROP_CHANGE: i32 = 3;
pub const CM_SETUP_WRITE_LOG_CONFS: i32 = 2;
pub const CM_SET_DEVINST_PROBLEM_BITS: i32 = 1;
pub const CM_SET_DEVINST_PROBLEM_NORMAL: i32 = 0;
pub const CM_SET_DEVINST_PROBLEM_OVERRIDE: i32 = 1;
pub const CM_SET_DEVNODE_PROBLEM_BITS: i32 = 1;
pub const CM_SET_DEVNODE_PROBLEM_NORMAL: i32 = 0;
pub const CM_SET_DEVNODE_PROBLEM_OVERRIDE: i32 = 1;
pub const CM_SET_HW_PROF_FLAGS_BITS: i32 = 1;
pub const CM_SET_HW_PROF_FLAGS_UI_NOT_OK: i32 = 1;
pub const CONFIGMG_VERSION: i32 = 1024;
pub type CONFIGRET = RETURN_TYPE;
#[cfg(feature = "shared")]
pub type CONFLICT_DETAILS = CONFLICT_DETAILS_A;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONFLICT_DETAILS_A {
    pub CD_ulSize: super::ULONG,
    pub CD_ulMask: super::ULONG,
    pub CD_dnDevInst: DEVINST,
    pub CD_rdResDes: RES_DES,
    pub CD_ulFlags: super::ULONG,
    pub CD_szDescription: [super::CHAR; 260],
}
#[cfg(feature = "shared")]
impl Default for CONFLICT_DETAILS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CONFLICT_DETAILS_W {
    pub CD_ulSize: super::ULONG,
    pub CD_ulMask: super::ULONG,
    pub CD_dnDevInst: DEVINST,
    pub CD_rdResDes: RES_DES,
    pub CD_ulFlags: super::ULONG,
    pub CD_szDescription: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CONFLICT_DETAILS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CONFLICT_LIST = usize;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CONNECTION_DES {
    pub COND_Type: super::DWORD,
    pub COND_Flags: super::DWORD,
    pub COND_Class: super::BYTE,
    pub COND_ClassType: super::BYTE,
    pub COND_Reserved1: super::BYTE,
    pub COND_Reserved2: super::BYTE,
    pub COND_Id: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CONNECTION_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CONNECTION_RESOURCE {
    pub Connection_Header: CONNECTION_DES,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CONNECTION_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CR_ACCESS_DENIED: i32 = 51;
pub const CR_ALREADY_SUCH_DEVINST: i32 = 16;
pub const CR_ALREADY_SUCH_DEVNODE: i32 = 16;
pub const CR_APM_VETOED: i32 = 24;
pub const CR_BUFFER_SMALL: i32 = 26;
pub const CR_CALL_NOT_IMPLEMENTED: i32 = 52;
pub const CR_CANT_SHARE_IRQ: i32 = 43;
pub const CR_CREATE_BLOCKED: i32 = 21;
pub const CR_DEFAULT: i32 = 1;
pub const CR_DEVICE_INTERFACE_ACTIVE: i32 = 54;
pub const CR_DEVICE_NOT_THERE: i32 = 36;
pub const CR_DEVINST_HAS_REQS: i32 = 10;
pub const CR_DEVLOADER_NOT_READY: i32 = 33;
pub const CR_DEVNODE_HAS_REQS: i32 = 10;
pub const CR_DLVXD_NOT_FOUND: i32 = 12;
pub const CR_FAILURE: i32 = 19;
pub const CR_FREE_RESOURCES: i32 = 41;
pub const CR_INVALID_API: i32 = 32;
pub const CR_INVALID_ARBITRATOR: i32 = 8;
pub const CR_INVALID_CONFLICT_LIST: i32 = 57;
pub const CR_INVALID_DATA: i32 = 31;
pub const CR_INVALID_DEVICE_ID: i32 = 30;
pub const CR_INVALID_DEVINST: i32 = 5;
pub const CR_INVALID_DEVNODE: i32 = 5;
pub const CR_INVALID_FLAG: i32 = 4;
pub const CR_INVALID_INDEX: i32 = 58;
pub const CR_INVALID_LOAD_TYPE: i32 = 25;
pub const CR_INVALID_LOG_CONF: i32 = 7;
pub const CR_INVALID_MACHINENAME: i32 = 47;
pub const CR_INVALID_NODELIST: i32 = 9;
pub const CR_INVALID_POINTER: i32 = 3;
pub const CR_INVALID_PRIORITY: i32 = 39;
pub const CR_INVALID_PROPERTY: i32 = 53;
pub const CR_INVALID_RANGE: i32 = 18;
pub const CR_INVALID_RANGE_LIST: i32 = 17;
pub const CR_INVALID_REFERENCE_STRING: i32 = 56;
pub const CR_INVALID_RESOURCEID: i32 = 11;
pub const CR_INVALID_RES_DES: i32 = 6;
pub const CR_INVALID_STRUCTURE_SIZE: i32 = 59;
pub const CR_MACHINE_UNAVAILABLE: i32 = 49;
pub const CR_NEED_RESTART: i32 = 34;
pub const CR_NOT_DISABLEABLE: i32 = 40;
pub const CR_NOT_SYSTEM_VM: i32 = 22;
pub const CR_NO_ARBITRATOR: i32 = 27;
pub const CR_NO_CM_SERVICES: i32 = 50;
pub const CR_NO_DEPENDENT: i32 = 44;
pub const CR_NO_MORE_HW_PROFILES: i32 = 35;
pub const CR_NO_MORE_LOG_CONF: i32 = 14;
pub const CR_NO_MORE_RES_DES: i32 = 15;
pub const CR_NO_REGISTRY_HANDLE: i32 = 28;
pub const CR_NO_SUCH_DEVICE_INTERFACE: i32 = 55;
pub const CR_NO_SUCH_DEVINST: i32 = 13;
pub const CR_NO_SUCH_DEVNODE: i32 = 13;
pub const CR_NO_SUCH_LOGICAL_DEV: i32 = 20;
pub const CR_NO_SUCH_REGISTRY_KEY: i32 = 46;
pub const CR_NO_SUCH_VALUE: i32 = 37;
pub const CR_OUT_OF_MEMORY: i32 = 2;
pub const CR_QUERY_VETOED: i32 = 42;
pub const CR_REGISTRY_ERROR: i32 = 29;
pub const CR_REMOTE_COMM_FAILURE: i32 = 48;
pub const CR_REMOVE_VETOED: i32 = 23;
pub const CR_SAME_RESOURCES: i32 = 45;
pub const CR_SUCCESS: i32 = 0;
pub const CR_WRONG_TYPE: i32 = 38;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CS_DES {
    pub CSD_SignatureLength: super::DWORD,
    pub CSD_LegacyDataOffset: super::DWORD,
    pub CSD_LegacyDataSize: super::DWORD,
    pub CSD_Flags: super::DWORD,
    pub CSD_ClassGuid: windows_core::GUID,
    pub CSD_Signature: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for CS_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CS_RESOURCE {
    pub CS_Header: CS_DES,
}
pub type DEVINST = u32;
pub type DEVINSTID = *mut i8;
pub type DEVINSTID_A = *mut i8;
pub type DEVINSTID_W = *mut u16;
pub type DEVNODE = u32;
pub type DEVNODEID = *mut i8;
pub type DEVNODEID_A = *mut i8;
pub type DEVNODEID_W = *mut u16;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEVPRIVATE_DES {
    pub PD_Count: super::DWORD,
    pub PD_Type: super::DWORD,
    pub PD_Data1: super::DWORD,
    pub PD_Data2: super::DWORD,
    pub PD_Data3: super::DWORD,
    pub PD_Flags: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEVPRIVATE_RANGE {
    pub PR_Data1: super::DWORD,
    pub PR_Data2: super::DWORD,
    pub PR_Data3: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEVPRIVATE_RESOURCE {
    pub PRV_Header: DEVPRIVATE_DES,
    pub PRV_Data: [DEVPRIVATE_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for DEVPRIVATE_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DMA_DES {
    pub DD_Count: super::DWORD,
    pub DD_Type: super::DWORD,
    pub DD_Flags: super::DWORD,
    pub DD_Alloc_Chan: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DMA_RANGE {
    pub DR_Min: super::ULONG,
    pub DR_Max: super::ULONG,
    pub DR_Flags: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DMA_RESOURCE {
    pub DMA_Header: DMA_DES,
    pub DMA_Data: [DMA_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for DMA_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const DType_Range: u32 = 12;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DType_Range: u64 = 12;
pub const FILTERED_LOG_CONF: i32 = 1;
pub const FORCED_LOG_CONF: i32 = 4;
pub type HCMNOTIFICATION = *mut HCMNOTIFICATION__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HCMNOTIFICATION__ {
    pub unused: i32,
}
pub type HMACHINE = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type HWPROFILEINFO = HWPROFILEINFO_A;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HWPROFILEINFO_A {
    pub HWPI_ulHWProfile: super::ULONG,
    pub HWPI_szFriendlyName: [super::CHAR; 80],
    pub HWPI_dwFlags: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for HWPROFILEINFO_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct HWPROFILEINFO_W {
    pub HWPI_ulHWProfile: super::ULONG,
    pub HWPI_szFriendlyName: [super::WCHAR; 80],
    pub HWPI_dwFlags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HWPROFILEINFO_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IOA_Local: i32 = 255;
#[cfg(target_arch = "x86")]
pub const IOType_Range: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IOType_Range: u64 = 40;
pub const IO_ALIAS_10_BIT_DECODE: i32 = 4;
pub const IO_ALIAS_12_BIT_DECODE: i32 = 16;
pub const IO_ALIAS_16_BIT_DECODE: i32 = 0;
pub const IO_ALIAS_POSITIVE_DECODE: i32 = 255;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IO_DES {
    pub IOD_Count: super::DWORD,
    pub IOD_Type: super::DWORD,
    pub IOD_Alloc_Base: super::DWORDLONG,
    pub IOD_Alloc_End: super::DWORDLONG,
    pub IOD_DesFlags: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IO_RANGE {
    pub IOR_Align: super::DWORDLONG,
    pub IOR_nPorts: super::DWORD,
    pub IOR_Min: super::DWORDLONG,
    pub IOR_Max: super::DWORDLONG,
    pub IOR_RangeFlags: super::DWORD,
    pub IOR_Alias: super::DWORDLONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE {
    pub IO_Header: IO_DES,
    pub IO_Data: [IO_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for IO_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const IRQType_Range: u32 = 12;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IRQType_Range: u64 = 12;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type IRQ_DES = IRQ_DES_32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type IRQ_DES = IRQ_DES_64;
#[repr(C, packed(1))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct IRQ_DES_32 {
    pub IRQD_Count: super::DWORD,
    pub IRQD_Type: super::DWORD,
    pub IRQD_Flags: super::DWORD,
    pub IRQD_Alloc_Num: super::ULONG,
    pub IRQD_Affinity: super::ULONG32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IRQ_DES_64 {
    pub IRQD_Count: super::DWORD,
    pub IRQD_Type: super::DWORD,
    pub IRQD_Flags: super::DWORD,
    pub IRQD_Alloc_Num: super::ULONG,
    pub IRQD_Affinity: super::ULONG64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IRQ_RANGE {
    pub IRQR_Min: super::ULONG,
    pub IRQR_Max: super::ULONG,
    pub IRQR_Flags: super::ULONG,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type IRQ_RESOURCE = IRQ_RESOURCE_32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type IRQ_RESOURCE = IRQ_RESOURCE_64;
#[repr(C, packed(1))]
#[cfg(all(feature = "basetsd", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct IRQ_RESOURCE_32 {
    pub IRQ_Header: IRQ_DES_32,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
#[cfg(all(feature = "basetsd", feature = "shared"))]
impl Default for IRQ_RESOURCE_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IRQ_RESOURCE_64 {
    pub IRQ_Header: IRQ_DES_64,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for IRQ_RESOURCE_64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LOG_CONF = usize;
pub const LOG_CONF_BITS: i32 = 7;
pub const MAX_CLASS_NAME_LEN: i32 = 32;
pub const MAX_CONFIG_VALUE: i32 = 9999;
pub const MAX_DEVICE_ID_LEN: i32 = 200;
pub const MAX_DEVNODE_ID_LEN: i32 = 200;
pub const MAX_DMA_CHANNELS: i32 = 7;
pub const MAX_GUID_STRING_LEN: i32 = 39;
pub const MAX_INSTANCE_VALUE: i32 = 9999;
pub const MAX_IO_PORTS: i32 = 20;
pub const MAX_IRQS: i32 = 7;
pub const MAX_MEM_REGISTERS: i32 = 9;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MEM_DES {
    pub MD_Count: super::DWORD,
    pub MD_Type: super::DWORD,
    pub MD_Alloc_Base: super::DWORDLONG,
    pub MD_Alloc_End: super::DWORDLONG,
    pub MD_Flags: super::DWORD,
    pub MD_Reserved: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MEM_LARGE_DES {
    pub MLD_Count: super::DWORD,
    pub MLD_Type: super::DWORD,
    pub MLD_Alloc_Base: super::DWORDLONG,
    pub MLD_Alloc_End: super::DWORDLONG,
    pub MLD_Flags: super::DWORD,
    pub MLD_Reserved: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MEM_LARGE_RANGE {
    pub MLR_Align: super::DWORDLONG,
    pub MLR_nBytes: super::ULONGLONG,
    pub MLR_Min: super::DWORDLONG,
    pub MLR_Max: super::DWORDLONG,
    pub MLR_Flags: super::DWORD,
    pub MLR_Reserved: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MEM_LARGE_RESOURCE {
    pub MEM_LARGE_Header: MEM_LARGE_DES,
    pub MEM_LARGE_Data: [MEM_LARGE_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for MEM_LARGE_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MEM_RANGE {
    pub MR_Align: super::DWORDLONG,
    pub MR_nBytes: super::ULONG,
    pub MR_Min: super::DWORDLONG,
    pub MR_Max: super::DWORDLONG,
    pub MR_Flags: super::DWORD,
    pub MR_Reserved: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MEM_RESOURCE {
    pub MEM_Header: MEM_DES,
    pub MEM_Data: [MEM_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for MEM_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MFCARD_DES {
    pub PMF_Count: super::DWORD,
    pub PMF_Type: super::DWORD,
    pub PMF_Flags: super::DWORD,
    pub PMF_ConfigOptions: super::BYTE,
    pub PMF_IoResourceIndex: super::BYTE,
    pub PMF_Reserved: [super::BYTE; 2],
    pub PMF_ConfigRegisterBase: super::DWORD,
}
#[cfg(feature = "shared")]
impl Default for MFCARD_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MFCARD_RESOURCE {
    pub MfCard_Header: MFCARD_DES,
}
#[cfg(target_arch = "x86")]
pub const MLType_Range: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MLType_Range: u64 = 40;
#[cfg(target_arch = "x86")]
pub const MType_Range: u32 = 36;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MType_Range: u64 = 36;
pub const NUM_CR_RESULTS: i32 = 60;
pub const NUM_LOG_CONF: i32 = 6;
pub const OVERRIDE_LOG_CONF: i32 = 5;
#[cfg(feature = "shared")]
pub type PBUSNUMBER_DES = *mut BUSNUMBER_DES;
#[cfg(feature = "shared")]
pub type PBUSNUMBER_RANGE = *mut BUSNUMBER_RANGE;
#[cfg(feature = "shared")]
pub type PBUSNUMBER_RESOURCE = *mut BUSNUMBER_RESOURCE;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCCARD_DES {
    pub PCD_Count: super::DWORD,
    pub PCD_Type: super::DWORD,
    pub PCD_Flags: super::DWORD,
    pub PCD_ConfigIndex: super::BYTE,
    pub PCD_Reserved: [super::BYTE; 3],
    pub PCD_MemoryCardBase1: super::DWORD,
    pub PCD_MemoryCardBase2: super::DWORD,
    pub PCD_MemoryCardBase: [super::DWORD; 2],
    pub PCD_MemoryFlags: [super::WORD; 2],
    pub PCD_IoFlags: [super::BYTE; 2],
}
#[cfg(feature = "shared")]
impl Default for PCCARD_DES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCCARD_RESOURCE {
    pub PcCard_Header: PCCARD_DES,
}
pub const PCD_MAX_IO: i32 = 2;
pub const PCD_MAX_MEMORY: i32 = 2;
pub type PCM_NOTIFY_ACTION = *mut CM_NOTIFY_ACTION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCM_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(hnotify: HCMNOTIFICATION, context: super::PVOID, action: CM_NOTIFY_ACTION, eventdata: PCM_NOTIFY_EVENT_DATA, eventdatasize: super::DWORD) -> super::DWORD>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCM_NOTIFY_EVENT_DATA = *mut CM_NOTIFY_EVENT_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCM_NOTIFY_FILTER = *mut CM_NOTIFY_FILTER;
pub type PCM_NOTIFY_FILTER_TYPE = *mut CM_NOTIFY_FILTER_TYPE;
#[cfg(feature = "shared")]
pub type PCONFLICT_DETAILS = *mut CONFLICT_DETAILS_A;
#[cfg(feature = "shared")]
pub type PCONFLICT_DETAILS_A = *mut CONFLICT_DETAILS_A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCONFLICT_DETAILS_W = *mut CONFLICT_DETAILS_W;
pub type PCONFLICT_LIST = *mut CONFLICT_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCONNECTION_DES = *mut CONNECTION_DES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCONNECTION_RESOURCE = *mut CONNECTION_RESOURCE;
#[cfg(feature = "shared")]
pub type PCS_DES = *mut CS_DES;
#[cfg(feature = "shared")]
pub type PCS_RESOURCE = *mut CS_RESOURCE;
pub type PCVOID = *const core::ffi::c_void;
pub type PDEVINST = *mut u32;
pub type PDEVNODE = *mut u32;
#[cfg(feature = "shared")]
pub type PDEVPRIVATE_DES = *mut DEVPRIVATE_DES;
#[cfg(feature = "shared")]
pub type PDEVPRIVATE_RANGE = *mut DEVPRIVATE_RANGE;
#[cfg(feature = "shared")]
pub type PDEVPRIVATE_RESOURCE = *mut DEVPRIVATE_RESOURCE;
#[cfg(feature = "shared")]
pub type PDMA_DES = *mut DMA_DES;
#[cfg(feature = "shared")]
pub type PDMA_RANGE = *mut DMA_RANGE;
#[cfg(feature = "shared")]
pub type PDMA_RESOURCE = *mut DMA_RESOURCE;
pub type PHCMNOTIFICATION = *mut *mut HCMNOTIFICATION__;
pub type PHMACHINE = *mut *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type PHWPROFILEINFO = *mut HWPROFILEINFO_A;
#[cfg(feature = "shared")]
pub type PHWPROFILEINFO_A = *mut HWPROFILEINFO_A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHWPROFILEINFO_W = *mut HWPROFILEINFO_W;
#[cfg(feature = "shared")]
pub type PIO_DES = *mut IO_DES;
#[cfg(feature = "shared")]
pub type PIO_RANGE = *mut IO_RANGE;
#[cfg(feature = "shared")]
pub type PIO_RESOURCE = *mut IO_RESOURCE;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PIRQ_DES = *mut IRQ_DES_32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PIRQ_DES = *mut IRQ_DES_64;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PIRQ_DES_32 = *mut IRQ_DES_32;
#[cfg(feature = "shared")]
pub type PIRQ_DES_64 = *mut IRQ_DES_64;
#[cfg(feature = "shared")]
pub type PIRQ_RANGE = *mut IRQ_RANGE;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PIRQ_RESOURCE = *mut IRQ_RESOURCE_32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub type PIRQ_RESOURCE = *mut IRQ_RESOURCE_64;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PIRQ_RESOURCE_32 = *mut IRQ_RESOURCE_32;
#[cfg(feature = "shared")]
pub type PIRQ_RESOURCE_64 = *mut IRQ_RESOURCE_64;
pub type PLOG_CONF = *mut LOG_CONF;
#[cfg(feature = "shared")]
pub type PMEM_DES = *mut MEM_DES;
#[cfg(feature = "shared")]
pub type PMEM_LARGE_DES = *mut MEM_LARGE_DES;
#[cfg(feature = "shared")]
pub type PMEM_LARGE_RANGE = *mut MEM_LARGE_RANGE;
#[cfg(feature = "shared")]
pub type PMEM_LARGE_RESOURCE = *mut MEM_LARGE_RESOURCE;
#[cfg(feature = "shared")]
pub type PMEM_RANGE = *mut MEM_RANGE;
#[cfg(feature = "shared")]
pub type PMEM_RESOURCE = *mut MEM_RESOURCE;
#[cfg(feature = "shared")]
pub type PMFCARD_DES = *mut MFCARD_DES;
#[cfg(feature = "shared")]
pub type PMFCARD_RESOURCE = *mut MFCARD_RESOURCE;
#[cfg(feature = "shared")]
pub type PPCCARD_DES = *mut PCCARD_DES;
#[cfg(feature = "shared")]
pub type PPCCARD_RESOURCE = *mut PCCARD_RESOURCE;
pub type PPRIORITY = *mut u32;
pub type PRANGE_ELEMENT = *mut RANGE_ELEMENT;
pub type PRANGE_LIST = *mut RANGE_LIST;
pub type PRESOURCEID = *mut u32;
pub type PRES_DES = *mut RES_DES;
pub type PRIORITY = u32;
pub const PRIORITY_BIT: i32 = 8;
pub const PRIORITY_EQUAL_FIRST: i32 = 8;
pub const PRIORITY_EQUAL_LAST: i32 = 0;
#[cfg(target_arch = "x86")]
pub const PType_Range: u32 = 12;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PType_Range: u64 = 12;
pub type RANGE_ELEMENT = usize;
pub type RANGE_LIST = usize;
pub type REGDISPOSITION = u32;
pub type RESOURCEID = u32;
pub type RES_DES = usize;
pub type RETURN_TYPE = u32;
pub const RegDisposition_Bits: i32 = 1;
pub const RegDisposition_OpenAlways: i32 = 0;
pub const RegDisposition_OpenExisting: i32 = 1;
pub const ResType_All: i32 = 0;
pub const ResType_BusNumber: i32 = 6;
pub const ResType_ClassSpecific: i32 = 65535;
pub const ResType_Connection: i32 = 32772;
pub const ResType_DMA: i32 = 3;
pub const ResType_DevicePrivate: i32 = 32769;
pub const ResType_DoNotUse: i32 = 5;
pub const ResType_IO: i32 = 2;
pub const ResType_IRQ: i32 = 4;
pub const ResType_Ignored_Bit: i32 = 32768;
pub const ResType_MAX: i32 = 7;
pub const ResType_Mem: i32 = 1;
pub const ResType_MemLarge: i32 = 7;
pub const ResType_MfCardConfig: i32 = 32771;
pub const ResType_None: i32 = 0;
pub const ResType_PcCardConfig: i32 = 32770;
pub const ResType_Reserved: i32 = 32768;
pub const fDD_BYTE: i32 = 0;
pub const fDD_BYTE_AND_WORD: i32 = 3;
pub const fDD_BusMaster: i32 = 4;
pub const fDD_DWORD: i32 = 2;
pub const fDD_NoBusMaster: i32 = 0;
pub const fDD_TypeA: i32 = 8;
pub const fDD_TypeB: i32 = 16;
pub const fDD_TypeF: i32 = 24;
pub const fDD_TypeStandard: i32 = 0;
pub const fDD_WORD: i32 = 1;
pub const fIOD_10_BIT_DECODE: i32 = 4;
pub const fIOD_12_BIT_DECODE: i32 = 8;
pub const fIOD_16_BIT_DECODE: i32 = 16;
pub const fIOD_DECODE: i32 = 252;
pub const fIOD_IO: i32 = 1;
pub const fIOD_Memory: i32 = 0;
pub const fIOD_PASSIVE_DECODE: i32 = 64;
pub const fIOD_PORT_BAR: i32 = 256;
pub const fIOD_POSITIVE_DECODE: i32 = 32;
pub const fIOD_PortType: i32 = 1;
pub const fIOD_WINDOW_DECODE: i32 = 128;
pub const fIRQD_Edge: i32 = 2;
pub const fIRQD_Exclusive: i32 = 0;
pub const fIRQD_Level: i32 = 0;
pub const fIRQD_Level_Bit: i32 = 1;
pub const fIRQD_Share: i32 = 1;
pub const fIRQD_Share_Bit: i32 = 0;
pub const fMD_24: i32 = 0;
pub const fMD_32: i32 = 2;
pub const fMD_32_24: i32 = 2;
pub const fMD_Cacheable: i32 = 32;
pub const fMD_CombinedWrite: i32 = 16;
pub const fMD_CombinedWriteAllowed: i32 = 16;
pub const fMD_CombinedWriteDisallowed: i32 = 0;
pub const fMD_MEMORY_BAR: i32 = 128;
pub const fMD_MemoryType: i32 = 1;
pub const fMD_NonCacheable: i32 = 0;
pub const fMD_Pref: i32 = 4;
pub const fMD_PrefetchAllowed: i32 = 4;
pub const fMD_PrefetchDisallowed: i32 = 0;
pub const fMD_Prefetchable: i32 = 4;
pub const fMD_RAM: i32 = 1;
pub const fMD_ROM: i32 = 0;
pub const fMD_ReadAllowed: i32 = 0;
pub const fMD_ReadDisallowed: i32 = 8;
pub const fMD_Readable: i32 = 8;
pub const fMD_WINDOW_DECODE: i32 = 64;
pub const fPCD_ATTRIBUTES_PER_WINDOW: i32 = 32768;
pub const fPCD_IO1_16: i32 = 65536;
pub const fPCD_IO1_SRC_16: i32 = 262144;
pub const fPCD_IO1_WS_16: i32 = 524288;
pub const fPCD_IO1_ZW_8: i32 = 131072;
pub const fPCD_IO2_16: i32 = 1048576;
pub const fPCD_IO2_SRC_16: i32 = 4194304;
pub const fPCD_IO2_WS_16: i32 = 8388608;
pub const fPCD_IO2_ZW_8: i32 = 2097152;
pub const fPCD_IO_16: i32 = 1;
pub const fPCD_IO_8: i32 = 0;
pub const fPCD_IO_SRC_16: i32 = 32;
pub const fPCD_IO_WS_16: i32 = 64;
pub const fPCD_IO_ZW_8: i32 = 16;
pub const fPCD_MEM1_16: i32 = 67108864;
pub const fPCD_MEM1_A: i32 = 4;
pub const fPCD_MEM1_WS_ONE: i32 = 16777216;
pub const fPCD_MEM1_WS_THREE: i32 = 50331648;
pub const fPCD_MEM1_WS_TWO: i32 = 33554432;
pub const fPCD_MEM2_16: i32 = 1073741824;
pub const fPCD_MEM2_A: i32 = 8;
pub const fPCD_MEM2_WS_ONE: i32 = 268435456;
pub const fPCD_MEM2_WS_THREE: i32 = 805306368;
pub const fPCD_MEM2_WS_TWO: i32 = 536870912;
pub const fPCD_MEM_16: i32 = 2;
pub const fPCD_MEM_8: i32 = 0;
pub const fPCD_MEM_A: i32 = 4;
pub const fPCD_MEM_WS_ONE: i32 = 256;
pub const fPCD_MEM_WS_THREE: i32 = 768;
pub const fPCD_MEM_WS_TWO: i32 = 512;
pub const fPMF_AUDIO_ENABLE: i32 = 8;
pub const mDD_BusMaster: i32 = 4;
pub const mDD_Type: i32 = 24;
pub const mDD_Width: i32 = 3;
pub const mIRQD_Edge_Level: i32 = 2;
pub const mIRQD_Share: i32 = 1;
pub const mMD_32_24: i32 = 2;
pub const mMD_Cacheable: i32 = 32;
pub const mMD_CombinedWrite: i32 = 16;
pub const mMD_MemoryType: i32 = 1;
pub const mMD_Prefetchable: i32 = 4;
pub const mMD_Readable: i32 = 8;
pub const mPCD_IO_8_16: i32 = 1;
pub const mPCD_MEM1_WS: i32 = 50331648;
pub const mPCD_MEM2_WS: i32 = 805306368;
pub const mPCD_MEM_8_16: i32 = 2;
pub const mPCD_MEM_A_C: i32 = 12;
pub const mPCD_MEM_WS: i32 = 768;
pub const mPMF_AUDIO_ENABLE: i32 = 8;
