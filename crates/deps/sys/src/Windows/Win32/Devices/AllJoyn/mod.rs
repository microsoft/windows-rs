#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynAcceptBusConnection(serverbushandle: super::super::Foundation::HANDLE, abortevent: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynCloseBusHandle(bushandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynConnectToBus(connectionspec: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn AllJoynCreateBus(outbuffersize: u32, inbuffersize: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynEnumEvents(connectedbushandle: super::super::Foundation::HANDLE, eventtoreset: super::super::Foundation::HANDLE, eventtypes: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynEventSelect(connectedbushandle: super::super::Foundation::HANDLE, eventhandle: super::super::Foundation::HANDLE, eventtypes: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynReceiveFromBus(connectedbushandle: super::super::Foundation::HANDLE, buffer: *mut ::core::ffi::c_void, bytestoread: u32, bytestransferred: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynSendToBus(connectedbushandle: super::super::Foundation::HANDLE, buffer: *const ::core::ffi::c_void, bytestowrite: u32, bytestransferred: *mut u32, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QCC_StatusText(status: QStatus) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_create(defaultlanguage: super::super::Foundation::PSTR) -> alljoyn_aboutdata;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_create_empty() -> alljoyn_aboutdata;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_create_full(arg: alljoyn_msgarg, language: super::super::Foundation::PSTR) -> alljoyn_aboutdata;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_createfrommsgarg(data: alljoyn_aboutdata, arg: alljoyn_msgarg, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_createfromxml(data: alljoyn_aboutdata, aboutdataxml: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_destroy(data: alljoyn_aboutdata);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getaboutdata(data: alljoyn_aboutdata, msgarg: alljoyn_msgarg, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getajsoftwareversion(data: alljoyn_aboutdata, ajsoftwareversion: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getannouncedaboutdata(data: alljoyn_aboutdata, msgarg: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getappid(data: alljoyn_aboutdata, appid: *mut *mut u8, num: *mut usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getappname(data: alljoyn_aboutdata, appname: *mut *mut i8, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getdateofmanufacture(data: alljoyn_aboutdata, dateofmanufacture: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getdefaultlanguage(data: alljoyn_aboutdata, defaultlanguage: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getdescription(data: alljoyn_aboutdata, description: *mut *mut i8, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getdeviceid(data: alljoyn_aboutdata, deviceid: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getdevicename(data: alljoyn_aboutdata, devicename: *mut *mut i8, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getfield(data: alljoyn_aboutdata, name: super::super::Foundation::PSTR, value: *mut alljoyn_msgarg, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getfields(data: alljoyn_aboutdata, fields: *const *const i8, num_fields: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getfieldsignature(data: alljoyn_aboutdata, fieldname: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_gethardwareversion(data: alljoyn_aboutdata, hardwareversion: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getmanufacturer(data: alljoyn_aboutdata, manufacturer: *mut *mut i8, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getmodelnumber(data: alljoyn_aboutdata, modelnumber: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getsoftwareversion(data: alljoyn_aboutdata, softwareversion: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getsupportedlanguages(data: alljoyn_aboutdata, languagetags: *const *const i8, num: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getsupporturl(data: alljoyn_aboutdata, supporturl: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_isfieldannounced(data: alljoyn_aboutdata, fieldname: super::super::Foundation::PSTR) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_isfieldlocalized(data: alljoyn_aboutdata, fieldname: super::super::Foundation::PSTR) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_isfieldrequired(data: alljoyn_aboutdata, fieldname: super::super::Foundation::PSTR) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_isvalid(data: alljoyn_aboutdata, language: super::super::Foundation::PSTR) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_setappid(data: alljoyn_aboutdata, appid: *const u8, num: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setappid_fromstring(data: alljoyn_aboutdata, appid: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setappname(data: alljoyn_aboutdata, appname: super::super::Foundation::PSTR, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdateofmanufacture(data: alljoyn_aboutdata, dateofmanufacture: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdefaultlanguage(data: alljoyn_aboutdata, defaultlanguage: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdescription(data: alljoyn_aboutdata, description: super::super::Foundation::PSTR, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdeviceid(data: alljoyn_aboutdata, deviceid: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdevicename(data: alljoyn_aboutdata, devicename: super::super::Foundation::PSTR, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setfield(data: alljoyn_aboutdata, name: super::super::Foundation::PSTR, value: alljoyn_msgarg, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_sethardwareversion(data: alljoyn_aboutdata, hardwareversion: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setmanufacturer(data: alljoyn_aboutdata, manufacturer: super::super::Foundation::PSTR, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setmodelnumber(data: alljoyn_aboutdata, modelnumber: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setsoftwareversion(data: alljoyn_aboutdata, softwareversion: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setsupportedlanguage(data: alljoyn_aboutdata, language: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setsupporturl(data: alljoyn_aboutdata, supporturl: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdatalistener_create(callbacks: *const alljoyn_aboutdatalistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_aboutdatalistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdatalistener_destroy(listener: alljoyn_aboutdatalistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_clear(icon: *mut _alljoyn_abouticon_handle);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_create() -> *mut _alljoyn_abouticon_handle;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_destroy(icon: *mut _alljoyn_abouticon_handle);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_getcontent(icon: *mut _alljoyn_abouticon_handle, data: *const *const u8, size: *mut usize);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_geturl(icon: *mut _alljoyn_abouticon_handle, r#type: *const *const i8, url: *const *const i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_abouticon_setcontent(icon: *mut _alljoyn_abouticon_handle, r#type: super::super::Foundation::PSTR, data: *mut u8, csize: usize, ownsdata: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_setcontent_frommsgarg(icon: *mut _alljoyn_abouticon_handle, arg: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_abouticon_seturl(icon: *mut _alljoyn_abouticon_handle, r#type: super::super::Foundation::PSTR, url: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonobj_create(bus: alljoyn_busattachment, icon: *mut _alljoyn_abouticon_handle) -> *mut _alljoyn_abouticonobj_handle;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonobj_destroy(icon: *mut _alljoyn_abouticonobj_handle);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_abouticonproxy_create(bus: alljoyn_busattachment, busname: super::super::Foundation::PSTR, sessionid: u32) -> *mut _alljoyn_abouticonproxy_handle;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonproxy_destroy(proxy: *mut _alljoyn_abouticonproxy_handle);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonproxy_geticon(proxy: *mut _alljoyn_abouticonproxy_handle, icon: *mut _alljoyn_abouticon_handle) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonproxy_getversion(proxy: *mut _alljoyn_abouticonproxy_handle, version: *mut u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutlistener_create(callback: *const alljoyn_aboutlistener_callback, context: *const ::core::ffi::c_void) -> alljoyn_aboutlistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutlistener_destroy(listener: alljoyn_aboutlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_announce(obj: alljoyn_aboutobj, sessionport: u16, aboutdata: alljoyn_aboutdata) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_announce_using_datalistener(obj: alljoyn_aboutobj, sessionport: u16, aboutlistener: alljoyn_aboutdatalistener) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_create(bus: alljoyn_busattachment, isannounced: alljoyn_about_announceflag) -> alljoyn_aboutobj;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_destroy(obj: alljoyn_aboutobj);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_unannounce(obj: alljoyn_aboutobj) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_clear(description: alljoyn_aboutobjectdescription);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_create() -> alljoyn_aboutobjectdescription;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_create_full(arg: alljoyn_msgarg) -> alljoyn_aboutobjectdescription;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_createfrommsgarg(description: alljoyn_aboutobjectdescription, arg: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_destroy(description: alljoyn_aboutobjectdescription);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_getinterfacepaths(description: alljoyn_aboutobjectdescription, interfacename: super::super::Foundation::PSTR, paths: *const *const i8, numpaths: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_getinterfaces(description: alljoyn_aboutobjectdescription, path: super::super::Foundation::PSTR, interfaces: *const *const i8, numinterfaces: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_getmsgarg(description: alljoyn_aboutobjectdescription, msgarg: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_getpaths(description: alljoyn_aboutobjectdescription, paths: *const *const i8, numpaths: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_hasinterface(description: alljoyn_aboutobjectdescription, interfacename: super::super::Foundation::PSTR) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_hasinterfaceatpath(description: alljoyn_aboutobjectdescription, path: super::super::Foundation::PSTR, interfacename: super::super::Foundation::PSTR) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_haspath(description: alljoyn_aboutobjectdescription, path: super::super::Foundation::PSTR) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutproxy_create(bus: alljoyn_busattachment, busname: super::super::Foundation::PSTR, sessionid: u32) -> alljoyn_aboutproxy;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutproxy_destroy(proxy: alljoyn_aboutproxy);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutproxy_getaboutdata(proxy: alljoyn_aboutproxy, language: super::super::Foundation::PSTR, data: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutproxy_getobjectdescription(proxy: alljoyn_aboutproxy, objectdesc: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutproxy_getversion(proxy: alljoyn_aboutproxy, version: *mut u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_applicationstatelistener_create(callbacks: *const alljoyn_applicationstatelistener_callbacks, context: *mut ::core::ffi::c_void) -> alljoyn_applicationstatelistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_applicationstatelistener_destroy(listener: alljoyn_applicationstatelistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_authlistener_create(callbacks: *const alljoyn_authlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_authlistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistener_destroy(listener: alljoyn_authlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistener_requestcredentialsresponse(listener: alljoyn_authlistener, authcontext: *mut ::core::ffi::c_void, accept: i32, credentials: alljoyn_credentials) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistener_setsharedsecret(listener: alljoyn_authlistener, sharedsecret: *const u8, sharedsecretsize: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistener_verifycredentialsresponse(listener: alljoyn_authlistener, authcontext: *mut ::core::ffi::c_void, accept: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_authlistenerasync_create(callbacks: *const alljoyn_authlistenerasync_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_authlistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistenerasync_destroy(listener: alljoyn_authlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_adddestination(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR, destination: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_addpinggroup(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR, listener: alljoyn_pinglistener, pinginterval: u32);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_autopinger_create(bus: alljoyn_busattachment) -> alljoyn_autopinger;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_autopinger_destroy(autopinger: alljoyn_autopinger);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_autopinger_pause(autopinger: alljoyn_autopinger);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_removedestination(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR, destination: super::super::Foundation::PSTR, removeall: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_removepinggroup(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_autopinger_resume(autopinger: alljoyn_autopinger);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_setpinginterval(autopinger: alljoyn_autopinger, group: super::super::Foundation::PSTR, pinginterval: u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_addlogonentry(bus: alljoyn_busattachment, authmechanism: super::super::Foundation::PSTR, username: super::super::Foundation::PSTR, password: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_addmatch(bus: alljoyn_busattachment, rule: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_advertisename(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, transports: u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_bindsessionport(bus: alljoyn_busattachment, sessionport: *mut u16, opts: alljoyn_sessionopts, listener: alljoyn_sessionportlistener) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_canceladvertisename(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, transports: u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_cancelfindadvertisedname(bus: alljoyn_busattachment, nameprefix: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_cancelfindadvertisednamebytransport(bus: alljoyn_busattachment, nameprefix: super::super::Foundation::PSTR, transports: u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_cancelwhoimplements_interface(bus: alljoyn_busattachment, implementsinterface: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_cancelwhoimplements_interfaces(bus: alljoyn_busattachment, implementsinterfaces: *const *const i8, numberinterfaces: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_clearkeys(bus: alljoyn_busattachment, guid: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_clearkeystore(bus: alljoyn_busattachment);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_connect(bus: alljoyn_busattachment, connectspec: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_create(applicationname: super::super::Foundation::PSTR, allowremotemessages: i32) -> alljoyn_busattachment;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_create_concurrency(applicationname: super::super::Foundation::PSTR, allowremotemessages: i32, concurrency: u32) -> alljoyn_busattachment;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_createinterface(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, iface: *mut alljoyn_interfacedescription) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_createinterface_secure(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, iface: *mut alljoyn_interfacedescription, secpolicy: alljoyn_interfacedescription_securitypolicy) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_createinterfacesfromxml(bus: alljoyn_busattachment, xml: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_deletedefaultkeystore(applicationname: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_deleteinterface(bus: alljoyn_busattachment, iface: alljoyn_interfacedescription) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_destroy(bus: alljoyn_busattachment);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_disconnect(bus: alljoyn_busattachment, unused: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_enableconcurrentcallbacks(bus: alljoyn_busattachment);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_enablepeersecurity(bus: alljoyn_busattachment, authmechanisms: super::super::Foundation::PSTR, listener: alljoyn_authlistener, keystorefilename: super::super::Foundation::PSTR, isshared: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_enablepeersecuritywithpermissionconfigurationlistener(bus: alljoyn_busattachment, authmechanisms: super::super::Foundation::PSTR, authlistener: alljoyn_authlistener, keystorefilename: super::super::Foundation::PSTR, isshared: i32, permissionconfigurationlistener: alljoyn_permissionconfigurationlistener) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_findadvertisedname(bus: alljoyn_busattachment, nameprefix: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_findadvertisednamebytransport(bus: alljoyn_busattachment, nameprefix: super::super::Foundation::PSTR, transports: u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getalljoyndebugobj(bus: alljoyn_busattachment) -> alljoyn_proxybusobject;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getalljoynproxyobj(bus: alljoyn_busattachment) -> alljoyn_proxybusobject;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getconcurrency(bus: alljoyn_busattachment) -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getconnectspec(bus: alljoyn_busattachment) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getdbusproxyobj(bus: alljoyn_busattachment) -> alljoyn_proxybusobject;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getglobalguidstring(bus: alljoyn_busattachment) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getinterface(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR) -> alljoyn_interfacedescription;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getinterfaces(bus: alljoyn_busattachment, ifaces: *const alljoyn_interfacedescription, numifaces: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getkeyexpiration(bus: alljoyn_busattachment, guid: super::super::Foundation::PSTR, timeout: *mut u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getpeerguid(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, guid: super::super::Foundation::PSTR, guidsz: *mut usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getpermissionconfigurator(bus: alljoyn_busattachment) -> alljoyn_permissionconfigurator;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_gettimestamp() -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getuniquename(bus: alljoyn_busattachment) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_isconnected(bus: alljoyn_busattachment) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_ispeersecurityenabled(bus: alljoyn_busattachment) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_isstarted(bus: alljoyn_busattachment) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_isstopping(bus: alljoyn_busattachment) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_join(bus: alljoyn_busattachment) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_joinsession(bus: alljoyn_busattachment, sessionhost: super::super::Foundation::PSTR, sessionport: u16, listener: alljoyn_sessionlistener, sessionid: *mut u32, opts: alljoyn_sessionopts) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_joinsessionasync(bus: alljoyn_busattachment, sessionhost: super::super::Foundation::PSTR, sessionport: u16, listener: alljoyn_sessionlistener, opts: alljoyn_sessionopts, callback: alljoyn_busattachment_joinsessioncb_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_leavesession(bus: alljoyn_busattachment, sessionid: u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_namehasowner(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, hasowner: *mut i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_ping(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, timeout: u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registeraboutlistener(bus: alljoyn_busattachment, aboutlistener: alljoyn_aboutlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerapplicationstatelistener(bus: alljoyn_busattachment, listener: alljoyn_applicationstatelistener) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerbuslistener(bus: alljoyn_busattachment, listener: alljoyn_buslistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerbusobject(bus: alljoyn_busattachment, obj: alljoyn_busobject) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerbusobject_secure(bus: alljoyn_busattachment, obj: alljoyn_busobject) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerkeystorelistener(bus: alljoyn_busattachment, listener: alljoyn_keystorelistener) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_registersignalhandler(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, srcpath: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_registersignalhandlerwithrule(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, matchrule: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_releasename(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_reloadkeystore(bus: alljoyn_busattachment) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_removematch(bus: alljoyn_busattachment, rule: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_removesessionmember(bus: alljoyn_busattachment, sessionid: u32, membername: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_requestname(bus: alljoyn_busattachment, requestedname: super::super::Foundation::PSTR, flags: u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_secureconnection(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, forceauth: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_secureconnectionasync(bus: alljoyn_busattachment, name: super::super::Foundation::PSTR, forceauth: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_setdaemondebug(bus: alljoyn_busattachment, module: super::super::Foundation::PSTR, level: u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_setkeyexpiration(bus: alljoyn_busattachment, guid: super::super::Foundation::PSTR, timeout: u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_setlinktimeout(bus: alljoyn_busattachment, sessionid: u32, linktimeout: *mut u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_setlinktimeoutasync(bus: alljoyn_busattachment, sessionid: u32, linktimeout: u32, callback: alljoyn_busattachment_setlinktimeoutcb_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_setsessionlistener(bus: alljoyn_busattachment, sessionid: u32, listener: alljoyn_sessionlistener) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_start(bus: alljoyn_busattachment) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_stop(bus: alljoyn_busattachment) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unbindsessionport(bus: alljoyn_busattachment, sessionport: u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisteraboutlistener(bus: alljoyn_busattachment, aboutlistener: alljoyn_aboutlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterallaboutlisteners(bus: alljoyn_busattachment);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterallhandlers(bus: alljoyn_busattachment) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterapplicationstatelistener(bus: alljoyn_busattachment, listener: alljoyn_applicationstatelistener) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterbuslistener(bus: alljoyn_busattachment, listener: alljoyn_buslistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterbusobject(bus: alljoyn_busattachment, object: alljoyn_busobject);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_unregistersignalhandler(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, srcpath: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_unregistersignalhandlerwithrule(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, matchrule: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_whoimplements_interface(bus: alljoyn_busattachment, implementsinterface: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_whoimplements_interfaces(bus: alljoyn_busattachment, implementsinterfaces: *const *const i8, numberinterfaces: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_buslistener_create(callbacks: *const alljoyn_buslistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_buslistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_buslistener_destroy(listener: alljoyn_buslistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_addinterface(bus: alljoyn_busobject, iface: alljoyn_interfacedescription) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_addinterface_announced(bus: alljoyn_busobject, iface: alljoyn_interfacedescription) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_addmethodhandler(bus: alljoyn_busobject, member: alljoyn_interfacedescription_member, handler: alljoyn_messagereceiver_methodhandler_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_addmethodhandlers(bus: alljoyn_busobject, entries: *const alljoyn_busobject_methodentry, numentries: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_cancelsessionlessmessage(bus: alljoyn_busobject, msg: alljoyn_message) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_cancelsessionlessmessage_serial(bus: alljoyn_busobject, serialnumber: u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_create(path: super::super::Foundation::PSTR, isplaceholder: i32, callbacks_in: *const alljoyn_busobject_callbacks, context_in: *const ::core::ffi::c_void) -> alljoyn_busobject;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_destroy(bus: alljoyn_busobject);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_emitpropertieschanged(bus: alljoyn_busobject, ifcname: super::super::Foundation::PSTR, propnames: *const *const i8, numprops: usize, id: u32);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_emitpropertychanged(bus: alljoyn_busobject, ifcname: super::super::Foundation::PSTR, propname: super::super::Foundation::PSTR, val: alljoyn_msgarg, id: u32);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_getannouncedinterfacenames(bus: alljoyn_busobject, interfaces: *const *const i8, numinterfaces: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_getbusattachment(bus: alljoyn_busobject) -> alljoyn_busattachment;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_getname(bus: alljoyn_busobject, buffer: super::super::Foundation::PSTR, buffersz: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_getpath(bus: alljoyn_busobject) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_issecure(bus: alljoyn_busobject) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_methodreply_args(bus: alljoyn_busobject, msg: alljoyn_message, args: alljoyn_msgarg, numargs: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_methodreply_err(bus: alljoyn_busobject, msg: alljoyn_message, error: super::super::Foundation::PSTR, errormessage: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_methodreply_status(bus: alljoyn_busobject, msg: alljoyn_message, status: QStatus) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_setannounceflag(bus: alljoyn_busobject, iface: alljoyn_interfacedescription, isannounced: alljoyn_about_announceflag) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_signal(bus: alljoyn_busobject, destination: super::super::Foundation::PSTR, sessionid: u32, signal: alljoyn_interfacedescription_member, args: alljoyn_msgarg, numargs: usize, timetolive: u16, flags: u8, msg: alljoyn_message) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_clear(cred: alljoyn_credentials);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_create() -> alljoyn_credentials;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_destroy(cred: alljoyn_credentials);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getcertchain(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_getexpiration(cred: alljoyn_credentials) -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getlogonentry(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getpassword(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getprivateKey(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getusername(cred: alljoyn_credentials) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_isset(cred: alljoyn_credentials, creds: u16) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setcertchain(cred: alljoyn_credentials, certchain: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_setexpiration(cred: alljoyn_credentials, expiration: u32);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setlogonentry(cred: alljoyn_credentials, logonentry: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setpassword(cred: alljoyn_credentials, pwd: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setprivatekey(cred: alljoyn_credentials, pk: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setusername(cred: alljoyn_credentials, username: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_getbuildinfo() -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_getnumericversion() -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_getversion() -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_init() -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_activate(iface: alljoyn_interfacedescription);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addannotation(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addargannotation(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, argname: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addmember(iface: alljoyn_interfacedescription, r#type: alljoyn_messagetype, name: super::super::Foundation::PSTR, inputsig: super::super::Foundation::PSTR, outsig: super::super::Foundation::PSTR, argnames: super::super::Foundation::PSTR, annotation: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addmemberannotation(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addmethod(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, inputsig: super::super::Foundation::PSTR, outsig: super::super::Foundation::PSTR, argnames: super::super::Foundation::PSTR, annotation: u8, accessperms: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addproperty(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, signature: super::super::Foundation::PSTR, access: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addpropertyannotation(iface: alljoyn_interfacedescription, property: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addsignal(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, sig: super::super::Foundation::PSTR, argnames: super::super::Foundation::PSTR, annotation: u8, accessperms: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_eql(one: alljoyn_interfacedescription, other: alljoyn_interfacedescription) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getannotation(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getannotationatindex(iface: alljoyn_interfacedescription, index: usize, name: super::super::Foundation::PSTR, name_size: *mut usize, value: super::super::Foundation::PSTR, value_size: *mut usize);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_getannotationscount(iface: alljoyn_interfacedescription) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getargdescriptionforlanguage(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, arg: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, maxlanguagelength: usize, languagetag: super::super::Foundation::PSTR) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getdescriptionforlanguage(iface: alljoyn_interfacedescription, description: super::super::Foundation::PSTR, maxlanguagelength: usize, languagetag: super::super::Foundation::PSTR) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_getdescriptionlanguages(iface: alljoyn_interfacedescription, languages: *const *const i8, size: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getdescriptionlanguages2(iface: alljoyn_interfacedescription, languages: super::super::Foundation::PSTR, languagessize: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getdescriptiontranslationcallback(iface: alljoyn_interfacedescription) -> ::core::option::Option<alljoyn_interfacedescription_translation_callback_ptr>;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmember(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, member: *mut alljoyn_interfacedescription_member) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmemberannotation(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmemberargannotation(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, argname: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmemberdescriptionforlanguage(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, maxlanguagelength: usize, languagetag: super::super::Foundation::PSTR) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmembers(iface: alljoyn_interfacedescription, members: *mut alljoyn_interfacedescription_member, nummembers: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmethod(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, member: *mut alljoyn_interfacedescription_member) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getname(iface: alljoyn_interfacedescription) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getproperties(iface: alljoyn_interfacedescription, props: *mut alljoyn_interfacedescription_property, numprops: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getproperty(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, property: *mut alljoyn_interfacedescription_property) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getpropertyannotation(iface: alljoyn_interfacedescription, property: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, str_size: *mut usize) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getpropertydescriptionforlanguage(iface: alljoyn_interfacedescription, property: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, maxlanguagelength: usize, languagetag: super::super::Foundation::PSTR) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_getsecuritypolicy(iface: alljoyn_interfacedescription) -> alljoyn_interfacedescription_securitypolicy;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getsignal(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, member: *mut alljoyn_interfacedescription_member) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_hasdescription(iface: alljoyn_interfacedescription) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_hasmember(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, insig: super::super::Foundation::PSTR, outsig: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_hasproperties(iface: alljoyn_interfacedescription) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_hasproperty(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_introspect(iface: alljoyn_interfacedescription, str: super::super::Foundation::PSTR, buf: usize, indent: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_issecure(iface: alljoyn_interfacedescription) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_eql(one: alljoyn_interfacedescription_member, other: alljoyn_interfacedescription_member) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getannotation(member: alljoyn_interfacedescription_member, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getannotationatindex(member: alljoyn_interfacedescription_member, index: usize, name: super::super::Foundation::PSTR, name_size: *mut usize, value: super::super::Foundation::PSTR, value_size: *mut usize);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getannotationscount(member: alljoyn_interfacedescription_member) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getargannotation(member: alljoyn_interfacedescription_member, argname: super::super::Foundation::PSTR, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getargannotationatindex(member: alljoyn_interfacedescription_member, argname: super::super::Foundation::PSTR, index: usize, name: super::super::Foundation::PSTR, name_size: *mut usize, value: super::super::Foundation::PSTR, value_size: *mut usize);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getargannotationscount(member: alljoyn_interfacedescription_member, argname: super::super::Foundation::PSTR) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_property_eql(one: alljoyn_interfacedescription_property, other: alljoyn_interfacedescription_property) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_property_getannotation(property: alljoyn_interfacedescription_property, name: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, value_size: *mut usize) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_property_getannotationatindex(property: alljoyn_interfacedescription_property, index: usize, name: super::super::Foundation::PSTR, name_size: *mut usize, value: super::super::Foundation::PSTR, value_size: *mut usize);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_property_getannotationscount(property: alljoyn_interfacedescription_property) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setargdescription(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, argname: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setargdescriptionforlanguage(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, arg: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, languagetag: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setdescription(iface: alljoyn_interfacedescription, description: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setdescriptionforlanguage(iface: alljoyn_interfacedescription, description: super::super::Foundation::PSTR, languagetag: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setdescriptionlanguage(iface: alljoyn_interfacedescription, language: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setdescriptiontranslationcallback(iface: alljoyn_interfacedescription, translationcallback: alljoyn_interfacedescription_translation_callback_ptr);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setmemberdescription(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setmemberdescriptionforlanguage(iface: alljoyn_interfacedescription, member: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, languagetag: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setpropertydescription(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setpropertydescriptionforlanguage(iface: alljoyn_interfacedescription, name: super::super::Foundation::PSTR, description: super::super::Foundation::PSTR, languagetag: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_keystorelistener_create(callbacks: *const alljoyn_keystorelistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_keystorelistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_keystorelistener_destroy(listener: alljoyn_keystorelistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_keystorelistener_getkeys(listener: alljoyn_keystorelistener, keystore: alljoyn_keystore, sink: super::super::Foundation::PSTR, sink_sz: *mut usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_keystorelistener_putkeys(listener: alljoyn_keystorelistener, keystore: alljoyn_keystore, source: super::super::Foundation::PSTR, password: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_keystorelistener_with_synchronization_create(callbacks: *const alljoyn_keystorelistener_with_synchronization_callbacks, context: *mut ::core::ffi::c_void) -> alljoyn_keystorelistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_create(bus: alljoyn_busattachment) -> alljoyn_message;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_description(msg: alljoyn_message, str: super::super::Foundation::PSTR, buf: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_destroy(msg: alljoyn_message);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_eql(one: alljoyn_message, other: alljoyn_message) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getarg(msg: alljoyn_message, argn: usize) -> alljoyn_msgarg;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getargs(msg: alljoyn_message, numargs: *mut usize, args: *mut alljoyn_msgarg);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getauthmechanism(msg: alljoyn_message) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getcallserial(msg: alljoyn_message) -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getcompressiontoken(msg: alljoyn_message) -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getdestination(msg: alljoyn_message) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_geterrorname(msg: alljoyn_message, errormessage: super::super::Foundation::PSTR, errormessage_size: *mut usize) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getflags(msg: alljoyn_message) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getinterface(msg: alljoyn_message) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getmembername(msg: alljoyn_message) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getobjectpath(msg: alljoyn_message) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getreceiveendpointname(msg: alljoyn_message) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getreplyserial(msg: alljoyn_message) -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getsender(msg: alljoyn_message) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getsessionid(msg: alljoyn_message) -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getsignature(msg: alljoyn_message) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_gettimestamp(msg: alljoyn_message) -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_gettype(msg: alljoyn_message) -> alljoyn_messagetype;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isbroadcastsignal(msg: alljoyn_message) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isencrypted(msg: alljoyn_message) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isexpired(msg: alljoyn_message, tillexpirems: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isglobalbroadcast(msg: alljoyn_message) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_issessionless(msg: alljoyn_message) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isunreliable(msg: alljoyn_message) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_parseargs(msg: alljoyn_message, signature: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_setendianess(endian: i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_tostring(msg: alljoyn_message, str: super::super::Foundation::PSTR, buf: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_array_create(size: usize) -> alljoyn_msgarg;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_array_element(arg: alljoyn_msgarg, index: usize) -> alljoyn_msgarg;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_get(args: alljoyn_msgarg, numargs: usize, signature: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_set(args: alljoyn_msgarg, numargs: *mut usize, signature: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_set_offset(args: alljoyn_msgarg, argoffset: usize, numargs: *mut usize, signature: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_signature(values: alljoyn_msgarg, numvalues: usize, str: super::super::Foundation::PSTR, buf: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_tostring(args: alljoyn_msgarg, numargs: usize, str: super::super::Foundation::PSTR, buf: usize, indent: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_clear(arg: alljoyn_msgarg);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_clone(destination: alljoyn_msgarg, source: alljoyn_msgarg);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_copy(source: alljoyn_msgarg) -> alljoyn_msgarg;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_create() -> alljoyn_msgarg;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_create_and_set(signature: super::super::Foundation::PSTR) -> alljoyn_msgarg;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_destroy(arg: alljoyn_msgarg);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_equal(lhv: alljoyn_msgarg, rhv: alljoyn_msgarg) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_get(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_array_element(arg: alljoyn_msgarg, index: usize, element: *mut alljoyn_msgarg);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_get_array_elementsignature(arg: alljoyn_msgarg, index: usize) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_array_numberofelements(arg: alljoyn_msgarg) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_bool(arg: alljoyn_msgarg, b: *mut i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_bool_array(arg: alljoyn_msgarg, length: *mut usize, ab: *mut i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_double(arg: alljoyn_msgarg, d: *mut f64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_double_array(arg: alljoyn_msgarg, length: *mut usize, ad: *mut f64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int16(arg: alljoyn_msgarg, n: *mut i16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int16_array(arg: alljoyn_msgarg, length: *mut usize, an: *mut i16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int32(arg: alljoyn_msgarg, i: *mut i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int32_array(arg: alljoyn_msgarg, length: *mut usize, ai: *mut i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int64(arg: alljoyn_msgarg, x: *mut i64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int64_array(arg: alljoyn_msgarg, length: *mut usize, ax: *mut i64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_objectpath(arg: alljoyn_msgarg, o: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_signature(arg: alljoyn_msgarg, g: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_string(arg: alljoyn_msgarg, s: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint16(arg: alljoyn_msgarg, q: *mut u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint16_array(arg: alljoyn_msgarg, length: *mut usize, aq: *mut u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint32(arg: alljoyn_msgarg, u: *mut u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint32_array(arg: alljoyn_msgarg, length: *mut usize, au: *mut u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint64(arg: alljoyn_msgarg, t: *mut u64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint64_array(arg: alljoyn_msgarg, length: *mut usize, at: *mut u64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint8(arg: alljoyn_msgarg, y: *mut u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint8_array(arg: alljoyn_msgarg, length: *mut usize, ay: *mut u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_variant(arg: alljoyn_msgarg, v: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_get_variant_array(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR, length: *mut usize, av: *mut alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_getdictelement(arg: alljoyn_msgarg, elemsig: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_getkey(arg: alljoyn_msgarg) -> alljoyn_msgarg;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_getmember(arg: alljoyn_msgarg, index: usize) -> alljoyn_msgarg;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_getnummembers(arg: alljoyn_msgarg) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_gettype(arg: alljoyn_msgarg) -> alljoyn_typeid;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_getvalue(arg: alljoyn_msgarg) -> alljoyn_msgarg;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_hassignature(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set_and_stabilize(arg: alljoyn_msgarg, signature: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_bool(arg: alljoyn_msgarg, b: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_bool_array(arg: alljoyn_msgarg, length: usize, ab: *mut i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_double(arg: alljoyn_msgarg, d: f64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_double_array(arg: alljoyn_msgarg, length: usize, ad: *mut f64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int16(arg: alljoyn_msgarg, n: i16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int16_array(arg: alljoyn_msgarg, length: usize, an: *mut i16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int32(arg: alljoyn_msgarg, i: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int32_array(arg: alljoyn_msgarg, length: usize, ai: *mut i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int64(arg: alljoyn_msgarg, x: i64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int64_array(arg: alljoyn_msgarg, length: usize, ax: *mut i64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set_objectpath(arg: alljoyn_msgarg, o: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_objectpath_array(arg: alljoyn_msgarg, length: usize, ao: *const *const i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set_signature(arg: alljoyn_msgarg, g: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_signature_array(arg: alljoyn_msgarg, length: usize, ag: *const *const i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set_string(arg: alljoyn_msgarg, s: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_string_array(arg: alljoyn_msgarg, length: usize, r#as: *const *const i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint16(arg: alljoyn_msgarg, q: u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint16_array(arg: alljoyn_msgarg, length: usize, aq: *mut u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint32(arg: alljoyn_msgarg, u: u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint32_array(arg: alljoyn_msgarg, length: usize, au: *mut u32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint64(arg: alljoyn_msgarg, t: u64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint64_array(arg: alljoyn_msgarg, length: usize, at: *mut u64) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint8(arg: alljoyn_msgarg, y: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint8_array(arg: alljoyn_msgarg, length: usize, ay: *mut u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_setdictentry(arg: alljoyn_msgarg, key: alljoyn_msgarg, value: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_setstruct(arg: alljoyn_msgarg, struct_members: alljoyn_msgarg, num_members: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_signature(arg: alljoyn_msgarg, str: super::super::Foundation::PSTR, buf: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_stabilize(arg: alljoyn_msgarg);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_tostring(arg: alljoyn_msgarg, str: super::super::Foundation::PSTR, buf: usize, indent: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_create(bus: alljoyn_busattachment, mandatoryinterfaces: *const *const i8, nummandatoryinterfaces: usize) -> alljoyn_observer;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_destroy(observer: alljoyn_observer);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_observer_get(observer: alljoyn_observer, uniquebusname: super::super::Foundation::PSTR, objectpath: super::super::Foundation::PSTR) -> alljoyn_proxybusobject_ref;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_getfirst(observer: alljoyn_observer) -> alljoyn_proxybusobject_ref;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_getnext(observer: alljoyn_observer, proxyref: alljoyn_proxybusobject_ref) -> alljoyn_proxybusobject_ref;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_registerlistener(observer: alljoyn_observer, listener: alljoyn_observerlistener, triggeronexisting: i32);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_unregisteralllisteners(observer: alljoyn_observer);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_unregisterlistener(observer: alljoyn_observer, listener: alljoyn_observerlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observerlistener_create(callback: *const alljoyn_observerlistener_callback, context: *const ::core::ffi::c_void) -> alljoyn_observerlistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observerlistener_destroy(listener: alljoyn_observerlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_passwordmanager_setcredentials(authmechanism: super::super::Foundation::PSTR, password: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurationlistener_create(callbacks: *const alljoyn_permissionconfigurationlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_permissionconfigurationlistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurationlistener_destroy(listener: alljoyn_permissionconfigurationlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_certificatechain_destroy(certificatechain: *mut i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_certificateid_cleanup(certificateid: *mut alljoyn_certificateid);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_certificateidarray_cleanup(certificateidarray: *mut alljoyn_certificateidarray);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_claim(configurator: alljoyn_permissionconfigurator, cakey: *mut i8, identitycertificatechain: *mut i8, groupid: *const u8, groupsize: usize, groupauthority: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_endmanagement(configurator: alljoyn_permissionconfigurator) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getapplicationstate(configurator: alljoyn_permissionconfigurator, state: *mut alljoyn_applicationstate) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getclaimcapabilities(configurator: alljoyn_permissionconfigurator, claimcapabilities: *mut u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getclaimcapabilitiesadditionalinfo(configurator: alljoyn_permissionconfigurator, additionalinfo: *mut u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getdefaultclaimcapabilities() -> u16;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getdefaultpolicy(configurator: alljoyn_permissionconfigurator, policyxml: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getidentity(configurator: alljoyn_permissionconfigurator, identitycertificatechain: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getidentitycertificateid(configurator: alljoyn_permissionconfigurator, certificateid: *mut alljoyn_certificateid) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getmanifests(configurator: alljoyn_permissionconfigurator, manifestarray: *mut alljoyn_manifestarray) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getmanifesttemplate(configurator: alljoyn_permissionconfigurator, manifesttemplatexml: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getmembershipsummaries(configurator: alljoyn_permissionconfigurator, certificateids: *mut alljoyn_certificateidarray) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getpolicy(configurator: alljoyn_permissionconfigurator, policyxml: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getpublickey(configurator: alljoyn_permissionconfigurator, publickey: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_installmanifests(configurator: alljoyn_permissionconfigurator, manifestsxmls: *mut *mut i8, manifestscount: usize, append: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_installmembership(configurator: alljoyn_permissionconfigurator, membershipcertificatechain: *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_manifestarray_cleanup(manifestarray: *mut alljoyn_manifestarray);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_manifesttemplate_destroy(manifesttemplatexml: *mut i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_policy_destroy(policyxml: *mut i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_publickey_destroy(publickey: *mut i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_removemembership(configurator: alljoyn_permissionconfigurator, serial: *const u8, seriallen: usize, issuerpublickey: *mut i8, issueraki: *const u8, issuerakilen: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_reset(configurator: alljoyn_permissionconfigurator) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_resetpolicy(configurator: alljoyn_permissionconfigurator) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_setapplicationstate(configurator: alljoyn_permissionconfigurator, state: alljoyn_applicationstate) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_setclaimcapabilities(configurator: alljoyn_permissionconfigurator, claimcapabilities: u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_setclaimcapabilitiesadditionalinfo(configurator: alljoyn_permissionconfigurator, additionalinfo: u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_setmanifesttemplatefromxml(configurator: alljoyn_permissionconfigurator, manifesttemplatexml: *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_startmanagement(configurator: alljoyn_permissionconfigurator) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_updateidentity(configurator: alljoyn_permissionconfigurator, identitycertificatechain: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_updatepolicy(configurator: alljoyn_permissionconfigurator, policyxml: *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_pinglistener_create(callback: *const alljoyn_pinglistener_callback, context: *const ::core::ffi::c_void) -> alljoyn_pinglistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_pinglistener_destroy(listener: alljoyn_pinglistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_addchild(proxyobj: alljoyn_proxybusobject, child: alljoyn_proxybusobject) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_addinterface(proxyobj: alljoyn_proxybusobject, iface: alljoyn_interfacedescription) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_addinterface_by_name(proxyobj: alljoyn_proxybusobject, name: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_copy(source: alljoyn_proxybusobject) -> alljoyn_proxybusobject;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_create(bus: alljoyn_busattachment, service: super::super::Foundation::PSTR, path: super::super::Foundation::PSTR, sessionid: u32) -> alljoyn_proxybusobject;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_create_secure(bus: alljoyn_busattachment, service: super::super::Foundation::PSTR, path: super::super::Foundation::PSTR, sessionid: u32) -> alljoyn_proxybusobject;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_destroy(proxyobj: alljoyn_proxybusobject);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_enablepropertycaching(proxyobj: alljoyn_proxybusobject);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getallproperties(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, values: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getallpropertiesasync(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, callback: alljoyn_proxybusobject_listener_getallpropertiescb_ptr, timeout: u32, context: *mut ::core::ffi::c_void) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getchild(proxyobj: alljoyn_proxybusobject, path: super::super::Foundation::PSTR) -> alljoyn_proxybusobject;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_getchildren(proxyobj: alljoyn_proxybusobject, children: *mut alljoyn_proxybusobject, numchildren: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getinterface(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR) -> alljoyn_interfacedescription;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_getinterfaces(proxyobj: alljoyn_proxybusobject, ifaces: *const alljoyn_interfacedescription, numifaces: usize) -> usize;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getpath(proxyobj: alljoyn_proxybusobject) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getproperty(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, property: super::super::Foundation::PSTR, value: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getpropertyasync(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, property: super::super::Foundation::PSTR, callback: alljoyn_proxybusobject_listener_getpropertycb_ptr, timeout: u32, context: *mut ::core::ffi::c_void) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getservicename(proxyobj: alljoyn_proxybusobject) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_getsessionid(proxyobj: alljoyn_proxybusobject) -> u32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getuniquename(proxyobj: alljoyn_proxybusobject) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_implementsinterface(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_introspectremoteobject(proxyobj: alljoyn_proxybusobject) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_introspectremoteobjectasync(proxyobj: alljoyn_proxybusobject, callback: alljoyn_proxybusobject_listener_introspectcb_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_issecure(proxyobj: alljoyn_proxybusobject) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_isvalid(proxyobj: alljoyn_proxybusobject) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcall(proxyobj: alljoyn_proxybusobject, ifacename: super::super::Foundation::PSTR, methodname: super::super::Foundation::PSTR, args: alljoyn_msgarg, numargs: usize, replymsg: alljoyn_message, timeout: u32, flags: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcall_member(proxyobj: alljoyn_proxybusobject, method: alljoyn_interfacedescription_member, args: alljoyn_msgarg, numargs: usize, replymsg: alljoyn_message, timeout: u32, flags: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcall_member_noreply(proxyobj: alljoyn_proxybusobject, method: alljoyn_interfacedescription_member, args: alljoyn_msgarg, numargs: usize, flags: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcall_noreply(proxyobj: alljoyn_proxybusobject, ifacename: super::super::Foundation::PSTR, methodname: super::super::Foundation::PSTR, args: alljoyn_msgarg, numargs: usize, flags: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcallasync(proxyobj: alljoyn_proxybusobject, ifacename: super::super::Foundation::PSTR, methodname: super::super::Foundation::PSTR, replyfunc: alljoyn_messagereceiver_replyhandler_ptr, args: alljoyn_msgarg, numargs: usize, context: *mut ::core::ffi::c_void, timeout: u32, flags: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcallasync_member(proxyobj: alljoyn_proxybusobject, method: alljoyn_interfacedescription_member, replyfunc: alljoyn_messagereceiver_replyhandler_ptr, args: alljoyn_msgarg, numargs: usize, context: *mut ::core::ffi::c_void, timeout: u32, flags: u8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_parsexml(proxyobj: alljoyn_proxybusobject, xml: super::super::Foundation::PSTR, identifier: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_ref_create(proxy: alljoyn_proxybusobject) -> alljoyn_proxybusobject_ref;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_ref_decref(r#ref: alljoyn_proxybusobject_ref);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_ref_get(r#ref: alljoyn_proxybusobject_ref) -> alljoyn_proxybusobject;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_ref_incref(r#ref: alljoyn_proxybusobject_ref);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_registerpropertieschangedlistener(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, properties: *const *const i8, numproperties: usize, callback: alljoyn_proxybusobject_listener_propertieschanged_ptr, context: *mut ::core::ffi::c_void) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_removechild(proxyobj: alljoyn_proxybusobject, path: super::super::Foundation::PSTR) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_secureconnection(proxyobj: alljoyn_proxybusobject, forceauth: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_secureconnectionasync(proxyobj: alljoyn_proxybusobject, forceauth: i32) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_setproperty(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, property: super::super::Foundation::PSTR, value: alljoyn_msgarg) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_setpropertyasync(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, property: super::super::Foundation::PSTR, value: alljoyn_msgarg, callback: alljoyn_proxybusobject_listener_setpropertycb_ptr, timeout: u32, context: *mut ::core::ffi::c_void) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_unregisterpropertieschangedlistener(proxyobj: alljoyn_proxybusobject, iface: super::super::Foundation::PSTR, callback: alljoyn_proxybusobject_listener_propertieschanged_ptr) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_routerinit() -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_routerinitwithconfig(configxml: *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_routershutdown() -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_claim(proxy: alljoyn_securityapplicationproxy, cakey: *mut i8, identitycertificatechain: *mut i8, groupid: *const u8, groupsize: usize, groupauthority: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_computemanifestdigest(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, digest: *mut *mut u8, digestsize: *mut usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_create(bus: alljoyn_busattachment, appbusname: *mut i8, sessionid: u32) -> alljoyn_securityapplicationproxy;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_destroy(proxy: alljoyn_securityapplicationproxy);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_digest_destroy(digest: *mut u8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_eccpublickey_destroy(eccpublickey: *mut i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_endmanagement(proxy: alljoyn_securityapplicationproxy) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getapplicationstate(proxy: alljoyn_securityapplicationproxy, applicationstate: *mut alljoyn_applicationstate) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getclaimcapabilities(proxy: alljoyn_securityapplicationproxy, capabilities: *mut u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getclaimcapabilitiesadditionalinfo(proxy: alljoyn_securityapplicationproxy, additionalinfo: *mut u16) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getdefaultpolicy(proxy: alljoyn_securityapplicationproxy, policyxml: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_geteccpublickey(proxy: alljoyn_securityapplicationproxy, eccpublickey: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getmanifesttemplate(proxy: alljoyn_securityapplicationproxy, manifesttemplatexml: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getpermissionmanagementsessionport() -> u16;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getpolicy(proxy: alljoyn_securityapplicationproxy, policyxml: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_installmembership(proxy: alljoyn_securityapplicationproxy, membershipcertificatechain: *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_manifest_destroy(signedmanifestxml: *mut i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_manifesttemplate_destroy(manifesttemplatexml: *mut i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_policy_destroy(policyxml: *mut i8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_reset(proxy: alljoyn_securityapplicationproxy) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_resetpolicy(proxy: alljoyn_securityapplicationproxy) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_setmanifestsignature(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, signature: *const u8, signaturesize: usize, signedmanifestxml: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_signmanifest(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, signingprivatekeypem: *mut i8, signedmanifestxml: *mut *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_startmanagement(proxy: alljoyn_securityapplicationproxy) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_updateidentity(proxy: alljoyn_securityapplicationproxy, identitycertificatechain: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_updatepolicy(proxy: alljoyn_securityapplicationproxy, policyxml: *mut i8) -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_sessionlistener_create(callbacks: *const alljoyn_sessionlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_sessionlistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionlistener_destroy(listener: alljoyn_sessionlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_cmp(one: alljoyn_sessionopts, other: alljoyn_sessionopts) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_create(traffic: u8, ismultipoint: i32, proximity: u8, transports: u16) -> alljoyn_sessionopts;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_destroy(opts: alljoyn_sessionopts);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_get_multipoint(opts: alljoyn_sessionopts) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_get_proximity(opts: alljoyn_sessionopts) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_get_traffic(opts: alljoyn_sessionopts) -> u8;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_get_transports(opts: alljoyn_sessionopts) -> u16;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_iscompatible(one: alljoyn_sessionopts, other: alljoyn_sessionopts) -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_set_multipoint(opts: alljoyn_sessionopts, ismultipoint: i32);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_set_proximity(opts: alljoyn_sessionopts, proximity: u8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_set_traffic(opts: alljoyn_sessionopts, traffic: u8);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_set_transports(opts: alljoyn_sessionopts, transports: u16);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_sessionportlistener_create(callbacks: *const alljoyn_sessionportlistener_callbacks, context: *const ::core::ffi::c_void) -> alljoyn_sessionportlistener;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionportlistener_destroy(listener: alljoyn_sessionportlistener);
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_shutdown() -> QStatus;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_unity_deferred_callbacks_process() -> i32;
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_unity_set_deferred_callback_mainthread_only(mainthread_only: i32);
}
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_BIG_ENDIAN: u8 = 66u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_CRED_CERT_CHAIN: u16 = 4u16;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_CRED_EXPIRATION: u16 = 32u16;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_CRED_LOGON_ENTRY: u16 = 16u16;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_CRED_NEW_PASSWORD: u16 = 4097u16;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_CRED_ONE_TIME_PWD: u16 = 8193u16;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_CRED_PASSWORD: u16 = 1u16;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_CRED_PRIVATE_KEY: u16 = 8u16;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_CRED_USER_NAME: u16 = 2u16;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_DISCONNECTED: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_LITTLE_ENDIAN: u8 = 108u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_DEPRECATED: u8 = 2u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_GLOBAL_BROADCAST: u8 = 32u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_NO_REPLY: u8 = 1u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONCAST: u8 = 4u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONLESS: u8 = 8u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MEMBER_ANNOTATE_UNICAST: u8 = 16u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MESSAGE_DEFAULT_TIMEOUT: u32 = 25000u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MESSAGE_FLAG_ALLOW_REMOTE_MSG: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MESSAGE_FLAG_AUTO_START: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MESSAGE_FLAG_ENCRYPTED: u32 = 128u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MESSAGE_FLAG_GLOBAL_BROADCAST: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MESSAGE_FLAG_NO_REPLY_EXPECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_MESSAGE_FLAG_SESSIONLESS: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_PROP_ACCESS_READ: u8 = 1u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_PROP_ACCESS_RW: u8 = 3u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_PROP_ACCESS_WRITE: u8 = 2u8;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_PROXIMITY_ANY: u32 = 255u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_PROXIMITY_NETWORK: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_PROXIMITY_PHYSICAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_READ_READY: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_TRAFFIC_TYPE_MESSAGES: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_TRAFFIC_TYPE_RAW_RELIABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_TRAFFIC_TYPE_RAW_UNRELIABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const ALLJOYN_WRITE_READY: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const QCC_FALSE: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
pub const QCC_TRUE: u32 = 1u32;
pub struct QStatus(i32);
pub struct _alljoyn_abouticon_handle(i32);
pub struct _alljoyn_abouticonobj_handle(i32);
pub struct _alljoyn_abouticonproxy_handle(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_about_announced_ptr(i32);
pub struct alljoyn_about_announceflag(i32);
pub struct alljoyn_aboutdata(i32);
pub struct alljoyn_aboutdatalistener(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_aboutdatalistener_callbacks(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_aboutdatalistener_getaboutdata_ptr(i32);
pub struct alljoyn_aboutdatalistener_getannouncedaboutdata_ptr(i32);
pub struct alljoyn_aboutlistener(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_aboutlistener_callback(i32);
pub struct alljoyn_aboutobj(i32);
pub struct alljoyn_aboutobjectdescription(i32);
pub struct alljoyn_aboutproxy(i32);
pub struct alljoyn_applicationstate(i32);
pub struct alljoyn_applicationstatelistener(i32);
pub struct alljoyn_applicationstatelistener_callbacks(i32);
pub struct alljoyn_applicationstatelistener_state_ptr(i32);
pub struct alljoyn_authlistener(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_authlistener_authenticationcomplete_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_authlistener_callbacks(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_authlistener_requestcredentials_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_authlistener_requestcredentialsasync_ptr(i32);
pub struct alljoyn_authlistener_securityviolation_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_authlistener_verifycredentials_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_authlistener_verifycredentialsasync_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_authlistenerasync_callbacks(i32);
pub struct alljoyn_autopinger(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_autopinger_destination_found_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_autopinger_destination_lost_ptr(i32);
pub struct alljoyn_busattachment(i32);
pub struct alljoyn_busattachment_joinsessioncb_ptr(i32);
pub struct alljoyn_busattachment_setlinktimeoutcb_ptr(i32);
pub struct alljoyn_buslistener(i32);
pub struct alljoyn_buslistener_bus_disconnected_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_buslistener_bus_prop_changed_ptr(i32);
pub struct alljoyn_buslistener_bus_stopping_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_buslistener_callbacks(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_buslistener_found_advertised_name_ptr(i32);
pub struct alljoyn_buslistener_listener_registered_ptr(i32);
pub struct alljoyn_buslistener_listener_unregistered_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_buslistener_lost_advertised_name_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_buslistener_name_owner_changed_ptr(i32);
pub struct alljoyn_busobject(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_busobject_callbacks(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_busobject_methodentry(i32);
pub struct alljoyn_busobject_object_registration_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_busobject_prop_get_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_busobject_prop_set_ptr(i32);
pub struct alljoyn_certificateid(i32);
pub struct alljoyn_certificateidarray(i32);
pub struct alljoyn_claimcapability_masks(i32);
pub struct alljoyn_claimcapabilityadditionalinfo_masks(i32);
pub struct alljoyn_credentials(i32);
pub struct alljoyn_interfacedescription(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_interfacedescription_member(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_interfacedescription_property(i32);
pub struct alljoyn_interfacedescription_securitypolicy(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_interfacedescription_translation_callback_ptr(i32);
pub struct alljoyn_keystore(i32);
pub struct alljoyn_keystorelistener(i32);
pub struct alljoyn_keystorelistener_acquireexclusivelock_ptr(i32);
pub struct alljoyn_keystorelistener_callbacks(i32);
pub struct alljoyn_keystorelistener_loadrequest_ptr(i32);
pub struct alljoyn_keystorelistener_releaseexclusivelock_ptr(i32);
pub struct alljoyn_keystorelistener_storerequest_ptr(i32);
pub struct alljoyn_keystorelistener_with_synchronization_callbacks(i32);
pub struct alljoyn_manifestarray(i32);
pub struct alljoyn_message(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_messagereceiver_methodhandler_ptr(i32);
pub struct alljoyn_messagereceiver_replyhandler_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_messagereceiver_signalhandler_ptr(i32);
pub struct alljoyn_messagetype(i32);
pub struct alljoyn_msgarg(i32);
pub struct alljoyn_observer(i32);
pub struct alljoyn_observer_object_discovered_ptr(i32);
pub struct alljoyn_observer_object_lost_ptr(i32);
pub struct alljoyn_observerlistener(i32);
pub struct alljoyn_observerlistener_callback(i32);
pub struct alljoyn_permissionconfigurationlistener(i32);
pub struct alljoyn_permissionconfigurationlistener_callbacks(i32);
pub struct alljoyn_permissionconfigurationlistener_endmanagement_ptr(i32);
pub struct alljoyn_permissionconfigurationlistener_factoryreset_ptr(i32);
pub struct alljoyn_permissionconfigurationlistener_policychanged_ptr(i32);
pub struct alljoyn_permissionconfigurationlistener_startmanagement_ptr(i32);
pub struct alljoyn_permissionconfigurator(i32);
pub struct alljoyn_pinglistener(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_pinglistener_callback(i32);
pub struct alljoyn_proxybusobject(i32);
pub struct alljoyn_proxybusobject_listener_getallpropertiescb_ptr(i32);
pub struct alljoyn_proxybusobject_listener_getpropertycb_ptr(i32);
pub struct alljoyn_proxybusobject_listener_introspectcb_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_proxybusobject_listener_propertieschanged_ptr(i32);
pub struct alljoyn_proxybusobject_listener_setpropertycb_ptr(i32);
pub struct alljoyn_proxybusobject_ref(i32);
pub struct alljoyn_securityapplicationproxy(i32);
pub struct alljoyn_sessionlistener(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_sessionlistener_callbacks(i32);
pub struct alljoyn_sessionlistener_sessionlost_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_sessionlistener_sessionmemberadded_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_sessionlistener_sessionmemberremoved_ptr(i32);
pub struct alljoyn_sessionlostreason(i32);
pub struct alljoyn_sessionopts(i32);
pub struct alljoyn_sessionportlistener(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_sessionportlistener_acceptsessionjoiner_ptr(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_sessionportlistener_callbacks(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct alljoyn_sessionportlistener_sessionjoined_ptr(i32);
pub struct alljoyn_typeid(i32);
