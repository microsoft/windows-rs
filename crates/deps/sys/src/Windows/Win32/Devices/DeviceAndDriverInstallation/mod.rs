#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CMP_WaitNoPendingInstallEvents();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Data_HtmlHelp`*"]
    #[cfg(feature = "Win32_Data_HtmlHelp")]
    pub fn CM_Add_Empty_Log_Conf();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Data_HtmlHelp`*"]
    #[cfg(feature = "Win32_Data_HtmlHelp")]
    pub fn CM_Add_Empty_Log_Conf_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Add_IDA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Add_IDW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Add_ID_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Add_ID_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Add_Range();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Add_Res_Des();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Add_Res_Des_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Connect_MachineA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Connect_MachineW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_DevNodeA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_DevNodeW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_DevNode_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_DevNode_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Create_Range_List();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_Class_Key();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_Class_Key_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_DevNode_Key();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_DevNode_Key_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Delete_Device_Interface_KeyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Delete_Device_Interface_KeyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Delete_Device_Interface_Key_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Delete_Device_Interface_Key_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Delete_Range();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Detect_Resource_Conflict();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Detect_Resource_Conflict_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Disable_DevNode();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Disable_DevNode_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Disconnect_Machine();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Dup_Range_List();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Enable_DevNode();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Enable_DevNode_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Enumerate_Classes();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Enumerate_Classes_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Enumerate_EnumeratorsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Enumerate_EnumeratorsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Enumerate_Enumerators_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Enumerate_Enumerators_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Find_Range();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_First_Range();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Log_Conf();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Log_Conf_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Log_Conf_Handle();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Range_List();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Res_Des();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Res_Des_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Res_Des_Handle();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Free_Resource_Conflict_Handle();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Child();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Child_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Key_NameA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Key_NameW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Key_Name_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Key_Name_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_NameA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_NameW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Name_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Class_Name_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_Property_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_Property_Keys();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_Class_Property_Keys_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Class_Registry_PropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Class_Registry_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Depth();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Depth_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_DevNode_Custom_PropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_DevNode_Custom_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_DevNode_Custom_Property_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_DevNode_Custom_Property_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_Property_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_Property_Keys();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Get_DevNode_Property_Keys_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Registry_PropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Registry_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Registry_Property_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Registry_Property_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Status();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_DevNode_Status_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_IDA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_IDW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_ListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_ListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_SizeA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_SizeW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_Size_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_ID_List_Size_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_ID_Size();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_ID_Size_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_AliasA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_AliasW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_Alias_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_Alias_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_ListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_ListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_List_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Device_Interface_List_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_Interface_List_SizeA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_Interface_List_SizeW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_Interface_List_Size_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Device_Interface_List_Size_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Get_Device_Interface_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Get_Device_Interface_Property_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Get_Device_Interface_Property_KeysW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Get_Device_Interface_Property_Keys_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_First_Log_Conf();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_First_Log_Conf_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Global_State();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Global_State_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_HW_Prof_FlagsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_HW_Prof_FlagsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_HW_Prof_Flags_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_HW_Prof_Flags_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Hardware_Profile_InfoA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Hardware_Profile_InfoW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Hardware_Profile_Info_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Hardware_Profile_Info_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Log_Conf_Priority();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Log_Conf_Priority_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Next_Log_Conf();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Next_Log_Conf_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Next_Res_Des();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Next_Res_Des_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Parent();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Parent_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Res_Des_Data();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Res_Des_Data_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Res_Des_Data_Size();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Res_Des_Data_Size_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Resource_Conflict_Count();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Get_Resource_Conflict_DetailsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Resource_Conflict_DetailsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Sibling();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Sibling_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Version();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Get_Version_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Intersect_Range_List();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Invert_Range_List();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Dock_Station_Present();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Dock_Station_Present_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Version_Available();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Is_Version_Available_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Locate_DevNodeA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Locate_DevNodeW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Locate_DevNode_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Locate_DevNode_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_MapCrToWin32Err();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Merge_Range_List();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Modify_Res_Des();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Modify_Res_Des_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Move_DevNode();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Move_DevNode_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Next_Range();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Class_KeyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Class_KeyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Class_Key_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Class_Key_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_DevNode_Key();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn CM_Open_DevNode_Key_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Device_Interface_KeyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Device_Interface_KeyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Device_Interface_Key_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CM_Open_Device_Interface_Key_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Query_And_Remove_SubTreeA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Query_And_Remove_SubTreeW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Query_And_Remove_SubTree_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Query_And_Remove_SubTree_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Arbitrator_Free_Data();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Arbitrator_Free_Data_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Arbitrator_Free_Size();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Arbitrator_Free_Size_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Remove_SubTree();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Remove_SubTree_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Query_Resource_Conflict_List();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Reenumerate_DevNode();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Reenumerate_DevNode_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Register_Device_Driver();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Register_Device_Driver_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Device_InterfaceA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Device_InterfaceW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Device_Interface_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Device_Interface_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Register_Notification();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Remove_SubTree();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Remove_SubTree_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Request_Device_EjectA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Request_Device_EjectW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Request_Device_Eject_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Request_Device_Eject_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Request_Eject_PC();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Request_Eject_PC_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Run_Detection();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Run_Detection_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_Class_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_Class_Property_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_Class_Registry_PropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_Class_Registry_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Problem();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Problem_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_DevNode_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`*"]
    #[cfg(feature = "Win32_Devices_Properties")]
    pub fn CM_Set_DevNode_Property_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Registry_PropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Registry_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Registry_Property_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_DevNode_Registry_Property_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Set_Device_Interface_PropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn CM_Set_Device_Interface_Property_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_FlagsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_FlagsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_Flags_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Set_HW_Prof_Flags_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Setup_DevNode();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Setup_DevNode_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Test_Range_Available();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Uninstall_DevNode();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Uninstall_DevNode_Ex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Unregister_Device_InterfaceA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Unregister_Device_InterfaceW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Unregister_Device_Interface_ExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CM_Unregister_Device_Interface_ExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn CM_Unregister_Notification();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiInstallDevice();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiInstallDriverA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiInstallDriverW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiRollbackDriver();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiShowUpdateDevice();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiShowUpdateDriver();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiUninstallDevice();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiUninstallDriverA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiUninstallDriverW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallHinfSectionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallHinfSectionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddInstallSectionToDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddInstallSectionToDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddSectionToDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddSectionToDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToSourceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAddToSourceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAdjustDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupAdjustDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupBackupErrorA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupBackupErrorW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCancelTemporarySourceList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCloseFileQueue();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupCloseInfFile();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupCloseLog();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCommitFileQueueA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCommitFileQueueW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupConfigureWmiFromInfSectionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupConfigureWmiFromInfSectionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyErrorA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyErrorW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyOEMInfA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupCopyOEMInfW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupCreateDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupCreateDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDecompressOrCopyFileA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDecompressOrCopyFileW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupDefaultQueueCallbackA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupDefaultQueueCallbackW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDeleteErrorA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDeleteErrorW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDestroyDiskSpaceList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiAskForOEMDisk();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildClassInfoList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildClassInfoListExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildClassInfoListExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiBuildDriverInfoList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCallClassInstaller();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCancelDriverInfoSearch();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiChangeState();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassGuidsFromNameW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiClassNameFromGuidW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiCreateDevRegKeyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiCreateDevRegKeyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoListExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoListExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInfoW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInterfaceA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiCreateDeviceInterfaceRegKeyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiCreateDeviceInterfaceRegKeyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiCreateDeviceInterfaceW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDevRegKey();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDeviceInfo();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDeviceInterfaceData();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDeleteDeviceInterfaceRegKey();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiDestroyClassImageList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDestroyDeviceInfoList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiDestroyDriverInfoList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetupDiDrawMiniIcon();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDeviceInfo();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDeviceInterfaces();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDriverInfoA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiEnumDriverInfoW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualModelsSectionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualModelsSectionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetActualSectionToInstallA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualSectionToInstallExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupDiGetActualSectionToInstallExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetActualSectionToInstallW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassBitmapIndex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDescriptionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiGetClassDevPropertySheetsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiGetClassDevPropertySheetsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassDevsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageIndex();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageListExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetClassImageListExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassInstallParamsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassInstallParamsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyKeys();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyKeysExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetClassPropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassRegistryPropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetClassRegistryPropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetCustomDevicePropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetCustomDevicePropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInfoListClass();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInfoListDetailA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInfoListDetailW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstallParamsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstallParamsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstanceIdA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInstanceIdW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInterfaceAlias();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInterfaceDetailA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceInterfaceDetailW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDeviceInterfacePropertyKeys();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDeviceInterfacePropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDevicePropertyKeys();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiGetDevicePropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceRegistryPropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDeviceRegistryPropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInfoDetailA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInfoDetailW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInstallParamsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetDriverInstallParamsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileFriendlyNameW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileList();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileListExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetHwProfileListExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetINFClassA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetINFClassW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetSelectedDevice();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetSelectedDriverA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiGetSelectedDriverW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SetupDiGetWizardPage();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallClassW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallDevice();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallDeviceInterfaces();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiInstallDriverFiles();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiLoadClassIcon();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetupDiLoadDeviceIcon();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenClassRegKey();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiOpenClassRegKeyExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupDiOpenClassRegKeyExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenDevRegKey();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInfoA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInfoW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInterfaceA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SetupDiOpenDeviceInterfaceRegKey();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiOpenDeviceInterfaceW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRegisterCoDeviceInstallers();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRegisterDeviceInfo();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRemoveDevice();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRemoveDeviceInterface();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiRestartDevices();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSelectBestCompatDrv();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSelectDevice();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSelectOEMDrv();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassInstallParamsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassInstallParamsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetClassPropertyExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetClassPropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassRegistryPropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetClassRegistryPropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceInstallParamsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceInstallParamsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceInterfaceDefault();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetDeviceInterfacePropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Devices_Properties`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
    pub fn SetupDiSetDevicePropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceRegistryPropertyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDeviceRegistryPropertyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDriverInstallParamsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetDriverInstallParamsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetSelectedDevice();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetSelectedDriverA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiSetSelectedDriverW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupDiUnremoveDevice();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupDuplicateDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupDuplicateDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupEnumInfSectionsA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupEnumInfSectionsW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindFirstLineA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindFirstLineW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindNextLine();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindNextMatchLineA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFindNextMatchLineW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFreeSourceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupFreeSourceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetBackupInformationA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetBackupInformationW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetBinaryField();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupGetFieldCount();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileCompressionInfoW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileQueueCount();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetFileQueueFlags();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupGetInfDriverStoreLocationA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupGetInfDriverStoreLocationW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfFileListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfFileListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfInformationA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfInformationW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfPublishedNameA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetInfPublishedNameW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetIntField();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineByIndexA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineByIndexW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineCountA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineCountW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineTextA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetLineTextW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetMultiSzFieldA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetMultiSzFieldW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetNonInteractiveMode();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileLocationA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileLocationW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileSizeA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceFileSizeW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceInfoA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetSourceInfoW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetStringFieldA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetStringFieldW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetTargetPathA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupGetTargetPathW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupGetThreadLogToken();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitDefaultQueueCallback();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitDefaultQueueCallbackEx();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitializeFileLogA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInitializeFileLogW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFileW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFilesFromInfSectionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallFilesFromInfSectionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupInstallFromInfSectionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SetupInstallFromInfSectionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupInstallServicesFromInfSectionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupIterateCabinetA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupIterateCabinetW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogErrorA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogErrorW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogFileA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupLogFileW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenAppendInfFileA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenAppendInfFileW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupOpenFileQueue();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenInfFileA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenInfFileW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupOpenLog();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupOpenMasterInf();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPrepareQueueForRestoreA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPrepareQueueForRestoreW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPromptForDiskA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPromptForDiskW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupPromptReboot();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryDrivesInDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryDrivesInDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryFileLogA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryFileLogW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfFileInformationA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfFileInformationW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupQueryInfOriginalFileInformationA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupQueryInfOriginalFileInformationW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfVersionInformationA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueryInfVersionInformationW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySourceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySourceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySpaceRequiredOnDriveA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQuerySpaceRequiredOnDriveW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyIndirectA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyIndirectW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopySectionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopySectionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueCopyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDefaultCopyA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDefaultCopyW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteSectionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteSectionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueDeleteW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameSectionA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameSectionW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupQueueRenameW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFileLogEntryA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFileLogEntryW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromSourceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveFromSourceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveInstallSectionFromDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveInstallSectionFromDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveSectionFromDiskSpaceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRemoveSectionFromDiskSpaceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRenameErrorA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupRenameErrorW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupScanFileQueueA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupScanFileQueueW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdExA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdExW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetDirectoryIdW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupSetFileQueueAlternatePlatformA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupSetFileQueueAlternatePlatformW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetFileQueueFlags();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetNonInteractiveMode();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetPlatformPathOverrideA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetPlatformPathOverrideW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetSourceListA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupSetSourceListW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupSetThreadLogToken();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupTermDefaultQueueCallback();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupTerminateFileLog();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupUninstallNewlyCopiedInfs();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupUninstallOEMInfA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupUninstallOEMInfW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupVerifyInfFileA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
    pub fn SetupVerifyInfFileW();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupWriteTextLog();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetupWriteTextLogError();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`*"]
    pub fn SetupWriteTextLogInfLine();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDriverForPlugAndPlayDevicesA();
    #[doc = "*Required features: `Win32_Devices_DeviceAndDriverInstallation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDriverForPlugAndPlayDevicesW();
}
