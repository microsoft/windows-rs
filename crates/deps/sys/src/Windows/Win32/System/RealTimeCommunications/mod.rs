#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const FACILITY_PINT_STATUS_CODE: u32 = 240u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const FACILITY_RTC_INTERFACE: u32 = 238u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const FACILITY_SIP_STATUS_CODE: u32 = 239u32;
#[repr(transparent)]
pub struct INetworkTransportSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotificationTransportSync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCBuddy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCBuddy2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCBuddyEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCBuddyEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCBuddyGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCBuddyGroupEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCClient2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCClientEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCClientPortManagement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCClientPresence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCClientPresence2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCClientProvisioning(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCClientProvisioning2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCDispatchEventNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCEnumBuddies(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCEnumGroups(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCEnumParticipants(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCEnumPresenceDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCEnumProfiles(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCEnumUserSearchResults(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCEnumWatchers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCEventNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCInfoEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCIntensityEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCMediaEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCMediaRequestEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCMessagingEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCParticipant(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCParticipantStateChangeEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCPortManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCPresenceContact(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCPresenceDataEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCPresenceDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCPresencePropertyEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCPresenceStatusEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCProfile2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCProfileEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCProfileEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCReInviteEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCRegistrationStateChangeEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCRoamingEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSessionCallControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSessionDescriptionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSessionOperationCompleteEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSessionOperationCompleteEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSessionPortManagement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSessionReferStatusEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSessionReferredEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSessionStateChangeEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCSessionStateChangeEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCUserSearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCUserSearchQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCUserSearchResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCUserSearchResultsEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCWatcher2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCWatcherEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRTCWatcherEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransportSettingsInternal(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCAU_BASIC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCAU_DIGEST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCAU_KERBEROS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCAU_NTLM: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCAU_USE_LOGON_CRED: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCCS_FAIL_ON_REDIRECT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCCS_FORCE_PROFILE: u32 = 1u32;
pub struct RTCClient(i32);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_ALL: u32 = 33554431u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_BUDDY: u32 = 256u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_BUDDY2: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_CLIENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_GROUP: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_INFO: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_INTENSITY: u32 = 64u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_MEDIA: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_MEDIA_REQUEST: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_MESSAGING: u32 = 128u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_PARTICIPANT_STATE_CHANGE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_PRESENCE_DATA: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_PRESENCE_PROPERTY: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_PRESENCE_STATUS: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_PROFILE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_REGISTRATION_STATE_CHANGE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_REINVITE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_ROAMING: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_SESSION_OPERATION_COMPLETE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_SESSION_REFERRED: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_SESSION_REFER_STATUS: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_SESSION_STATE_CHANGE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_USERSEARCH: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_WATCHER: u32 = 512u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCEF_WATCHER2: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCIF_DISABLE_MEDIA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCIF_DISABLE_STRICT_DNS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCIF_DISABLE_UPNP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCIF_ENABLE_SERVER_CLASS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCMT_AUDIO_RECEIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCMT_AUDIO_SEND: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCMT_T120_SENDRECV: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCMT_VIDEO_RECEIVE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCMT_VIDEO_SEND: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRF_REGISTER_ALL: u32 = 15u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRF_REGISTER_INVITE_SESSIONS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRF_REGISTER_MESSAGE_SESSIONS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRF_REGISTER_NOTIFY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRF_REGISTER_PRESENCE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRMF_ALL_ROAMING: u32 = 15u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRMF_BUDDY_ROAMING: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRMF_PRESENCE_ROAMING: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRMF_PROFILE_ROAMING: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCRMF_WATCHER_ROAMING: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCSI_APPLICATION: u32 = 32u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCSI_IM: u32 = 8u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCSI_MULTIPARTY_IM: u32 = 16u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCSI_PC_TO_PC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCSI_PC_TO_PHONE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCSI_PHONE_TO_PHONE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCTR_TCP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCTR_TLS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTCTR_UDP: u32 = 1u32;
pub struct RTC_ACE_SCOPE(i32);
pub struct RTC_ANSWER_MODE(i32);
pub struct RTC_AUDIO_DEVICE(i32);
pub struct RTC_BUDDY_EVENT_TYPE(i32);
pub struct RTC_BUDDY_SUBSCRIPTION_TYPE(i32);
pub struct RTC_CLIENT_EVENT_TYPE(i32);
pub struct RTC_DTMF(i32);
pub struct RTC_EVENT(i32);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_ANOTHER_MEDIA_SESSION_ACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885961i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_BASIC_AUTH_SET_TLS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886017i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_CLIENT_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886042i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_CLIENT_ALREADY_SHUT_DOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886041i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_CLIENT_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886043i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_DESTINATION_ADDRESS_LOCAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886061i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_DESTINATION_ADDRESS_MULTICAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886059i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_DUPLICATE_BUDDY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886006i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_DUPLICATE_GROUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885998i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_DUPLICATE_REALM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886013i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_DUPLICATE_WATCHER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886005i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_ACL_LIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886000i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_ADDRESS_LOCAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886060i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_BUDDY_LIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886001i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_LISTEN_SOCKET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885957i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_OBJECT_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885983i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_PORTRANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885988i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_PREFERENCE_LIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885991i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_PROFILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886034i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_PROXY_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886058i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_REGISTRATION_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885971i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_SESSION_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886038i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_SESSION_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886039i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_INVALID_SIP_URL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886062i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_LISTENING_SOCKET_NOT_EXIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885958i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_LOCAL_PHONE_NEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886036i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MALFORMED_XML: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886004i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MAX_PENDING_OPERATIONS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885990i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MAX_REDIRECTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885960i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MEDIA_AEC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886044i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MEDIA_AUDIO_DEVICE_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886047i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MEDIA_CONTROLLER_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886049i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MEDIA_DISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885970i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MEDIA_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885969i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MEDIA_NEED_TERMINAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886048i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MEDIA_SESSION_IN_HOLD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885962i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MEDIA_SESSION_NOT_EXIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885963i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_MEDIA_VIDEO_DEVICE_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886046i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885950i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NOT_EXIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885992i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NOT_PRESENCE_PROFILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885974i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NO_BUDDY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885996i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NO_DEVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886035i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NO_GROUP: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885999i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NO_PROFILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886037i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NO_REALM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885994i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NO_TRANSPORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885993i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_NO_WATCHER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885995i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_OPERATION_WITH_TOO_MANY_PARTICIPANTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886018i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PINT_STATUS_REJECTED_ALL_BUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131755001i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PINT_STATUS_REJECTED_BADNUMBER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131754997i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PINT_STATUS_REJECTED_BUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131755003i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PINT_STATUS_REJECTED_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131754998i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PINT_STATUS_REJECTED_NO_ANSWER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131755002i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PINT_STATUS_REJECTED_PL_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131755000i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PINT_STATUS_REJECTED_SW_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131754999i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PLATFORM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885952i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_POLICY_NOT_ALLOW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886012i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PORT_MANAGER_ALREADY_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885956i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PORT_MAPPING_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886010i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PORT_MAPPING_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886011i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PRESENCE_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885982i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PRESENCE_NOT_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886040i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_INVALID_SERVER_AUTHMETHOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886024i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_INVALID_SERVER_PROTOCOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886025i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_INVALID_SERVER_ROLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886023i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_INVALID_SESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886021i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_INVALID_SESSION_PARTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886020i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_INVALID_SESSION_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886019i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_MULTIPLE_REGISTRARS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886022i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_NO_KEY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886032i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_NO_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886031i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_NO_PROVISION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886033i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_NO_SERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886028i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_NO_SERVER_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886027i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_NO_SERVER_PROTOCOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886026i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_NO_USER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886030i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_NO_USER_URI: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886029i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_PROFILE_SERVER_UNAUTHORIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886014i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_REDIRECT_PROCESSING_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885959i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_REFER_NOT_ACCEPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885968i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_REFER_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885967i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_REFER_NOT_EXIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885966i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_REGISTRATION_DEACTIVATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885949i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_REGISTRATION_REJECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885948i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_REGISTRATION_UNREGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885947i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_ROAMING_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885981i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_ROAMING_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886002i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_ROAMING_OPERATION_INTERRUPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886003i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SDP_CONNECTION_ADDR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886070i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SDP_FAILED_TO_BUILD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886067i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SDP_MULTICAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886071i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SDP_NOT_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886074i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SDP_NO_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886069i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SDP_PARSE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886073i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SDP_UPDATE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886072i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SECURITY_LEVEL_ALREADY_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885955i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SECURITY_LEVEL_NOT_COMPATIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886009i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SECURITY_LEVEL_NOT_DEFINED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886008i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SECURITY_LEVEL_NOT_SUPPORTED_BY_PARTICIPANT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886007i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_ADDITIONAL_PARTY_IN_TWO_PARTY_SESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885986i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_AUTH_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886063i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_AUTH_HEADER_SENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886065i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_AUTH_TIME_SKEW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885972i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_AUTH_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886064i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_CALL_CONNECTION_NOT_ESTABLISHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885987i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_CALL_DISCONNECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886055i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_CODECS_DO_NOT_MATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886080i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_DNS_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885978i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_HEADER_NOT_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886075i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_HIGH_SECURITY_SET_TLS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886016i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_HOLD_OPERATION_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885965i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_INVALID_CERTIFICATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885979i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_INVITEE_PARTY_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885973i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_INVITE_TRANSACTION_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886066i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_NEED_MORE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886056i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_NO_STREAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886077i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_OTHER_PARTY_JOIN_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885984i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_PARSE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886076i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_PARTY_ALREADY_IN_SESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885985i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_PEER_PARTICIPANT_IN_MULTIPARTY_SESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885951i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_REFER_OPERATION_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885953i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_REQUEST_DESTINATION_ADDR_NOT_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886054i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_SSL_NEGOTIATION_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886051i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_SSL_TUNNEL_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886052i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_STACK_SHUTDOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886050i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_STREAM_NOT_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886078i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_STREAM_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886079i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_TCP_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885977i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886068i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_TLS_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885975i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_TLS_INCOMPATIBLE_ENCRYPTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885980i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_TRANSPORT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886057i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_UDP_SIZE_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886053i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_SIP_UNHOLD_OPERATION_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885964i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_START_STREAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131886045i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_ADDRESS_INCOMPLETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820060i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_AMBIGUOUS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820059i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_BAD_EXTENSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820124i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_BAD_REQUEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820144i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_BUSY_HERE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820058i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_CONFLICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820135i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_FORBIDDEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820141i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_GONE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820134i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_LENGTH_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820133i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_LOOP_DETECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820062i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_METHOD_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820139i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_NOT_ACCEPTABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820138i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820140i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_PAYMENT_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820142i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_PROXY_AUTHENTICATION_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820137i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_REQUEST_ENTITY_TOO_LARGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820131i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_REQUEST_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820136i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_REQUEST_URI_TOO_LARGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820130i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_TEMPORARILY_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820064i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_TOO_MANY_HOPS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820061i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_TRANSACTION_DOES_NOT_EXIST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820063i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_UNAUTHORIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820143i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_CLIENT_UNSUPPORTED_MEDIA_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820129i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_GLOBAL_BUSY_EVERYWHERE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131819944i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_GLOBAL_DECLINE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131819941i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_GLOBAL_DOES_NOT_EXIST_ANYWHERE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131819940i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_GLOBAL_NOT_ACCEPTABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131819938i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_INFO_CALL_FORWARDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(15663285i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_INFO_QUEUED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(15663286i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_INFO_RINGING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(15663284i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_INFO_TRYING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(15663204i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_NOT_ACCEPTABLE_HERE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820056i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_REDIRECT_ALTERNATIVE_SERVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820164i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_REDIRECT_MOVED_PERMANENTLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820243i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_REDIRECT_MOVED_TEMPORARILY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820242i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_REDIRECT_MULTIPLE_CHOICES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820244i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_REDIRECT_SEE_OTHER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820241i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_REDIRECT_USE_PROXY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820239i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_REQUEST_TERMINATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820057i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_SERVER_BAD_GATEWAY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820042i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_SERVER_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820044i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_SERVER_NOT_IMPLEMENTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820043i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_SERVER_SERVER_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820040i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_SERVER_SERVICE_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820041i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_SERVER_VERSION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131820039i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_SESSION_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(15663287i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_STATUS_SUCCESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(15663304i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_TOO_MANY_GROUPS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885997i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_TOO_MANY_RETRIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885989i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_TOO_SMALL_EXPIRES_VALUE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885976i32 as _);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_E_UDP_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2131885954i32 as _);
pub struct RTC_GROUP_EVENT_TYPE(i32);
pub struct RTC_LISTEN_MODE(i32);
pub struct RTC_MEDIA_EVENT_REASON(i32);
pub struct RTC_MEDIA_EVENT_TYPE(i32);
pub struct RTC_MESSAGING_EVENT_TYPE(i32);
pub struct RTC_MESSAGING_USER_STATUS(i32);
pub struct RTC_OFFER_WATCHER_MODE(i32);
pub struct RTC_PARTICIPANT_STATE(i32);
pub struct RTC_PORT_TYPE(i32);
pub struct RTC_PRESENCE_PROPERTY(i32);
pub struct RTC_PRESENCE_STATUS(i32);
pub struct RTC_PRIVACY_MODE(i32);
pub struct RTC_PROFILE_EVENT_TYPE(i32);
pub struct RTC_PROVIDER_URI(i32);
pub struct RTC_REGISTRATION_STATE(i32);
pub struct RTC_REINVITE_STATE(i32);
pub struct RTC_RING_TYPE(i32);
pub struct RTC_ROAMING_EVENT_TYPE(i32);
pub struct RTC_SECURITY_LEVEL(i32);
pub struct RTC_SECURITY_TYPE(i32);
pub struct RTC_SESSION_REFER_STATUS(i32);
pub struct RTC_SESSION_STATE(i32);
pub struct RTC_SESSION_TYPE(i32);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const RTC_S_ROAMING_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(15597633i32 as _);
pub struct RTC_T120_APPLET(i32);
pub struct RTC_TERMINATE_REASON(i32);
pub struct RTC_USER_SEARCH_COLUMN(i32);
pub struct RTC_USER_SEARCH_PREFERENCE(i32);
pub struct RTC_VIDEO_DEVICE(i32);
pub struct RTC_WATCHER_EVENT_TYPE(i32);
pub struct RTC_WATCHER_MATCH_MODE(i32);
pub struct RTC_WATCHER_STATE(i32);
#[doc = "*Required features: `Win32_System_RealTimeCommunications`*"]
pub const STATUS_SEVERITY_RTC_ERROR: u32 = 2u32;
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct TRANSPORT_SETTING(i32);
