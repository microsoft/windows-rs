windows_link::link!("setupapi.dll" "system" fn CM_Add_Empty_Log_Conf(plclogconf : *mut LOG_CONF, dndevinst : DEVINST, priority : PRIORITY, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Add_Empty_Log_Conf_Ex(plclogconf : *mut LOG_CONF, dndevinst : DEVINST, priority : PRIORITY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Add_IDA(dndevinst : DEVINST, pszid : windows_sys::core::PCSTR, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Add_IDW(dndevinst : DEVINST, pszid : windows_sys::core::PCWSTR, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Add_ID_ExA(dndevinst : DEVINST, pszid : windows_sys::core::PCSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Add_ID_ExW(dndevinst : DEVINST, pszid : windows_sys::core::PCWSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Add_Range(ullstartvalue : super::DWORDLONG, ullendvalue : super::DWORDLONG, rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Add_Res_Des(prdresdes : *mut RES_DES, lclogconf : LOG_CONF, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Add_Res_Des_Ex(prdresdes : *mut RES_DES, lclogconf : LOG_CONF, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Connect_MachineA(uncservername : windows_sys::core::PCSTR, phmachine : *mut HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Connect_MachineW(uncservername : windows_sys::core::PCWSTR, phmachine : *mut HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Create_DevNodeA(pdndevinst : *mut DEVNODE, pdeviceid : *const i8, dnparent : DEVINST, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Create_DevNodeW(pdndevinst : *mut DEVNODE, pdeviceid : *const u16, dnparent : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Create_DevNode_ExA(pdndevinst : *mut DEVNODE, pdeviceid : *const i8, dnparent : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Create_DevNode_ExW(pdndevinst : *mut DEVNODE, pdeviceid : *const u16, dnparent : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Create_Range_List(prlh : *mut RANGE_LIST, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Delete_Class_Key(classguid : *const windows_sys::core::GUID, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Delete_Class_Key_Ex(classguid : *const windows_sys::core::GUID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Delete_DevNode_Key(dndevnode : DEVNODE, ulhardwareprofile : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Delete_DevNode_Key_Ex(dndevnode : DEVNODE, ulhardwareprofile : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_KeyA(pszdeviceinterface : windows_sys::core::PCSTR, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_KeyW(pszdeviceinterface : windows_sys::core::PCWSTR, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface : windows_sys::core::PCSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface : windows_sys::core::PCWSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Delete_Range(ullstartvalue : super::DWORDLONG, ullendvalue : super::DWORDLONG, rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Detect_Resource_Conflict(dndevinst : DEVINST, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, pbconflictdetected : *mut windows_sys::core::BOOL, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Detect_Resource_Conflict_Ex(dndevinst : DEVINST, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, pbconflictdetected : *mut windows_sys::core::BOOL, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Disable_DevNode(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Disable_DevNode_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Disconnect_Machine(hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Dup_Range_List(rlhold : RANGE_LIST, rlhnew : RANGE_LIST, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Enable_DevNode(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Enable_DevNode_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Enumerate_Classes(ulclassindex : u32, classguid : *mut windows_sys::core::GUID, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Enumerate_Classes_Ex(ulclassindex : u32, classguid : *mut windows_sys::core::GUID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Enumerate_EnumeratorsA(ulenumindex : u32, buffer : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Enumerate_EnumeratorsW(ulenumindex : u32, buffer : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Enumerate_Enumerators_ExA(ulenumindex : u32, buffer : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Enumerate_Enumerators_ExW(ulenumindex : u32, buffer : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Find_Range(pullstart : *mut super::DWORDLONG, ullstart : super::DWORDLONG, ullength : u32, ullalignment : super::DWORDLONG, ullend : super::DWORDLONG, rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_First_Range(rlh : RANGE_LIST, pullstart : *mut super::DWORDLONG, pullend : *mut super::DWORDLONG, preelement : *mut RANGE_ELEMENT, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Free_Log_Conf(lclogconftobefreed : LOG_CONF, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Free_Log_Conf_Ex(lclogconftobefreed : LOG_CONF, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Free_Log_Conf_Handle(lclogconf : LOG_CONF) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Free_Range_List(rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Free_Res_Des(prdresdes : *mut RES_DES, rdresdes : RES_DES, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Free_Res_Des_Ex(prdresdes : *mut RES_DES, rdresdes : RES_DES, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Free_Res_Des_Handle(rdresdes : RES_DES) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Free_Resource_Conflict_Handle(clconflictlist : CONFLICT_LIST) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Child(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Child_Ex(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_Key_NameA(classguid : *const windows_sys::core::GUID, pszkeyname : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_Key_NameW(classguid : *const windows_sys::core::GUID, pszkeyname : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_Key_Name_ExA(classguid : *const windows_sys::core::GUID, pszkeyname : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_Key_Name_ExW(classguid : *const windows_sys::core::GUID, pszkeyname : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_NameA(classguid : *const windows_sys::core::GUID, buffer : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_NameW(classguid : *const windows_sys::core::GUID, buffer : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_Name_ExA(classguid : *const windows_sys::core::GUID, buffer : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_Name_ExW(classguid : *const windows_sys::core::GUID, buffer : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "devpropdef")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_Class_PropertyW(classguid : *const windows_sys::core::GUID, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_ExW(classguid : *const windows_sys::core::GUID, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "devpropdef")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_Keys(classguid : *const windows_sys::core::GUID, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_Class_Property_Keys_Ex(classguid : *const windows_sys::core::GUID, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_Registry_PropertyA(classguid : *const windows_sys::core::GUID, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Class_Registry_PropertyW(classguid : *const windows_sys::core::GUID, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Depth(puldepth : *mut u32, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Depth_Ex(puldepth : *mut u32, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_PropertyA(dndevinst : DEVINST, pszcustompropertyname : windows_sys::core::PCSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_PropertyW(dndevinst : DEVINST, pszcustompropertyname : windows_sys::core::PCWSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_Property_ExA(dndevinst : DEVINST, pszcustompropertyname : windows_sys::core::PCSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Custom_Property_ExW(dndevinst : DEVINST, pszcustompropertyname : windows_sys::core::PCWSTR, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "devpropdef")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_PropertyW(dndevinst : DEVINST, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_ExW(dndevinst : DEVINST, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "devpropdef")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_Keys(dndevinst : DEVINST, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_DevNode_Property_Keys_Ex(dndevinst : DEVINST, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_PropertyA(dndevinst : DEVINST, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_PropertyW(dndevinst : DEVINST, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_Property_ExA(dndevinst : DEVINST, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Registry_Property_ExW(dndevinst : DEVINST, ulproperty : u32, pulregdatatype : *mut u32, buffer : *mut core::ffi::c_void, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Status(pulstatus : *mut u32, pulproblemnumber : *mut u32, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_DevNode_Status_Ex(pulstatus : *mut u32, pulproblemnumber : *mut u32, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_IDA(dndevinst : DEVINST, buffer : windows_sys::core::PSTR, bufferlen : u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_IDW(dndevinst : DEVINST, buffer : windows_sys::core::PWSTR, bufferlen : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ExA(dndevinst : DEVINST, buffer : windows_sys::core::PSTR, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ExW(dndevinst : DEVINST, buffer : windows_sys::core::PWSTR, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ListA(pszfilter : windows_sys::core::PCSTR, buffer : *mut i8, bufferlen : u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_ListW(pszfilter : windows_sys::core::PCWSTR, buffer : *mut u16, bufferlen : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_ExA(pszfilter : windows_sys::core::PCSTR, buffer : *mut i8, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_ExW(pszfilter : windows_sys::core::PCWSTR, buffer : *mut u16, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_SizeA(pullen : *mut u32, pszfilter : windows_sys::core::PCSTR, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_SizeW(pullen : *mut u32, pszfilter : windows_sys::core::PCWSTR, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_Size_ExA(pullen : *mut u32, pszfilter : windows_sys::core::PCSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_List_Size_ExW(pullen : *mut u32, pszfilter : windows_sys::core::PCWSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_Size(pullen : *mut u32, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_ID_Size_Ex(pullen : *mut u32, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_AliasA(pszdeviceinterface : windows_sys::core::PCSTR, aliasinterfaceguid : *const windows_sys::core::GUID, pszaliasdeviceinterface : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_AliasW(pszdeviceinterface : windows_sys::core::PCWSTR, aliasinterfaceguid : *const windows_sys::core::GUID, pszaliasdeviceinterface : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface : windows_sys::core::PCSTR, aliasinterfaceguid : *const windows_sys::core::GUID, pszaliasdeviceinterface : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface : windows_sys::core::PCWSTR, aliasinterfaceguid : *const windows_sys::core::GUID, pszaliasdeviceinterface : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_ListA(interfaceclassguid : *const windows_sys::core::GUID, pdeviceid : *const i8, buffer : *mut i8, bufferlen : u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_ListW(interfaceclassguid : *const windows_sys::core::GUID, pdeviceid : *const u16, buffer : *mut u16, bufferlen : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_ExA(interfaceclassguid : *const windows_sys::core::GUID, pdeviceid : *const i8, buffer : *mut i8, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_ExW(interfaceclassguid : *const windows_sys::core::GUID, pdeviceid : *const u16, buffer : *mut u16, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_SizeA(pullen : *mut u32, interfaceclassguid : *const windows_sys::core::GUID, pdeviceid : *const i8, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_SizeW(pullen : *mut u32, interfaceclassguid : *const windows_sys::core::GUID, pdeviceid : *const u16, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_Size_ExA(pullen : *mut u32, interfaceclassguid : *const windows_sys::core::GUID, pdeviceid : *const i8, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Device_Interface_List_Size_ExW(pullen : *mut u32, interfaceclassguid : *const windows_sys::core::GUID, pdeviceid : *const u16, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "devpropdef")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_PropertyW(pszdeviceinterface : windows_sys::core::PCWSTR, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_ExW(pszdeviceinterface : windows_sys::core::PCWSTR, propertykey : *const super::DEVPROPKEY, propertytype : *mut super::DEVPROPTYPE, propertybuffer : *mut u8, propertybuffersize : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "devpropdef")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface : windows_sys::core::PCWSTR, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface : windows_sys::core::PCWSTR, propertykeyarray : *mut super::DEVPROPKEY, propertykeycount : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_First_Log_Conf(plclogconf : *mut LOG_CONF, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_First_Log_Conf_Ex(plclogconf : *mut LOG_CONF, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Global_State(pulstate : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Global_State_Ex(pulstate : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_FlagsA(pdeviceid : *const i8, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_FlagsW(pdeviceid : *const u16, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_Flags_ExA(pdeviceid : *const i8, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_HW_Prof_Flags_ExW(pdeviceid : *const u16, ulhardwareprofile : u32, pulvalue : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_InfoA(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_A, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_InfoW(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_W, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_Info_ExA(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_A, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Hardware_Profile_Info_ExW(ulindex : u32, phwprofileinfo : *mut HWPROFILEINFO_W, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Log_Conf_Priority(lclogconf : LOG_CONF, ppriority : *mut PRIORITY, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Log_Conf_Priority_Ex(lclogconf : LOG_CONF, ppriority : *mut PRIORITY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Next_Log_Conf(plclogconf : *mut LOG_CONF, lclogconf : LOG_CONF, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Next_Log_Conf_Ex(plclogconf : *mut LOG_CONF, lclogconf : LOG_CONF, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Next_Res_Des(prdresdes : *mut RES_DES, rdresdes : RES_DES, forresource : RESOURCEID, presourceid : *mut RESOURCEID, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Next_Res_Des_Ex(prdresdes : *mut RES_DES, rdresdes : RES_DES, forresource : RESOURCEID, presourceid : *mut RESOURCEID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Parent(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Parent_Ex(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data(rdresdes : RES_DES, buffer : *mut core::ffi::c_void, bufferlen : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data_Ex(rdresdes : RES_DES, buffer : *mut core::ffi::c_void, bufferlen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data_Size(pulsize : *mut u32, rdresdes : RES_DES, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Res_Des_Data_Size_Ex(pulsize : *mut u32, rdresdes : RES_DES, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Resource_Conflict_Count(clconflictlist : CONFLICT_LIST, pulcount : *mut u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Resource_Conflict_DetailsA(clconflictlist : CONFLICT_LIST, ulindex : u32, pconflictdetails : *mut CONFLICT_DETAILS_A) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Resource_Conflict_DetailsW(clconflictlist : CONFLICT_LIST, ulindex : u32, pconflictdetails : *mut CONFLICT_DETAILS_W) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Sibling(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Sibling_Ex(pdndevinst : *mut DEVNODE, dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Get_Version() -> u16);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Get_Version_Ex(hmachine : HMACHINE) -> u16);
windows_link::link!("setupapi.dll" "system" fn CM_Intersect_Range_List(rlhold1 : RANGE_LIST, rlhold2 : RANGE_LIST, rlhnew : RANGE_LIST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Invert_Range_List(rlhold : RANGE_LIST, rlhnew : RANGE_LIST, ullmaxvalue : super::DWORDLONG, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Is_Dock_Station_Present(pbpresent : *mut windows_sys::core::BOOL) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Is_Dock_Station_Present_Ex(pbpresent : *mut windows_sys::core::BOOL, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Is_Version_Available(wversion : u16) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Is_Version_Available_Ex(wversion : u16, hmachine : HMACHINE) -> windows_sys::core::BOOL);
windows_link::link!("setupapi.dll" "system" fn CM_Locate_DevNodeA(pdndevinst : *mut DEVNODE, pdeviceid : *const i8, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Locate_DevNodeW(pdndevinst : *mut DEVNODE, pdeviceid : *const u16, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Locate_DevNode_ExA(pdndevinst : *mut DEVNODE, pdeviceid : *const i8, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Locate_DevNode_ExW(pdndevinst : *mut DEVNODE, pdeviceid : *const u16, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("cfgmgr32.dll" "system" fn CM_MapCrToWin32Err(cmreturncode : CONFIGRET, defaulterr : u32) -> u32);
windows_link::link!("setupapi.dll" "system" fn CM_Merge_Range_List(rlhold1 : RANGE_LIST, rlhold2 : RANGE_LIST, rlhnew : RANGE_LIST, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Modify_Res_Des(prdresdes : *mut RES_DES, rdresdes : RES_DES, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Modify_Res_Des_Ex(prdresdes : *mut RES_DES, rdresdes : RES_DES, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Move_DevNode(dnfromdevinst : DEVINST, dntodevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Move_DevNode_Ex(dnfromdevinst : DEVINST, dntodevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Next_Range(preelement : *mut RANGE_ELEMENT, pullstart : *mut super::DWORDLONG, pullend : *mut super::DWORDLONG, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_Class_KeyA(classguid : *const windows_sys::core::GUID, pszclassname : windows_sys::core::PCSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkclass : *mut super::HKEY, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_Class_KeyW(classguid : *const windows_sys::core::GUID, pszclassname : windows_sys::core::PCWSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkclass : *mut super::HKEY, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_Class_Key_ExA(classguid : *const windows_sys::core::GUID, pszclassname : windows_sys::core::PCSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkclass : *mut super::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_Class_Key_ExW(classguid : *const windows_sys::core::GUID, pszclassname : windows_sys::core::PCWSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkclass : *mut super::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_DevNode_Key(dndevnode : DEVINST, samdesired : super::REGSAM, ulhardwareprofile : u32, disposition : REGDISPOSITION, phkdevice : *mut super::HKEY, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_DevNode_Key_Ex(dndevnode : DEVINST, samdesired : super::REGSAM, ulhardwareprofile : u32, disposition : REGDISPOSITION, phkdevice : *mut super::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_KeyA(pszdeviceinterface : windows_sys::core::PCSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : *mut super::HKEY, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_KeyW(pszdeviceinterface : windows_sys::core::PCWSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : *mut super::HKEY, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_Key_ExA(pszdeviceinterface : windows_sys::core::PCSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : *mut super::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("setupapi.dll" "system" fn CM_Open_Device_Interface_Key_ExW(pszdeviceinterface : windows_sys::core::PCWSTR, samdesired : super::REGSAM, disposition : REGDISPOSITION, phkdeviceinterface : *mut super::HKEY, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "cfg")]
windows_link::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTreeA(dnancestor : DEVINST, pvetotype : *mut super::PNP_VETO_TYPE, pszvetoname : windows_sys::core::PSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "cfg")]
windows_link::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTreeW(dnancestor : DEVINST, pvetotype : *mut super::PNP_VETO_TYPE, pszvetoname : windows_sys::core::PWSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "cfg", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTree_ExA(dnancestor : DEVINST, pvetotype : *mut super::PNP_VETO_TYPE, pszvetoname : windows_sys::core::PSTR, ulnamelength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(all(feature = "cfg", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn CM_Query_And_Remove_SubTree_ExW(dnancestor : DEVINST, pvetotype : *mut super::PNP_VETO_TYPE, pszvetoname : windows_sys::core::PWSTR, ulnamelength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Data(pdata : *mut core::ffi::c_void, datalen : u32, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Data_Ex(pdata : *mut core::ffi::c_void, datalen : u32, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Size(pulsize : *mut u32, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Query_Arbitrator_Free_Size_Ex(pulsize : *mut u32, dndevinst : DEVINST, resourceid : RESOURCEID, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Query_Remove_SubTree(dnancestor : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Query_Remove_SubTree_Ex(dnancestor : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Query_Resource_Conflict_List(pclconflictlist : *mut CONFLICT_LIST, dndevinst : DEVINST, resourceid : RESOURCEID, resourcedata : *const core::ffi::c_void, resourcelen : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Reenumerate_DevNode(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Reenumerate_DevNode_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Register_Device_Driver(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Register_Device_Driver_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Register_Device_InterfaceA(dndevinst : DEVINST, interfaceclassguid : *const windows_sys::core::GUID, pszreference : windows_sys::core::PCSTR, pszdeviceinterface : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Register_Device_InterfaceW(dndevinst : DEVINST, interfaceclassguid : *const windows_sys::core::GUID, pszreference : windows_sys::core::PCWSTR, pszdeviceinterface : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Register_Device_Interface_ExA(dndevinst : DEVINST, interfaceclassguid : *const windows_sys::core::GUID, pszreference : windows_sys::core::PCSTR, pszdeviceinterface : windows_sys::core::PSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Register_Device_Interface_ExW(dndevinst : DEVINST, interfaceclassguid : *const windows_sys::core::GUID, pszreference : windows_sys::core::PCWSTR, pszdeviceinterface : windows_sys::core::PWSTR, pullength : *mut u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Register_Notification(pfilter : *const CM_NOTIFY_FILTER, pcontext : *const core::ffi::c_void, pcallback : PCM_NOTIFY_CALLBACK, pnotifycontext : *mut HCMNOTIFICATION) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Remove_SubTree(dnancestor : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Remove_SubTree_Ex(dnancestor : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "cfg")]
windows_link::link!("setupapi.dll" "system" fn CM_Request_Device_EjectA(dndevinst : DEVINST, pvetotype : *mut super::PNP_VETO_TYPE, pszvetoname : windows_sys::core::PSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "cfg")]
windows_link::link!("setupapi.dll" "system" fn CM_Request_Device_EjectW(dndevinst : DEVINST, pvetotype : *mut super::PNP_VETO_TYPE, pszvetoname : windows_sys::core::PWSTR, ulnamelength : u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "cfg", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn CM_Request_Device_Eject_ExA(dndevinst : DEVINST, pvetotype : *mut super::PNP_VETO_TYPE, pszvetoname : windows_sys::core::PSTR, ulnamelength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(all(feature = "cfg", feature = "winnt"))]
windows_link::link!("setupapi.dll" "system" fn CM_Request_Device_Eject_ExW(dndevinst : DEVINST, pvetotype : *mut super::PNP_VETO_TYPE, pszvetoname : windows_sys::core::PWSTR, ulnamelength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Request_Eject_PC() -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Request_Eject_PC_Ex(hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Run_Detection(ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Run_Detection_Ex(ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "devpropdef")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Set_Class_PropertyW(classguid : *const windows_sys::core::GUID, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Set_Class_Property_ExW(classguid : *const windows_sys::core::GUID, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Set_Class_Registry_PropertyA(classguid : *const windows_sys::core::GUID, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Set_Class_Registry_PropertyW(classguid : *const windows_sys::core::GUID, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Set_DevNode_Problem(dndevinst : DEVINST, ulproblem : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Set_DevNode_Problem_Ex(dndevinst : DEVINST, ulproblem : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "devpropdef")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_PropertyW(dndevinst : DEVINST, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Set_DevNode_Property_ExW(dndevinst : DEVINST, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_PropertyA(dndevinst : DEVINST, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_PropertyW(dndevinst : DEVINST, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_Property_ExA(dndevinst : DEVINST, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Set_DevNode_Registry_Property_ExW(dndevinst : DEVINST, ulproperty : u32, buffer : *const core::ffi::c_void, ullength : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "devpropdef")]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Set_Device_Interface_PropertyW(pszdeviceinterface : windows_sys::core::PCWSTR, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32) -> CONFIGRET);
#[cfg(all(feature = "devpropdef", feature = "winnt"))]
windows_link::link!("cfgmgr32.dll" "system" fn CM_Set_Device_Interface_Property_ExW(pszdeviceinterface : windows_sys::core::PCWSTR, propertykey : *const super::DEVPROPKEY, propertytype : super::DEVPROPTYPE, propertybuffer : *const u8, propertybuffersize : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Set_HW_Prof(ulhardwareprofile : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_Ex(ulhardwareprofile : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_FlagsA(pdeviceid : *const i8, ulconfig : u32, ulvalue : u32, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_FlagsW(pdeviceid : *const u16, ulconfig : u32, ulvalue : u32, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_Flags_ExA(pdeviceid : *const i8, ulconfig : u32, ulvalue : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Set_HW_Prof_Flags_ExW(pdeviceid : *const u16, ulconfig : u32, ulvalue : u32, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Setup_DevNode(dndevinst : DEVINST, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Setup_DevNode_Ex(dndevinst : DEVINST, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Test_Range_Available(ullstartvalue : super::DWORDLONG, ullendvalue : super::DWORDLONG, rlh : RANGE_LIST, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Uninstall_DevNode(dndevinst : DEVNODE, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Uninstall_DevNode_Ex(dndevinst : DEVNODE, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Unregister_Device_InterfaceA(pszdeviceinterface : windows_sys::core::PCSTR, ulflags : u32) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" fn CM_Unregister_Device_InterfaceW(pszdeviceinterface : windows_sys::core::PCWSTR, ulflags : u32) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Unregister_Device_Interface_ExA(pszdeviceinterface : windows_sys::core::PCSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
#[cfg(feature = "winnt")]
windows_link::link!("setupapi.dll" "system" fn CM_Unregister_Device_Interface_ExW(pszdeviceinterface : windows_sys::core::PCWSTR, ulflags : u32, hmachine : HMACHINE) -> CONFIGRET);
windows_link::link!("cfgmgr32.dll" "system" fn CM_Unregister_Notification(notifycontext : HCMNOTIFICATION) -> CONFIGRET);
windows_link::link!("setupapi.dll" "system" "CMP_WaitNoPendingInstallEvents" fn CM_WaitNoPendingInstallEvents(dwtimeout : u32) -> u32);
pub const ALLOC_LOG_CONF: i32 = 2;
pub const BASIC_LOG_CONF: i32 = 0;
pub const BOOT_LOG_CONF: i32 = 3;
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
#[derive(Clone, Copy)]
pub struct CM_NOTIFY_EVENT_DATA_0_0 {
    pub ClassGuid: windows_sys::core::GUID,
    pub SymbolicLink: [u16; 1],
}
impl Default for CM_NOTIFY_EVENT_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_NOTIFY_EVENT_DATA_0_1 {
    pub EventGuid: windows_sys::core::GUID,
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct CM_NOTIFY_FILTER_0_0 {
    pub ClassGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct CM_NOTIFY_FILTER_0_1 {
    pub hTarget: super::HANDLE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct CM_NOTIFY_FILTER_0_2 {
    pub InstanceId: [u16; 200],
}
#[cfg(feature = "winnt")]
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
pub type CONFLICT_DETAILS = CONFLICT_DETAILS_A;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
pub type CONFLICT_LIST = usize;
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
#[derive(Clone, Copy)]
pub struct CS_DES {
    pub CSD_SignatureLength: u32,
    pub CSD_LegacyDataOffset: u32,
    pub CSD_LegacyDataSize: u32,
    pub CSD_Flags: u32,
    pub CSD_ClassGuid: windows_sys::core::GUID,
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
pub type DEVINST = u32;
pub type DEVINSTID = DEVINSTID_A;
pub type DEVINSTID_A = *mut i8;
pub type DEVINSTID_W = *mut u16;
pub type DEVNODE = u32;
pub type DEVNODEID = DEVNODEID_A;
pub type DEVNODEID_A = *mut i8;
pub type DEVNODEID_W = *mut u16;
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
pub const FILTERED_LOG_CONF: i32 = 1;
pub const FORCED_LOG_CONF: i32 = 4;
pub type HCMNOTIFICATION = *mut core::ffi::c_void;
#[cfg(feature = "winnt")]
pub type HMACHINE = super::HANDLE;
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
pub const IOA_Local: i32 = 255;
pub const IO_ALIAS_10_BIT_DECODE: i32 = 4;
pub const IO_ALIAS_12_BIT_DECODE: i32 = 16;
pub const IO_ALIAS_16_BIT_DECODE: i32 = 0;
pub const IO_ALIAS_POSITIVE_DECODE: i32 = 255;
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct IO_DES {
    pub IOD_Count: u32,
    pub IOD_Type: u32,
    pub IOD_Alloc_Base: super::DWORDLONG,
    pub IOD_Alloc_End: super::DWORDLONG,
    pub IOD_DesFlags: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct IO_RANGE {
    pub IOR_Align: super::DWORDLONG,
    pub IOR_nPorts: u32,
    pub IOR_Min: super::DWORDLONG,
    pub IOR_Max: super::DWORDLONG,
    pub IOR_RangeFlags: u32,
    pub IOR_Alias: super::DWORDLONG,
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
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct MEM_DES {
    pub MD_Count: u32,
    pub MD_Type: u32,
    pub MD_Alloc_Base: super::DWORDLONG,
    pub MD_Alloc_End: super::DWORDLONG,
    pub MD_Flags: u32,
    pub MD_Reserved: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct MEM_LARGE_DES {
    pub MLD_Count: u32,
    pub MLD_Type: u32,
    pub MLD_Alloc_Base: super::DWORDLONG,
    pub MLD_Alloc_End: super::DWORDLONG,
    pub MLD_Flags: u32,
    pub MLD_Reserved: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct MEM_LARGE_RANGE {
    pub MLR_Align: super::DWORDLONG,
    pub MLR_nBytes: u64,
    pub MLR_Min: super::DWORDLONG,
    pub MLR_Max: super::DWORDLONG,
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
    pub MR_Align: super::DWORDLONG,
    pub MR_nBytes: u32,
    pub MR_Min: super::DWORDLONG,
    pub MR_Max: super::DWORDLONG,
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
pub const NUM_CR_RESULTS: i32 = 60;
pub const NUM_LOG_CONF: i32 = 6;
pub const OVERRIDE_LOG_CONF: i32 = 5;
pub type PBUSNUMBER_DES = *mut BUSNUMBER_DES;
pub type PBUSNUMBER_RANGE = *mut BUSNUMBER_RANGE;
pub type PBUSNUMBER_RESOURCE = *mut BUSNUMBER_RESOURCE;
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
pub const PCD_MAX_IO: i32 = 2;
pub const PCD_MAX_MEMORY: i32 = 2;
pub type PCM_NOTIFY_ACTION = *mut CM_NOTIFY_ACTION;
pub type PCM_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(hnotify: HCMNOTIFICATION, context: *const core::ffi::c_void, action: CM_NOTIFY_ACTION, eventdata: *const CM_NOTIFY_EVENT_DATA, eventdatasize: u32) -> u32>;
pub type PCM_NOTIFY_EVENT_DATA = *mut CM_NOTIFY_EVENT_DATA;
#[cfg(feature = "winnt")]
pub type PCM_NOTIFY_FILTER = *mut CM_NOTIFY_FILTER;
pub type PCM_NOTIFY_FILTER_TYPE = *mut CM_NOTIFY_FILTER_TYPE;
pub type PCONFLICT_DETAILS = PCONFLICT_DETAILS_A;
pub type PCONFLICT_DETAILS_A = *mut CONFLICT_DETAILS_A;
pub type PCONFLICT_DETAILS_W = *mut CONFLICT_DETAILS_W;
pub type PCONFLICT_LIST = *mut CONFLICT_LIST;
pub type PCONNECTION_DES = *mut CONNECTION_DES;
pub type PCONNECTION_RESOURCE = *mut CONNECTION_RESOURCE;
pub type PCS_DES = *mut CS_DES;
pub type PCS_RESOURCE = *mut CS_RESOURCE;
pub type PDEVINST = *mut DEVNODE;
pub type PDEVNODE = *mut DEVNODE;
pub type PDEVPRIVATE_DES = *mut DEVPRIVATE_DES;
pub type PDEVPRIVATE_RANGE = *mut DEVPRIVATE_RANGE;
pub type PDEVPRIVATE_RESOURCE = *mut DEVPRIVATE_RESOURCE;
pub type PDMA_DES = *mut DMA_DES;
pub type PDMA_RANGE = *mut DMA_RANGE;
pub type PDMA_RESOURCE = *mut DMA_RESOURCE;
pub type PHCMNOTIFICATION = *mut HCMNOTIFICATION;
#[cfg(feature = "winnt")]
pub type PHMACHINE = *mut HMACHINE;
pub type PHWPROFILEINFO = PHWPROFILEINFO_A;
pub type PHWPROFILEINFO_A = *mut HWPROFILEINFO_A;
pub type PHWPROFILEINFO_W = *mut HWPROFILEINFO_W;
#[cfg(feature = "winnt")]
pub type PIO_DES = *mut IO_DES;
#[cfg(feature = "winnt")]
pub type PIO_RANGE = *mut IO_RANGE;
#[cfg(feature = "winnt")]
pub type PIO_RESOURCE = *mut IO_RESOURCE;
#[cfg(target_arch = "x86")]
pub type PIRQ_DES = PIRQ_DES_32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PIRQ_DES = PIRQ_DES_64;
pub type PIRQ_DES_32 = *mut IRQ_DES_32;
pub type PIRQ_DES_64 = *mut IRQ_DES_64;
pub type PIRQ_RANGE = *mut IRQ_RANGE;
#[cfg(target_arch = "x86")]
pub type PIRQ_RESOURCE = PIRQ_RESOURCE_32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PIRQ_RESOURCE = PIRQ_RESOURCE_64;
pub type PIRQ_RESOURCE_32 = *mut IRQ_RESOURCE_32;
pub type PIRQ_RESOURCE_64 = *mut IRQ_RESOURCE_64;
pub type PLOG_CONF = *mut LOG_CONF;
#[cfg(feature = "winnt")]
pub type PMEM_DES = *mut MEM_DES;
#[cfg(feature = "winnt")]
pub type PMEM_LARGE_DES = *mut MEM_LARGE_DES;
#[cfg(feature = "winnt")]
pub type PMEM_LARGE_RANGE = *mut MEM_LARGE_RANGE;
#[cfg(feature = "winnt")]
pub type PMEM_LARGE_RESOURCE = *mut MEM_LARGE_RESOURCE;
#[cfg(feature = "winnt")]
pub type PMEM_RANGE = *mut MEM_RANGE;
#[cfg(feature = "winnt")]
pub type PMEM_RESOURCE = *mut MEM_RESOURCE;
pub type PMFCARD_DES = *mut MFCARD_DES;
pub type PMFCARD_RESOURCE = *mut MFCARD_RESOURCE;
pub type PPCCARD_DES = *mut PCCARD_DES;
pub type PPCCARD_RESOURCE = *mut PCCARD_RESOURCE;
pub type PPRIORITY = *mut PRIORITY;
pub type PRANGE_ELEMENT = *mut RANGE_ELEMENT;
pub type PRANGE_LIST = *mut RANGE_LIST;
pub type PRESOURCEID = *mut RESOURCEID;
pub type PRES_DES = *mut RES_DES;
pub type PRIORITY = u32;
pub const PRIORITY_BIT: i32 = 8;
pub const PRIORITY_EQUAL_FIRST: i32 = 8;
pub const PRIORITY_EQUAL_LAST: i32 = 0;
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
