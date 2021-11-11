#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CMP_WaitNoPendingInstallEvents(dwtimeout: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Data_HtmlHelp`*"]
    #[cfg(feature = "Win32_Data_HtmlHelp")]
    pub fn CM_Add_Empty_Log_Conf(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Data_HtmlHelp`*"]
    #[cfg(feature = "Win32_Data_HtmlHelp")]
    pub fn CM_Add_Empty_Log_Conf_Ex(plclogconf: *mut usize, dndevinst: u32, priority: super::super::Data::HtmlHelp::PRIORITY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Add_IDA(dndevinst: u32, pszid: super::super::Foundation::PSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Add_IDW(dndevinst: u32, pszid: super::super::Foundation::PWSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Add_ID_ExA(dndevinst: u32, pszid: super::super::Foundation::PSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Add_ID_ExW(dndevinst: u32, pszid: super::super::Foundation::PWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Add_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Add_Res_Des(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Add_Res_Des_Ex(prdresdes: *mut usize, lclogconf: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Connect_MachineA(uncservername: super::super::Foundation::PSTR, phmachine: *mut isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Connect_MachineW(uncservername: super::super::Foundation::PWSTR, phmachine: *mut isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_DevNodeA(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_DevNodeW(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_DevNode_ExA(pdndevinst: *mut u32, pdeviceid: *const i8, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_DevNode_ExW(pdndevinst: *mut u32, pdeviceid: *const u16, dnparent: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_Range_List(prlh: *mut usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_Class_Key(classguid: *const ::windows::runtime::GUID, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_Class_Key_Ex(classguid: *const ::windows::runtime::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_DevNode_Key(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_DevNode_Key_Ex(dndevnode: u32, ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Delete_Device_Interface_KeyA(pszdeviceinterface: super::super::Foundation::PSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Delete_Device_Interface_KeyW(pszdeviceinterface: super::super::Foundation::PWSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Delete_Device_Interface_Key_ExA(pszdeviceinterface: super::super::Foundation::PSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Delete_Device_Interface_Key_ExW(pszdeviceinterface: super::super::Foundation::PWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_Range(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Detect_Resource_Conflict(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Detect_Resource_Conflict_Ex(dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, pbconflictdetected: *mut super::super::Foundation::BOOL, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Disable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Disable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Disconnect_Machine(hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Dup_Range_List(rlhold: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Enable_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Enable_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Enumerate_Classes(ulclassindex: u32, classguid: *mut ::windows::runtime::GUID, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Enumerate_Classes_Ex(ulclassindex: u32, classguid: *mut ::windows::runtime::GUID, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Enumerate_EnumeratorsA(ulenumindex: u32, buffer: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Enumerate_EnumeratorsW(ulenumindex: u32, buffer: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Enumerate_Enumerators_ExA(ulenumindex: u32, buffer: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Enumerate_Enumerators_ExW(ulenumindex: u32, buffer: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Find_Range(pullstart: *mut u64, ullstart: u64, ullength: u32, ullalignment: u64, ullend: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_First_Range(rlh: usize, pullstart: *mut u64, pullend: *mut u64, preelement: *mut usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Log_Conf(lclogconftobefreed: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Log_Conf_Ex(lclogconftobefreed: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Log_Conf_Handle(lclogconf: usize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Range_List(rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Res_Des(prdresdes: *mut usize, rdresdes: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Res_Des_Handle(rdresdes: usize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Resource_Conflict_Handle(clconflictlist: usize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Child(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Child_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Key_NameA(classguid: *const ::windows::runtime::GUID, pszkeyname: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Key_NameW(classguid: *const ::windows::runtime::GUID, pszkeyname: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Key_Name_ExA(classguid: *const ::windows::runtime::GUID, pszkeyname: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Key_Name_ExW(classguid: *const ::windows::runtime::GUID, pszkeyname: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_NameA(classguid: *const ::windows::runtime::GUID, buffer: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_NameW(classguid: *const ::windows::runtime::GUID, buffer: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Name_ExA(classguid: *const ::windows::runtime::GUID, buffer: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Name_ExW(classguid: *const ::windows::runtime::GUID, buffer: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_PropertyW(classguid: *const ::windows::runtime::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_Property_ExW(classguid: *const ::windows::runtime::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_Property_Keys(classguid: *const ::windows::runtime::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_Property_Keys_Ex(classguid: *const ::windows::runtime::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Class_Registry_PropertyA(classguid: *const ::windows::runtime::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Class_Registry_PropertyW(classguid: *const ::windows::runtime::GUID, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Depth(puldepth: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Depth_Ex(puldepth: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_DevNode_Custom_PropertyA(dndevinst: u32, pszcustompropertyname: super::super::Foundation::PSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_DevNode_Custom_PropertyW(dndevinst: u32, pszcustompropertyname: super::super::Foundation::PWSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_DevNode_Custom_Property_ExA(dndevinst: u32, pszcustompropertyname: super::super::Foundation::PSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_DevNode_Custom_Property_ExW(dndevinst: u32, pszcustompropertyname: super::super::Foundation::PWSTR, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_Property_Keys(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_Property_Keys_Ex(dndevinst: u32, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, pulregdatatype: *mut u32, buffer: *mut ::core::ffi::c_void, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Status(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Status_Ex(pulstatus: *mut u32, pulproblemnumber: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_IDA(dndevinst: u32, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_IDW(dndevinst: u32, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_ExA(dndevinst: u32, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_ExW(dndevinst: u32, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_ListA(pszfilter: super::super::Foundation::PSTR, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_ListW(pszfilter: super::super::Foundation::PWSTR, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_ExA(pszfilter: super::super::Foundation::PSTR, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_ExW(pszfilter: super::super::Foundation::PWSTR, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_SizeA(pullen: *mut u32, pszfilter: super::super::Foundation::PSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_SizeW(pullen: *mut u32, pszfilter: super::super::Foundation::PWSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_Size_ExA(pullen: *mut u32, pszfilter: super::super::Foundation::PSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_Size_ExW(pullen: *mut u32, pszfilter: super::super::Foundation::PWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_ID_Size(pullen: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_ID_Size_Ex(pullen: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_AliasA(pszdeviceinterface: super::super::Foundation::PSTR, aliasinterfaceguid: *const ::windows::runtime::GUID, pszaliasdeviceinterface: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_AliasW(pszdeviceinterface: super::super::Foundation::PWSTR, aliasinterfaceguid: *const ::windows::runtime::GUID, pszaliasdeviceinterface: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_Alias_ExA(pszdeviceinterface: super::super::Foundation::PSTR, aliasinterfaceguid: *const ::windows::runtime::GUID, pszaliasdeviceinterface: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_Alias_ExW(pszdeviceinterface: super::super::Foundation::PWSTR, aliasinterfaceguid: *const ::windows::runtime::GUID, pszaliasdeviceinterface: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_ListA(interfaceclassguid: *const ::windows::runtime::GUID, pdeviceid: *const i8, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_ListW(interfaceclassguid: *const ::windows::runtime::GUID, pdeviceid: *const u16, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_List_ExA(interfaceclassguid: *const ::windows::runtime::GUID, pdeviceid: *const i8, buffer: super::super::Foundation::PSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_List_ExW(interfaceclassguid: *const ::windows::runtime::GUID, pdeviceid: *const u16, buffer: super::super::Foundation::PWSTR, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_Interface_List_SizeA(pullen: *mut u32, interfaceclassguid: *const ::windows::runtime::GUID, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_Interface_List_SizeW(pullen: *mut u32, interfaceclassguid: *const ::windows::runtime::GUID, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_Interface_List_Size_ExA(pullen: *mut u32, interfaceclassguid: *const ::windows::runtime::GUID, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_Interface_List_Size_ExW(pullen: *mut u32, interfaceclassguid: *const ::windows::runtime::GUID, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Get_Device_Interface_PropertyW(pszdeviceinterface: super::super::Foundation::PWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Get_Device_Interface_Property_ExW(pszdeviceinterface: super::super::Foundation::PWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Get_Device_Interface_Property_KeysW(pszdeviceinterface: super::super::Foundation::PWSTR, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Get_Device_Interface_Property_Keys_ExW(pszdeviceinterface: super::super::Foundation::PWSTR, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_First_Log_Conf(plclogconf: *mut usize, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_First_Log_Conf_Ex(plclogconf: *mut usize, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Global_State(pulstate: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Global_State_Ex(pulstate: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_HW_Prof_FlagsA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_HW_Prof_FlagsW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulhardwareprofile: u32, pulvalue: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Hardware_Profile_InfoA(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Hardware_Profile_InfoW(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Hardware_Profile_Info_ExA(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sA, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Hardware_Profile_Info_ExW(ulindex: u32, phwprofileinfo: *mut HWProfileInfo_sW, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Log_Conf_Priority(lclogconf: usize, ppriority: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Log_Conf_Priority_Ex(lclogconf: usize, ppriority: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Next_Log_Conf(plclogconf: *mut usize, lclogconf: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Next_Log_Conf_Ex(plclogconf: *mut usize, lclogconf: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Next_Res_Des(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Next_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, forresource: u32, presourceid: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Parent(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Parent_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Res_Des_Data(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Res_Des_Data_Ex(rdresdes: usize, buffer: *mut ::core::ffi::c_void, bufferlen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Res_Des_Data_Size(pulsize: *mut u32, rdresdes: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Res_Des_Data_Size_Ex(pulsize: *mut u32, rdresdes: usize, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Resource_Conflict_Count(clconflictlist: usize, pulcount: *mut u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Resource_Conflict_DetailsA(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_A) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Resource_Conflict_DetailsW(clconflictlist: usize, ulindex: u32, pconflictdetails: *mut CONFLICT_DETAILS_W) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Sibling(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Sibling_Ex(pdndevinst: *mut u32, dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Version() -> u16;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Version_Ex(hmachine: isize) -> u16;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Intersect_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Invert_Range_List(rlhold: usize, rlhnew: usize, ullmaxvalue: u64, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Dock_Station_Present(pbpresent: *mut super::super::Foundation::BOOL) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Dock_Station_Present_Ex(pbpresent: *mut super::super::Foundation::BOOL, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Version_Available(wversion: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Version_Available_Ex(wversion: u16, hmachine: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Locate_DevNodeA(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Locate_DevNodeW(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Locate_DevNode_ExA(pdndevinst: *mut u32, pdeviceid: *const i8, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Locate_DevNode_ExW(pdndevinst: *mut u32, pdeviceid: *const u16, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_MapCrToWin32Err(cmreturncode: CONFIGRET, defaulterr: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Merge_Range_List(rlhold1: usize, rlhold2: usize, rlhnew: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Modify_Res_Des(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Modify_Res_Des_Ex(prdresdes: *mut usize, rdresdes: usize, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Move_DevNode(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Move_DevNode_Ex(dnfromdevinst: u32, dntodevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Next_Range(preelement: *mut usize, pullstart: *mut u64, pullend: *mut u64, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Class_KeyA(classguid: *const ::windows::runtime::GUID, pszclassname: super::super::Foundation::PSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Class_KeyW(classguid: *const ::windows::runtime::GUID, pszclassname: super::super::Foundation::PWSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Class_Key_ExA(classguid: *const ::windows::runtime::GUID, pszclassname: super::super::Foundation::PSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Class_Key_ExW(classguid: *const ::windows::runtime::GUID, pszclassname: super::super::Foundation::PWSTR, samdesired: u32, disposition: u32, phkclass: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_DevNode_Key(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_DevNode_Key_Ex(dndevnode: u32, samdesired: u32, ulhardwareprofile: u32, disposition: u32, phkdevice: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Device_Interface_KeyA(pszdeviceinterface: super::super::Foundation::PSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Device_Interface_KeyW(pszdeviceinterface: super::super::Foundation::PWSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Device_Interface_Key_ExA(pszdeviceinterface: super::super::Foundation::PSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Device_Interface_Key_ExW(pszdeviceinterface: super::super::Foundation::PWSTR, samdesired: u32, disposition: u32, phkdeviceinterface: *mut super::super::System::Registry::HKEY, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Query_And_Remove_SubTreeA(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Query_And_Remove_SubTreeW(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PWSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Query_And_Remove_SubTree_ExA(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Query_And_Remove_SubTree_ExW(dnancestor: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PWSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Arbitrator_Free_Data(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Arbitrator_Free_Data_Ex(pdata: *mut ::core::ffi::c_void, datalen: u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Arbitrator_Free_Size(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Arbitrator_Free_Size_Ex(pulsize: *mut u32, dndevinst: u32, resourceid: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Resource_Conflict_List(pclconflictlist: *mut usize, dndevinst: u32, resourceid: u32, resourcedata: *const ::core::ffi::c_void, resourcelen: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Reenumerate_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Reenumerate_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Register_Device_Driver(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Register_Device_Driver_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Device_InterfaceA(dndevinst: u32, interfaceclassguid: *const ::windows::runtime::GUID, pszreference: super::super::Foundation::PSTR, pszdeviceinterface: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Device_InterfaceW(dndevinst: u32, interfaceclassguid: *const ::windows::runtime::GUID, pszreference: super::super::Foundation::PWSTR, pszdeviceinterface: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Device_Interface_ExA(dndevinst: u32, interfaceclassguid: *const ::windows::runtime::GUID, pszreference: super::super::Foundation::PSTR, pszdeviceinterface: super::super::Foundation::PSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Device_Interface_ExW(dndevinst: u32, interfaceclassguid: *const ::windows::runtime::GUID, pszreference: super::super::Foundation::PWSTR, pszdeviceinterface: super::super::Foundation::PWSTR, pullength: *mut u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Notification(pfilter: *const CM_NOTIFY_FILTER, pcontext: *const ::core::ffi::c_void, pcallback: ::windows::runtime::RawPtr, pnotifycontext: *mut isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Remove_SubTree(dnancestor: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Remove_SubTree_Ex(dnancestor: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Request_Device_EjectA(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Request_Device_EjectW(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PWSTR, ulnamelength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Request_Device_Eject_ExA(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Request_Device_Eject_ExW(dndevinst: u32, pvetotype: *mut PNP_VETO_TYPE, pszvetoname: super::super::Foundation::PWSTR, ulnamelength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Request_Eject_PC() -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Request_Eject_PC_Ex(hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Run_Detection(ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Run_Detection_Ex(ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_Class_PropertyW(classguid: *const ::windows::runtime::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_Class_Property_ExW(classguid: *const ::windows::runtime::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_Class_Registry_PropertyA(classguid: *const ::windows::runtime::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_Class_Registry_PropertyW(classguid: *const ::windows::runtime::GUID, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Problem(dndevinst: u32, ulproblem: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Problem_Ex(dndevinst: u32, ulproblem: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_DevNode_PropertyW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_DevNode_Property_ExW(dndevinst: u32, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Registry_PropertyA(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Registry_PropertyW(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Registry_Property_ExA(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Registry_Property_ExW(dndevinst: u32, ulproperty: u32, buffer: *const ::core::ffi::c_void, ullength: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Set_Device_Interface_PropertyW(pszdeviceinterface: super::super::Foundation::PWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Set_Device_Interface_Property_ExW(pszdeviceinterface: super::super::Foundation::PWSTR, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof(ulhardwareprofile: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_Ex(ulhardwareprofile: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_FlagsA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_FlagsW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_Flags_ExA(pdeviceid: *const i8, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_Flags_ExW(pdeviceid: *const u16, ulconfig: u32, ulvalue: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Setup_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Setup_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Test_Range_Available(ullstartvalue: u64, ullendvalue: u64, rlh: usize, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Uninstall_DevNode(dndevinst: u32, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Uninstall_DevNode_Ex(dndevinst: u32, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Unregister_Device_InterfaceA(pszdeviceinterface: super::super::Foundation::PSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Unregister_Device_InterfaceW(pszdeviceinterface: super::super::Foundation::PWSTR, ulflags: u32) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Unregister_Device_Interface_ExA(pszdeviceinterface: super::super::Foundation::PSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Unregister_Device_Interface_ExW(pszdeviceinterface: super::super::Foundation::PWSTR, ulflags: u32, hmachine: isize) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Unregister_Notification(notifycontext: HCMNOTIFICATION) -> CONFIGRET;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiInstallDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiInstallDriverA(hwndparent: super::super::Foundation::HWND, infpath: super::super::Foundation::PSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiInstallDriverW(hwndparent: super::super::Foundation::HWND, infpath: super::super::Foundation::PWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiRollbackDriver(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, hwndparent: super::super::Foundation::HWND, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiShowUpdateDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiShowUpdateDriver(hwndparent: super::super::Foundation::HWND, filepath: super::super::Foundation::PWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiUninstallDevice(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiUninstallDriverA(hwndparent: super::super::Foundation::HWND, infpath: super::super::Foundation::PSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiUninstallDriverW(hwndparent: super::super::Foundation::HWND, infpath: super::super::Foundation::PWSTR, flags: u32, needreboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallHinfSectionA(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: super::super::Foundation::PSTR, showcommand: i32);
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallHinfSectionW(window: super::super::Foundation::HWND, modulehandle: super::super::Foundation::HINSTANCE, commandline: super::super::Foundation::PWSTR, showcommand: i32);
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddInstallSectionToDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddInstallSectionToDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddSectionToDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddSectionToDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToDiskSpaceListA(diskspace: *const ::core::ffi::c_void, targetfilespec: super::super::Foundation::PSTR, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToDiskSpaceListW(diskspace: *const ::core::ffi::c_void, targetfilespec: super::super::Foundation::PWSTR, filesize: i64, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToSourceListA(flags: u32, source: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToSourceListW(flags: u32, source: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAdjustDiskSpaceListA(diskspace: *const ::core::ffi::c_void, driveroot: super::super::Foundation::PSTR, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAdjustDiskSpaceListW(diskspace: *const ::core::ffi::c_void, driveroot: super::super::Foundation::PWSTR, amount: i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupBackupErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, sourcefile: super::super::Foundation::PSTR, targetfile: super::super::Foundation::PSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupBackupErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, sourcefile: super::super::Foundation::PWSTR, targetfile: super::super::Foundation::PWSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCancelTemporarySourceList() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCloseFileQueue(queuehandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupCloseInfFile(infhandle: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupCloseLog();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCommitFileQueueA(owner: super::super::Foundation::HWND, queuehandle: *const ::core::ffi::c_void, msghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCommitFileQueueW(owner: super::super::Foundation::HWND, queuehandle: *const ::core::ffi::c_void, msghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupConfigureWmiFromInfSectionA(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupConfigureWmiFromInfSectionW(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, diskname: super::super::Foundation::PSTR, pathtosource: super::super::Foundation::PSTR, sourcefile: super::super::Foundation::PSTR, targetpathfile: super::super::Foundation::PSTR, win32errorcode: u32, style: u32, pathbuffer: super::super::Foundation::PSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, diskname: super::super::Foundation::PWSTR, pathtosource: super::super::Foundation::PWSTR, sourcefile: super::super::Foundation::PWSTR, targetpathfile: super::super::Foundation::PWSTR, win32errorcode: u32, style: u32, pathbuffer: super::super::Foundation::PWSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyOEMInfA(sourceinffilename: super::super::Foundation::PSTR, oemsourcemedialocation: super::super::Foundation::PSTR, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: super::super::Foundation::PSTR, destinationinffilenamesize: u32, requiredsize: *mut u32, destinationinffilenamecomponent: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyOEMInfW(sourceinffilename: super::super::Foundation::PWSTR, oemsourcemedialocation: super::super::Foundation::PWSTR, oemsourcemediatype: OEM_SOURCE_MEDIA_TYPE, copystyle: u32, destinationinffilename: super::super::Foundation::PWSTR, destinationinffilenamesize: u32, requiredsize: *mut u32, destinationinffilenamecomponent: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupCreateDiskSpaceListA(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupCreateDiskSpaceListW(reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDecompressOrCopyFileA(sourcefilename: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, compressiontype: *const u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDecompressOrCopyFileW(sourcefilename: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, compressiontype: *const u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupDefaultQueueCallbackA(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupDefaultQueueCallbackW(context: *const ::core::ffi::c_void, notification: u32, param1: usize, param2: usize) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDeleteErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, file: super::super::Foundation::PSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDeleteErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, file: super::super::Foundation::PWSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDestroyDiskSpaceList(diskspace: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiAskForOEMDisk(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildClassInfoList(flags: u32, classguidlist: *mut ::windows::runtime::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildClassInfoListExA(flags: u32, classguidlist: *mut ::windows::runtime::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildClassInfoListExW(flags: u32, classguidlist: *mut ::windows::runtime::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildDriverInfoList(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, drivertype: SETUP_DI_BUILD_DRIVER_DRIVER_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCallClassInstaller(installfunction: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCancelDriverInfoSearch(deviceinfoset: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiChangeState(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameA(classname: super::super::Foundation::PSTR, classguidlist: *mut ::windows::runtime::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameExA(classname: super::super::Foundation::PSTR, classguidlist: *mut ::windows::runtime::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameExW(classname: super::super::Foundation::PWSTR, classguidlist: *mut ::windows::runtime::GUID, classguidlistsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameW(classname: super::super::Foundation::PWSTR, classguidlist: *mut ::windows::runtime::GUID, classguidlistsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidA(classguid: *const ::windows::runtime::GUID, classname: super::super::Foundation::PSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidExA(classguid: *const ::windows::runtime::GUID, classname: super::super::Foundation::PSTR, classnamesize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidExW(classguid: *const ::windows::runtime::GUID, classname: super::super::Foundation::PWSTR, classnamesize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidW(classguid: *const ::windows::runtime::GUID, classname: super::super::Foundation::PWSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiCreateDevRegKeyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PSTR) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiCreateDevRegKeyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PWSTR) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoA(deviceinfoset: *const ::core::ffi::c_void, devicename: super::super::Foundation::PSTR, classguid: *const ::windows::runtime::GUID, devicedescription: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoList(classguid: *const ::windows::runtime::GUID, hwndparent: super::super::Foundation::HWND) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoListExA(classguid: *const ::windows::runtime::GUID, hwndparent: super::super::Foundation::HWND, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoListExW(classguid: *const ::windows::runtime::GUID, hwndparent: super::super::Foundation::HWND, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoW(deviceinfoset: *const ::core::ffi::c_void, devicename: super::super::Foundation::PWSTR, classguid: *const ::windows::runtime::GUID, devicedescription: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, creationflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInterfaceA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows::runtime::GUID, referencestring: super::super::Foundation::PSTR, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiCreateDeviceInterfaceRegKeyA(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PSTR) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiCreateDeviceInterfaceRegKeyW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32, infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PWSTR) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInterfaceW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows::runtime::GUID, referencestring: super::super::Foundation::PWSTR, creationflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDevRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDeviceInfo(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDeviceInterfaceData(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDeviceInterfaceRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiDestroyClassImageList(classimagelistdata: *const SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDestroyDeviceInfoList(deviceinfoset: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDestroyDriverInfoList(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetupDiDrawMiniIcon(hdc: super::super::Graphics::Gdi::HDC, rc: super::super::Foundation::RECT, miniiconindex: i32, flags: u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDeviceInfo(deviceinfoset: *const ::core::ffi::c_void, memberindex: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDeviceInterfaces(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, interfaceclassguid: *const ::windows::runtime::GUID, memberindex: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDriverInfoA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDriverInfoW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, drivertype: u32, memberindex: u32, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualModelsSectionA(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: super::super::Foundation::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualModelsSectionW(context: *const INFCONTEXT, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: super::super::Foundation::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetActualSectionToInstallA(infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PSTR, infsectionwithext: super::super::Foundation::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualSectionToInstallExA(infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: super::super::Foundation::PSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualSectionToInstallExW(infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PWSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsectionwithext: super::super::Foundation::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetActualSectionToInstallW(infhandle: *const ::core::ffi::c_void, infsectionname: super::super::Foundation::PWSTR, infsectionwithext: super::super::Foundation::PWSTR, infsectionwithextsize: u32, requiredsize: *mut u32, extension: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassBitmapIndex(classguid: *const ::windows::runtime::GUID, miniiconindex: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionA(classguid: *const ::windows::runtime::GUID, classdescription: super::super::Foundation::PSTR, classdescriptionsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionExA(classguid: *const ::windows::runtime::GUID, classdescription: super::super::Foundation::PSTR, classdescriptionsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionExW(classguid: *const ::windows::runtime::GUID, classdescription: super::super::Foundation::PWSTR, classdescriptionsize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionW(classguid: *const ::windows::runtime::GUID, classdescription: super::super::Foundation::PWSTR, classdescriptionsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiGetClassDevPropertySheetsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const ::core::mem::ManuallyDrop<super::super::UI::Controls::PROPSHEETHEADERA_V2>, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiGetClassDevPropertySheetsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertysheetheader: *const ::core::mem::ManuallyDrop<super::super::UI::Controls::PROPSHEETHEADERW_V2>, propertysheetheaderpagelistsize: u32, requiredsize: *mut u32, propertysheettype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsA(classguid: *const ::windows::runtime::GUID, enumerator: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsExA(classguid: *const ::windows::runtime::GUID, enumerator: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, flags: u32, deviceinfoset: *const ::core::ffi::c_void, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsExW(classguid: *const ::windows::runtime::GUID, enumerator: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, flags: u32, deviceinfoset: *const ::core::ffi::c_void, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsW(classguid: *const ::windows::runtime::GUID, enumerator: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageIndex(classimagelistdata: *const SP_CLASSIMAGELIST_DATA, classguid: *const ::windows::runtime::GUID, imageindex: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageList(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageListExA(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageListExW(classimagelistdata: *mut SP_CLASSIMAGELIST_DATA, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *mut SP_CLASSINSTALL_HEADER, classinstallparamssize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyExW(classguid: *const ::windows::runtime::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyKeys(classguid: *const ::windows::runtime::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyKeysExW(classguid: *const ::windows::runtime::GUID, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyW(classguid: *const ::windows::runtime::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassRegistryPropertyA(classguid: *const ::windows::runtime::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassRegistryPropertyW(classguid: *const ::windows::runtime::GUID, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetCustomDevicePropertyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: super::super::Foundation::PSTR, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetCustomDevicePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, custompropertyname: super::super::Foundation::PWSTR, flags: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInfoListClass(deviceinfoset: *const ::core::ffi::c_void, classguid: *mut ::windows::runtime::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInfoListDetailA(deviceinfoset: *const ::core::ffi::c_void, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInfoListDetailW(deviceinfoset: *const ::core::ffi::c_void, deviceinfosetdetaildata: *mut SP_DEVINFO_LIST_DETAIL_DATA_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut ::core::mem::ManuallyDrop<SP_DEVINSTALL_PARAMS_A>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *mut ::core::mem::ManuallyDrop<SP_DEVINSTALL_PARAMS_W>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstanceIdA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: super::super::Foundation::PSTR, deviceinstanceidsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstanceIdW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstanceid: super::super::Foundation::PWSTR, deviceinstanceidsize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInterfaceAlias(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, aliasinterfaceclassguid: *const ::windows::runtime::GUID, aliasdeviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInterfaceDetailA(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_A, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInterfaceDetailW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, deviceinterfacedetaildata: *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W, deviceinterfacedetaildatasize: u32, requiredsize: *mut u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDeviceInterfacePropertyKeys(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDeviceInterfacePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDevicePropertyKeys(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykeyarray: *mut super::Properties::DEVPROPKEY, propertykeycount: u32, requiredpropertykeycount: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDevicePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceRegistryPropertyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceRegistryPropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, property: u32, propertyregdatatype: *mut u32, propertybuffer: *mut u8, propertybuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInfoDetailA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_A, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInfoDetailW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinfodetaildata: *mut SP_DRVINFO_DETAIL_DATA_W, driverinfodetaildatasize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *mut SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameA(hwprofile: u32, friendlyname: super::super::Foundation::PSTR, friendlynamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameExA(hwprofile: u32, friendlyname: super::super::Foundation::PSTR, friendlynamesize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameExW(hwprofile: u32, friendlyname: super::super::Foundation::PWSTR, friendlynamesize: u32, requiredsize: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameW(hwprofile: u32, friendlyname: super::super::Foundation::PWSTR, friendlynamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileList(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileListExA(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileListExW(hwprofilelist: *mut u32, hwprofilelistsize: u32, requiredsize: *mut u32, currentlyactiveindex: *mut u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetINFClassA(infname: super::super::Foundation::PSTR, classguid: *mut ::windows::runtime::GUID, classname: super::super::Foundation::PSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetINFClassW(infname: super::super::Foundation::PWSTR, classguid: *mut ::windows::runtime::GUID, classname: super::super::Foundation::PWSTR, classnamesize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetSelectedDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetSelectedDriverA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetSelectedDriverW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetWizardPage(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, installwizarddata: *const SP_INSTALLWIZARD_DATA, pagetype: u32, flags: u32) -> super::super::UI::Controls::HPROPSHEETPAGE;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassA(hwndparent: super::super::Foundation::HWND, inffilename: super::super::Foundation::PSTR, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassExA(hwndparent: super::super::Foundation::HWND, inffilename: super::super::Foundation::PSTR, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows::runtime::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassExW(hwndparent: super::super::Foundation::HWND, inffilename: super::super::Foundation::PWSTR, flags: u32, filequeue: *const ::core::ffi::c_void, interfaceclassguid: *const ::windows::runtime::GUID, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassW(hwndparent: super::super::Foundation::HWND, inffilename: super::super::Foundation::PWSTR, flags: u32, filequeue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallDeviceInterfaces(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallDriverFiles(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiLoadClassIcon(classguid: *const ::windows::runtime::GUID, largeicon: *mut super::super::UI::WindowsAndMessaging::HICON, miniiconindex: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiLoadDeviceIcon(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, cxicon: u32, cyicon: u32, flags: u32, hicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenClassRegKey(classguid: *const ::windows::runtime::GUID, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiOpenClassRegKeyExA(classguid: *const ::windows::runtime::GUID, samdesired: u32, flags: u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiOpenClassRegKeyExW(classguid: *const ::windows::runtime::GUID, samdesired: u32, flags: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenDevRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, scope: u32, hwprofile: u32, keytype: u32, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInfoA(deviceinfoset: *const ::core::ffi::c_void, deviceinstanceid: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInfoW(deviceinfoset: *const ::core::ffi::c_void, deviceinstanceid: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, openflags: u32, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInterfaceA(deviceinfoset: *const ::core::ffi::c_void, devicepath: super::super::Foundation::PSTR, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenDeviceInterfaceRegKey(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, reserved: u32, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInterfaceW(deviceinfoset: *const ::core::ffi::c_void, devicepath: super::super::Foundation::PWSTR, openflags: u32, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRegisterCoDeviceInstallers(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRegisterDeviceInfo(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, flags: u32, compareproc: ::windows::runtime::RawPtr, comparecontext: *const ::core::ffi::c_void, dupdeviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRemoveDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRemoveDeviceInterface(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRestartDevices(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSelectBestCompatDrv(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSelectDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSelectOEMDrv(hwndparent: super::super::Foundation::HWND, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, classinstallparams: *const SP_CLASSINSTALL_HEADER, classinstallparamssize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetClassPropertyExW(classguid: *const ::windows::runtime::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetClassPropertyW(classguid: *const ::windows::runtime::GUID, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassRegistryPropertyA(classguid: *const ::windows::runtime::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: super::super::Foundation::PSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassRegistryPropertyW(classguid: *const ::windows::runtime::GUID, property: u32, propertybuffer: *const u8, propertybuffersize: u32, machinename: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const ::core::mem::ManuallyDrop<SP_DEVINSTALL_PARAMS_A>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, deviceinstallparams: *const ::core::mem::ManuallyDrop<SP_DEVINSTALL_PARAMS_W>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceInterfaceDefault(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *mut SP_DEVICE_INTERFACE_DATA, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetDeviceInterfacePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinterfacedata: *const SP_DEVICE_INTERFACE_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetDevicePropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, propertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, propertybuffer: *const u8, propertybuffersize: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceRegistryPropertyA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceRegistryPropertyW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, property: u32, propertybuffer: *const u8, propertybuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDriverInstallParamsA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_A, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDriverInstallParamsW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, driverinfodata: *const SP_DRVINFO_DATA_V2_W, driverinstallparams: *const SP_DRVINSTALL_PARAMS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetSelectedDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetSelectedDriverA(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetSelectedDriverW(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA, driverinfodata: *mut SP_DRVINFO_DATA_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiUnremoveDevice(deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *mut SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupDuplicateDiskSpaceListA(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupDuplicateDiskSpaceListW(diskspace: *const ::core::ffi::c_void, reserved1: *mut ::core::ffi::c_void, reserved2: u32, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupEnumInfSectionsA(infhandle: *const ::core::ffi::c_void, index: u32, buffer: super::super::Foundation::PSTR, size: u32, sizeneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupEnumInfSectionsW(infhandle: *const ::core::ffi::c_void, index: u32, buffer: super::super::Foundation::PWSTR, size: u32, sizeneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindFirstLineA(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR, key: super::super::Foundation::PSTR, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindFirstLineW(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR, key: super::super::Foundation::PWSTR, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindNextLine(contextin: *const INFCONTEXT, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindNextMatchLineA(contextin: *const INFCONTEXT, key: super::super::Foundation::PSTR, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindNextMatchLineW(contextin: *const INFCONTEXT, key: super::super::Foundation::PWSTR, contextout: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFreeSourceListA(list: *mut *mut super::super::Foundation::PSTR, count: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFreeSourceListW(list: *mut *mut super::super::Foundation::PWSTR, count: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetBackupInformationA(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetBackupInformationW(queuehandle: *const ::core::ffi::c_void, backupparams: *mut SP_BACKUP_QUEUE_PARAMS_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetBinaryField(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: *mut u8, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupGetFieldCount(context: *const INFCONTEXT) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoA(sourcefilename: super::super::Foundation::PSTR, actualsourcefilename: *mut super::super::Foundation::PSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoExA(sourcefilename: super::super::Foundation::PSTR, actualsourcefilenamebuffer: super::super::Foundation::PSTR, actualsourcefilenamebufferlen: u32, requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoExW(sourcefilename: super::super::Foundation::PWSTR, actualsourcefilenamebuffer: super::super::Foundation::PWSTR, actualsourcefilenamebufferlen: u32, requiredbufferlen: *mut u32, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoW(sourcefilename: super::super::Foundation::PWSTR, actualsourcefilename: *mut super::super::Foundation::PWSTR, sourcefilesize: *mut u32, targetfilesize: *mut u32, compressiontype: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileQueueCount(filequeue: *const ::core::ffi::c_void, subqueuefileop: u32, numoperations: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileQueueFlags(filequeue: *const ::core::ffi::c_void, flags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupGetInfDriverStoreLocationA(filename: super::super::Foundation::PSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupGetInfDriverStoreLocationW(filename: super::super::Foundation::PWSTR, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, localename: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfFileListA(directorypath: super::super::Foundation::PSTR, infstyle: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfFileListW(directorypath: super::super::Foundation::PWSTR, infstyle: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfInformationA(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfInformationW(infspec: *const ::core::ffi::c_void, searchcontrol: u32, returnbuffer: *mut SP_INF_INFORMATION, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfPublishedNameA(driverstorelocation: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfPublishedNameW(driverstorelocation: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetIntField(context: *const INFCONTEXT, fieldindex: u32, integervalue: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineByIndexA(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineByIndexW(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR, index: u32, context: *mut INFCONTEXT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineCountA(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineCountW(infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineTextA(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR, key: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineTextW(context: *const INFCONTEXT, infhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR, key: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetMultiSzFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetMultiSzFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetNonInteractiveMode() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileLocationA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: super::super::Foundation::PSTR, sourceid: *mut u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileLocationW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: super::super::Foundation::PWSTR, sourceid: *mut u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileSizeA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: super::super::Foundation::PSTR, section: super::super::Foundation::PSTR, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileSizeW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, filename: super::super::Foundation::PWSTR, section: super::super::Foundation::PWSTR, filesize: *mut u32, roundingfactor: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceInfoA(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceInfoW(infhandle: *const ::core::ffi::c_void, sourceid: u32, infodesired: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetStringFieldA(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetStringFieldW(context: *const INFCONTEXT, fieldindex: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetTargetPathA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetTargetPathW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, section: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupGetThreadLogToken() -> u64;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitDefaultQueueCallback(ownerwindow: super::super::Foundation::HWND) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitDefaultQueueCallbackEx(ownerwindow: super::super::Foundation::HWND, alternateprogresswindow: super::super::Foundation::HWND, progressmessage: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitializeFileLogA(logfilename: super::super::Foundation::PSTR, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitializeFileLogW(logfilename: super::super::Foundation::PWSTR, flags: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: super::super::Foundation::PSTR, sourcepathroot: super::super::Foundation::PSTR, destinationname: super::super::Foundation::PSTR, copystyle: SP_COPY_STYLE, copymsghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileExA(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: super::super::Foundation::PSTR, sourcepathroot: super::super::Foundation::PSTR, destinationname: super::super::Foundation::PSTR, copystyle: SP_COPY_STYLE, copymsghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileExW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: super::super::Foundation::PWSTR, sourcepathroot: super::super::Foundation::PWSTR, destinationname: super::super::Foundation::PWSTR, copystyle: SP_COPY_STYLE, copymsghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, filewasinuse: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileW(infhandle: *const ::core::ffi::c_void, infcontext: *const INFCONTEXT, sourcefile: super::super::Foundation::PWSTR, sourcepathroot: super::super::Foundation::PWSTR, destinationname: super::super::Foundation::PWSTR, copystyle: SP_COPY_STYLE, copymsghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFilesFromInfSectionA(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, sourcerootpath: super::super::Foundation::PSTR, copyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFilesFromInfSectionW(infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, filequeue: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, sourcerootpath: super::super::Foundation::PWSTR, copyflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupInstallFromInfSectionA(owner: super::super::Foundation::HWND, infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, flags: u32, relativekeyroot: super::super::System::Registry::HKEY, sourcerootpath: super::super::Foundation::PSTR, copyflags: u32, msghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupInstallFromInfSectionW(owner: super::super::Foundation::HWND, infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, flags: u32, relativekeyroot: super::super::System::Registry::HKEY, sourcerootpath: super::super::Foundation::PWSTR, copyflags: u32, msghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionA(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionExA(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, flags: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionExW(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, flags: u32, deviceinfoset: *const ::core::ffi::c_void, deviceinfodata: *const SP_DEVINFO_DATA, reserved1: *mut ::core::ffi::c_void, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionW(infhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupIterateCabinetA(cabinetfile: super::super::Foundation::PSTR, reserved: u32, msghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupIterateCabinetW(cabinetfile: super::super::Foundation::PWSTR, reserved: u32, msghandler: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogErrorA(messagestring: super::super::Foundation::PSTR, severity: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogErrorW(messagestring: super::super::Foundation::PWSTR, severity: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogFileA(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PSTR, sourcefilename: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, checksum: u32, disktagfile: super::super::Foundation::PSTR, diskdescription: super::super::Foundation::PSTR, otherinfo: super::super::Foundation::PSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogFileW(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PWSTR, sourcefilename: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, checksum: u32, disktagfile: super::super::Foundation::PWSTR, diskdescription: super::super::Foundation::PWSTR, otherinfo: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenAppendInfFileA(filename: super::super::Foundation::PSTR, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenAppendInfFileW(filename: super::super::Foundation::PWSTR, infhandle: *const ::core::ffi::c_void, errorline: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupOpenFileQueue() -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenInfFileA(filename: super::super::Foundation::PSTR, infclass: super::super::Foundation::PSTR, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenInfFileW(filename: super::super::Foundation::PWSTR, infclass: super::super::Foundation::PWSTR, infstyle: u32, errorline: *mut u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenLog(erase: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupOpenMasterInf() -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPrepareQueueForRestoreA(queuehandle: *const ::core::ffi::c_void, backuppath: super::super::Foundation::PSTR, restoreflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPrepareQueueForRestoreW(queuehandle: *const ::core::ffi::c_void, backuppath: super::super::Foundation::PWSTR, restoreflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPromptForDiskA(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, diskname: super::super::Foundation::PSTR, pathtosource: super::super::Foundation::PSTR, filesought: super::super::Foundation::PSTR, tagfile: super::super::Foundation::PSTR, diskpromptstyle: u32, pathbuffer: super::super::Foundation::PSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPromptForDiskW(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, diskname: super::super::Foundation::PWSTR, pathtosource: super::super::Foundation::PWSTR, filesought: super::super::Foundation::PWSTR, tagfile: super::super::Foundation::PWSTR, diskpromptstyle: u32, pathbuffer: super::super::Foundation::PWSTR, pathbuffersize: u32, pathrequiredsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPromptReboot(filequeue: *const ::core::ffi::c_void, owner: super::super::Foundation::HWND, scanonly: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryDrivesInDiskSpaceListA(diskspace: *const ::core::ffi::c_void, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryDrivesInDiskSpaceListW(diskspace: *const ::core::ffi::c_void, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryFileLogA(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, desiredinfo: SetupFileLogInfo, dataout: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryFileLogW(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, desiredinfo: SetupFileLogInfo, dataout: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupQueryInfOriginalFileInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupQueryInfOriginalFileInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, originalfileinfo: *mut SP_ORIGINAL_FILE_INFO_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfVersionInformationA(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: super::super::Foundation::PSTR, returnbuffer: super::super::Foundation::PSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfVersionInformationW(infinformation: *const SP_INF_INFORMATION, infindex: u32, key: super::super::Foundation::PWSTR, returnbuffer: super::super::Foundation::PWSTR, returnbuffersize: u32, requiredsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySourceListA(flags: u32, list: *mut *mut super::super::Foundation::PSTR, count: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySourceListW(flags: u32, list: *mut *mut super::super::Foundation::PWSTR, count: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySpaceRequiredOnDriveA(diskspace: *const ::core::ffi::c_void, drivespec: super::super::Foundation::PSTR, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySpaceRequiredOnDriveW(diskspace: *const ::core::ffi::c_void, drivespec: super::super::Foundation::PWSTR, spacerequired: *mut i64, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyA(queuehandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PSTR, sourcepath: super::super::Foundation::PSTR, sourcefilename: super::super::Foundation::PSTR, sourcedescription: super::super::Foundation::PSTR, sourcetagfile: super::super::Foundation::PSTR, targetdirectory: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyIndirectA(copyparams: *const SP_FILE_COPY_PARAMS_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyIndirectW(copyparams: *const SP_FILE_COPY_PARAMS_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopySectionA(queuehandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PSTR, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopySectionW(queuehandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PWSTR, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyW(queuehandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PWSTR, sourcepath: super::super::Foundation::PWSTR, sourcefilename: super::super::Foundation::PWSTR, sourcedescription: super::super::Foundation::PWSTR, sourcetagfile: super::super::Foundation::PWSTR, targetdirectory: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDefaultCopyA(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PSTR, sourcefilename: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDefaultCopyW(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, sourcerootpath: super::super::Foundation::PWSTR, sourcefilename: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR, copystyle: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteA(queuehandle: *const ::core::ffi::c_void, pathpart1: super::super::Foundation::PSTR, pathpart2: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteSectionA(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteSectionW(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteW(queuehandle: *const ::core::ffi::c_void, pathpart1: super::super::Foundation::PWSTR, pathpart2: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameA(queuehandle: *const ::core::ffi::c_void, sourcepath: super::super::Foundation::PSTR, sourcefilename: super::super::Foundation::PSTR, targetpath: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameSectionA(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameSectionW(queuehandle: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, section: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameW(queuehandle: *const ::core::ffi::c_void, sourcepath: super::super::Foundation::PWSTR, sourcefilename: super::super::Foundation::PWSTR, targetpath: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFileLogEntryA(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PSTR, targetfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFileLogEntryW(fileloghandle: *const ::core::ffi::c_void, logsectionname: super::super::Foundation::PWSTR, targetfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromDiskSpaceListA(diskspace: *const ::core::ffi::c_void, targetfilespec: super::super::Foundation::PSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromDiskSpaceListW(diskspace: *const ::core::ffi::c_void, targetfilespec: super::super::Foundation::PWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromSourceListA(flags: u32, source: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromSourceListW(flags: u32, source: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveInstallSectionFromDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveInstallSectionFromDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, layoutinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveSectionFromDiskSpaceListA(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveSectionFromDiskSpaceListW(diskspace: *const ::core::ffi::c_void, infhandle: *const ::core::ffi::c_void, listinfhandle: *const ::core::ffi::c_void, sectionname: super::super::Foundation::PWSTR, operation: SETUP_FILE_OPERATION, reserved1: *mut ::core::ffi::c_void, reserved2: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRenameErrorA(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PSTR, sourcefile: super::super::Foundation::PSTR, targetfile: super::super::Foundation::PSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRenameErrorW(hwndparent: super::super::Foundation::HWND, dialogtitle: super::super::Foundation::PWSTR, sourcefile: super::super::Foundation::PWSTR, targetfile: super::super::Foundation::PWSTR, win32errorcode: u32, style: u32) -> u32;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupScanFileQueueA(filequeue: *const ::core::ffi::c_void, flags: u32, window: super::super::Foundation::HWND, callbackroutine: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupScanFileQueueW(filequeue: *const ::core::ffi::c_void, flags: u32, window: super::super::Foundation::HWND, callbackroutine: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void, result: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdA(infhandle: *const ::core::ffi::c_void, id: u32, directory: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdExA(infhandle: *const ::core::ffi::c_void, id: u32, directory: super::super::Foundation::PSTR, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdExW(infhandle: *const ::core::ffi::c_void, id: u32, directory: super::super::Foundation::PWSTR, flags: u32, reserved1: u32, reserved2: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdW(infhandle: *const ::core::ffi::c_void, id: u32, directory: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupSetFileQueueAlternatePlatformA(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupSetFileQueueAlternatePlatformW(queuehandle: *const ::core::ffi::c_void, alternateplatforminfo: *const SP_ALTPLATFORM_INFO_V2, alternatedefaultcatalogfile: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetFileQueueFlags(filequeue: *const ::core::ffi::c_void, flagmask: u32, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetNonInteractiveMode(noninteractiveflag: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetPlatformPathOverrideA(r#override: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetPlatformPathOverrideW(r#override: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetSourceListA(flags: u32, sourcelist: *const super::super::Foundation::PSTR, sourcecount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetSourceListW(flags: u32, sourcelist: *const super::super::Foundation::PWSTR, sourcecount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupSetThreadLogToken(logtoken: u64);
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupTermDefaultQueueCallback(context: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupTerminateFileLog(fileloghandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupUninstallNewlyCopiedInfs(filequeue: *const ::core::ffi::c_void, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupUninstallOEMInfA(inffilename: super::super::Foundation::PSTR, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupUninstallOEMInfW(inffilename: super::super::Foundation::PWSTR, flags: u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupVerifyInfFileA(infname: super::super::Foundation::PSTR, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_A) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupVerifyInfFileW(infname: super::super::Foundation::PWSTR, altplatforminfo: *const SP_ALTPLATFORM_INFO_V2, infsignerinfo: *mut SP_INF_SIGNER_INFO_V2_W) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupWriteTextLog(logtoken: u64, category: u32, flags: u32, messagestr: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupWriteTextLogError(logtoken: u64, category: u32, logflags: u32, error: u32, messagestr: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupWriteTextLogInfLine(logtoken: u64, flags: u32, infhandle: *const ::core::ffi::c_void, context: *const INFCONTEXT);
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDriverForPlugAndPlayDevicesA(hwndparent: super::super::Foundation::HWND, hardwareid: super::super::Foundation::PSTR, fullinfpath: super::super::Foundation::PSTR, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDriverForPlugAndPlayDevicesW(hwndparent: super::super::Foundation::HWND, hardwareid: super::super::Foundation::PWSTR, fullinfpath: super::super::Foundation::PWSTR, installflags: u32, brebootrequired: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
}
