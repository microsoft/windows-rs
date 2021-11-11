#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynAcceptBusConnection();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynCloseBusHandle();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynConnectToBus();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn AllJoynCreateBus();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynEnumEvents();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynEventSelect();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynReceiveFromBus();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllJoynSendToBus();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QCC_StatusText();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_create_empty();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_create_full();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_createfrommsgarg();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_createfromxml();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getaboutdata();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getajsoftwareversion();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getannouncedaboutdata();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getappid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getappname();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getdateofmanufacture();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getdefaultlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getdescription();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getdeviceid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getdevicename();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getfield();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getfields();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getfieldsignature();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_gethardwareversion();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_getmanufacturer();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getmodelnumber();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getsoftwareversion();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getsupportedlanguages();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_getsupporturl();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_isfieldannounced();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_isfieldlocalized();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_isfieldrequired();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_isvalid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdata_setappid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setappid_fromstring();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setappname();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdateofmanufacture();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdefaultlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdescription();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdeviceid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setdevicename();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setfield();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_sethardwareversion();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setmanufacturer();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setmodelnumber();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setsoftwareversion();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setsupportedlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdata_setsupporturl();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutdatalistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutdatalistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_clear();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_getcontent();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_geturl();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_abouticon_setcontent();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticon_setcontent_frommsgarg();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_abouticon_seturl();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonobj_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonobj_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_abouticonproxy_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonproxy_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonproxy_geticon();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_abouticonproxy_getversion();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutlistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutlistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_announce();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_announce_using_datalistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobj_unannounce();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_clear();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_create_full();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_createfrommsgarg();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_getinterfacepaths();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_getinterfaces();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_getmsgarg();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutobjectdescription_getpaths();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_hasinterface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_hasinterfaceatpath();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutobjectdescription_haspath();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutproxy_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutproxy_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_aboutproxy_getaboutdata();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutproxy_getobjectdescription();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_aboutproxy_getversion();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_applicationstatelistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_applicationstatelistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_authlistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistener_requestcredentialsresponse();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistener_setsharedsecret();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistener_verifycredentialsresponse();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_authlistenerasync_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_authlistenerasync_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_adddestination();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_addpinggroup();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_autopinger_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_autopinger_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_autopinger_pause();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_removedestination();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_removepinggroup();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_autopinger_resume();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_autopinger_setpinginterval();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_addlogonentry();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_addmatch();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_advertisename();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_bindsessionport();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_canceladvertisename();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_cancelfindadvertisedname();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_cancelfindadvertisednamebytransport();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_cancelwhoimplements_interface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_cancelwhoimplements_interfaces();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_clearkeys();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_clearkeystore();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_connect();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_create_concurrency();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_createinterface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_createinterface_secure();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_createinterfacesfromxml();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_deletedefaultkeystore();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_deleteinterface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_disconnect();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_enableconcurrentcallbacks();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_enablepeersecurity();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_enablepeersecuritywithpermissionconfigurationlistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_findadvertisedname();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_findadvertisednamebytransport();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getalljoyndebugobj();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getalljoynproxyobj();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getconcurrency();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getconnectspec();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getdbusproxyobj();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getglobalguidstring();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getinterface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getinterfaces();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getkeyexpiration();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getpeerguid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_getpermissionconfigurator();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_gettimestamp();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_getuniquename();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_isconnected();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_ispeersecurityenabled();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_isstarted();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_isstopping();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_join();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_joinsession();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_joinsessionasync();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_leavesession();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_namehasowner();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_ping();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registeraboutlistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerapplicationstatelistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerbuslistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerbusobject();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerbusobject_secure();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_registerkeystorelistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_registersignalhandler();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_registersignalhandlerwithrule();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_releasename();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_reloadkeystore();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_removematch();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_removesessionmember();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_requestname();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_secureconnection();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_secureconnectionasync();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_setdaemondebug();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_setkeyexpiration();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_setlinktimeout();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_setlinktimeoutasync();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_setsessionlistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_start();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_stop();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unbindsessionport();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisteraboutlistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterallaboutlisteners();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterallhandlers();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterapplicationstatelistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterbuslistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_unregisterbusobject();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_unregistersignalhandler();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_unregistersignalhandlerwithrule();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busattachment_whoimplements_interface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busattachment_whoimplements_interfaces();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_buslistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_buslistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_addinterface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_addinterface_announced();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_addmethodhandler();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_addmethodhandlers();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_cancelsessionlessmessage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_cancelsessionlessmessage_serial();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_emitpropertieschanged();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_emitpropertychanged();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_getannouncedinterfacenames();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_getbusattachment();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_getname();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_getpath();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_issecure();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_methodreply_args();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_methodreply_err();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_methodreply_status();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_busobject_setannounceflag();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_busobject_signal();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_clear();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getcertchain();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_getexpiration();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getlogonentry();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getpassword();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getprivateKey();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_getusername();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_isset();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setcertchain();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_credentials_setexpiration();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setlogonentry();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setpassword();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setprivatekey();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_credentials_setusername();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_getbuildinfo();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_getnumericversion();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_getversion();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_init();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_activate();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addargannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addmember();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addmemberannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addmethod();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addproperty();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addpropertyannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_addsignal();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_eql();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getannotationatindex();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_getannotationscount();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getargdescriptionforlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getdescriptionforlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_getdescriptionlanguages();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getdescriptionlanguages2();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getdescriptiontranslationcallback();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmember();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmemberannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmemberargannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmemberdescriptionforlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmembers();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getmethod();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getname();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getproperties();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getproperty();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getpropertyannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getpropertydescriptionforlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_getsecuritypolicy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_getsignal();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_hasdescription();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_hasmember();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_hasproperties();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_hasproperty();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_introspect();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_interfacedescription_issecure();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_eql();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getannotationatindex();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getannotationscount();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getargannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getargannotationatindex();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_member_getargannotationscount();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_property_eql();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_property_getannotation();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_property_getannotationatindex();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_property_getannotationscount();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setargdescription();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setargdescriptionforlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setdescription();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setdescriptionforlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setdescriptionlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setdescriptiontranslationcallback();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setmemberdescription();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setmemberdescriptionforlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setpropertydescription();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_interfacedescription_setpropertydescriptionforlanguage();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_keystorelistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_keystorelistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_keystorelistener_getkeys();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_keystorelistener_putkeys();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_keystorelistener_with_synchronization_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_description();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_eql();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getarg();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getargs();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getauthmechanism();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getcallserial();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getcompressiontoken();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getdestination();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_geterrorname();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getflags();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getinterface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getmembername();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getobjectpath();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getreceiveendpointname();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getreplyserial();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getsender();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_getsessionid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_getsignature();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_gettimestamp();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_gettype();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isbroadcastsignal();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isencrypted();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isexpired();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isglobalbroadcast();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_issessionless();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_isunreliable();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_parseargs();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_message_setendianess();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_message_tostring();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_array_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_array_element();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_get();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_set();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_set_offset();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_signature();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_array_tostring();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_clear();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_clone();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_copy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_create_and_set();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_equal();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_get();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_array_element();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_get_array_elementsignature();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_array_numberofelements();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_bool();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_bool_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_double();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_double_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int16();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int16_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int32();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int32_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int64();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_int64_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_objectpath();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_signature();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_string();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint16();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint16_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint32();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint32_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint64();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint64_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint8();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_uint8_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_get_variant();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_get_variant_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_getdictelement();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_getkey();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_getmember();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_getnummembers();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_gettype();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_getvalue();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_hassignature();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set_and_stabilize();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_bool();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_bool_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_double();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_double_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int16();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int16_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int32();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int32_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int64();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_int64_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set_objectpath();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_objectpath_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set_signature();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_signature_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_set_string();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_string_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint16();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint16_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint32();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint32_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint64();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint64_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint8();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_set_uint8_array();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_setdictentry();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_setstruct();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_signature();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_msgarg_stabilize();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_msgarg_tostring();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_observer_get();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_getfirst();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_getnext();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_registerlistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_unregisteralllisteners();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observer_unregisterlistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observerlistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_observerlistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_passwordmanager_setcredentials();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurationlistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurationlistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_certificatechain_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_certificateid_cleanup();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_certificateidarray_cleanup();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_claim();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_endmanagement();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getapplicationstate();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getclaimcapabilities();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getclaimcapabilitiesadditionalinfo();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getdefaultclaimcapabilities();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getdefaultpolicy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getidentity();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getidentitycertificateid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getmanifests();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getmanifesttemplate();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getmembershipsummaries();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getpolicy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_getpublickey();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_installmanifests();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_installmembership();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_manifestarray_cleanup();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_manifesttemplate_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_policy_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_publickey_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_removemembership();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_reset();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_resetpolicy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_setapplicationstate();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_setclaimcapabilities();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_setclaimcapabilitiesadditionalinfo();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_setmanifesttemplatefromxml();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_startmanagement();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_updateidentity();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_permissionconfigurator_updatepolicy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_pinglistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_pinglistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_addchild();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_addinterface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_addinterface_by_name();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_copy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_create_secure();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_enablepropertycaching();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getallproperties();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getallpropertiesasync();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getchild();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_getchildren();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getinterface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_getinterfaces();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getpath();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getproperty();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getpropertyasync();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getservicename();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_getsessionid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_getuniquename();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_implementsinterface();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_introspectremoteobject();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_introspectremoteobjectasync();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_issecure();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_isvalid();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcall();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcall_member();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcall_member_noreply();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcall_noreply();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcallasync();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_methodcallasync_member();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_parsexml();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_ref_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_ref_decref();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_ref_get();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_ref_incref();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_registerpropertieschangedlistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_removechild();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_secureconnection();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_proxybusobject_secureconnectionasync();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_setproperty();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_setpropertyasync();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_proxybusobject_unregisterpropertieschangedlistener();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_routerinit();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_routerinitwithconfig();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_routershutdown();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_claim();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_computemanifestdigest();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_digest_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_eccpublickey_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_endmanagement();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getapplicationstate();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getclaimcapabilities();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getclaimcapabilitiesadditionalinfo();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getdefaultpolicy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_geteccpublickey();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getmanifesttemplate();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getpermissionmanagementsessionport();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_getpolicy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_installmembership();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_manifest_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_manifesttemplate_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_policy_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_reset();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_resetpolicy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_setmanifestsignature();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_signmanifest();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_startmanagement();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_updateidentity();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_securityapplicationproxy_updatepolicy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_sessionlistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionlistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_cmp();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_get_multipoint();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_get_proximity();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_get_traffic();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_get_transports();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_iscompatible();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_set_multipoint();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_set_proximity();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_set_traffic();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionopts_set_transports();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn alljoyn_sessionportlistener_create();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_sessionportlistener_destroy();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_shutdown();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_unity_deferred_callbacks_process();
    #[doc = "*Required features: `Win32_Devices_AllJoyn`*"]
    pub fn alljoyn_unity_set_deferred_callback_mainthread_only();
}
