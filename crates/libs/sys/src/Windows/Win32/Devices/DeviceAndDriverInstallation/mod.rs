#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CMP_WaitNoPendingInstallEvents(dwtimeout: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Data_HtmlHelp\"`*"]
    #[cfg(feature = "Win32_Data_HtmlHelp")]
    pub fn CM_Add_Empty_Log_Conf(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Data_HtmlHelp\"`*"]
    #[cfg(feature = "Win32_Data_HtmlHelp")]
    pub fn CM_Add_Empty_Log_Conf_Ex(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Add_IDA(dndevinst: u32, pszid: ::windows_sys::core::PCSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Add_IDW(dndevinst: u32, pszid: ::windows_sys::core::PCWSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Add_ID_ExA(dndevinst: u32, pszid: ::windows_sys::core::PCSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Add_ID_ExW(dndevinst: u32, pszid: ::windows_sys::core::PCWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Add_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Add_Res_Des(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Add_Res_Des_Ex(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Connect_MachineA(uncservername: ::windows_sys::core::PCSTR, phmachine: *mut isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Connect_MachineW(uncservername: ::windows_sys::core::PCWSTR, phmachine: *mut isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Create_DevNodeA(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Create_DevNodeW(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Create_DevNode_ExA(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Create_DevNode_ExW(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Create_Range_List(prlh: *mut usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Delete_Class_Key(classguid: *const ::windows_sys::core::GUID, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Delete_Class_Key_Ex(classguid: *const ::windows_sys::core::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Delete_DevNode_Key(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Delete_DevNode_Key_Ex(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Delete_Device_Interface_KeyA(pszdeviceinterface: ::windows_sys::core::PCSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Delete_Device_Interface_KeyW(pszdeviceinterface: ::windows_sys::core::PCWSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface: ::windows_sys::core::PCSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface: ::windows_sys::core::PCWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Delete_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Detect_Resource_Conflict(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Detect_Resource_Conflict_Ex(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Disable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Disable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Disconnect_Machine(hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Dup_Range_List(rlhold: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Enable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Enable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Enumerate_Classes(ulclassindex: u32, classguid: *mut ::windows_sys::core::GUID, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Enumerate_Classes_Ex(ulclassindex: u32, classguid: *mut ::windows_sys::core::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Enumerate_EnumeratorsA(ulenumindex: u32, buffer: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Enumerate_EnumeratorsW(ulenumindex: u32, buffer: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Enumerate_Enumerators_ExA(ulenumindex: u32, buffer: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Enumerate_Enumerators_ExW(ulenumindex: u32, buffer: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Find_Range(pullstart: *mut u64, ullstart: u64, ullength: u32, ullalignment: u64, ullend: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_First_Range(rlh: usize, pullstart: *mut u64, pullend: *mut u64, preelement: *mut usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Free_Log_Conf(lclogconftobefreed: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Free_Log_Conf_Ex(lclogconftobefreed: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Free_Log_Conf_Handle(lclogconf: usize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Free_Range_List(rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Free_Res_Des(prdresdes: *mut usize, rdresdes: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Free_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Free_Res_Des_Handle(rdresdes: usize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Free_Resource_Conflict_Handle(clconflictlist: usize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Child(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Child_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_Key_NameA(classguid: *const ::windows_sys::core::GUID, pszkeyname: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_Key_NameW(classguid: *const ::windows_sys::core::GUID, pszkeyname: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_Key_Name_ExA(classguid: *const ::windows_sys::core::GUID, pszkeyname: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_Key_Name_ExW(classguid: *const ::windows_sys::core::GUID, pszkeyname: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_NameA(classguid: *const ::windows_sys::core::GUID, buffer: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_NameW(classguid: *const ::windows_sys::core::GUID, buffer: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_Name_ExA(classguid: *const ::windows_sys::core::GUID, buffer: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_Name_ExW(classguid: *const ::windows_sys::core::GUID, buffer: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_PropertyW(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_Property_ExW(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_Property_Keys(classguid: *const ::windows_sys::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_Property_Keys_Ex(classguid: *const ::windows_sys::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_Registry_PropertyA(classguid: *const ::windows_sys::core::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Class_Registry_PropertyW(classguid: *const ::windows_sys::core::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Depth(puldepth: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Depth_Ex(puldepth: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Custom_PropertyA(dndevinst: u32, pszcustompropertyname: ::windows_sys::core::PCSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Custom_PropertyW(dndevinst: u32, pszcustompropertyname: ::windows_sys::core::PCWSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Custom_Property_ExA(dndevinst: u32, pszcustompropertyname: ::windows_sys::core::PCSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Custom_Property_ExW(dndevinst: u32, pszcustompropertyname: ::windows_sys::core::PCWSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_Property_Keys(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_Property_Keys_Ex(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Status(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_DevNode_Status_Ex(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_IDA(dndevinst: u32, buffer: ::windows_sys::core::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_IDW(dndevinst: u32, buffer: ::windows_sys::core::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_ExA(dndevinst: u32, buffer: ::windows_sys::core::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_ExW(dndevinst: u32, buffer: ::windows_sys::core::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_ListA(pszfilter: ::windows_sys::core::PCSTR, buffer: ::windows_sys::core::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_ListW(pszfilter: ::windows_sys::core::PCWSTR, buffer: ::windows_sys::core::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_List_ExA(pszfilter: ::windows_sys::core::PCSTR, buffer: ::windows_sys::core::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_List_ExW(pszfilter: ::windows_sys::core::PCWSTR, buffer: ::windows_sys::core::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_List_SizeA(pullen: *mut u32, pszfilter: ::windows_sys::core::PCSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_List_SizeW(pullen: *mut u32, pszfilter: ::windows_sys::core::PCWSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_List_Size_ExA(pullen: *mut u32, pszfilter: ::windows_sys::core::PCSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_List_Size_ExW(pullen: *mut u32, pszfilter: ::windows_sys::core::PCWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_Size(pullen: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_ID_Size_Ex(pullen: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_AliasA(pszdeviceinterface: ::windows_sys::core::PCSTR, aliasinterfaceguid: *const ::windows_sys::core::GUID, pszaliasdeviceinterface: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_AliasW(pszdeviceinterface: ::windows_sys::core::PCWSTR, aliasinterfaceguid: *const ::windows_sys::core::GUID, pszaliasdeviceinterface: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface: ::windows_sys::core::PCSTR, aliasinterfaceguid: *const ::windows_sys::core::GUID, pszaliasdeviceinterface: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface: ::windows_sys::core::PCWSTR, aliasinterfaceguid: *const ::windows_sys::core::GUID, pszaliasdeviceinterface: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_ListA(interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const i8, buffer: ::windows_sys::core::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_ListW(interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const u16, buffer: ::windows_sys::core::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_List_ExA(interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const i8, buffer: ::windows_sys::core::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_List_ExW(interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const u16, buffer: ::windows_sys::core::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_List_SizeA(pullen: *mut u32, interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_List_SizeW(pullen: *mut u32, interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_List_Size_ExA(pullen: *mut u32, interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Device_Interface_List_Size_ExW(pullen: *mut u32, interfaceclassguid: *const ::windows_sys::core::GUID, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Device_Interface_PropertyW(pszdeviceinterface: ::windows_sys::core::PCWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Device_Interface_Property_ExW(pszdeviceinterface: ::windows_sys::core::PCWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface: ::windows_sys::core::PCWSTR, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface: ::windows_sys::core::PCWSTR, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_First_Log_Conf(plclogconf: *mut usize, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_First_Log_Conf_Ex(plclogconf: *mut usize, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Global_State(pulstate: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Global_State_Ex(pulstate: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_HW_Prof_FlagsA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_HW_Prof_FlagsW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Hardware_Profile_InfoA(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Hardware_Profile_InfoW(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Hardware_Profile_Info_ExA(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Hardware_Profile_Info_ExW(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Log_Conf_Priority(lclogconf: usize, ppriority: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Log_Conf_Priority_Ex(lclogconf: usize, ppriority: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Next_Log_Conf(plclogconf: *mut usize, lclogconf: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Next_Log_Conf_Ex(plclogconf: *mut usize, lclogconf: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Next_Res_Des(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Next_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Parent(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Parent_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Res_Des_Data(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Res_Des_Data_Ex(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Res_Des_Data_Size(pulsize: *mut u32, rdresdes: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Res_Des_Data_Size_Ex(pulsize: *mut u32, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Resource_Conflict_Count(clconflictlist: usize, pulcount: *mut u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Resource_Conflict_DetailsA(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_A) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Resource_Conflict_DetailsW(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_W) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Sibling(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Sibling_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Version() -> u16;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Get_Version_Ex(hmachine: isize) -> u16;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Intersect_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Invert_Range_List(rlhold: usize, rlhnew: usize, ullmaxvalue: u64, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Dock_Station_Present(pbpresent: *mut super::super::Foundation::BOOL) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Dock_Station_Present_Ex(pbpresent: *mut super::super::Foundation::BOOL, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Version_Available(wversion: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Version_Available_Ex(wversion: u16, hmachine: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Locate_DevNodeA(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Locate_DevNodeW(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Locate_DevNode_ExA(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Locate_DevNode_ExW(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_MapCrToWin32Err(cmreturncode: CONFIGRET, defaulterr: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Merge_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Modify_Res_Des(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Modify_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Move_DevNode(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Move_DevNode_Ex(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Next_Range(preelement: *mut usize, pullstart: *mut u64, pullend: *mut u64, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_Class_KeyA(classguid: *const ::windows_sys::core::GUID, pszclassname: ::windows_sys::core::PCSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_Class_KeyW(classguid: *const ::windows_sys::core::GUID, pszclassname: ::windows_sys::core::PCWSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_Class_Key_ExA(classguid: *const ::windows_sys::core::GUID, pszclassname: ::windows_sys::core::PCSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_Class_Key_ExW(classguid: *const ::windows_sys::core::GUID, pszclassname: ::windows_sys::core::PCWSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_DevNode_Key(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_DevNode_Key_Ex(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_Device_Interface_KeyA(pszdeviceinterface: ::windows_sys::core::PCSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_Device_Interface_KeyW(pszdeviceinterface: ::windows_sys::core::PCWSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_Device_Interface_Key_ExA(pszdeviceinterface: ::windows_sys::core::PCSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_Device_Interface_Key_ExW(pszdeviceinterface: ::windows_sys::core::PCWSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_And_Remove_SubTreeA(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows_sys::core::PSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_And_Remove_SubTreeW(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows_sys::core::PWSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_And_Remove_SubTree_ExA(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows_sys::core::PSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_And_Remove_SubTree_ExW(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows_sys::core::PWSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_Arbitrator_Free_Data(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_Arbitrator_Free_Data_Ex(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_Arbitrator_Free_Size(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_Arbitrator_Free_Size_Ex(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Query_Resource_Conflict_List(pclconflictlist: *mut usize, dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Reenumerate_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Reenumerate_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Register_Device_Driver(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Register_Device_Driver_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Register_Device_InterfaceA(dndevinst: u32, interfaceclassguid: *const ::windows_sys::core::GUID, pszreference: ::windows_sys::core::PCSTR, pszdeviceinterface: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Register_Device_InterfaceW(dndevinst: u32, interfaceclassguid: *const ::windows_sys::core::GUID, pszreference: ::windows_sys::core::PCWSTR, pszdeviceinterface: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Register_Device_Interface_ExA(dndevinst: u32, interfaceclassguid: *const ::windows_sys::core::GUID, pszreference: ::windows_sys::core::PCSTR, pszdeviceinterface: ::windows_sys::core::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Register_Device_Interface_ExW(dndevinst: u32, interfaceclassguid: *const ::windows_sys::core::GUID, pszreference: ::windows_sys::core::PCWSTR, pszdeviceinterface: ::windows_sys::core::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Notification(pfilter: *const CM_NOTIFY_FILTER, pcontext: *const ::core::ffi::c_void, pcallback: PCM_NOTIFY_CALLBACK, pnotifycontext: *mut isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Request_Device_EjectA(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows_sys::core::PSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Request_Device_EjectW(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows_sys::core::PWSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Request_Device_Eject_ExA(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows_sys::core::PSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Request_Device_Eject_ExW(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: ::windows_sys::core::PWSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Request_Eject_PC() -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Request_Eject_PC_Ex(hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Run_Detection(ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Run_Detection_Ex(ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_Class_PropertyW(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_Class_Property_ExW(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_Class_Registry_PropertyA(classguid: *const ::windows_sys::core::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_Class_Registry_PropertyW(classguid: *const ::windows_sys::core::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_DevNode_Problem(dndevinst: u32, ulproblem: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_DevNode_Problem_Ex(dndevinst: u32, ulproblem: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_Device_Interface_PropertyW(pszdeviceinterface: ::windows_sys::core::PCWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_Device_Interface_Property_ExW(pszdeviceinterface: ::windows_sys::core::PCWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_HW_Prof(ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_HW_Prof_Ex(ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_HW_Prof_FlagsA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_HW_Prof_FlagsW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Set_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Setup_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Setup_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Test_Range_Available(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Uninstall_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Uninstall_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Unregister_Device_InterfaceA(pszdeviceinterface: ::windows_sys::core::PCSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Unregister_Device_InterfaceW(pszdeviceinterface: ::windows_sys::core::PCWSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Unregister_Device_Interface_ExA(pszdeviceinterface: ::windows_sys::core::PCSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Unregister_Device_Interface_ExW(pszdeviceinterface: ::windows_sys::core::PCWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn CM_Unregister_Notification(notifycontext: HCMNOTIFICATION) -> CONFIGRET;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiInstallDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiInstallDriverA(hwndparent: super::super::Foundation::HWND, infpath: ::windows_sys::core::PCSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiInstallDriverW(hwndparent: super::super::Foundation::HWND, infpath: ::windows_sys::core::PCWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiRollbackDriver(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, hwndparent: super::super::Foundation::HWND, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiShowUpdateDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiShowUpdateDriver(hwndparent: super::super::Foundation::HWND, filepath: ::windows_sys::core::PCWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiUninstallDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiUninstallDriverA(hwndparent: super::super::Foundation::HWND, infpath: ::windows_sys::core::PCSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiUninstallDriverW(hwndparent: super::super::Foundation::HWND, infpath: ::windows_sys::core::PCWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallHinfSectionA(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: ::windows_sys::core::PCSTR, showcommand: i32);
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallHinfSectionW(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: ::windows_sys::core::PCWSTR, showcommand: i32);
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddInstallSectionToDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddInstallSectionToDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCWSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddSectionToDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddSectionToDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToDiskSpaceListA(diskspace: *const ::core::ffi::c_void, targetfilespec: ::windows_sys::core::PCSTR, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToDiskSpaceListW(diskspace: *const ::core::ffi::c_void, targetfilespec: ::windows_sys::core::PCWSTR, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToSourceListA(flags: u32, source: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToSourceListW(flags: u32, source: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAdjustDiskSpaceListA(diskspace: *const ::core::ffi::c_void, driveroot: ::windows_sys::core::PCSTR, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAdjustDiskSpaceListW(diskspace: *const ::core::ffi::c_void, driveroot: ::windows_sys::core::PCWSTR, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupBackupErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCSTR, sourcefile: ::windows_sys::core::PCSTR, targetfile: ::windows_sys::core::PCSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupBackupErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCWSTR, sourcefile: ::windows_sys::core::PCWSTR, targetfile: ::windows_sys::core::PCWSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCancelTemporarySourceList() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCloseFileQueue(queuehandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupCloseInfFile(infhandle: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupCloseLog();
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCommitFileQueueA(owner: super::super::Foundation::HWND, queuehandle: *const ::core::ffi::c_void, msghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCommitFileQueueW(owner: super::super::Foundation::HWND, queuehandle: *const ::core::ffi::c_void, msghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupConfigureWmiFromInfSectionA(infhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupConfigureWmiFromInfSectionW(infhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCSTR, diskname: ::windows_sys::core::PCSTR, pathtosource: ::windows_sys::core::PCSTR, sourcefile: ::windows_sys::core::PCSTR, targetpathfile: ::windows_sys::core::PCSTR, win32errorcode: u32, style: u32, pathbuffer: ::windows_sys::core::PSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCWSTR, diskname: ::windows_sys::core::PCWSTR, pathtosource: ::windows_sys::core::PCWSTR, sourcefile: ::windows_sys::core::PCWSTR, targetpathfile: ::windows_sys::core::PCWSTR, win32errorcode: u32, style: u32, pathbuffer: ::windows_sys::core::PWSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyOEMInfA(sourceinffilename: ::windows_sys::core::PCSTR, oemsourcemedialocation: ::windows_sys::core::PCSTR, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: ::windows_sys::core::PSTR, destinationinffilenamesize: u32, requiredsize: *mut u32, destinationinffilenamecomponent: *mut ::windows_sys::core::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyOEMInfW(sourceinffilename: ::windows_sys::core::PCWSTR, oemsourcemedialocation: ::windows_sys::core::PCWSTR, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: ::windows_sys::core::PWSTR, destinationinffilenamesize: u32, requiredsize: *mut u32, destinationinffilenamecomponent: *mut ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupCreateDiskSpaceListA(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupCreateDiskSpaceListW(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupDecompressOrCopyFileA(sourcefilename: ::windows_sys::core::PCSTR, targetfilename: ::windows_sys::core::PCSTR, compressiontype: *const u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupDecompressOrCopyFileW(sourcefilename: ::windows_sys::core::PCWSTR, targetfilename: ::windows_sys::core::PCWSTR, compressiontype: *const u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupDefaultQueueCallbackA(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupDefaultQueueCallbackW(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDeleteErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCSTR, file: ::windows_sys::core::PCSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDeleteErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCWSTR, file: ::windows_sys::core::PCWSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDestroyDiskSpaceList(diskspace: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiAskForOEMDisk(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildClassInfoList(flags: u32, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildClassInfoListExA(flags: u32, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildClassInfoListExW(flags: u32, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildDriverInfoList(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, drivertype: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCallClassInstaller(installfunction: u32, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCancelDriverInfoSearch(deviceinfoset: HDEVINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiChangeState(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameA(classname: ::windows_sys::core::PCSTR, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameExA(classname: ::windows_sys::core::PCSTR, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameExW(classname: ::windows_sys::core::PCWSTR, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameW(classname: ::windows_sys::core::PCWSTR, classguidlist: *mut ::windows_sys::core::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidA(classguid: *const ::windows_sys::core::GUID, classname: ::windows_sys::core::PSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidExA(classguid: *const ::windows_sys::core::GUID, classname: ::windows_sys::core::PSTR, classnamesize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidExW(classguid: *const ::windows_sys::core::GUID, classname: ::windows_sys::core::PWSTR, classnamesize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidW(classguid: *const ::windows_sys::core::GUID, classname: ::windows_sys::core::PWSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiCreateDevRegKeyA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: ::windows_sys::core::PCSTR) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiCreateDevRegKeyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: ::windows_sys::core::PCWSTR) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoA(deviceinfoset: HDEVINFO, devicename: ::windows_sys::core::PCSTR, classguid: *const ::windows_sys::core::GUID, devicedescription: ::windows_sys::core::PCSTR, hwndparent: super::super::Foundation::HWND, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoList(classguid: *const ::windows_sys::core::GUID, hwndparent: super::super::Foundation::HWND) -> HDEVINFO;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoListExA(classguid: *const ::windows_sys::core::GUID, hwndparent: super::super::Foundation::HWND, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> HDEVINFO;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoListExW(classguid: *const ::windows_sys::core::GUID, hwndparent: super::super::Foundation::HWND, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> HDEVINFO;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoW(deviceinfoset: HDEVINFO, devicename: ::windows_sys::core::PCWSTR, classguid: *const ::windows_sys::core::GUID, devicedescription: ::windows_sys::core::PCWSTR, hwndparent: super::super::Foundation::HWND, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInterfaceA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows_sys::core::GUID, referencestring: ::windows_sys::core::PCSTR, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiCreateDeviceInterfaceRegKeyA(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: ::windows_sys::core::PCSTR) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiCreateDeviceInterfaceRegKeyW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: ::windows_sys::core::PCWSTR) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInterfaceW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows_sys::core::GUID, referencestring: ::windows_sys::core::PCWSTR, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDevRegKey(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDeviceInfo(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDeviceInterfaceData(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiDestroyClassImageList(classimagelistdata: *const SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDestroyDeviceInfoList(deviceinfoset: HDEVINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDestroyDriverInfoList(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetupDiDrawMiniIcon(hdc: super::super::Graphics::Gdi::HDC, rc: super::super::Foundation::RECT, miniiconindex: i32, flags: u32) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDeviceInfo(deviceinfoset: HDEVINFO, memberindex: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDeviceInterfaces(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows_sys::core::GUID, memberindex: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDriverInfoA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDriverInfoW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualModelsSectionA(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: ::windows_sys::core::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualModelsSectionW(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: ::windows_sys::core::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetActualSectionToInstallA(infhandle: *const ::core::ffi::c_void, infsectionname: ::windows_sys::core::PCSTR, infsectionwithext: ::windows_sys::core::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut ::windows_sys::core::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualSectionToInstallExA(infhandle: *const ::core::ffi::c_void, infsectionname: ::windows_sys::core::PCSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: ::windows_sys::core::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut ::windows_sys::core::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualSectionToInstallExW(infhandle: *const ::core::ffi::c_void, infsectionname: ::windows_sys::core::PCWSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: ::windows_sys::core::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut ::windows_sys::core::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetActualSectionToInstallW(infhandle: *const ::core::ffi::c_void, infsectionname: ::windows_sys::core::PCWSTR, infsectionwithext: ::windows_sys::core::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut ::windows_sys::core::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassBitmapIndex(classguid: *const ::windows_sys::core::GUID, miniiconindex: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionA(classguid: *const ::windows_sys::core::GUID, classdescription: ::windows_sys::core::PSTR, classdescriptionsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionExA(classguid: *const ::windows_sys::core::GUID, classdescription: ::windows_sys::core::PSTR, classdescriptionsize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionExW(classguid: *const ::windows_sys::core::GUID, classdescription: ::windows_sys::core::PWSTR, classdescriptionsize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionW(classguid: *const ::windows_sys::core::GUID, classdescription: ::windows_sys::core::PWSTR, classdescriptionsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiGetClassDevPropertySheetsA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERA_V2, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_Controls\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiGetClassDevPropertySheetsW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const super::super::UI::Controls::PROPSHEETHEADERW_V2, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsA(classguid: *const ::windows_sys::core::GUID, enumerator: ::windows_sys::core::PCSTR, hwndparent: super::super::Foundation::HWND, flags: u32) -> HDEVINFO;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsExA(classguid: *const ::windows_sys::core::GUID, enumerator: ::windows_sys::core::PCSTR, hwndparent: super::super::Foundation::HWND, flags: u32, deviceinfoset: HDEVINFO, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> HDEVINFO;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsExW(classguid: *const ::windows_sys::core::GUID, enumerator: ::windows_sys::core::PCWSTR, hwndparent: super::super::Foundation::HWND, flags: u32, deviceinfoset: HDEVINFO, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> HDEVINFO;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsW(classguid: *const ::windows_sys::core::GUID, enumerator: ::windows_sys::core::PCWSTR, hwndparent: super::super::Foundation::HWND, flags: u32) -> HDEVINFO;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageIndex(classimagelistdata: *const SP_CLASSIMAGELIST_DATA, classguid: *const ::windows_sys::core::GUID, imageindex: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageList(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageListExA(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageListExW(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyExW(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyKeys(classguid: *const ::windows_sys::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyKeysExW(classguid: *const ::windows_sys::core::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyW(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassRegistryPropertyA(classguid: *const ::windows_sys::core::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassRegistryPropertyW(classguid: *const ::windows_sys::core::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetCustomDevicePropertyA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: ::windows_sys::core::PCSTR, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetCustomDevicePropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: ::windows_sys::core::PCWSTR, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInfoListClass(deviceinfoset: HDEVINFO, classguid: *mut ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInfoListDetailA(deviceinfoset: HDEVINFO, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInfoListDetailW(deviceinfoset: HDEVINFO, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut SP_DEVINSTALL_PARAMS_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstanceIdA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: ::windows_sys::core::PSTR, deviceinstanceidsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstanceIdW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: ::windows_sys::core::PWSTR, deviceinstanceidsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInterfaceAlias(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid: *const ::windows_sys::core::GUID, aliasdeviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInterfaceDetailA(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInterfaceDetailW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDeviceInterfacePropertyW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDevicePropertyKeys(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDevicePropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceRegistryPropertyA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceRegistryPropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInfoDetailA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_A, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInfoDetailW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_W, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameA(hwprofile: u32, friendlyname: ::windows_sys::core::PSTR, friendlynamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameExA(hwprofile: u32, friendlyname: ::windows_sys::core::PSTR, friendlynamesize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameExW(hwprofile: u32, friendlyname: ::windows_sys::core::PWSTR, friendlynamesize: u32, requiredsize: *mut u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameW(hwprofile: u32, friendlyname: ::windows_sys::core::PWSTR, friendlynamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileList(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileListExA(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileListExW(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetINFClassA(infname: ::windows_sys::core::PCSTR, classguid: *mut ::windows_sys::core::GUID, classname: ::windows_sys::core::PSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetINFClassW(infname: ::windows_sys::core::PCWSTR, classguid: *mut ::windows_sys::core::GUID, classname: ::windows_sys::core::PWSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetSelectedDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetSelectedDriverA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetSelectedDriverW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetWizardPage(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, installwizarddata: *const SP_INSTALLWIZARD_DATA, pagetype: u32, flags: u32) -> super::super::UI::Controls::HPROPSHEETPAGE;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassA(hwndparent: super::super::Foundation::HWND, inffilename: ::windows_sys::core::PCSTR, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassExA(hwndparent: super::super::Foundation::HWND, inffilename: ::windows_sys::core::PCSTR, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows_sys::core::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassExW(hwndparent: super::super::Foundation::HWND, inffilename: ::windows_sys::core::PCWSTR, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows_sys::core::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassW(hwndparent: super::super::Foundation::HWND, inffilename: ::windows_sys::core::PCWSTR, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallDeviceInterfaces(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallDriverFiles(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiLoadClassIcon(classguid: *const ::windows_sys::core::GUID, largeicon: *mut super::super::UI::WindowsAndMessaging::HICON, miniiconindex: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiLoadDeviceIcon(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, cxicon: u32, cyicon: u32, flags: u32, hicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenClassRegKey(classguid: *const ::windows_sys::core::GUID, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenClassRegKeyExA(classguid: *const ::windows_sys::core::GUID, samdesired: u32, flags: u32, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenClassRegKeyExW(classguid: *const ::windows_sys::core::GUID, samdesired: u32, flags: u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenDevRegKey(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInfoA(deviceinfoset: HDEVINFO, deviceinstanceid: ::windows_sys::core::PCSTR, hwndparent: super::super::Foundation::HWND, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInfoW(deviceinfoset: HDEVINFO, deviceinstanceid: ::windows_sys::core::PCWSTR, hwndparent: super::super::Foundation::HWND, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInterfaceA(deviceinfoset: HDEVINFO, devicepath: ::windows_sys::core::PCSTR, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenDeviceInterfaceRegKey(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInterfaceW(deviceinfoset: HDEVINFO, devicepath: ::windows_sys::core::PCWSTR, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRegisterCoDeviceInstallers(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRegisterDeviceInfo(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, flags: u32, compareproc: PSP_DETSIG_CMPPROC, comparecontext: *const ::core::ffi::c_void, dupdeviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRemoveDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRemoveDeviceInterface(deviceinfoset: HDEVINFO, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRestartDevices(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSelectBestCompatDrv(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSelectDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSelectOEMDrv(hwndparent: super::super::Foundation::HWND, deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetClassPropertyExW(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetClassPropertyW(classguid: *const ::windows_sys::core::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassRegistryPropertyA(classguid: *const ::windows_sys::core::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: ::windows_sys::core::PCSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassRegistryPropertyW(classguid: *const ::windows_sys::core::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: ::windows_sys::core::PCWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const SP_DEVINSTALL_PARAMS_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceInterfaceDefault(deviceinfoset: HDEVINFO, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetDeviceInterfacePropertyW(deviceinfoset: HDEVINFO, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Devices_Properties\"`, `\"Win32_Foundation\"`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetDevicePropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceRegistryPropertyA(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceRegistryPropertyW(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDriverInstallParamsA(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDriverInstallParamsW(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetSelectedDevice(deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetSelectedDriverA(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetSelectedDriverW(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiUnremoveDevice(deviceinfoset: HDEVINFO, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupDuplicateDiskSpaceListA(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupDuplicateDiskSpaceListW(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupEnumInfSectionsA(infhandle: *const ::core::ffi::c_void, index: u32, buffer: ::windows_sys::core::PSTR, size: u32, sizeneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupEnumInfSectionsW(infhandle: *const ::core::ffi::c_void, index: u32, buffer: ::windows_sys::core::PWSTR, size: u32, sizeneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindFirstLineA(infhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCSTR, key: ::windows_sys::core::PCSTR, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindFirstLineW(infhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCWSTR, key: ::windows_sys::core::PCWSTR, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindNextLine(contextin: *const INFCONTEXT, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindNextMatchLineA(contextin: *const INFCONTEXT, key: ::windows_sys::core::PCSTR, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindNextMatchLineW(contextin: *const INFCONTEXT, key: ::windows_sys::core::PCWSTR, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFreeSourceListA(list: *mut *mut ::windows_sys::core::PSTR, count: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFreeSourceListW(list: *mut *mut ::windows_sys::core::PWSTR, count: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetBackupInformationA(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetBackupInformationW(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetBinaryField(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: *mut u8, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupGetFieldCount(context: *const INFCONTEXT) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupGetFileCompressionInfoA(sourcefilename: ::windows_sys::core::PCSTR, actualsourcefilename: *mut ::windows_sys::core::PSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoExA(sourcefilename: ::windows_sys::core::PCSTR, actualsourcefilenamebuffer: ::windows_sys::core::PCSTR, actualsourcefilenamebufferlen: u32, requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoExW(sourcefilename: ::windows_sys::core::PCWSTR, actualsourcefilenamebuffer: ::windows_sys::core::PCWSTR, actualsourcefilenamebufferlen: u32, requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupGetFileCompressionInfoW(sourcefilename: ::windows_sys::core::PCWSTR, actualsourcefilename: *mut ::windows_sys::core::PWSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileQueueCount(filequeue: *const ::core::ffi::c_void, subqueuefileop: u32, numoperations: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileQueueFlags(filequeue: *const ::core::ffi::c_void, flags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupGetInfDriverStoreLocationA(filename: ::windows_sys::core::PCSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: ::windows_sys::core::PCSTR, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupGetInfDriverStoreLocationW(filename: ::windows_sys::core::PCWSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: ::windows_sys::core::PCWSTR, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfFileListA(directorypath: ::windows_sys::core::PCSTR, infstyle: u32, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfFileListW(directorypath: ::windows_sys::core::PCWSTR, infstyle: u32, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfInformationA(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfInformationW(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfPublishedNameA(driverstorelocation: ::windows_sys::core::PCSTR, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfPublishedNameW(driverstorelocation: ::windows_sys::core::PCWSTR, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetIntField(context: *const INFCONTEXT, fieldindex: u32, integervalue: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineByIndexA(infhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCSTR, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineByIndexW(infhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCWSTR, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupGetLineCountA(infhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupGetLineCountW(infhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCWSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineTextA(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCSTR, key: ::windows_sys::core::PCSTR, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineTextW(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCWSTR, key: ::windows_sys::core::PCWSTR, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetMultiSzFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetMultiSzFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetNonInteractiveMode() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileLocationA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: ::windows_sys::core::PCSTR, sourceid: *mut u32, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileLocationW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: ::windows_sys::core::PCWSTR, sourceid: *mut u32, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileSizeA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: ::windows_sys::core::PCSTR, section: ::windows_sys::core::PCSTR, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileSizeW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: ::windows_sys::core::PCWSTR, section: ::windows_sys::core::PCWSTR, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceInfoA(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceInfoW(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetStringFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetStringFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetTargetPathA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: ::windows_sys::core::PCSTR, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetTargetPathW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: ::windows_sys::core::PCWSTR, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupGetThreadLogToken() -> u64;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitDefaultQueueCallback(ownerwindow: super::super::Foundation::HWND) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitDefaultQueueCallbackEx(ownerwindow: super::super::Foundation::HWND, alternateprogresswindow: super::super::Foundation::HWND, progressmessage: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupInitializeFileLogA(logfilename: ::windows_sys::core::PCSTR, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupInitializeFileLogW(logfilename: ::windows_sys::core::PCWSTR, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: ::windows_sys::core::PCSTR, sourcepathroot: ::windows_sys::core::PCSTR, destinationname: ::windows_sys::core::PCSTR, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileExA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: ::windows_sys::core::PCSTR, sourcepathroot: ::windows_sys::core::PCSTR, destinationname: ::windows_sys::core::PCSTR, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileExW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: ::windows_sys::core::PCWSTR, sourcepathroot: ::windows_sys::core::PCWSTR, destinationname: ::windows_sys::core::PCWSTR, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: ::windows_sys::core::PCWSTR, sourcepathroot: ::windows_sys::core::PCWSTR, destinationname: ::windows_sys::core::PCWSTR, copystyle: SP_COPY_STYLE, copymsghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFilesFromInfSectionA(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCSTR, sourcerootpath: ::windows_sys::core::PCSTR, copyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFilesFromInfSectionW(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCWSTR, sourcerootpath: ::windows_sys::core::PCWSTR, copyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupInstallFromInfSectionA(owner: super::super::Foundation::HWND, infhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCSTR, flags: u32, relativekeyroot: super::super::System::Registry::HKEY, sourcerootpath: ::windows_sys::core::PCSTR, copyflags: u32, msghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupInstallFromInfSectionW(owner: super::super::Foundation::HWND, infhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCWSTR, flags: u32, relativekeyroot: super::super::System::Registry::HKEY, sourcerootpath: ::windows_sys::core::PCWSTR, copyflags: u32, msghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionA(infhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionExA(infhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCSTR, flags: u32, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionExW(infhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCWSTR, flags: u32, deviceinfoset: HDEVINFO, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionW(infhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupIterateCabinetA(cabinetfile: ::windows_sys::core::PCSTR, reserved: u32, msghandler: PSP_FILE_CALLBACK_A, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupIterateCabinetW(cabinetfile: ::windows_sys::core::PCWSTR, reserved: u32, msghandler: PSP_FILE_CALLBACK_W, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogErrorA(messagestring: ::windows_sys::core::PCSTR, severity: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogErrorW(messagestring: ::windows_sys::core::PCWSTR, severity: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogFileA(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows_sys::core::PCSTR, sourcefilename: ::windows_sys::core::PCSTR, targetfilename: ::windows_sys::core::PCSTR, checksum: u32, disktagfile: ::windows_sys::core::PCSTR, diskdescription: ::windows_sys::core::PCSTR, otherinfo: ::windows_sys::core::PCSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogFileW(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows_sys::core::PCWSTR, sourcefilename: ::windows_sys::core::PCWSTR, targetfilename: ::windows_sys::core::PCWSTR, checksum: u32, disktagfile: ::windows_sys::core::PCWSTR, diskdescription: ::windows_sys::core::PCWSTR, otherinfo: ::windows_sys::core::PCWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenAppendInfFileA(filename: ::windows_sys::core::PCSTR, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenAppendInfFileW(filename: ::windows_sys::core::PCWSTR, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupOpenFileQueue() -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupOpenInfFileA(filename: ::windows_sys::core::PCSTR, infclass: ::windows_sys::core::PCSTR, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupOpenInfFileW(filename: ::windows_sys::core::PCWSTR, infclass: ::windows_sys::core::PCWSTR, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenLog(erase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupOpenMasterInf() -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPrepareQueueForRestoreA(queuehandle: *const ::core::ffi::c_void, backuppath: ::windows_sys::core::PCSTR, restoreflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPrepareQueueForRestoreW(queuehandle: *const ::core::ffi::c_void, backuppath: ::windows_sys::core::PCWSTR, restoreflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPromptForDiskA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCSTR, diskname: ::windows_sys::core::PCSTR, pathtosource: ::windows_sys::core::PCSTR, filesought: ::windows_sys::core::PCSTR, tagfile: ::windows_sys::core::PCSTR, diskpromptstyle: u32, pathbuffer: ::windows_sys::core::PSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPromptForDiskW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCWSTR, diskname: ::windows_sys::core::PCWSTR, pathtosource: ::windows_sys::core::PCWSTR, filesought: ::windows_sys::core::PCWSTR, tagfile: ::windows_sys::core::PCWSTR, diskpromptstyle: u32, pathbuffer: ::windows_sys::core::PWSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPromptReboot(filequeue: *const ::core::ffi::c_void, owner: super::super::Foundation::HWND, scanonly: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryDrivesInDiskSpaceListA(diskspace: *const ::core::ffi::c_void, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryDrivesInDiskSpaceListW(diskspace: *const ::core::ffi::c_void, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryFileLogA(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows_sys::core::PCSTR, targetfilename: ::windows_sys::core::PCSTR, desiredinfo: SetupFileLogInfo, dataout: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryFileLogW(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows_sys::core::PCWSTR, targetfilename: ::windows_sys::core::PCWSTR, desiredinfo: SetupFileLogInfo, dataout: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupQueryInfOriginalFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupQueryInfOriginalFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfVersionInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: ::windows_sys::core::PCSTR, returnbuffer: ::windows_sys::core::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfVersionInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: ::windows_sys::core::PCWSTR, returnbuffer: ::windows_sys::core::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySourceListA(flags: u32, list: *mut *mut ::windows_sys::core::PSTR, count: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySourceListW(flags: u32, list: *mut *mut ::windows_sys::core::PWSTR, count: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySpaceRequiredOnDriveA(diskspace: *const ::core::ffi::c_void, drivespec: ::windows_sys::core::PCSTR, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySpaceRequiredOnDriveW(diskspace: *const ::core::ffi::c_void, drivespec: ::windows_sys::core::PCWSTR, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyA(queuehandle: *const ::core::ffi::c_void, sourcerootpath: ::windows_sys::core::PCSTR, sourcepath: ::windows_sys::core::PCSTR, sourcefilename: ::windows_sys::core::PCSTR, sourcedescription: ::windows_sys::core::PCSTR, sourcetagfile: ::windows_sys::core::PCSTR, targetdirectory: ::windows_sys::core::PCSTR, targetfilename: ::windows_sys::core::PCSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyIndirectA(copyparams: *const SP_FILE_COPY_PARAMS_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyIndirectW(copyparams: *const SP_FILE_COPY_PARAMS_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopySectionA(queuehandle: *const ::core::ffi::c_void, sourcerootpath: ::windows_sys::core::PCSTR, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopySectionW(queuehandle: *const ::core::ffi::c_void, sourcerootpath: ::windows_sys::core::PCWSTR, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyW(queuehandle: *const ::core::ffi::c_void, sourcerootpath: ::windows_sys::core::PCWSTR, sourcepath: ::windows_sys::core::PCWSTR, sourcefilename: ::windows_sys::core::PCWSTR, sourcedescription: ::windows_sys::core::PCWSTR, sourcetagfile: ::windows_sys::core::PCWSTR, targetdirectory: ::windows_sys::core::PCWSTR, targetfilename: ::windows_sys::core::PCWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDefaultCopyA(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: ::windows_sys::core::PCSTR, sourcefilename: ::windows_sys::core::PCSTR, targetfilename: ::windows_sys::core::PCSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDefaultCopyW(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: ::windows_sys::core::PCWSTR, sourcefilename: ::windows_sys::core::PCWSTR, targetfilename: ::windows_sys::core::PCWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteA(queuehandle: *const ::core::ffi::c_void, pathpart1: ::windows_sys::core::PCSTR, pathpart2: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteSectionA(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteSectionW(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteW(queuehandle: *const ::core::ffi::c_void, pathpart1: ::windows_sys::core::PCWSTR, pathpart2: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameA(queuehandle: *const ::core::ffi::c_void, sourcepath: ::windows_sys::core::PCSTR, sourcefilename: ::windows_sys::core::PCSTR, targetpath: ::windows_sys::core::PCSTR, targetfilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameSectionA(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameSectionW(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameW(queuehandle: *const ::core::ffi::c_void, sourcepath: ::windows_sys::core::PCWSTR, sourcefilename: ::windows_sys::core::PCWSTR, targetpath: ::windows_sys::core::PCWSTR, targetfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFileLogEntryA(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows_sys::core::PCSTR, targetfilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFileLogEntryW(fileloghandle: *const ::core::ffi::c_void, logsectionname: ::windows_sys::core::PCWSTR, targetfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromDiskSpaceListA(diskspace: *const ::core::ffi::c_void, targetfilespec: ::windows_sys::core::PCSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromDiskSpaceListW(diskspace: *const ::core::ffi::c_void, targetfilespec: ::windows_sys::core::PCWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromSourceListA(flags: u32, source: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromSourceListW(flags: u32, source: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveInstallSectionFromDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveInstallSectionFromDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCWSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveSectionFromDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveSectionFromDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: ::windows_sys::core::PCWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRenameErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCSTR, sourcefile: ::windows_sys::core::PCSTR, targetfile: ::windows_sys::core::PCSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRenameErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: ::windows_sys::core::PCWSTR, sourcefile: ::windows_sys::core::PCWSTR, targetfile: ::windows_sys::core::PCWSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupScanFileQueueA(filequeue: *const ::core::ffi::c_void, flags: u32, window: super::super::Foundation::HWND, callbackroutine: PSP_FILE_CALLBACK_A, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupScanFileQueueW(filequeue: *const ::core::ffi::c_void, flags: u32, window: super::super::Foundation::HWND, callbackroutine: PSP_FILE_CALLBACK_W, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdA(infhandle: *const ::core::ffi::c_void, id: u32, directory: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdExA(infhandle: *const ::core::ffi::c_void, id: u32, directory: ::windows_sys::core::PCSTR, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdExW(infhandle: *const ::core::ffi::c_void, id: u32, directory: ::windows_sys::core::PCWSTR, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdW(infhandle: *const ::core::ffi::c_void, id: u32, directory: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupSetFileQueueAlternatePlatformA(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupSetFileQueueAlternatePlatformW(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetFileQueueFlags(filequeue: *const ::core::ffi::c_void, flagmask: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetNonInteractiveMode(noninteractiveflag: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetPlatformPathOverrideA(r#override: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetPlatformPathOverrideW(r#override: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetSourceListA(flags: u32, sourcelist: *const ::windows_sys::core::PSTR, sourcecount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetSourceListW(flags: u32, sourcelist: *const ::windows_sys::core::PWSTR, sourcecount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupSetThreadLogToken(logtoken: u64);
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupTermDefaultQueueCallback(context: *const ::core::ffi::c_void);
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupTerminateFileLog(fileloghandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupUninstallNewlyCopiedInfs(filequeue: *const ::core::ffi::c_void, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupUninstallOEMInfA(inffilename: ::windows_sys::core::PCSTR, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupUninstallOEMInfW(inffilename: ::windows_sys::core::PCWSTR, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupVerifyInfFileA(infname: ::windows_sys::core::PCSTR, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupVerifyInfFileW(infname: ::windows_sys::core::PCWSTR, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupWriteTextLog(logtoken: u64, category: u32, flags: u32, messagestr: ::windows_sys::core::PCSTR);
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupWriteTextLogError(logtoken: u64, category: u32, logflags: u32, error: u32, messagestr: ::windows_sys::core::PCSTR);
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
    pub fn SetupWriteTextLogInfLine(logtoken: u64, flags: u32, infhandle: *const ::core::ffi::c_void, context: *const INFCONTEXT);
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDriverForPlugAndPlayDevicesA(hwndparent: super::super::Foundation::HWND, hardwareid: ::windows_sys::core::PCSTR, fullinfpath: ::windows_sys::core::PCSTR, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDriverForPlugAndPlayDevicesW(hwndparent: super::super::Foundation::HWND, hardwareid: ::windows_sys::core::PCWSTR, fullinfpath: ::windows_sys::core::PCWSTR, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ALLOC_LOG_CONF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const BASIC_LOG_CONF: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const BOOT_LOG_CONF: u32 = 3u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct BUSNUMBER_DES {
    pub BUSD_Count: u32,
    pub BUSD_Type: u32,
    pub BUSD_Flags: u32,
    pub BUSD_Alloc_Base: u32,
    pub BUSD_Alloc_End: u32,
}
impl ::core::marker::Copy for BUSNUMBER_DES {}
impl ::core::clone::Clone for BUSNUMBER_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct BUSNUMBER_RANGE {
    pub BUSR_Min: u32,
    pub BUSR_Max: u32,
    pub BUSR_nBusNumbers: u32,
    pub BUSR_Flags: u32,
}
impl ::core::marker::Copy for BUSNUMBER_RANGE {}
impl ::core::clone::Clone for BUSNUMBER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct BUSNUMBER_RESOURCE {
    pub BusNumber_Header: BUSNUMBER_DES,
    pub BusNumber_Data: [BUSNUMBER_RANGE; 1],
}
impl ::core::marker::Copy for BUSNUMBER_RESOURCE {}
impl ::core::clone::Clone for BUSNUMBER_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct CABINET_INFO_A {
    pub CabinetPath: ::windows_sys::core::PCSTR,
    pub CabinetFile: ::windows_sys::core::PCSTR,
    pub DiskName: ::windows_sys::core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for CABINET_INFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct CABINET_INFO_A {
    pub CabinetPath: ::windows_sys::core::PCSTR,
    pub CabinetFile: ::windows_sys::core::PCSTR,
    pub DiskName: ::windows_sys::core::PCSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for CABINET_INFO_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct CABINET_INFO_W {
    pub CabinetPath: ::windows_sys::core::PCWSTR,
    pub CabinetFile: ::windows_sys::core::PCWSTR,
    pub DiskName: ::windows_sys::core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for CABINET_INFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct CABINET_INFO_W {
    pub CabinetPath: ::windows_sys::core::PCWSTR,
    pub CabinetFile: ::windows_sys::core::PCWSTR,
    pub DiskName: ::windows_sys::core::PCWSTR,
    pub SetId: u16,
    pub CabinetNumber: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for CABINET_INFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_ID_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_ID_COMPATIBLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_ID_HARDWARE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_RANGE_ADDIFCONFLICT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_RANGE_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ADD_RANGE_DONOTADDIFCONFLICT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDFLAGS_DRIVER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDFLAGS_RESERVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDFLAGS_ROOT_OWNED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_DESCRIPTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_DEVINST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_FLAGS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_RESDES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CDMASK_VALID: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CLASS_PROPERTY_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CLASS_PROPERTY_INSTALLER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CLASS_PROPERTY_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_DO_NOT_INSTALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_GENERATE_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_NO_WAIT_INSTALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVINST_PHANTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_DO_NOT_INSTALL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_GENERATE_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_NO_WAIT_INSTALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CREATE_DEVNODE_PHANTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_CHARACTERISTICS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_DEVTYPE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_EXCLUSIVE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_LOWERFILTERS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_MAX: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_SECURITY: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_SECURITY_SDS: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CRP_UPPERFILTERS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CUSTOMDEVPROP_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_CUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DELETE_CLASS_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DELETE_CLASS_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DELETE_CLASS_ONLY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DELETE_CLASS_SUBKEYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_BITS: u32 = 2147483655u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_CRASHED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_HWPROF_FIRST_BOOT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_NEW_PROFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DETECT_RUN: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_DOCKDEVICE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_EJECTSUPPORTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_HARDWAREDISABLED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_LOCKSUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_NONDYNAMIC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_RAWDEVICEOK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_REMOVABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_SECUREDEVICE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_SILENTINSTALL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_SURPRISEREMOVALOK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVCAP_UNIQUEID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_BOTTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_LEFT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_RIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_TOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_EDGE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_HINGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_PIVOT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_PLANAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_SWIVEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_JOINT_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_ORIENTATION_HORIZONTAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_ORIENTATION_VERTICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SHAPE_OVAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SHAPE_RECTANGLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SHAPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_BACK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_BOTTOM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_FRONT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_LEFT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_RIGHT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_TOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DEVICE_PANEL_SIDE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_ABSOLUTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_HARDWARE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_PERSIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_POLITE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DISABLE_UI_NOT_OK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_ADDRESS: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_BASE_CONTAINERID: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_BUSNUMBER: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_BUSTYPEGUID: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CAPABILITIES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CHARACTERISTICS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CLASS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CLASSGUID: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_COMPATIBLEIDS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_CONFIGFLAGS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_DEVICEDESC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_DEVICE_POWER_DATA: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_DEVTYPE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_DRIVER: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_ENUMERATOR_NAME: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_EXCLUSIVE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_FRIENDLYNAME: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_HARDWAREID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_INSTALL_STATE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_LEGACYBUSTYPE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_LOCATION_INFORMATION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_LOCATION_PATHS: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_LOWERFILTERS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_MAX: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_MFG: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_MIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_PHYSICAL_DEVICE_OBJECT_NAME: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_REMOVAL_POLICY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_REMOVAL_POLICY_HW_DEFAULT: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_REMOVAL_POLICY_OVERRIDE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_SECURITY: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_SECURITY_SDS: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_SERVICE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UI_NUMBER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UI_NUMBER_DESC_FORMAT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UNUSED0: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UNUSED1: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UNUSED2: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_DRP_UPPERFILTERS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ENUMERATE_CLASSES_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ENUMERATE_CLASSES_INSTALLER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_ENUMERATE_CLASSES_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_DONOTGENERATE: u32 = 268435520u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_BITS: u32 = 268435583u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_BUSRELATIONS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_CLASS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_EJECTRELATIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_ENUMERATOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_POWERRELATIONS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_PRESENT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_REMOVALRELATIONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_SERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GETIDLIST_FILTER_TRANSPORTRELATIONS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GET_DEVICE_INTERFACE_LIST_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GET_DEVICE_INTERFACE_LIST_PRESENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_CAN_DO_UI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_DETECTION_PENDING: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_ON_BIG_STACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_REBOOT_REQUIRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_SERVICES_AVAILABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_GLOBAL_STATE_SHUTTING_DOWN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_HWPI_DOCKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_HWPI_NOT_DOCKABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_HWPI_UNDOCKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_INSTALL_STATE_FAILED_INSTALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_INSTALL_STATE_FINISH_INSTALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_INSTALL_STATE_INSTALLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_INSTALL_STATE_NEEDS_REINSTALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_CANCELREMOVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_NOVALIDATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVINST_PHANTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_CANCELREMOVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_NOVALIDATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_LOCATE_DEVNODE_PHANTOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NAME_ATTRIBUTE_NAME_RETRIEVED_FROM_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NAME_ATTRIBUTE_USER_ASSIGNED_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type CM_NOTIFY_ACTION = i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEARRIVAL: CM_NOTIFY_ACTION = 0i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINTERFACEREMOVAL: CM_NOTIFY_ACTION = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVE: CM_NOTIFY_ACTION = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEQUERYREMOVEFAILED: CM_NOTIFY_ACTION = 3i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEREMOVEPENDING: CM_NOTIFY_ACTION = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEREMOVECOMPLETE: CM_NOTIFY_ACTION = 5i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICECUSTOMEVENT: CM_NOTIFY_ACTION = 6i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEENUMERATED: CM_NOTIFY_ACTION = 7i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINSTANCESTARTED: CM_NOTIFY_ACTION = 8i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_DEVICEINSTANCEREMOVED: CM_NOTIFY_ACTION = 9i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_ACTION_MAX: CM_NOTIFY_ACTION = 10i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CM_NOTIFY_EVENT_DATA {
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_EVENT_DATA_0,
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub union CM_NOTIFY_EVENT_DATA_0 {
    pub DeviceInterface: CM_NOTIFY_EVENT_DATA_0_2,
    pub DeviceHandle: CM_NOTIFY_EVENT_DATA_0_0,
    pub DeviceInstance: CM_NOTIFY_EVENT_DATA_0_1,
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CM_NOTIFY_EVENT_DATA_0_0 {
    pub EventGuid: ::windows_sys::core::GUID,
    pub NameOffset: i32,
    pub DataSize: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0_0 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CM_NOTIFY_EVENT_DATA_0_1 {
    pub InstanceId: [u16; 1],
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0_1 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CM_NOTIFY_EVENT_DATA_0_2 {
    pub ClassGuid: ::windows_sys::core::GUID,
    pub SymbolicLink: [u16; 1],
}
impl ::core::marker::Copy for CM_NOTIFY_EVENT_DATA_0_2 {}
impl ::core::clone::Clone for CM_NOTIFY_EVENT_DATA_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER {
    pub cbSize: u32,
    pub Flags: u32,
    pub FilterType: CM_NOTIFY_FILTER_TYPE,
    pub Reserved: u32,
    pub u: CM_NOTIFY_FILTER_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union CM_NOTIFY_FILTER_0 {
    pub DeviceInterface: CM_NOTIFY_FILTER_0_2,
    pub DeviceHandle: CM_NOTIFY_FILTER_0_0,
    pub DeviceInstance: CM_NOTIFY_FILTER_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER_0_0 {
    pub hTarget: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER_0_1 {
    pub InstanceId: [u16; 200],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CM_NOTIFY_FILTER_0_2 {
    pub ClassGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CM_NOTIFY_FILTER_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CM_NOTIFY_FILTER_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_FLAG_ALL_DEVICE_INSTANCES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_FLAG_ALL_INTERFACE_CLASSES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type CM_NOTIFY_FILTER_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINTERFACE: CM_NOTIFY_FILTER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_TYPE_DEVICEHANDLE: CM_NOTIFY_FILTER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_TYPE_DEVICEINSTANCE: CM_NOTIFY_FILTER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_NOTIFY_FILTER_TYPE_MAX: CM_NOTIFY_FILTER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_OPEN_CLASS_KEY_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_OPEN_CLASS_KEY_INSTALLER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_OPEN_CLASS_KEY_INTERFACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_BIOS_TABLE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_BOOT_CONFIG_CONFLICT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_CANT_SHARE_IRQ: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_CONSOLE_LOCKED: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVICE_NOT_THERE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVICE_RESET: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVLOADER_FAILED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVLOADER_NOT_FOUND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DEVLOADER_NOT_READY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DISABLED: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DISABLED_SERVICE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DRIVER_BLOCKED: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DRIVER_FAILED_LOAD: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DRIVER_FAILED_PRIOR_UNLOAD: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DRIVER_SERVICE_KEY_INVALID: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_DUPLICATE_DEVICE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_ENTRY_IS_WRONG_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_ADD: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_DRIVER_ENTRY: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_FILTER: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_INSTALL: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_POST_START: u32 = 43u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_FAILED_START: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_GUEST_ASSIGNMENT_FAILED: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_HALTED: u32 = 44u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_HARDWARE_DISABLED: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_HELD_FOR_EJECT: u32 = 47u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_INVALID_DATA: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_IRQ_TRANSLATION_FAILED: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_LACKED_ARBITRATOR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_LEGACY_SERVICE_NO_DEVICES: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_LIAR: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_MOVED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NEED_CLASS_CONFIG: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NEED_RESTART: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NORMAL_CONFLICT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NOT_CONFIGURED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NOT_VERIFIED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NO_SOFTCONFIG: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_NO_VALID_LOG_CONF: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_OUT_OF_MEMORY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_PARTIAL_LOG_CONF: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_PHANTOM: u32 = 45u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_REENUMERATION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_REGISTRY: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_REGISTRY_TOO_LARGE: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_REINSTALL: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_SETPROPERTIES_FAILED: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_SYSTEM_SHUTDOWN: u32 = 46u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_TOO_EARLY: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_TRANSLATION_FAILED: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_UNKNOWN_RESOURCE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_UNSIGNED_DRIVER: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_USED_BY_DEBUGGER: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_VXDLDR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_WAITING_ON_DEPENDENCY: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_PROB_WILL_BE_REMOVED: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_ARBITRATOR_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_ARBITRATOR_RAW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_ARBITRATOR_TRANSLATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_REMOVE_UI_NOT_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_QUERY_REMOVE_UI_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_ASYNCHRONOUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_RETRY_INSTALLATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REENUMERATE_SYNCHRONOUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTER_DEVICE_DRIVER_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTER_DEVICE_DRIVER_DISABLEABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTER_DEVICE_DRIVER_REMOVABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTER_DEVICE_DRIVER_STATIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_BITS: u32 = 769u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_CONFIG: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_HARDWARE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_SOFTWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REGISTRY_USER: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVAL_POLICY_EXPECT_NO_REMOVAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVAL_POLICY_EXPECT_ORDERLY_REMOVAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVAL_POLICY_EXPECT_SURPRISE_REMOVAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_DISABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_NO_RESTART: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_UI_NOT_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_REMOVE_UI_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_RESDES_WIDTH_32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_RESDES_WIDTH_64: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_RESDES_WIDTH_BITS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_RESDES_WIDTH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_CONFIG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_CONFIG_CLASS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_CONFIG_EXTENSIONS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_CONFIG_RESET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_READY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVINST_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_CONFIG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_CONFIG_CLASS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_CONFIG_EXTENSIONS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_CONFIG_RESET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_READY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DEVNODE_RESET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_DOWNLOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_PROP_CHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SETUP_WRITE_LOG_CONFS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVINST_PROBLEM_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVINST_PROBLEM_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVINST_PROBLEM_OVERRIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVNODE_PROBLEM_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVNODE_PROBLEM_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_DEVNODE_PROBLEM_OVERRIDE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_HW_PROF_FLAGS_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CM_SET_HW_PROF_FLAGS_UI_NOT_OK: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: super::super::Foundation::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COINSTALLER_CONTEXT_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COINSTALLER_CONTEXT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct COINSTALLER_CONTEXT_DATA {
    pub PostProcessing: super::super::Foundation::BOOL,
    pub InstallResult: u32,
    pub PrivateData: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COINSTALLER_CONTEXT_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COINSTALLER_CONTEXT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CONFIGMG_VERSION: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type CONFIGRET = u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_SUCCESS: CONFIGRET = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEFAULT: CONFIGRET = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_OUT_OF_MEMORY: CONFIGRET = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_POINTER: CONFIGRET = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_FLAG: CONFIGRET = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_DEVNODE: CONFIGRET = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_DEVINST: CONFIGRET = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_RES_DES: CONFIGRET = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_LOG_CONF: CONFIGRET = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_ARBITRATOR: CONFIGRET = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_NODELIST: CONFIGRET = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVNODE_HAS_REQS: CONFIGRET = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVINST_HAS_REQS: CONFIGRET = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_RESOURCEID: CONFIGRET = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DLVXD_NOT_FOUND: CONFIGRET = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_DEVNODE: CONFIGRET = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_DEVINST: CONFIGRET = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_MORE_LOG_CONF: CONFIGRET = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_MORE_RES_DES: CONFIGRET = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_ALREADY_SUCH_DEVNODE: CONFIGRET = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_ALREADY_SUCH_DEVINST: CONFIGRET = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_RANGE_LIST: CONFIGRET = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_RANGE: CONFIGRET = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_FAILURE: CONFIGRET = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_LOGICAL_DEV: CONFIGRET = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_CREATE_BLOCKED: CONFIGRET = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NOT_SYSTEM_VM: CONFIGRET = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_REMOVE_VETOED: CONFIGRET = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_APM_VETOED: CONFIGRET = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_LOAD_TYPE: CONFIGRET = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_BUFFER_SMALL: CONFIGRET = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_ARBITRATOR: CONFIGRET = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_REGISTRY_HANDLE: CONFIGRET = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_REGISTRY_ERROR: CONFIGRET = 29u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_DEVICE_ID: CONFIGRET = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_DATA: CONFIGRET = 31u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_API: CONFIGRET = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVLOADER_NOT_READY: CONFIGRET = 33u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NEED_RESTART: CONFIGRET = 34u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_MORE_HW_PROFILES: CONFIGRET = 35u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVICE_NOT_THERE: CONFIGRET = 36u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_VALUE: CONFIGRET = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_WRONG_TYPE: CONFIGRET = 38u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_PRIORITY: CONFIGRET = 39u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NOT_DISABLEABLE: CONFIGRET = 40u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_FREE_RESOURCES: CONFIGRET = 41u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_QUERY_VETOED: CONFIGRET = 42u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_CANT_SHARE_IRQ: CONFIGRET = 43u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_DEPENDENT: CONFIGRET = 44u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_SAME_RESOURCES: CONFIGRET = 45u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_REGISTRY_KEY: CONFIGRET = 46u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_MACHINENAME: CONFIGRET = 47u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_REMOTE_COMM_FAILURE: CONFIGRET = 48u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_MACHINE_UNAVAILABLE: CONFIGRET = 49u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_CM_SERVICES: CONFIGRET = 50u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_ACCESS_DENIED: CONFIGRET = 51u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_CALL_NOT_IMPLEMENTED: CONFIGRET = 52u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_PROPERTY: CONFIGRET = 53u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_DEVICE_INTERFACE_ACTIVE: CONFIGRET = 54u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_NO_SUCH_DEVICE_INTERFACE: CONFIGRET = 55u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_REFERENCE_STRING: CONFIGRET = 56u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_CONFLICT_LIST: CONFIGRET = 57u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_INDEX: CONFIGRET = 58u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const CR_INVALID_STRUCTURE_SIZE: CONFIGRET = 59u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CR_RESULTS: CONFIGRET = 60u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONFLICT_DETAILS_A {
    pub CD_ulSize: u32,
    pub CD_ulMask: u32,
    pub CD_dnDevInst: u32,
    pub CD_rdResDes: usize,
    pub CD_ulFlags: u32,
    pub CD_szDescription: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONFLICT_DETAILS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONFLICT_DETAILS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CONFLICT_DETAILS_W {
    pub CD_ulSize: u32,
    pub CD_ulMask: u32,
    pub CD_dnDevInst: u32,
    pub CD_rdResDes: usize,
    pub CD_ulFlags: u32,
    pub CD_szDescription: [u16; 260],
}
impl ::core::marker::Copy for CONFLICT_DETAILS_W {}
impl ::core::clone::Clone for CONFLICT_DETAILS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_FORCE_FILE_IN_USE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_IN_USE_TRY_RENAME: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NODECOMP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NOPRUNE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NOSKIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NOVERSIONCHECK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NO_OVERWRITE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_NO_VERSION_DIALOG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_OVERWRITE_OLDER_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_PROTECTED_WINDOWS_DRIVER_FILE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_REPLACEONLY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_REPLACE_BOOT_FILE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const COPYFLG_WARN_IF_SKIP: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CS_DES {
    pub CSD_SignatureLength: u32,
    pub CSD_LegacyDataOffset: u32,
    pub CSD_LegacyDataSize: u32,
    pub CSD_Flags: u32,
    pub CSD_ClassGuid: ::windows_sys::core::GUID,
    pub CSD_Signature: [u8; 1],
}
impl ::core::marker::Copy for CS_DES {}
impl ::core::clone::Clone for CS_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct CS_RESOURCE {
    pub CS_Header: CS_DES,
}
impl ::core::marker::Copy for CS_RESOURCE {}
impl ::core::clone::Clone for CS_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Connection_Des_s {
    pub COND_Type: u32,
    pub COND_Flags: u32,
    pub COND_Class: u8,
    pub COND_ClassType: u8,
    pub COND_Reserved1: u8,
    pub COND_Reserved2: u8,
    pub COND_Id: i64,
}
impl ::core::marker::Copy for Connection_Des_s {}
impl ::core::clone::Clone for Connection_Des_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Connection_Resource_s {
    pub Connection_Header: Connection_Des_s,
}
impl ::core::marker::Copy for Connection_Resource_s {}
impl ::core::clone::Clone for Connection_Resource_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DELFLG_IN_USE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DELFLG_IN_USE1: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIBCI_NODISPLAYCLASS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIBCI_NOINSTALLCLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICD_GENERATE_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICD_INHERIT_CLASSDRVS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICLASSPROP_INSTALLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICLASSPROP_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_FLAG_CONFIGGENERAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_FLAG_CONFIGSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_FLAG_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_PROPCHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_START: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICS_STOP: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DICUSTOMDEVPROP_MERGE_MULTISZ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ADDPROPERTYPAGE_ADVANCED: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ADDPROPERTYPAGE_BASIC: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ADDREMOTEPROPERTYPAGE_ADVANCED: u32 = 40u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ALLOW_INSTALL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ASSIGNRESOURCES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_CALCDISKSPACE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DESTROYPRIVATEDATA: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DESTROYWIZARDDATA: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DETECT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DETECTCANCEL: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_DETECTVERIFY: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_ENABLECLASS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_FINISHINSTALL_ACTION: u32 = 42u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_FIRSTTIMESETUP: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_FOUNDDEVICE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLCLASSDRIVERS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLDEVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLDEVICEFILES: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLINTERFACES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_INSTALLWIZARD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_MOVEDEVICE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_FINISHINSTALL: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_POSTANALYZE: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_PREANALYZE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_PRESELECT: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_NEWDEVICEWIZARD_SELECT: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_POWERMESSAGEWAKE: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_PROPERTIES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_PROPERTYCHANGE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_REGISTERDEVICE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_REGISTER_COINSTALLERS: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_REMOVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_RESERVED1: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_RESERVED2: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_SELECTBESTCOMPATDRV: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_SELECTCLASSDRIVERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_SELECTDEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_TROUBLESHOOTER: u32 = 38u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_UNREMOVE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_UNUSED1: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_UPDATEDRIVER_UI: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_VALIDATECLASSDRIVERS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIF_VALIDATEDRIVER: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCDP_FLAG_ADVANCED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCDP_FLAG_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCDP_FLAG_REMOTE_ADVANCED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCDP_FLAG_REMOTE_BASIC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_ALLCLASSES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_DEVICEINTERFACE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_INTERFACEDEVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_PRESENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIGCF_PROFILE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_BITS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_INSTALLCOPYINFDRIVERS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_INSTALLNULLDRIVER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_NOFINISHINSTALLUI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIDFLAG_SHOWSEARCHUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_FORCE_INF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_HOTPATCH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_HW_USING_THE_INF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_INF_ALREADY_COPIED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_INSTALL_AS_SET: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_NOBACKUP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIIRFLAG_PRE_CONFIGURE_INF: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIOCR_INSTALLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIOCR_INTERFACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIODI_NO_ADD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIOD_CANCEL_REMOVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIOD_INHERIT_CLASSDRVS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIREG_BOTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIREG_DEV: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIREG_DRV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_ABSOLUTE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_ABSOLUTE_16BIT: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_APPS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_BOOT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COLOR: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_APPDATA: u32 = 16419u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_DESKTOPDIRECTORY: u32 = 16409u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_DOCUMENTS: u32 = 16430u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_FAVORITES: u32 = 16415u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_PROGRAMS: u32 = 16407u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_STARTMENU: u32 = 16406u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_STARTUP: u32 = 16408u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_COMMON_TEMPLATES: u32 = 16429u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_DEFAULT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_DRIVERS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_DRIVER_STORE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_FONTS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_HELP: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_INF: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_IOSUBSYS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_LOADER: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PRINTPROCESSOR: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PROGRAM_FILES: u32 = 16422u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PROGRAM_FILES_COMMON: u32 = 16427u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PROGRAM_FILES_COMMONX86: u32 = 16428u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_PROGRAM_FILES_X86: u32 = 16426u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SHARED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SPOOL: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SPOOLDRIVERS: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SRCPATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SYSTEM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SYSTEM16: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_SYSTEM_X86: u32 = 16425u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_USER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_USERPROFILE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_VIEWERS: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIRID_WINDOWS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIURFLAG_NO_REMOVE_INF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DIURFLAG_RESERVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_AUTOASSIGNRES: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_CLASSINSTALLPARAMS: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_COMPAT_FROM_CLASS: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DIDCLASS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DIDCOMPAT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DISABLED: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DONOTCALLCONFIGMG: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_DRIVERPAGE_ADDED: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_ENUMSINGLEINF: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_ALLOWEXCLUDEDDRVS: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_ALTPLATFORM_DRVSEARCH: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_ALWAYSWRITEIDS: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_APPENDDRIVERLIST: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_BACKUPONREPLACE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_CI_FAILED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_DEVICECHANGE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_DIDCOMPATINFO: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_DIDINFOLIST: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_DRIVERLIST_FROM_URL: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_EXCLUDE_OLD_INET_DRIVERS: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_FILTERCLASSES: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_FILTERSIMILARDRIVERS: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_FINISHINSTALL_ACTION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_INET_DRIVER: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_INSTALLEDDRIVER: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_IN_SYSTEM_SETUP: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_NOUIONQUERYREMOVE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_NO_CLASSLIST_NODE_MERGE: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_NO_DRVREG_MODIFY: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_POWERPAGE_ADDED: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_PREINSTALLBACKUP: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_PROPCHANGE_PENDING: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RECURSIVESEARCH: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESERVED1: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESERVED2: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESERVED3: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESERVED4: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_RESTART_DEVICE_ONLY: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_SEARCH_PUBLISHED_INFS: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_SETFAILEDINSTALL: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FLAGSEX_USECLASSFORCOMPAT: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_FORCECOPY: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_GENERALPAGE_ADDED: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_INF_IS_SORTED: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_INSTALLDISABLED: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_MULTMFGS: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NEEDREBOOT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NEEDRESTART: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOBROWSE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NODI_DEFAULTACTION: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOFILECOPY: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOSELECTICONS: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOVCP: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_NOWRITE_IDS: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_OVERRIDE_INFFLAGS: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_PROPERTIES_CHANGE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_PROPS_NOCHANGEUSAGE: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_QUIETINSTALL: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_REMOVEDEVICE_CONFIGSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_REMOVEDEVICE_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_RESOURCEPAGE_ADDED: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_SHOWALL: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_SHOWCLASS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_SHOWCOMPAT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_SHOWOEM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_UNREMOVEDEVICE_CONFIGSPECIFIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DI_USECI_SELECTSTRINGS: i32 = 134217728i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DMA_DES {
    pub DD_Count: u32,
    pub DD_Type: u32,
    pub DD_Flags: u32,
    pub DD_Alloc_Chan: u32,
}
impl ::core::marker::Copy for DMA_DES {}
impl ::core::clone::Clone for DMA_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DMA_RANGE {
    pub DR_Min: u32,
    pub DR_Max: u32,
    pub DR_Flags: u32,
}
impl ::core::marker::Copy for DMA_RANGE {}
impl ::core::clone::Clone for DMA_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DMA_RESOURCE {
    pub DMA_Header: DMA_DES,
    pub DMA_Data: [DMA_RANGE; 1],
}
impl ::core::marker::Copy for DMA_RESOURCE {}
impl ::core::clone::Clone for DMA_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DMI_BKCOLOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DMI_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DMI_USERECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_ALWAYSEXCLUDEFROMLIST: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_AUTHENTICODE_SIGNED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_BAD_DRIVER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_BASIC_DRIVER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_CLASS_DRIVER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_COMPATIBLE_DRIVER: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_DUPDESC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_DUPDRIVERVER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_DUPPROVIDER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_EXCLUDEFROMLIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_INBOX_DRIVER: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_INET_DRIVER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_INF_IS_SIGNED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_INSTALLEDDRIVER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_LEGACYINF: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_NODRIVER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_OEM_F6_INF: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_OLDDRIVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_OLD_INET_DRIVER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_REQUESTADDITIONALSOFTWARE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED1: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED2: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_22: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_23: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_24: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_25: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_26: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_27: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_28: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_29: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_30: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DNF_UNUSED_31: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_APM_DRIVER: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_APM_ENUMERATOR: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_ARM_WAKEUP: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_BAD_PARTIAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_BOOT_LOG_PROB: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_CHILD_WITH_INVALID_ID: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_DEVICE_DISCONNECTED: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_DISABLEABLE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_DRIVER_BLOCKED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_DRIVER_LOADED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_ENUM_LOADED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_FILTERED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_HARDWARE_ENUM: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_HAS_MARK: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_HAS_PROBLEM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_LEGACY_DRIVER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_LIAR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_MANUAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_MF_CHILD: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_MF_PARENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_MOVED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NEEDS_LOCKING: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NEED_RESTART: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NEED_TO_ENUM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NOT_FIRST_TIME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NOT_FIRST_TIMEE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NO_SHOW_IN_DM: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NT_DRIVER: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_NT_ENUMERATOR: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_PRIVATE_PROBLEM: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_QUERY_REMOVE_ACTIVE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_QUERY_REMOVE_PENDING: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_REBAL_CANDIDATE: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_REMOVABLE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_ROOT_ENUMERATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_SILENT_INSTALL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_STARTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_STOP_FREE_RES: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DN_WILL_BE_REMOVED: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_BUFFERTOOSMALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_CANCEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_OUTOFMEMORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_SKIPFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DPROMPT_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_COMPATID_RANK: u32 = 16383u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_HARDWAREID_MASK: u32 = 2147487743u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_HARDWAREID_RANK: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_UNTRUSTED_COMPATID_RANK: u32 = 49151u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_UNTRUSTED_HARDWAREID_RANK: u32 = 36863u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_UNTRUSTED_RANK: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_W9X_SUSPECT_COMPATID_RANK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_W9X_SUSPECT_HARDWAREID_RANK: u32 = 53247u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DRIVER_W9X_SUSPECT_RANK: u32 = 3221225472u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DWORD_MAX: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DYNAWIZ_FLAG_ANALYZE_HANDLECONFLICT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DYNAWIZ_FLAG_INSTALLDET_NEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DYNAWIZ_FLAG_INSTALLDET_PREV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const DYNAWIZ_FLAG_PAGESADDED: u32 = 1u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DevPrivate_Des_s {
    pub PD_Count: u32,
    pub PD_Type: u32,
    pub PD_Data1: u32,
    pub PD_Data2: u32,
    pub PD_Data3: u32,
    pub PD_Flags: u32,
}
impl ::core::marker::Copy for DevPrivate_Des_s {}
impl ::core::clone::Clone for DevPrivate_Des_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DevPrivate_Range_s {
    pub PR_Data1: u32,
    pub PR_Data2: u32,
    pub PR_Data3: u32,
}
impl ::core::marker::Copy for DevPrivate_Range_s {}
impl ::core::clone::Clone for DevPrivate_Range_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct DevPrivate_Resource_s {
    pub PRV_Header: DevPrivate_Des_s,
    pub PRV_Data: [DevPrivate_Range_s; 1],
}
impl ::core::marker::Copy for DevPrivate_Resource_s {}
impl ::core::clone::Clone for DevPrivate_Resource_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ENABLECLASS_FAILURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ENABLECLASS_QUERY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ENABLECLASS_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_ABORT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_BACKUP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_DOIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_NEWPATH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_RENAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_RETRY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_SKIP: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILEPATHS_A {
    pub Target: ::windows_sys::core::PCSTR,
    pub Source: ::windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILEPATHS_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILEPATHS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILEPATHS_A {
    pub Target: ::windows_sys::core::PCSTR,
    pub Source: ::windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILEPATHS_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILEPATHS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: ::windows_sys::core::PCSTR,
    pub Source: ::windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: ::windows_sys::core::PCSTR,
    pub Version: ::windows_sys::core::PCSTR,
    pub CatalogFile: ::windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILEPATHS_SIGNERINFO_A {
    pub Target: ::windows_sys::core::PCSTR,
    pub Source: ::windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: ::windows_sys::core::PCSTR,
    pub Version: ::windows_sys::core::PCSTR,
    pub CatalogFile: ::windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: ::windows_sys::core::PCWSTR,
    pub Source: ::windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: ::windows_sys::core::PCWSTR,
    pub Version: ::windows_sys::core::PCWSTR,
    pub CatalogFile: ::windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILEPATHS_SIGNERINFO_W {
    pub Target: ::windows_sys::core::PCWSTR,
    pub Source: ::windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
    pub DigitalSigner: ::windows_sys::core::PCWSTR,
    pub Version: ::windows_sys::core::PCWSTR,
    pub CatalogFile: ::windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILEPATHS_SIGNERINFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILEPATHS_SIGNERINFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILEPATHS_W {
    pub Target: ::windows_sys::core::PCWSTR,
    pub Source: ::windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILEPATHS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILEPATHS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILEPATHS_W {
    pub Target: ::windows_sys::core::PCWSTR,
    pub Source: ::windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILEPATHS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILEPATHS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILE_COMPRESSION_MSZIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILE_COMPRESSION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILE_COMPRESSION_NTCAB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILE_COMPRESSION_WINLZA: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: ::windows_sys::core::PCSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct FILE_IN_CABINET_INFO_A {
    pub NameInCabinet: ::windows_sys::core::PCSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: ::windows_sys::core::PCWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct FILE_IN_CABINET_INFO_W {
    pub NameInCabinet: ::windows_sys::core::PCWSTR,
    pub FileSize: u32,
    pub Win32Error: u32,
    pub DosDate: u16,
    pub DosTime: u16,
    pub DosAttribs: u16,
    pub FullTargetName: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for FILE_IN_CABINET_INFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for FILE_IN_CABINET_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILTERED_LOG_CONF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_AND: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_APPEND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_NOCLOBBER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_OR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDPROPERTY_OVERWRITEONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_32BITKEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_64BITKEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_APPEND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_BINVALUETYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_DELREG_BIT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_DELVAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_KEYONLY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_KEYONLY_COMMON: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_NOCLOBBER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_OVERWRITEONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_TYPE_EXPAND_SZ: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_TYPE_MULTI_SZ: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_ADDREG_TYPE_SZ: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_BITREG_32BITKEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_BITREG_64BITKEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_BITREG_CLEARBITS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_BITREG_SETBITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELPROPERTY_MULTI_SZ_DELSTRING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_32BITKEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_64BITKEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_KEYONLY_COMMON: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_OPERATION_MASK: u32 = 254u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_TYPE_EXPAND_SZ: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_TYPE_MULTI_SZ: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_TYPE_SZ: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_DELREG_VALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_INI2REG_32BITKEY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_INI2REG_64BITKEY: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_PROFITEM_CSIDL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_PROFITEM_CURRENTUSER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_PROFITEM_DELETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_PROFITEM_GROUP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_REGSVR_DLLINSTALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FLG_REGSVR_DLLREGISTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FORCED_LOG_CONF: u32 = 4u32;
pub const GUID_ACPI_CMOS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 982320004, data2: 25861, data3: 16586, data4: [188, 57, 86, 193, 95, 140, 95, 237] };
pub const GUID_ACPI_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2962333834, data2: 47767, data3: 4560, data4: [189, 20, 0, 170, 0, 183, 179, 42] };
pub const GUID_ACPI_INTERFACE_STANDARD2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3899219811, data2: 6193, data3: 18544, data4: [168, 207, 156, 47, 3, 249, 220, 181] };
pub const GUID_ACPI_PORT_RANGES_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4048511131, data2: 52157, data3: 18775, data4: [166, 116, 188, 0, 33, 63, 28, 151] };
pub const GUID_ACPI_REGS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 101980518, data2: 29253, data3: 25449, data4: [70, 46, 78, 101, 108, 115, 111, 110] };
pub const GUID_AGP_TARGET_BUS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2975661288, data2: 1745, data3: 19767, data4: [157, 76, 190, 221, 224, 194, 166, 255] };
pub const GUID_ARBITER_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3863277957, data2: 35854, data3: 4560, data4: [190, 207, 8, 0, 43, 226, 9, 47] };
pub const GUID_BUS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1231782528, data2: 28453, data3: 4560, data4: [190, 175, 8, 0, 43, 226, 9, 47] };
pub const GUID_BUS_RESOURCE_UPDATE_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 667947053, data2: 49074, data3: 16740, data4: [129, 221, 219, 184, 47, 150, 139, 72] };
pub const GUID_BUS_TYPE_1394: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4149113835, data2: 39621, data3: 17899, data4: [190, 77, 119, 44, 199, 29, 223, 179] };
pub const GUID_BUS_TYPE_ACPI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3618924693, data2: 26, data3: 18754, data4: [137, 31, 167, 212, 102, 16, 168, 67] };
pub const GUID_BUS_TYPE_AVC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3228562021, data2: 44553, data3: 18672, data4: [129, 44, 22, 117, 61, 124, 186, 131] };
pub const GUID_BUS_TYPE_DOT4PRT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1142874113, data2: 17218, data3: 4565, data4: [161, 132, 0, 192, 79, 96, 82, 77] };
pub const GUID_BUS_TYPE_EISA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3720566025, data2: 62460, data3: 4560, data4: [165, 55, 0, 0, 248, 117, 62, 209] };
pub const GUID_BUS_TYPE_HID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4004460496, data2: 6499, data3: 18372, data4: [170, 72, 114, 71, 109, 183, 207, 73] };
pub const GUID_BUS_TYPE_INTERNAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 355527283, data2: 2155, data3: 4561, data4: [160, 159, 0, 192, 79, 195, 64, 177] };
pub const GUID_BUS_TYPE_IRDA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2061598145, data2: 51524, data3: 17622, data4: [136, 31, 76, 46, 97, 5, 59, 193] };
pub const GUID_BUS_TYPE_ISAPNP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3866556500, data2: 55421, data3: 4560, data4: [146, 178, 0, 160, 201, 5, 95, 197] };
pub const GUID_BUS_TYPE_LPTENUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3301576704, data2: 11740, data3: 4565, data4: [161, 122, 0, 192, 79, 96, 82, 77] };
pub const GUID_BUS_TYPE_MCA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 477469050, data2: 56371, data3: 4560, data4: [146, 178, 0, 160, 201, 5, 95, 197] };
pub const GUID_BUS_TYPE_PCI: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3370901424, data2: 46352, data3: 4560, data4: [128, 229, 0, 160, 201, 37, 66, 227] };
pub const GUID_BUS_TYPE_PCMCIA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 154416688, data2: 44959, data3: 4560, data4: [146, 233, 0, 0, 248, 30, 27, 48] };
pub const GUID_BUS_TYPE_SCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 928667922, data2: 32844, data3: 17834, data4: [189, 194, 253, 210, 90, 29, 149, 18] };
pub const GUID_BUS_TYPE_SD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3875589124, data2: 16438, data3: 20105, data4: [149, 121, 137, 235, 244, 95, 0, 205] };
pub const GUID_BUS_TYPE_SERENUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1997621895, data2: 35140, data3: 4561, data4: [189, 144, 0, 160, 201, 6, 190, 45] };
pub const GUID_BUS_TYPE_SW_DEVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 114361122, data2: 32224, data3: 19695, data4: [142, 37, 25, 125, 14, 116, 66, 226] };
pub const GUID_BUS_TYPE_USB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2642275260, data2: 51293, data3: 4561, data4: [158, 180, 0, 96, 8, 195, 161, 154] };
pub const GUID_BUS_TYPE_USBPRINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1142874112, data2: 17218, data3: 4565, data4: [161, 132, 0, 192, 79, 96, 82, 77] };
pub const GUID_D3COLD_AUX_POWER_AND_TIMING_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4511914, data2: 63076, data3: 17800, data4: [159, 252, 42, 254, 175, 89, 80, 185] };
pub const GUID_D3COLD_SUPPORT_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3011678437, data2: 15568, data3: 20381, data4: [153, 55, 245, 254, 43, 68, 212, 122] };
pub const GUID_DEVCLASS_1394: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653697, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_1394DEBUG: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1727156438, data2: 30721, data3: 19044, data4: [177, 57, 238, 168, 10, 69, 11, 36] };
pub const GUID_DEVCLASS_61883: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2126445504, data2: 12800, data3: 4562, data4: [180, 194, 0, 160, 201, 105, 125, 7] };
pub const GUID_DEVCLASS_ADAPTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444324, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_APMSUPPORT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3562740760, data2: 51450, data3: 4561, data4: [159, 119, 0, 0, 248, 5, 245, 48] };
pub const GUID_DEVCLASS_AVC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3228562021, data2: 44553, data3: 18672, data4: [129, 44, 22, 117, 61, 124, 186, 131] };
pub const GUID_DEVCLASS_BATTERY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1919098452, data2: 30884, data3: 4560, data4: [188, 247, 0, 170, 0, 183, 179, 42] };
pub const GUID_DEVCLASS_BIOMETRIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1406312183, data2: 14204, data3: 19732, data4: [134, 75, 235, 58, 133, 118, 147, 89] };
pub const GUID_DEVCLASS_BLUETOOTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3771461740, data2: 52619, data3: 17991, data4: [187, 138, 38, 59, 67, 240, 249, 116] };
pub const GUID_DEVCLASS_CAMERA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3393092281, data2: 46275, data3: 19174, data4: [130, 81, 87, 158, 249, 51, 137, 15] };
pub const GUID_DEVCLASS_CDROM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444325, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_COMPUTEACCELERATOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4028276051, data2: 16374, data3: 18642, data4: [159, 151, 200, 167, 0, 75, 225, 12] };
pub const GUID_DEVCLASS_COMPUTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444326, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_DECODER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653698, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_DISKDRIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444327, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_DISPLAY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444328, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_DOT4: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1215437654, data2: 26517, data3: 4562, data4: [177, 168, 0, 128, 199, 46, 116, 162] };
pub const GUID_DEVCLASS_DOT4PRINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1238264520, data2: 28550, data3: 4562, data4: [177, 229, 0, 128, 199, 46, 116, 162] };
pub const GUID_DEVCLASS_EHSTORAGESILO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2644686863, data2: 63647, data3: 19017, data4: [165, 194, 81, 27, 8, 91, 158, 138] };
pub const GUID_DEVCLASS_ENUM1394: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3294224213, data2: 56072, data3: 4561, data4: [176, 9, 0, 160, 201, 8, 31, 246] };
pub const GUID_DEVCLASS_EXTENSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3807923431, data2: 36602, data3: 16668, data4: [170, 105, 151, 69, 76, 164, 203, 87] };
pub const GUID_DEVCLASS_FDC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444329, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_FIRMWARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075281778, data2: 25704, data3: 20022, data4: [182, 241, 100, 136, 244, 44, 27, 82] };
pub const GUID_DEVCLASS_FLOPPYDISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444352, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_FSFILTER_ACTIVITYMONITOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3094216529, data2: 41758, data3: 19372, data4: [179, 207, 232, 207, 231, 92, 159, 194] };
pub const GUID_DEVCLASS_FSFILTER_ANTIVIRUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2983305577, data2: 50511, data3: 17273, data4: [129, 219, 190, 231, 216, 141, 116, 84] };
pub const GUID_DEVCLASS_FSFILTER_BOTTOM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 930504352, data2: 22872, data3: 20425, data4: [176, 75, 47, 223, 239, 151, 229, 158] };
pub const GUID_DEVCLASS_FSFILTER_CFSMETADATASERVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3452897593, data2: 46939, data3: 17968, data4: [191, 118, 128, 247, 186, 101, 88, 132] };
pub const GUID_DEVCLASS_FSFILTER_COMPRESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4082658223, data2: 46506, data3: 18869, data4: [141, 108, 5, 105, 40, 76, 99, 159] };
pub const GUID_DEVCLASS_FSFILTER_CONTENTSCREENER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1044317812, data2: 51260, data3: 17752, data4: [187, 38, 152, 32, 225, 235, 165, 197] };
pub const GUID_DEVCLASS_FSFILTER_CONTINUOUSBACKUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1906971896, data2: 28589, data3: 17954, data4: [173, 119, 146, 187, 157, 126, 105, 71] };
pub const GUID_DEVCLASS_FSFILTER_COPYPROTECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2306371569, data2: 39954, data3: 16431, data4: [156, 158, 23, 117, 60, 127, 67, 117] };
pub const GUID_DEVCLASS_FSFILTER_ENCRYPTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2695299520, data2: 42257, data3: 17151, data4: [170, 108, 6, 220, 3, 149, 87, 111] };
pub const GUID_DEVCLASS_FSFILTER_HSM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3578155018, data2: 10987, data3: 17910, data4: [148, 130, 244, 177, 121, 156, 49, 119] };
pub const GUID_DEVCLASS_FSFILTER_INFRASTRUCTURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3848251129, data2: 4748, data3: 19716, data4: [171, 171, 99, 12, 116, 177, 69, 58] };
pub const GUID_DEVCLASS_FSFILTER_OPENFILEBACKUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4176261030, data2: 26321, data3: 16805, data4: [137, 155, 102, 88, 93, 114, 22, 183] };
pub const GUID_DEVCLASS_FSFILTER_PHYSICALQUOTAMANAGEMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1779076728, data2: 48038, data3: 20420, data4: [167, 9, 30, 51, 205, 9, 214, 126] };
pub const GUID_DEVCLASS_FSFILTER_QUOTAMANAGEMENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2231617809, data2: 42695, data3: 18713, data4: [143, 121, 80, 40, 245, 134, 107, 12] };
pub const GUID_DEVCLASS_FSFILTER_REPLICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1221848004, data2: 19704, data3: 18687, data4: [184, 105, 156, 104, 173, 66, 235, 159] };
pub const GUID_DEVCLASS_FSFILTER_SECURITYENHANCER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3492529114, data2: 3214, data3: 18757, data4: [155, 213, 241, 136, 60, 34, 108, 140] };
pub const GUID_DEVCLASS_FSFILTER_SYSTEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1562090154, data2: 482, data3: 18095, data4: [132, 159, 39, 43, 63, 50, 76, 70] };
pub const GUID_DEVCLASS_FSFILTER_SYSTEMRECOVERY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 766595956, data2: 28782, data3: 16689, data4: [160, 199, 215, 199, 142, 176, 40, 154] };
pub const GUID_DEVCLASS_FSFILTER_TOP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3010050804, data2: 21864, data3: 20098, data4: [168, 126, 169, 62, 177, 107, 202, 135] };
pub const GUID_DEVCLASS_FSFILTER_UNDELETE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4270790002, data2: 50810, data3: 18624, data4: [187, 172, 11, 92, 109, 102, 202, 251] };
pub const GUID_DEVCLASS_FSFILTER_VIRTUALIZATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4149905088, data2: 4312, data3: 19514, data4: [178, 51, 237, 96, 228, 205, 250, 172] };
pub const GUID_DEVCLASS_GPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653699, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_HDC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444330, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_HIDCLASS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1952061344, data2: 29907, data3: 4560, data4: [182, 254, 0, 160, 201, 15, 87, 218] };
pub const GUID_DEVCLASS_HOLOGRAPHIC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3591525693, data2: 1713, data3: 18890, data4: [137, 56, 227, 158, 248, 14, 177, 111] };
pub const GUID_DEVCLASS_IMAGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653702, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_INFINIBAND: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 820998450, data2: 55384, data3: 18956, data4: [172, 36, 185, 2, 138, 92, 202, 63] };
pub const GUID_DEVCLASS_INFRARED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1809653701, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_KEYBOARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444331, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_LEGACYDRIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2395735389, data2: 1151, data3: 4561, data4: [165, 55, 0, 0, 248, 117, 62, 209] };
pub const GUID_DEVCLASS_MEDIA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444332, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MEDIUM_CHANGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3461953966, data2: 60382, data3: 4560, data4: [177, 129, 0, 0, 248, 117, 62, 196] };
pub const GUID_DEVCLASS_MEMORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1352242250, data2: 63161, data3: 16471, data4: [160, 86, 140, 85, 2, 40, 84, 76] };
pub const GUID_DEVCLASS_MODEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444333, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MONITOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444334, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MOUSE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444335, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MTD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444336, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MULTIFUNCTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444337, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_MULTIPORTSERIAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1351642296, data2: 47634, data3: 4561, data4: [191, 93, 0, 0, 248, 5, 245, 48] };
pub const GUID_DEVCLASS_NET: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444338, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_NETCLIENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444339, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_NETDRIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2280626897, data2: 36720, data3: 18926, data4: [178, 21, 171, 31, 202, 220, 190, 60] };
pub const GUID_DEVCLASS_NETSERVICE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444340, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_NETTRANS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444341, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_NETUIO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2022779841, data2: 52110, data3: 19240, data4: [163, 41, 243, 34, 235, 173, 190, 15] };
pub const GUID_DEVCLASS_NODRIVER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444342, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PCMCIA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444343, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PNPPRINTERS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1180233342, data2: 61520, data3: 4561, data4: [182, 189, 0, 192, 79, 163, 114, 167] };
pub const GUID_DEVCLASS_PORTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444344, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PRINTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444345, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PRINTERUPGRADE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444346, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_PRINTQUEUE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 517127161, data2: 4592, data3: 16516, data4: [178, 31, 173, 131, 168, 230, 220, 220] };
pub const GUID_DEVCLASS_PROCESSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1343389123, data2: 3894, data3: 16734, data4: [166, 204, 76, 179, 190, 145, 11, 101] };
pub const GUID_DEVCLASS_SBP2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3565255102, data2: 60448, data3: 4561, data4: [182, 184, 0, 192, 79, 163, 114, 167] };
pub const GUID_DEVCLASS_SCMDISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1402367153, data2: 19782, data3: 16742, data4: [191, 35, 197, 34, 64, 60, 212, 149] };
pub const GUID_DEVCLASS_SCMVOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1405923657, data2: 58691, data3: 19588, data4: [182, 224, 188, 228, 246, 183, 232, 6] };
pub const GUID_DEVCLASS_SCSIADAPTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444347, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_SECURITYACCELERATOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 646747553, data2: 60926, data3: 4563, data4: [149, 195, 0, 16, 220, 64, 80, 165] };
pub const GUID_DEVCLASS_SENSOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1366676276, data2: 50033, data3: 18438, data4: [179, 186, 113, 253, 83, 201, 37, 141] };
pub const GUID_DEVCLASS_SIDESHOW: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2574998925, data2: 50242, data3: 20270, data4: [186, 243, 156, 142, 103, 30, 158, 33] };
pub const GUID_DEVCLASS_SMARTCARDREADER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1356681776, data2: 47754, data3: 4561, data4: [191, 93, 0, 0, 248, 5, 245, 48] };
pub const GUID_DEVCLASS_SMRDISK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1397259299, data2: 26639, data3: 17797, data4: [172, 195, 31, 16, 214, 119, 126, 130] };
pub const GUID_DEVCLASS_SMRVOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1404292867, data2: 36698, data3: 18312, data4: [145, 182, 209, 158, 217, 252, 204, 191] };
pub const GUID_DEVCLASS_SOFTWARECOMPONENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1548497714, data2: 13389, data3: 18492, data4: [135, 57, 37, 158, 147, 76, 156, 200] };
pub const GUID_DEVCLASS_SOUND: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444348, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_SYSTEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444349, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_TAPEDRIVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1837136004, data2: 32033, data3: 4559, data4: [128, 28, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_UCM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3874597404, data2: 32571, data3: 17523, data4: [178, 232, 201, 125, 138, 199, 29, 83] };
pub const GUID_DEVCLASS_UNKNOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1295444350, data2: 58149, data3: 4558, data4: [191, 193, 8, 0, 43, 225, 3, 24] };
pub const GUID_DEVCLASS_USB: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 922525280, data2: 50277, data3: 4559, data4: [128, 86, 68, 69, 83, 84, 0, 0] };
pub const GUID_DEVCLASS_VOLUME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1906474205, data2: 33066, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_DEVCLASS_VOLUMESNAPSHOT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1396464516, data2: 60528, data3: 4562, data4: [149, 5, 0, 192, 79, 121, 222, 175] };
pub const GUID_DEVCLASS_WCEUSBS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 635162193, data2: 27791, data3: 19058, data4: [138, 109, 181, 76, 43, 79, 200, 53] };
pub const GUID_DEVCLASS_WPD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4005932440, data2: 32896, data3: 16991, data4: [146, 42, 218, 191, 61, 227, 246, 154] };
pub const GUID_DEVICE_INTERFACE_ARRIVAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592324, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_DEVICE_INTERFACE_REMOVAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592325, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_DEVICE_RESET_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1688198950, data2: 15296, data3: 18451, data4: [173, 36, 126, 12, 30, 218, 63, 163] };
pub const GUID_DMA_CACHE_COHERENCY_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3038836730, data2: 35418, data3: 20032, data4: [163, 246, 107, 225, 225, 98, 217, 53] };
pub const GUID_HWPROFILE_CHANGE_CANCELLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592322, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_HWPROFILE_CHANGE_COMPLETE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592323, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_HWPROFILE_QUERY_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592321, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_INT_ROUTE_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1888754676, data2: 115, data3: 4561, data4: [160, 158, 0, 192, 79, 195, 64, 177] };
pub const GUID_IOMMU_BUS_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 520020146, data2: 53880, data3: 19172, data4: [189, 220, 27, 52, 221, 100, 128, 67] };
pub const GUID_KERNEL_SOFT_RESTART_CANCEL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 836188135, data2: 35851, data3: 18058, data4: [149, 110, 159, 67, 62, 195, 88, 251] };
pub const GUID_KERNEL_SOFT_RESTART_FINALIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 552147645, data2: 13578, data3: 19791, data4: [133, 119, 153, 200, 21, 7, 71, 58] };
pub const GUID_KERNEL_SOFT_RESTART_PREPARE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3728162287, data2: 43100, data3: 20342, data4: [140, 191, 249, 107, 234, 139, 209, 15] };
pub const GUID_LEGACY_DEVICE_DETECTION_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1358868702, data2: 22890, data3: 4562, data4: [165, 184, 0, 0, 248, 26, 70, 25] };
pub const GUID_MF_ENUMERATION_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2931332592, data2: 21894, data3: 4561, data4: [141, 132, 0, 160, 201, 6, 178, 68] };
pub const GUID_MSIX_TABLE_CONFIG_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 443172363, data2: 6479, data3: 17757, data4: [179, 75, 184, 76, 91, 5, 113, 43] };
pub const GUID_NPEM_CONTROL_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1301632829, data2: 46964, data3: 18570, data4: [177, 32, 79, 40, 74, 158, 255, 81] };
pub const GUID_PARTITION_UNIT_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1379286875, data2: 55441, data3: 17051, data4: [129, 149, 174, 197, 254, 246, 133, 60] };
pub const GUID_PCC_INTERFACE_INTERNAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2093900494, data2: 49545, data3: 18452, data4: [166, 167, 18, 17, 32, 137, 233, 56] };
pub const GUID_PCC_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1055439459, data2: 3929, data3: 18980, data4: [138, 69, 53, 128, 139, 221, 18, 73] };
pub const GUID_PCI_ATS_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 17465320, data2: 38645, data3: 18755, data4: [190, 223, 149, 230, 81, 185, 52, 18] };
pub const GUID_PCI_BUS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1231782529, data2: 28453, data3: 4560, data4: [190, 175, 8, 0, 43, 226, 9, 47] };
pub const GUID_PCI_BUS_INTERFACE_STANDARD2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3734301030, data2: 65023, data3: 19612, data4: [153, 152, 103, 71, 177, 80, 231, 76] };
pub const GUID_PCI_DEVICE_PRESENT_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3518508070, data2: 48969, data3: 17903, data4: [178, 22, 113, 203, 215, 136, 155, 87] };
pub const GUID_PCI_EXPRESS_LINK_QUIESCENT_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 342676508, data2: 56035, data3: 17463, data4: [138, 255, 42, 243, 240, 56, 9, 155] };
pub const GUID_PCI_EXPRESS_ROOT_PORT_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2208789322, data2: 33991, data3: 16737, data4: [154, 152, 96, 0, 237, 12, 74, 51] };
pub const GUID_PCI_FPGA_CONTROL_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 770963368, data2: 47539, data3: 16483, data4: [146, 21, 181, 209, 74, 11, 38, 110] };
pub const GUID_PCI_PTM_CONTROL_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 881483451, data2: 47652, data3: 17591, data4: [153, 22, 40, 86, 135, 115, 81, 23] };
pub const GUID_PCI_SECURITY_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1853822033, data2: 6558, data3: 19148, data4: [186, 45, 118, 43, 78, 223, 70, 116] };
pub const GUID_PCI_VIRTUALIZATION_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1686731591, data2: 14922, data3: 19829, data4: [188, 116, 137, 221, 108, 7, 130, 147] };
pub const GUID_PCMCIA_BUS_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1981233904, data2: 50436, data3: 4561, data4: [148, 127, 0, 192, 79, 185, 96, 238] };
pub const GUID_PNP_CUSTOM_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2896641934, data2: 36131, data3: 4561, data4: [172, 125, 0, 0, 248, 117, 113, 208] };
pub const GUID_PNP_EXTENDED_ADDRESS_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3102315244, data2: 42903, data3: 19908, data4: [136, 70, 132, 208, 65, 112, 116, 70] };
pub const GUID_PNP_LOCATION_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1881217806, data2: 2811, data3: 18395, data4: [175, 193, 65, 11, 248, 66, 73, 122] };
pub const GUID_PNP_POWER_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3268347488, data2: 60282, data3: 4561, data4: [189, 127, 0, 0, 248, 117, 113, 208] };
pub const GUID_PNP_POWER_SETTING_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 700881726, data2: 51098, data3: 17343, data4: [187, 222, 169, 50, 250, 27, 234, 126] };
pub const GUID_POWER_DEVICE_ENABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2189167215, data2: 65200, data3: 4560, data4: [189, 38, 0, 170, 0, 183, 179, 42] };
pub const GUID_POWER_DEVICE_TIMEOUTS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2757601077, data2: 65200, data3: 4560, data4: [189, 38, 0, 170, 0, 183, 179, 42] };
pub const GUID_POWER_DEVICE_WAKE_ENABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2840881794, data2: 65200, data3: 4560, data4: [189, 38, 0, 170, 0, 183, 179, 42] };
pub const GUID_PROCESSOR_PCC_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 934379162, data2: 49692, data3: 17046, data4: [151, 45, 17, 196, 179, 43, 40, 240] };
pub const GUID_QUERY_CRASHDUMP_FUNCTIONS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2630269183, data2: 13026, data3: 18484, data4: [177, 222, 179, 46, 248, 136, 10, 75] };
pub const GUID_RECOVERY_NVMED_PREPARE_SHUTDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1268216042, data2: 48615, data3: 16395, data4: [169, 185, 79, 104, 79, 84, 204, 42] };
pub const GUID_RECOVERY_PCI_PREPARE_SHUTDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2430110174, data2: 34564, data3: 17615, data4: [129, 21, 237, 133, 40, 210, 178, 218] };
pub const GUID_REENUMERATE_SELF_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 720044611, data2: 27246, data3: 18539, data4: [130, 252, 216, 21, 246, 185, 112, 6] };
pub const GUID_SCM_BUS_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 630474627, data2: 52857, data3: 16946, data4: [129, 94, 74, 48, 1, 78, 142, 180] };
pub const GUID_SCM_BUS_LD_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2609459325, data2: 55147, data3: 20296, data4: [177, 134, 84, 4, 26, 233, 46, 141] };
pub const GUID_SCM_BUS_NVD_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2380293375, data2: 46640, data3: 17124, data4: [136, 234, 111, 36, 200, 100, 17, 117] };
pub const GUID_SCM_PHYSICAL_NVDIMM_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 7979547, data2: 37246, data3: 16478, data4: [169, 206, 7, 50, 181, 187, 206, 189] };
pub const GUID_SDEV_IDENTIFIER_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1238792952, data2: 37228, data3: 20200, data4: [157, 241, 136, 159, 23, 210, 30, 145] };
pub const GUID_SECURE_DRIVER_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 923756513, data2: 20469, data3: 19092, data4: [154, 53, 6, 197, 217, 204, 48, 226] };
pub const GUID_TARGET_DEVICE_QUERY_REMOVE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592326, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_TARGET_DEVICE_REMOVE_CANCELLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592327, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_TARGET_DEVICE_REMOVE_COMPLETE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3409592328, data2: 18160, data3: 4560, data4: [176, 143, 0, 96, 151, 19, 5, 63] };
pub const GUID_TARGET_DEVICE_TRANSPORT_RELATIONS_CHANGED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4243925238, data2: 43055, data3: 18353, data4: [173, 58, 128, 80, 89, 76, 173, 40] };
pub const GUID_THERMAL_COOLING_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3971893160, data2: 50328, data3: 19385, data4: [189, 112, 232, 103, 224, 148, 13, 34] };
pub const GUID_TRANSLATOR_INTERFACE_STANDARD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1813334674, data2: 43727, data3: 4560, data4: [141, 42, 0, 160, 201, 6, 178, 68] };
pub const GUID_WUDF_DEVICE_HOST_PROBLEM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3292341693, data2: 37702, data3: 16622, data4: [162, 210, 215, 12, 21, 248, 183, 91] };
pub type HCMNOTIFICATION = isize;
pub type HDEVINFO = isize;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HWProfileInfo_sA {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [super::super::Foundation::CHAR; 80],
    pub HWPI_dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HWProfileInfo_sA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HWProfileInfo_sA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct HWProfileInfo_sW {
    pub HWPI_ulHWProfile: u32,
    pub HWPI_szFriendlyName: [u16; 80],
    pub HWPI_dwFlags: u32,
}
impl ::core::marker::Copy for HWProfileInfo_sW {}
impl ::core::clone::Clone for HWProfileInfo_sW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_ANALYZEDEV_PAGE: u32 = 10010u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_ANALYZE_NEXTPAGE: u32 = 10004u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_ANALYZE_PREVPAGE: u32 = 10003u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_FIRSTPAGE: u32 = 10000u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_INSTALLDETECTEDDEVS_PAGE: u32 = 10011u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_INSTALLDETECTED_NEXTPAGE: u32 = 10007u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_INSTALLDETECTED_NODEVS: u32 = 10008u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_INSTALLDETECTED_PREVPAGE: u32 = 10006u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_SELECTCLASS_PAGE: u32 = 10012u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_SELECTDEV_PAGE: u32 = 10009u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_SELECT_NEXTPAGE: u32 = 10002u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDD_DYNAWIZ_SELECT_PREVPAGE: u32 = 10001u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_CHECKFIRST: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOBEEP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOBROWSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOCOMPRESSED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NODETAILS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOFOREGROUND: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOREMOVABLEMEDIAPROMPT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_NOSKIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_OEMDISK: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_USEDISKNAMEASPROMPT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDF_WARNIFSKIP: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_CLASSICON_OVERLAYFIRST: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_CLASSICON_OVERLAYLAST: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_CONFLICT: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_DISABLED_OVL: u32 = 501u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_FORCED_OVL: u32 = 502u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_PROBLEM_OVL: u32 = 500u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCE: u32 = 159u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCEFIRST: u32 = 159u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCELAST: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCEOVERLAYFIRST: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IDI_RESOURCEOVERLAYLAST: u32 = 161u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct INFCONTEXT {
    pub Inf: *mut ::core::ffi::c_void,
    pub CurrentInf: *mut ::core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for INFCONTEXT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for INFCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct INFCONTEXT {
    pub Inf: *mut ::core::ffi::c_void,
    pub CurrentInf: *mut ::core::ffi::c_void,
    pub Section: u32,
    pub Line: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for INFCONTEXT {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for INFCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_DEFAULT_SEARCH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_INF_NAME_IS_ABSOLUTE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_INF_PATH_LIST_SEARCH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_INF_SPEC_IS_HINF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFINFO_REVERSE_DEFAULT_SEARCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_BUS_ALL: &str = "BUS_ALL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_BUS_EISA: &str = "BUS_EISA";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_BUS_ISA: &str = "BUS_ISA";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_BUS_MCA: &str = "BUS_MCA";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_DESIRED: &str = "DESIRED";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_DISABLED: &str = "DISABLED";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_FORCECONFIG: &str = "FORCECONFIG";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_HARDRECONFIG: &str = "HARDRECONFIG";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_HARDWIRED: &str = "HARDWIRED";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_NORMAL: &str = "NORMAL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_POWEROFF: &str = "POWEROFF";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_REBOOT: &str = "REBOOT";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_RESTART: &str = "RESTART";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGPRI_SUBOPTIMAL: &str = "SUBOPTIMAL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGTYPE_BASIC: &str = "BASIC";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGTYPE_FORCED: &str = "FORCED";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CFGTYPE_OVERRIDE: &str = "OVERRIDE";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CLASS_SAFEEXCL: &str = "SAFE_EXCL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_CONTROLFLAGS_SECTION: &str = "ControlFlags";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_DRIVERSELECT_FUNCTIONS: &str = "DriverSelectFunctions";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_DRIVERSELECT_SECTION: &str = "DriverSelect";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_DRIVERVERSION_SECTION: &str = "DriverVer";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ACTION: &str = "Action";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ALWAYSEXCLUDEFROMSELECT: &str = "AlwaysExcludeFromSelect";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_BUFFER_SIZE: &str = "BufferSize";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CATALOGFILE: &str = "CatalogFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CHANNEL_ACCESS: &str = "Access";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CHANNEL_ENABLED: &str = "Enabled";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CHANNEL_ISOLATION: &str = "Isolation";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CHANNEL_VALUE: &str = "Value";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CLASS: &str = "Class";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CLASSGUID: &str = "ClassGUID";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CLOCK_TYPE: &str = "ClockType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_CONFIGPRIORITY: &str = "ConfigPriority";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_COPYFILESONLY: &str = "CopyFilesOnly";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DATA_ITEM: &str = "DataItem";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DELAYEDAUTOSTART: &str = "DelayedAutoStart";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DEPENDENCIES: &str = "Dependencies";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DESCRIPTION: &str = "Description";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DETECTLIST: &str = "DetectList";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DETPARAMS: &str = "Params";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DISABLE_REALTIME_PERSISTENCE: &str = "DisableRealtimePersistence";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DISPLAYNAME: &str = "DisplayName";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DMA: &str = "DMA";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DMACONFIG: &str = "DMAConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_DRIVERSET: &str = "DriverSet";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ENABLED: &str = "Enabled";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ENABLE_FLAGS: &str = "EnableFlags";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ENABLE_LEVEL: &str = "EnableLevel";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ENABLE_PROPERTY: &str = "EnableProperty";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_ERRORCONTROL: &str = "ErrorControl";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_EXCLUDEFROMSELECT: &str = "ExcludeFromSelect";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_EXCLUDERES: &str = "ExcludeRes";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_EXTENSIONID: &str = "ExtensionId";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_FILE_MAX: &str = "FileMax";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_FILE_NAME: &str = "FileName";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_FLUSH_TIMER: &str = "FlushTimer";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_FROMINET: &str = "FromINet";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_HARDWARE_CLASS: &str = "Class";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_HARDWARE_CLASSGUID: &str = "ClassGUID";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_INTERACTIVEINSTALL: &str = "InteractiveInstall";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_IO: &str = "IO";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_IOCONFIG: &str = "IOConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_IRQ: &str = "IRQ";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_IRQCONFIG: &str = "IRQConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOADORDERGROUP: &str = "LoadOrderGroup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOGGING_AUTOBACKUP: &str = "LoggingAutoBackup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOGGING_MAXSIZE: &str = "LoggingMaxSize";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOGGING_RETENTION: &str = "LoggingRetention";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_LOG_FILE_MODE: &str = "LogFileMode";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MATCH_ALL_KEYWORD: &str = "MatchAllKeyword";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MATCH_ANY_KEYWORD: &str = "MatchAnyKeyword";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MAXIMUM_BUFFERS: &str = "MaximumBuffers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MAX_FILE_SIZE: &str = "MaxFileSize";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MEM: &str = "Mem";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MEMCONFIG: &str = "MemConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MEMLARGECONFIG: &str = "MemLargeConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MESSAGE_FILE: &str = "MessageFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MFCARDCONFIG: &str = "MfCardConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_MINIMUM_BUFFERS: &str = "MinimumBuffers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_NAME: &str = "Name";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_NOSETUPINF: &str = "NoSetupInf";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PARAMETER_FILE: &str = "ParameterFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PATH: &str = "Path";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PCCARDCONFIG: &str = "PcCardConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PNPLOCKDOWN: &str = "PnpLockDown";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PROVIDER: &str = "Provider";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_PROVIDER_NAME: &str = "ProviderName";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_REQUESTADDITIONALSOFTWARE: &str = "RequestAdditionalSoftware";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_REQUIREDPRIVILEGES: &str = "RequiredPrivileges";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_RESOURCE_FILE: &str = "ResourceFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SECURITY: &str = "Security";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SERVICEBINARY: &str = "ServiceBinary";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SERVICESIDTYPE: &str = "ServiceSidType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SERVICETYPE: &str = "ServiceType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SIGNATURE: &str = "Signature";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SKIPLIST: &str = "SkipList";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_START: &str = "Start";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_STARTNAME: &str = "StartName";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_STARTTYPE: &str = "StartType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_SUB_TYPE: &str = "SubType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_KEY_TRIGGER_TYPE: &str = "TriggerType";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NT: &str = "NT";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTALPHA: &str = "NTAlpha";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTAMD64: &str = "NTAMD64";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTARM: &str = "NTARM";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTARM64: &str = "NTARM64";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTAXP64: &str = "NTAXP64";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTIA64: &str = "NTIA64";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTMIPS: &str = "NTMIPS";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTPPC: &str = "NTPPC";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_NTX86: &str = "NTx86";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_PLATFORM_WIN: &str = "Win";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_REBOOT: &str = "Reboot";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RESTART: &str = "Restart";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_BIOSROMRD: &str = "RISK_BIOSROMRD";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_DELICATE: &str = "RISK_DELICATE";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_IORD: &str = "RISK_IORD";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_IOWR: &str = "RISK_IOWR";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_LOW: &str = "RISK_LOW";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_MEMRD: &str = "RISK_MEMRD";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_MEMWR: &str = "RISK_MEMWR";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_NONE: &str = "RISK_NONE";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_QUERYDRV: &str = "RISK_QUERYDRV";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_SWINT: &str = "RISK_SWINT";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_UNRELIABLE: &str = "RISK_UNRELIABLE";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_VERYHIGH: &str = "RISK_VERYHIGH";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_RISK_VERYLOW: &str = "RISK_VERYLOW";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_AUTOEXECBAT: &str = "AutoexecBatDrivers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_AVOIDCFGSYSDEV: &str = "Det.AvoidCfgSysDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_AVOIDENVDEV: &str = "Det.AvoidEnvDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_AVOIDINIDEV: &str = "Det.AvoidIniDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADACPIBIOS: &str = "BadACPIBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADDISKBIOS: &str = "BadDiskBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADDSBIOS: &str = "BadDSBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADPMCALLBIOS: &str = "BadProtectedModeCallBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADPNPBIOS: &str = "BadPnpBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADRMCALLBIOS: &str = "BadRealModeCallBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_BADROUTINGTABLEBIOS: &str = "BadPCIIRQRoutingTableBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_CFGSYS: &str = "ConfigSysDrivers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_CLASS_INSTALL: &str = "ClassInstall";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_CLASS_INSTALL_32: &str = "ClassInstall32";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DEFAULT_INSTALL: &str = "DefaultInstall";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DEFAULT_UNINSTALL: &str = "DefaultUninstall";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DETCLASSINFO: &str = "Det.ClassInfo";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DETMODULES: &str = "Det.Modules";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DETOPTIONS: &str = "Det.Options";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DEVINFS: &str = "Det.DevINFs";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_DISPLAY_CLEANUP: &str = "DisplayCleanup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_EXTENSIONCONTRACTS: &str = "ExtensionContracts";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_FORCEHWVERIFY: &str = "Det.ForceHWVerify";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_GOODACPIBIOS: &str = "GoodACPIBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_HPOMNIBOOK: &str = "Det.HPOmnibook";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_INTERFACE_INSTALL_32: &str = "InterfaceInstall32";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_MACHINEIDBIOS: &str = "MachineIDBios";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_MANUALDEV: &str = "Det.ManualDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_MFG: &str = "Manufacturer";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_REGCFGSYSDEV: &str = "Det.RegCfgSysDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_REGENVDEV: &str = "Det.RegEnvDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_REGINIDEV: &str = "Det.RegIniDev";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_SYSINI: &str = "SystemIniDrivers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_SYSINIDRV: &str = "SystemIniDriversLine";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_TARGETCOMPUTERS: &str = "TargetComputers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_VERSION: &str = "Version";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SECT_WININIRUN: &str = "WinIniRunLine";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SOFTWAREVERSION_SECTION: &str = "SoftwareVersion";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_STRKEY_DRVDESC: &str = "DriverDesc";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_COINSTALLERS: &str = "CoInstallers";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_CTL: &str = "CTL";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_DET: &str = "Det";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_EVENTS: &str = "Events";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_FACTDEF: &str = "FactDef";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_FILTERS: &str = "Filters";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_HW: &str = "Hw";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_INTERFACES: &str = "Interfaces";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_LOGCONFIG: &str = "LogConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_LOGCONFIGOVERRIDE: &str = "LogConfigOverride";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_NORESOURCEDUPS: &str = "NoResDup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_POSSIBLEDUPS: &str = "PosDup";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_SERVICES: &str = "Services";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_SOFTWARE: &str = "Software";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INFSTR_SUBKEY_WMI: &str = "WMI";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_CACHE_DISABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_CACHE_ENABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_CACHE_IGNORE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INSTALLFLAG_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INSTALLFLAG_FORCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INSTALLFLAG_NONINTERACTIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INSTALLFLAG_READONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IOA_Local: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IO_ALIAS_10_BIT_DECODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IO_ALIAS_12_BIT_DECODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IO_ALIAS_16_BIT_DECODE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const IO_ALIAS_POSITIVE_DECODE: u32 = 255u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IO_DES {
    pub IOD_Count: u32,
    pub IOD_Type: u32,
    pub IOD_Alloc_Base: u64,
    pub IOD_Alloc_End: u64,
    pub IOD_DesFlags: u32,
}
impl ::core::marker::Copy for IO_DES {}
impl ::core::clone::Clone for IO_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IO_RANGE {
    pub IOR_Align: u64,
    pub IOR_nPorts: u32,
    pub IOR_Min: u64,
    pub IOR_Max: u64,
    pub IOR_RangeFlags: u32,
    pub IOR_Alias: u64,
}
impl ::core::marker::Copy for IO_RANGE {}
impl ::core::clone::Clone for IO_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IO_RESOURCE {
    pub IO_Header: IO_DES,
    pub IO_Data: [IO_RANGE; 1],
}
impl ::core::marker::Copy for IO_RESOURCE {}
impl ::core::clone::Clone for IO_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_DES_32 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: u32,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u32,
}
impl ::core::marker::Copy for IRQ_DES_32 {}
impl ::core::clone::Clone for IRQ_DES_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_DES_64 {
    pub IRQD_Count: u32,
    pub IRQD_Type: u32,
    pub IRQD_Flags: u32,
    pub IRQD_Alloc_Num: u32,
    pub IRQD_Affinity: u64,
}
impl ::core::marker::Copy for IRQ_DES_64 {}
impl ::core::clone::Clone for IRQ_DES_64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_RANGE {
    pub IRQR_Min: u32,
    pub IRQR_Max: u32,
    pub IRQR_Flags: u32,
}
impl ::core::marker::Copy for IRQ_RANGE {}
impl ::core::clone::Clone for IRQ_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_RESOURCE_32 {
    pub IRQ_Header: IRQ_DES_32,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl ::core::marker::Copy for IRQ_RESOURCE_32 {}
impl ::core::clone::Clone for IRQ_RESOURCE_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct IRQ_RESOURCE_64 {
    pub IRQ_Header: IRQ_DES_64,
    pub IRQ_Data: [IRQ_RANGE; 1],
}
impl ::core::marker::Copy for IRQ_RESOURCE_64 {}
impl ::core::clone::Clone for IRQ_RESOURCE_64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_BOOTCONFIG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_DESIRED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_DISABLED: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_FORCECONFIG: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_HARDRECONFIG: u32 = 49152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_HARDWIRED: u32 = 57344u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_IMPOSSIBLE: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_LASTBESTCONFIG: u32 = 16383u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_LASTSOFTCONFIG: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_NORMAL: u32 = 12288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_POWEROFF: u32 = 40960u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_REBOOT: u32 = 36864u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_RESTART: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LCPRI_SUBOPTIMAL: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LINE_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LOG_CONF_BITS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevError: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevFatalError: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevInformation: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevMaximum: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const LogSevWarning: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_CLASS_NAME_LEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_CONFIG_VALUE: u32 = 9999u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_DEVICE_ID_LEN: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_DEVNODE_ID_LEN: u32 = 200u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_DMA_CHANNELS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_GUID_STRING_LEN: u32 = 39u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_IDD_DYNAWIZ_RESOURCE_ID: u32 = 11000u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INFSTR_STRKEY_LEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INF_FLAG: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INF_SECTION_NAME_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INF_STRING_LENGTH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INSTALLWIZARD_DYNAPAGES: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INSTANCE_VALUE: u32 = 9999u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_INSTRUCTION_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_IO_PORTS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_IRQS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_KEY_LEN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_LABEL_LEN: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_LCPRI: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_MEM_REGISTERS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_PRIORITYSTR_LEN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_PROFILE_LEN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_SERVICE_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_SUBTITLE_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MAX_TITLE_LEN: u32 = 60u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MEM_DES {
    pub MD_Count: u32,
    pub MD_Type: u32,
    pub MD_Alloc_Base: u64,
    pub MD_Alloc_End: u64,
    pub MD_Flags: u32,
    pub MD_Reserved: u32,
}
impl ::core::marker::Copy for MEM_DES {}
impl ::core::clone::Clone for MEM_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MEM_RANGE {
    pub MR_Align: u64,
    pub MR_nBytes: u32,
    pub MR_Min: u64,
    pub MR_Max: u64,
    pub MR_Flags: u32,
    pub MR_Reserved: u32,
}
impl ::core::marker::Copy for MEM_RANGE {}
impl ::core::clone::Clone for MEM_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MEM_RESOURCE {
    pub MEM_Header: MEM_DES,
    pub MEM_Data: [MEM_RANGE; 1],
}
impl ::core::marker::Copy for MEM_RESOURCE {}
impl ::core::clone::Clone for MEM_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MFCARD_DES {
    pub PMF_Count: u32,
    pub PMF_Type: u32,
    pub PMF_Flags: u32,
    pub PMF_ConfigOptions: u8,
    pub PMF_IoResourceIndex: u8,
    pub PMF_Reserved: [u8; 2],
    pub PMF_ConfigRegisterBase: u32,
}
impl ::core::marker::Copy for MFCARD_DES {}
impl ::core::clone::Clone for MFCARD_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct MFCARD_RESOURCE {
    pub MfCard_Header: MFCARD_DES,
}
impl ::core::marker::Copy for MFCARD_RESOURCE {}
impl ::core::clone::Clone for MFCARD_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const MIN_IDD_DYNAWIZ_RESOURCE_ID: u32 = 10000u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Mem_Large_Des_s {
    pub MLD_Count: u32,
    pub MLD_Type: u32,
    pub MLD_Alloc_Base: u64,
    pub MLD_Alloc_End: u64,
    pub MLD_Flags: u32,
    pub MLD_Reserved: u32,
}
impl ::core::marker::Copy for Mem_Large_Des_s {}
impl ::core::clone::Clone for Mem_Large_Des_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Mem_Large_Range_s {
    pub MLR_Align: u64,
    pub MLR_nBytes: u64,
    pub MLR_Min: u64,
    pub MLR_Max: u64,
    pub MLR_Flags: u32,
    pub MLR_Reserved: u32,
}
impl ::core::marker::Copy for Mem_Large_Range_s {}
impl ::core::clone::Clone for Mem_Large_Range_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct Mem_Large_Resource_s {
    pub MEM_LARGE_Header: Mem_Large_Des_s,
    pub MEM_LARGE_Data: [Mem_Large_Range_s; 1],
}
impl ::core::marker::Copy for Mem_Large_Resource_s {}
impl ::core::clone::Clone for Mem_Large_Resource_s {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_CI_PICKED_OEM: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_DIDFACTDEFS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_EXPRESSINTRO: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_HARDWAREALLREADYIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_INSTALLSPECIFIC: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_KNOWNCLASS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_NEEDREBOOT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_NEEDRESTART: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_NEEDSHUTDOWN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_NODETECTEDDEVS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_PCMCIADEVICE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_PCMCIAMODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_SKIPCLASSLIST: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_SKIPISDEVINSTALLED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NDW_INSTALLFLAG_USERCANCEL: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V1: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V2: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V3: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V4: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V5: u32 = 53u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V6: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V7: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V8: u32 = 57u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_CM_PROB_V9: u32 = 58u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const NUM_LOG_CONF: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type OEM_SOURCE_MEDIA_TYPE = u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPOST_NONE: OEM_SOURCE_MEDIA_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPOST_PATH: OEM_SOURCE_MEDIA_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPOST_URL: OEM_SOURCE_MEDIA_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const OVERRIDE_LOG_CONF: u32 = 5u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
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
impl ::core::marker::Copy for PCCARD_DES {}
impl ::core::clone::Clone for PCCARD_DES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub struct PCCARD_RESOURCE {
    pub PcCard_Header: PCCARD_DES,
}
impl ::core::marker::Copy for PCCARD_RESOURCE {}
impl ::core::clone::Clone for PCCARD_RESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PCD_MAX_IO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PCD_MAX_MEMORY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type PCM_NOTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hnotify: HCMNOTIFICATION, context: *const ::core::ffi::c_void, action: CM_NOTIFY_ACTION, eventdata: *const CM_NOTIFY_EVENT_DATA, eventdatasize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDETECT_PROGRESS_NOTIFY = ::core::option::Option<unsafe extern "system" fn(progressnotifyparam: *const ::core::ffi::c_void, detectcomplete: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type PNP_VETO_TYPE = i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoTypeUnknown: PNP_VETO_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoLegacyDevice: PNP_VETO_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoPendingClose: PNP_VETO_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoWindowsApp: PNP_VETO_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoWindowsService: PNP_VETO_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoOutstandingOpen: PNP_VETO_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoDevice: PNP_VETO_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoDriver: PNP_VETO_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoIllegalDeviceRequest: PNP_VETO_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoInsufficientPower: PNP_VETO_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoNonDisableable: PNP_VETO_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoLegacyDriver: PNP_VETO_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoInsufficientRights: PNP_VETO_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PNP_VetoAlreadyRemoved: PNP_VETO_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PRIORITY_BIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PRIORITY_EQUAL_FIRST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const PRIORITY_EQUAL_LAST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type PSP_DETSIG_CMPPROC = ::core::option::Option<unsafe extern "system" fn(deviceinfoset: HDEVINFO, newdevicedata: *const SP_DEVINFO_DATA, existingdevicedata: *const SP_DEVINFO_DATA, comparecontext: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type PSP_FILE_CALLBACK_A = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type PSP_FILE_CALLBACK_W = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32>;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ROLLBACK_BITS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ROLLBACK_FLAG_NO_UI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const RegDisposition_Bits: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const RegDisposition_OpenAlways: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const RegDisposition_OpenExisting: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_All: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_BusNumber: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_ClassSpecific: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_Connection: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_DMA: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_DevicePrivate: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_DoNotUse: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_IO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_IRQ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_Ignored_Bit: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_MAX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_Mem: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_MemLarge: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_MfCardConfig: u32 = 32771u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_None: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_PcCardConfig: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const ResType_Reserved: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SCWMI_CLOBBER_SECURITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SETDIRID_NOT_FULL_PATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type SETUP_DI_BUILD_DRIVER_DRIVER_TYPE = u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDIT_CLASSDRIVER: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDIT_COMPATDRIVER: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type SETUP_FILE_OPERATION = u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_DELETE: SETUP_FILE_OPERATION = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const FILEOP_COPY: SETUP_FILE_OPERATION = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_AUTHENTICODE: u32 = 251658240u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_INBOX: u32 = 218103811u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_LOGO_PREMIUM: u32 = 218103809u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_LOGO_STANDARD: u32 = 218103810u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_SIGNED_MASK: u32 = 4026531840u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_UNCLASSIFIED: u32 = 218103812u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_UNKNOWN: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_UNSIGNED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_W9X_SUSPECT: u32 = 3221225472u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SIGNERSCORE_WHQL: u32 = 218103813u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SOURCE_MEDIA_A {
    pub Reserved: ::windows_sys::core::PCSTR,
    pub Tagfile: ::windows_sys::core::PCSTR,
    pub Description: ::windows_sys::core::PCSTR,
    pub SourcePath: ::windows_sys::core::PCSTR,
    pub SourceFile: ::windows_sys::core::PCSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SOURCE_MEDIA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SOURCE_MEDIA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SOURCE_MEDIA_A {
    pub Reserved: ::windows_sys::core::PCSTR,
    pub Tagfile: ::windows_sys::core::PCSTR,
    pub Description: ::windows_sys::core::PCSTR,
    pub SourcePath: ::windows_sys::core::PCSTR,
    pub SourceFile: ::windows_sys::core::PCSTR,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SOURCE_MEDIA_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SOURCE_MEDIA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SOURCE_MEDIA_W {
    pub Reserved: ::windows_sys::core::PCWSTR,
    pub Tagfile: ::windows_sys::core::PCWSTR,
    pub Description: ::windows_sys::core::PCWSTR,
    pub SourcePath: ::windows_sys::core::PCWSTR,
    pub SourceFile: ::windows_sys::core::PCWSTR,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SOURCE_MEDIA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SOURCE_MEDIA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SOURCE_MEDIA_W {
    pub Reserved: ::windows_sys::core::PCWSTR,
    pub Tagfile: ::windows_sys::core::PCWSTR,
    pub Description: ::windows_sys::core::PCWSTR,
    pub SourcePath: ::windows_sys::core::PCWSTR,
    pub SourceFile: ::windows_sys::core::PCWSTR,
    pub Flags: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SOURCE_MEDIA_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SOURCE_MEDIA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_CHARACTERISTICS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_DEVTYPE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_EXCLUSIVE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_LOWERFILTERS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_MAXIMUM_PROPERTY: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_SECURITY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_SECURITY_SDS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPCRP_UPPERFILTERS: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDIT_NODRIVER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_ADDRESS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_BASE_CONTAINERID: u32 = 36u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_BUSNUMBER: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_BUSTYPEGUID: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CAPABILITIES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CHARACTERISTICS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CLASS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CLASSGUID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_COMPATIBLEIDS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_CONFIGFLAGS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_DEVICEDESC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_DEVICE_POWER_DATA: u32 = 30u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_DEVTYPE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_DRIVER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_ENUMERATOR_NAME: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_EXCLUSIVE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_FRIENDLYNAME: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_HARDWAREID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_INSTALL_STATE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_LEGACYBUSTYPE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_LOCATION_INFORMATION: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_LOCATION_PATHS: u32 = 35u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_LOWERFILTERS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_MAXIMUM_PROPERTY: u32 = 37u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_MFG: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_PHYSICAL_DEVICE_OBJECT_NAME: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_REMOVAL_POLICY: u32 = 31u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_REMOVAL_POLICY_HW_DEFAULT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_REMOVAL_POLICY_OVERRIDE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_SECURITY: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_SECURITY_SDS: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_SERVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UI_NUMBER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UI_NUMBER_DESC_FORMAT: u32 = 29u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UNUSED0: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UNUSED1: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UNUSED2: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDRP_UPPERFILTERS: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDSL_DISALLOW_NEGATIVE_ADJUST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPDSL_IGNORE_DISK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILELOG_FORCENEW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILELOG_OEMFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILELOG_QUERYONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILELOG_SYSTEMLOG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_BACKUPERROR: u32 = 22u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_CABINETINFO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_COPYERROR: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_DELETEERROR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDBACKUP: u32 = 23u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDCOPY: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDDELETE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDQUEUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDREGISTRATION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDRENAME: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_ENDSUBQUEUE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_FILEEXTRACTED: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_FILEINCABINET: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_FILEOPDELAYED: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_LANGMISMATCH: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_NEEDMEDIA: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_NEEDNEWCABINET: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_QUEUESCAN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_QUEUESCAN_EX: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_QUEUESCAN_SIGNERINFO: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_RENAMEERROR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTBACKUP: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTCOPY: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTDELETE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTQUEUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTREGISTRATION: u32 = 25u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTRENAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_STARTSUBQUEUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_TARGETEXISTS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILENOTIFY_TARGETNEWER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILEQ_FILE_IN_USE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILEQ_REBOOT_IN_PROGRESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPFILEQ_REBOOT_RECOMMENDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPID_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPID_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPID_REMOVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_ALL: u32 = 2047u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_BITREG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_COPYINF: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_DEVICEINSTALL: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_FILES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_INI2REG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_INIFILES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_LOGCONFIG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_LOGCONFIGS_ARE_OVERRIDES: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_LOGCONFIG_IS_FORCED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_PROFILEITEMS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_PROPERTIES: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_REGISTERCALLBACKAWARE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_REGISTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_REGSVR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_SINGLESECTION: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINST_UNREGSVR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINT_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPINT_REMOVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPOST_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPPSR_ENUM_ADV_DEVICE_PROPERTIES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPPSR_ENUM_BASIC_DEVICE_PROPERTIES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPPSR_SELECT_DEVICE_RESOURCES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_DELAYED_COPY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_ABORT_IF_UNSIGNED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_BACKUP_AWARE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_DO_SHUFFLEMOVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_FILES_MODIFIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_FLAG_VALID: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_ACTIVATE_DRP: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_FILE_COMPARISON: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_FILE_PRESENCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_FILE_PRESENCE_WITHOUT_SOURCE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_FILE_VALIDITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_INFORM_USER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_PRUNE_COPY_QUEUE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_PRUNE_DELREN: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_USE_CALLBACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_USE_CALLBACKEX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPQ_SCAN_USE_CALLBACK_SIGNERINFO: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPRDI_FIND_DUPS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_DLLINSTALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_GETPROCADDR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_LOADLIBRARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_REGSVR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_TIMEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPREG_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_ASSOCSERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_CLOBBER_SECURITY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_DELETEEVENTLOGENTRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_DELAYEDAUTOSTART: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_DEPENDENCIES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_DESCRIPTION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_DISPLAYNAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_ERRORCONTROL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_LOADORDERGROUP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_REQUIREDPRIVILEGES: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_SERVICESIDTYPE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_STARTTYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_NOCLOBBER_TRIGGERS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_STARTSERVICE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_STOPSERVICE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_TAGTOFRONT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPSVCINST_UNIQUE_NAME: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPWPT_SELECTDEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SPWP_USE_DEVINFO_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_ALTPLATFORM_FLAGS_SUITE_MASK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_ALTPLATFORM_FLAGS_VERSION_RANGE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V1 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V1 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Reserved: u16,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V1 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub struct SP_ALTPLATFORM_INFO_V2 {
    pub cbSize: u32,
    pub Platform: super::super::System::Diagnostics::Debug::VER_PLATFORM,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V2_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub union SP_ALTPLATFORM_INFO_V2_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V2_0 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3_0 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_ALTPLATFORM_INFO_V3 {
    pub cbSize: u32,
    pub Platform: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ProcessorArchitecture: u16,
    pub Anonymous: SP_ALTPLATFORM_INFO_V3_0,
    pub FirstValidatedMajorVersion: u32,
    pub FirstValidatedMinorVersion: u32,
    pub ProductType: u8,
    pub SuiteMask: u16,
    pub BuildNumber: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub union SP_ALTPLATFORM_INFO_V3_0 {
    pub Reserved: u16,
    pub Flags: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_ALTPLATFORM_INFO_V3_0 {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_ALTPLATFORM_INFO_V3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_BACKUP_BACKUPPASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_BACKUP_BOOTFILE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_BACKUP_DEMANDPASS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_BACKUP_QUEUE_PARAMS_V1_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V1_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_A {
    pub cbSize: u32,
    pub FullInfPath: [super::super::Foundation::CHAR; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_BACKUP_QUEUE_PARAMS_V2_W {
    pub cbSize: u32,
    pub FullInfPath: [u16; 260],
    pub FilenameOffset: i32,
    pub ReinstallInstance: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_BACKUP_QUEUE_PARAMS_V2_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_BACKUP_QUEUE_PARAMS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_BACKUP_SPECIAL: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::super::UI::Controls::HIMAGELIST,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::marker::Copy for SP_CLASSIMAGELIST_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::clone::Clone for SP_CLASSIMAGELIST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
pub struct SP_CLASSIMAGELIST_DATA {
    pub cbSize: u32,
    pub ImageList: super::super::UI::Controls::HIMAGELIST,
    pub Reserved: usize,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::marker::Copy for SP_CLASSIMAGELIST_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_UI_Controls")]
impl ::core::clone::Clone for SP_CLASSIMAGELIST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_CLASSINSTALL_HEADER {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_CLASSINSTALL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_CLASSINSTALL_HEADER {
    pub cbSize: u32,
    pub InstallFunction: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_CLASSINSTALL_HEADER {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_CLASSINSTALL_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type SP_COPY_STYLE = u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_DELETESOURCE: SP_COPY_STYLE = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_REPLACEONLY: SP_COPY_STYLE = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NEWER_OR_SAME: SP_COPY_STYLE = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NEWER_ONLY: SP_COPY_STYLE = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NOOVERWRITE: SP_COPY_STYLE = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NODECOMP: SP_COPY_STYLE = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_LANGUAGEAWARE: SP_COPY_STYLE = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_SOURCE_ABSOLUTE: SP_COPY_STYLE = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_SOURCEPATH_ABSOLUTE: SP_COPY_STYLE = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_FORCE_IN_USE: SP_COPY_STYLE = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_IN_USE_NEEDS_REBOOT: SP_COPY_STYLE = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NOSKIP: SP_COPY_STYLE = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_FORCE_NOOVERWRITE: SP_COPY_STYLE = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_FORCE_NEWER: SP_COPY_STYLE = 8192u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_WARNIFSKIP: SP_COPY_STYLE = 16384u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NOBROWSE: SP_COPY_STYLE = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NEWER: SP_COPY_STYLE = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_RESERVED: SP_COPY_STYLE = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_OEMINF_CATALOG_ONLY: SP_COPY_STYLE = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_REPLACE_BOOT_FILE: SP_COPY_STYLE = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_NOPRUNE: SP_COPY_STYLE = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_OEM_F6_INF: SP_COPY_STYLE = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_ALREADYDECOMP: SP_COPY_STYLE = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_WINDOWS_SIGNED: SP_COPY_STYLE = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_PNPLOCKED: SP_COPY_STYLE = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_IN_USE_TRY_RENAME: SP_COPY_STYLE = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_INBOX_INF: SP_COPY_STYLE = 134217728u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_COPY_HARDLINK: SP_COPY_STYLE = 268435456u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DETECTDEVICE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DETECTDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DETECTDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub DetectProgressNotify: PDETECT_PROGRESS_NOTIFY,
    pub ProgressNotifyParam: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DETECTDEVICE_PARAMS {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DETECTDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: ::windows_sys::core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DEVICE_INTERFACE_DATA {
    pub cbSize: u32,
    pub InterfaceClassGuid: ::windows_sys::core::GUID,
    pub Flags: u32,
    pub Reserved: usize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DATA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [super::super::Foundation::CHAR; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    pub cbSize: u32,
    pub DevicePath: [super::super::Foundation::CHAR; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    pub cbSize: u32,
    pub DevicePath: [u16; 1],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DEVICE_INTERFACE_DETAIL_DATA_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DEVICE_INTERFACE_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DEVINFO_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DEVINFO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DEVINFO_DATA {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub DevInst: u32,
    pub Reserved: usize,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DEVINFO_DATA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DEVINFO_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [super::super::Foundation::CHAR; 263],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_A {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [super::super::Foundation::CHAR; 263],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINFO_LIST_DETAIL_DATA_W {
    pub cbSize: u32,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub RemoteMachineHandle: super::super::Foundation::HANDLE,
    pub RemoteMachineName: [u16; 263],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINFO_LIST_DETAIL_DATA_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINFO_LIST_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_A {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DEVINSTALL_PARAMS_W {
    pub cbSize: u32,
    pub Flags: u32,
    pub FlagsEx: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub InstallMsgHandler: PSP_FILE_CALLBACK_A,
    pub InstallMsgHandlerContext: *mut ::core::ffi::c_void,
    pub FileQueue: *mut ::core::ffi::c_void,
    pub ClassInstallReserved: usize,
    pub Reserved: u32,
    pub DriverPath: [u16; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DEVINSTALL_PARAMS_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DEVINSTALL_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V1_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DRVINFO_DATA_V1_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V1_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_A {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [super::super::Foundation::CHAR; 256],
    pub MfgName: [super::super::Foundation::CHAR; 256],
    pub ProviderName: [super::super::Foundation::CHAR; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DATA_V2_W {
    pub cbSize: u32,
    pub DriverType: u32,
    pub Reserved: usize,
    pub Description: [u16; 256],
    pub MfgName: [u16; 256],
    pub ProviderName: [u16; 256],
    pub DriverDate: super::super::Foundation::FILETIME,
    pub DriverVersion: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DATA_V2_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DATA_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [super::super::Foundation::CHAR; 256],
    pub InfFileName: [super::super::Foundation::CHAR; 260],
    pub DrvDescription: [super::super::Foundation::CHAR; 256],
    pub HardwareID: [super::super::Foundation::CHAR; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_A {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [super::super::Foundation::CHAR; 256],
    pub InfFileName: [super::super::Foundation::CHAR; 260],
    pub DrvDescription: [super::super::Foundation::CHAR; 256],
    pub HardwareID: [super::super::Foundation::CHAR; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_DRVINFO_DETAIL_DATA_W {
    pub cbSize: u32,
    pub InfDate: super::super::Foundation::FILETIME,
    pub CompatIDsOffset: u32,
    pub CompatIDsLength: u32,
    pub Reserved: usize,
    pub SectionName: [u16; 256],
    pub InfFileName: [u16; 260],
    pub DrvDescription: [u16; 256],
    pub HardwareID: [u16; 1],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_DRVINFO_DETAIL_DATA_W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_DRVINFO_DETAIL_DATA_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: u32,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_DRVINSTALL_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_DRVINSTALL_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_DRVINSTALL_PARAMS {
    pub cbSize: u32,
    pub Rank: u32,
    pub Flags: u32,
    pub PrivateData: usize,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_DRVINSTALL_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_DRVINSTALL_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub EnableMessage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_ENABLECLASS_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_ENABLECLASS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_ENABLECLASS_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ClassGuid: ::windows_sys::core::GUID,
    pub EnableMessage: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_ENABLECLASS_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_ENABLECLASS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: ::windows_sys::core::PCSTR,
    pub SourcePath: ::windows_sys::core::PCSTR,
    pub SourceFilename: ::windows_sys::core::PCSTR,
    pub SourceDescription: ::windows_sys::core::PCSTR,
    pub SourceTagfile: ::windows_sys::core::PCSTR,
    pub TargetDirectory: ::windows_sys::core::PCSTR,
    pub TargetFilename: ::windows_sys::core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: ::windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_FILE_COPY_PARAMS_A {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: ::windows_sys::core::PCSTR,
    pub SourcePath: ::windows_sys::core::PCSTR,
    pub SourceFilename: ::windows_sys::core::PCSTR,
    pub SourceDescription: ::windows_sys::core::PCSTR,
    pub SourceTagfile: ::windows_sys::core::PCSTR,
    pub TargetDirectory: ::windows_sys::core::PCSTR,
    pub TargetFilename: ::windows_sys::core::PCSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: ::windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_A {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: ::windows_sys::core::PCWSTR,
    pub SourcePath: ::windows_sys::core::PCWSTR,
    pub SourceFilename: ::windows_sys::core::PCWSTR,
    pub SourceDescription: ::windows_sys::core::PCWSTR,
    pub SourceTagfile: ::windows_sys::core::PCWSTR,
    pub TargetDirectory: ::windows_sys::core::PCWSTR,
    pub TargetFilename: ::windows_sys::core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: ::windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_FILE_COPY_PARAMS_W {
    pub cbSize: u32,
    pub QueueHandle: *mut ::core::ffi::c_void,
    pub SourceRootPath: ::windows_sys::core::PCWSTR,
    pub SourcePath: ::windows_sys::core::PCWSTR,
    pub SourceFilename: ::windows_sys::core::PCWSTR,
    pub SourceDescription: ::windows_sys::core::PCWSTR,
    pub SourceTagfile: ::windows_sys::core::PCWSTR,
    pub TargetDirectory: ::windows_sys::core::PCWSTR,
    pub TargetFilename: ::windows_sys::core::PCWSTR,
    pub CopyStyle: u32,
    pub LayoutInf: *mut ::core::ffi::c_void,
    pub SecurityDescriptor: ::windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_FILE_COPY_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_FILE_COPY_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_FLAG_CABINETCONTINUATION: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_INF_INFORMATION {
    pub InfStyle: SP_INF_STYLE,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_INF_INFORMATION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_INF_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_INF_INFORMATION {
    pub InfStyle: SP_INF_STYLE,
    pub InfCount: u32,
    pub VersionData: [u8; 1],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_INF_INFORMATION {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_INF_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V1_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_INF_SIGNER_INFO_V1_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V1_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_INF_SIGNER_INFO_V2_A {
    pub cbSize: u32,
    pub CatalogFile: [super::super::Foundation::CHAR; 260],
    pub DigitalSigner: [super::super::Foundation::CHAR; 260],
    pub DigitalSignerVersion: [super::super::Foundation::CHAR; 260],
    pub SignerScore: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_INF_SIGNER_INFO_V2_W {
    pub cbSize: u32,
    pub CatalogFile: [u16; 260],
    pub DigitalSigner: [u16; 260],
    pub DigitalSignerVersion: [u16; 260],
    pub SignerScore: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_INF_SIGNER_INFO_V2_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_INF_SIGNER_INFO_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type SP_INF_STYLE = u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_NONE: SP_INF_STYLE = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_OLDNT: SP_INF_STYLE = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const INF_STYLE_WIN4: SP_INF_STYLE = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::super::Foundation::LPARAM,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_INSTALLWIZARD_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_INSTALLWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_INSTALLWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub DynamicPageFlags: u32,
    pub PrivateFlags: u32,
    pub PrivateData: super::super::Foundation::LPARAM,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_INSTALLWIZARD_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_INSTALLWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SP_MAX_MACHINENAME_LENGTH: u32 = 263u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_NEWDEVICEWIZARD_DATA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_NEWDEVICEWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct SP_NEWDEVICEWIZARD_DATA {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Flags: u32,
    pub DynamicPages: [super::super::UI::Controls::HPROPSHEETPAGE; 20],
    pub NumDynamicPages: u32,
    pub hwndWizardDlg: super::super::Foundation::HWND,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for SP_NEWDEVICEWIZARD_DATA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for SP_NEWDEVICEWIZARD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [super::super::Foundation::CHAR; 260],
    pub OriginalCatalogName: [super::super::Foundation::CHAR; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_ORIGINAL_FILE_INFO_A {
    pub cbSize: u32,
    pub OriginalInfName: [super::super::Foundation::CHAR; 260],
    pub OriginalCatalogName: [super::super::Foundation::CHAR; 260],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_ORIGINAL_FILE_INFO_W {
    pub cbSize: u32,
    pub OriginalInfName: [u16; 260],
    pub OriginalCatalogName: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_ORIGINAL_FILE_INFO_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_ORIGINAL_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_POWERMESSAGEWAKE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [super::super::Foundation::CHAR; 512],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_POWERMESSAGEWAKE_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_POWERMESSAGEWAKE_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_POWERMESSAGEWAKE_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_POWERMESSAGEWAKE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub PowerMessageWake: [u16; 512],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_POWERMESSAGEWAKE_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_POWERMESSAGEWAKE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: u32,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_PROPCHANGE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_PROPCHANGE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_PROPCHANGE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub StateChange: u32,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_PROPCHANGE_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_PROPCHANGE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: HDEVINFO,
    pub DeviceInfoData: *mut SP_DEVINFO_DATA,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_PROPSHEETPAGE_REQUEST {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_PROPSHEETPAGE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_PROPSHEETPAGE_REQUEST {
    pub cbSize: u32,
    pub PageRequested: u32,
    pub DeviceInfoSet: HDEVINFO,
    pub DeviceInfoData: *mut SP_DEVINFO_DATA,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_PROPSHEETPAGE_REQUEST {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_PROPSHEETPAGE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: ::windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_REGISTER_CONTROL_STATUSA {
    pub cbSize: u32,
    pub FileName: ::windows_sys::core::PCSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSA {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: ::windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_REGISTER_CONTROL_STATUSW {
    pub cbSize: u32,
    pub FileName: ::windows_sys::core::PCWSTR,
    pub Win32Error: u32,
    pub FailureCode: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_REGISTER_CONTROL_STATUSW {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_REGISTER_CONTROL_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_REMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_REMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_REMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_REMOVEDEVICE_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_REMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_SELECTDEVICE_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [super::super::Foundation::CHAR; 60],
    pub Instructions: [super::super::Foundation::CHAR; 256],
    pub ListLabel: [super::super::Foundation::CHAR; 30],
    pub SubTitle: [super::super::Foundation::CHAR; 256],
    pub Reserved: [u8; 2],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_SELECTDEVICE_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_SELECTDEVICE_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_SELECTDEVICE_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_SELECTDEVICE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_SELECTDEVICE_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Title: [u16; 60],
    pub Instructions: [u16; 256],
    pub ListLabel: [u16; 30],
    pub SubTitle: [u16; 256],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_SELECTDEVICE_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_SELECTDEVICE_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SP_TROUBLESHOOTER_PARAMS_A {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [super::super::Foundation::CHAR; 260],
    pub HtmlTroubleShooter: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SP_TROUBLESHOOTER_PARAMS_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SP_TROUBLESHOOTER_PARAMS_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_TROUBLESHOOTER_PARAMS_W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_TROUBLESHOOTER_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_TROUBLESHOOTER_PARAMS_W {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub ChmFile: [u16; 260],
    pub HtmlTroubleShooter: [u16; 260],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_TROUBLESHOOTER_PARAMS_W {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_TROUBLESHOOTER_PARAMS_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SP_UNREMOVEDEVICE_PARAMS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SP_UNREMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
#[cfg(target_arch = "x86")]
pub struct SP_UNREMOVEDEVICE_PARAMS {
    pub ClassInstallHeader: SP_CLASSINSTALL_HEADER,
    pub Scope: u32,
    pub HwProfile: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for SP_UNREMOVEDEVICE_PARAMS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for SP_UNREMOVEDEVICE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_DESCRIPTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_FLAGS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_PATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_TAGFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCINFO_TAGFILE2: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_APPEND: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_NOBROWSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_NOSTRIPPLATFORM: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_SUBDIRS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_SYSIFADMIN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_SYSTEM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_TEMPORARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRCLIST_USER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SRC_FLAGS_CABFILE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SUOI_FORCEDELETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SUOI_INTERNAL1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDAUTOLOGGER: &str = "AddAutoLogger";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDAUTOLOGGERPROVIDER: &str = "AddAutoLoggerProvider";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDCHANNEL: &str = "AddChannel";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDEVENTPROVIDER: &str = "AddEventProvider";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDFILTER: &str = "AddFilter";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDIME: &str = "AddIme";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDINTERFACE: &str = "AddInterface";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDPOWERSETTING: &str = "AddPowerSetting";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDPROP: &str = "AddProperty";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDREG: &str = "AddReg";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDREGNOCLOBBER: &str = "AddRegNoClobber";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDSERVICE: &str = "AddService";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_ADDTRIGGER: &str = "AddTrigger";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_BITREG: &str = "BitReg";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_CLEANONLY: &str = "CleanOnly";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_COPYFILES: &str = "CopyFiles";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_COPYINF: &str = "CopyINF";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DEFAULTOPTION: &str = "DefaultOption";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DEFDESTDIR: &str = "DefaultDestDir";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELFILES: &str = "DelFiles";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELIME: &str = "DelIme";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELPROP: &str = "DelProperty";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELREG: &str = "DelReg";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DELSERVICE: &str = "DelService";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_DESTDIRS: &str = "DestinationDirs";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_EXCLUDEID: &str = "ExcludeId";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_FEATURESCORE: &str = "FeatureScore";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_FILTERLEVEL: &str = "FilterLevel";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_FILTERPOSITION: &str = "FilterPosition";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_HARDWARE: &str = "Hardware";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_IMPORTCHANNEL: &str = "ImportChannel";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_INI2REG: &str = "Ini2Reg";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LAYOUT_FILE: &str = "LayoutFile";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LDIDOEM: &str = "LdidOEM";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LFN_SECTION: &str = "VarLDID.LFN";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LISTOPTIONS: &str = "ListOptions";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_LOGCONFIG: &str = "LogConfig";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_MODULES: &str = "Modules";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_OPTIONDESC: &str = "OptionDesc";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_PHASE1: &str = "Phase1";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_PROFILEITEMS: &str = "ProfileItems";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_REGSVR: &str = "RegisterDlls";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_RENFILES: &str = "RenFiles";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_SFN_SECTION: &str = "VarLDID.SFN";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_SRCDISKFILES: &str = "SourceDisksFiles";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_SRCDISKNAMES: &str = "SourceDisksNames";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_STRINGS: &str = "Strings";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UNREGSVR: &str = "UnregisterDlls";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UPDATEAUTOLOGGER: &str = "UpdateAutoLogger";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UPDATEINIFIELDS: &str = "UpdateIniFields";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UPDATEINIS: &str = "UpdateInis";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SZ_KEY_UPGRADEONLY: &str = "UpgradeOnly";
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub type SetupFileLogInfo = i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogSourceFilename: SetupFileLogInfo = 0i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogChecksum: SetupFileLogInfo = 1i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogDiskTagfile: SetupFileLogInfo = 2i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogDiskDescription: SetupFileLogInfo = 3i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogOtherInfo: SetupFileLogInfo = 4i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const SetupFileLogMax: SetupFileLogInfo = 5i32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_BYTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_BYTE_AND_WORD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_BusMaster: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_DWORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_NoBusMaster: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_TypeA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_TypeB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_TypeF: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_TypeStandard: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fDD_WORD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_10_BIT_DECODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_12_BIT_DECODE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_16_BIT_DECODE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_DECODE: u32 = 252u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_IO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_Memory: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_PASSIVE_DECODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_PORT_BAR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_POSITIVE_DECODE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_PortType: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIOD_WINDOW_DECODE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Edge: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Exclusive: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Level: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Level_Bit: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Share: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fIRQD_Share_Bit: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_24: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_32: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_32_24: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_Cacheable: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_CombinedWrite: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_CombinedWriteAllowed: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_CombinedWriteDisallowed: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_MEMORY_BAR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_MemoryType: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_NonCacheable: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_Pref: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_PrefetchAllowed: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_PrefetchDisallowed: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_Prefetchable: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_RAM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_ROM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_ReadAllowed: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_ReadDisallowed: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_Readable: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fMD_WINDOW_DECODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_ATTRIBUTES_PER_WINDOW: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO1_16: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO1_SRC_16: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO1_WS_16: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO1_ZW_8: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO2_16: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO2_SRC_16: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO2_WS_16: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO2_ZW_8: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_16: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_8: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_SRC_16: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_WS_16: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_IO_ZW_8: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_16: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_A: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_WS_ONE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_WS_THREE: u32 = 50331648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM1_WS_TWO: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_16: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_A: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_WS_ONE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_WS_THREE: u32 = 805306368u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM2_WS_TWO: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_16: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_8: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_A: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_WS_ONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_WS_THREE: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPCD_MEM_WS_TWO: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const fPMF_AUDIO_ENABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mDD_BusMaster: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mDD_Type: u32 = 24u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mDD_Width: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mIRQD_Edge_Level: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mIRQD_Share: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_32_24: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_Cacheable: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_CombinedWrite: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_MemoryType: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_Prefetchable: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mMD_Readable: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_IO_8_16: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM1_WS: u32 = 50331648u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM2_WS: u32 = 805306368u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM_8_16: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM_A_C: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPCD_MEM_WS: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Devices_DeviceAndDriverInstallation\"`*"]
pub const mPMF_AUDIO_ENABLE: u32 = 8u32;
