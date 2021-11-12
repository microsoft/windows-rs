#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct ATTENDEE_DISCONNECT_REASON(i32);
#[repr(C)]
pub struct CHANNEL_ACCESS_ENUM(i32);
#[repr(C)]
pub struct CHANNEL_FLAGS(i32);
#[repr(C)]
pub struct CHANNEL_PRIORITY(i32);
#[repr(C)]
pub struct CTRL_LEVEL(i32);
pub const DISPID_RDPAPI_EVENT_ON_BOUNDING_RECT_CHANGED: u32 = 340u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPFILTER_UPDATE: u32 = 322u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_CLOSE: u32 = 317u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_OPEN: u32 = 316u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_UPDATE: u32 = 318u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_CONNECTED: u32 = 301u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_DISCONNECTED: u32 = 302u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_UPDATE: u32 = 303u32;
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_REQUEST: u32 = 309u32;
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_RESPONSE: u32 = 338u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ERROR: u32 = 304u32;
pub const DISPID_RDPSRAPI_EVENT_ON_FOCUSRELEASED: u32 = 324u32;
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_PAUSED: u32 = 310u32;
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_RESUMED: u32 = 311u32;
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_DESKTOP_SETTINGS_CHANGED: u32 = 325u32;
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_RECT_CHANGED: u32 = 323u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_CLOSED: u32 = 634u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_DATARECEIVED: u32 = 633u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_SENDCOMPLETED: u32 = 632u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_AUTHENTICATED: u32 = 307u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTED: u32 = 305u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTFAILED: u32 = 308u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_DISCONNECTED: u32 = 306u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_DATARECEIVED: u32 = 314u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_JOIN: u32 = 312u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_LEAVE: u32 = 313u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_SENDCOMPLETED: u32 = 315u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_CLOSE: u32 = 320u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_OPEN: u32 = 319u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_UPDATE: u32 = 321u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_BUTTON_RECEIVED: u32 = 700u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_MOVE_RECEIVED: u32 = 701u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_WHEEL_RECEIVED: u32 = 702u32;
pub const DISPID_RDPSRAPI_METHOD_ADD_TOUCH_INPUT: u32 = 125u32;
pub const DISPID_RDPSRAPI_METHOD_BEGIN_TOUCH_FRAME: u32 = 124u32;
pub const DISPID_RDPSRAPI_METHOD_CLOSE: u32 = 101u32;
pub const DISPID_RDPSRAPI_METHOD_CONNECTTOCLIENT: u32 = 117u32;
pub const DISPID_RDPSRAPI_METHOD_CONNECTUSINGTRANSPORTSTREAM: u32 = 127u32;
pub const DISPID_RDPSRAPI_METHOD_CREATE_INVITATION: u32 = 107u32;
pub const DISPID_RDPSRAPI_METHOD_END_TOUCH_FRAME: u32 = 126u32;
pub const DISPID_RDPSRAPI_METHOD_GETFRAMEBUFFERBITS: u32 = 149u32;
pub const DISPID_RDPSRAPI_METHOD_GETSHAREDRECT: u32 = 103u32;
pub const DISPID_RDPSRAPI_METHOD_OPEN: u32 = 100u32;
pub const DISPID_RDPSRAPI_METHOD_PAUSE: u32 = 112u32;
pub const DISPID_RDPSRAPI_METHOD_REQUEST_COLOR_DEPTH_CHANGE: u32 = 115u32;
pub const DISPID_RDPSRAPI_METHOD_REQUEST_CONTROL: u32 = 108u32;
pub const DISPID_RDPSRAPI_METHOD_RESUME: u32 = 113u32;
pub const DISPID_RDPSRAPI_METHOD_SENDCONTROLLEVELCHANGERESPONSE: u32 = 148u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_KEYBOARD_EVENT: u32 = 122u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_BUTTON_EVENT: u32 = 119u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_MOVE_EVENT: u32 = 120u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_WHEEL_EVENT: u32 = 121u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_SYNC_EVENT: u32 = 123u32;
pub const DISPID_RDPSRAPI_METHOD_SETSHAREDRECT: u32 = 102u32;
pub const DISPID_RDPSRAPI_METHOD_SET_RENDERING_SURFACE: u32 = 118u32;
pub const DISPID_RDPSRAPI_METHOD_SHOW_WINDOW: u32 = 114u32;
pub const DISPID_RDPSRAPI_METHOD_STARTREVCONNECTLISTENER: u32 = 116u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMCLOSE: u32 = 426u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMOPEN: u32 = 425u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMREADDATA: u32 = 424u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMSENDDATA: u32 = 423u32;
pub const DISPID_RDPSRAPI_METHOD_STREAM_ALLOCBUFFER: u32 = 421u32;
pub const DISPID_RDPSRAPI_METHOD_STREAM_FREEBUFFER: u32 = 422u32;
pub const DISPID_RDPSRAPI_METHOD_TERMINATE_CONNECTION: u32 = 106u32;
pub const DISPID_RDPSRAPI_METHOD_VIEWERCONNECT: u32 = 104u32;
pub const DISPID_RDPSRAPI_METHOD_VIEWERDISCONNECT: u32 = 105u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_CREATE: u32 = 109u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SEND_DATA: u32 = 110u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SET_ACCESS: u32 = 111u32;
pub const DISPID_RDPSRAPI_PROP_APPFILTERENABLED: u32 = 219u32;
pub const DISPID_RDPSRAPI_PROP_APPFILTER_ENABLED: u32 = 218u32;
pub const DISPID_RDPSRAPI_PROP_APPFLAGS: u32 = 223u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION: u32 = 211u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION_FILTER: u32 = 215u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION_LIST: u32 = 217u32;
pub const DISPID_RDPSRAPI_PROP_APPNAME: u32 = 214u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEELIMIT: u32 = 235u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEES: u32 = 203u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEE_FLAGS: u32 = 230u32;
pub const DISPID_RDPSRAPI_PROP_CHANNELMANAGER: u32 = 206u32;
pub const DISPID_RDPSRAPI_PROP_CODE: u32 = 241u32;
pub const DISPID_RDPSRAPI_PROP_CONINFO: u32 = 231u32;
pub const DISPID_RDPSRAPI_PROP_CONNECTION_STRING: u32 = 232u32;
pub const DISPID_RDPSRAPI_PROP_COUNT: u32 = 244u32;
pub const DISPID_RDPSRAPI_PROP_CTRL_LEVEL: u32 = 242u32;
pub const DISPID_RDPSRAPI_PROP_DBG_CLX_CMDLINE: u32 = 222u32;
pub const DISPID_RDPSRAPI_PROP_DISCONNECTED_STRING: u32 = 237u32;
pub const DISPID_RDPSRAPI_PROP_DISPIDVALUE: u32 = 200u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER: u32 = 254u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_BPP: u32 = 253u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_HEIGHT: u32 = 251u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_WIDTH: u32 = 252u32;
pub const DISPID_RDPSRAPI_PROP_GROUP_NAME: u32 = 233u32;
pub const DISPID_RDPSRAPI_PROP_ID: u32 = 201u32;
pub const DISPID_RDPSRAPI_PROP_INVITATION: u32 = 205u32;
pub const DISPID_RDPSRAPI_PROP_INVITATIONITEM: u32 = 221u32;
pub const DISPID_RDPSRAPI_PROP_INVITATIONS: u32 = 204u32;
pub const DISPID_RDPSRAPI_PROP_LOCAL_IP: u32 = 227u32;
pub const DISPID_RDPSRAPI_PROP_LOCAL_PORT: u32 = 226u32;
pub const DISPID_RDPSRAPI_PROP_PASSWORD: u32 = 234u32;
pub const DISPID_RDPSRAPI_PROP_PEER_IP: u32 = 229u32;
pub const DISPID_RDPSRAPI_PROP_PEER_PORT: u32 = 228u32;
pub const DISPID_RDPSRAPI_PROP_PROTOCOL_TYPE: u32 = 225u32;
pub const DISPID_RDPSRAPI_PROP_REASON: u32 = 240u32;
pub const DISPID_RDPSRAPI_PROP_REMOTENAME: u32 = 243u32;
pub const DISPID_RDPSRAPI_PROP_REVOKED: u32 = 236u32;
pub const DISPID_RDPSRAPI_PROP_SESSION_COLORDEPTH: u32 = 239u32;
pub const DISPID_RDPSRAPI_PROP_SESSION_PROPERTIES: u32 = 202u32;
pub const DISPID_RDPSRAPI_PROP_SHARED: u32 = 220u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_CONTEXT: u32 = 560u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_FLAGS: u32 = 561u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADOFFSET: u32 = 559u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADSIZE: u32 = 558u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORAGE: u32 = 555u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORESIZE: u32 = 562u32;
pub const DISPID_RDPSRAPI_PROP_USESMARTSIZING: u32 = 238u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETFLAGS: u32 = 208u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETNAME: u32 = 207u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETPRIORITY: u32 = 209u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWID: u32 = 210u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWNAME: u32 = 213u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWSHARED: u32 = 212u32;
pub const DISPID_RDPSRAPI_PROP_WINDOW_LIST: u32 = 216u32;
pub const DISPID_RDPSRAPI_PROP_WNDFLAGS: u32 = 224u32;
#[repr(transparent)]
pub struct IRDPSRAPIApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIApplicationFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIApplicationList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIAttendee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIAttendeeDisconnectInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIAttendeeManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIAudioStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIClipboardUseEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIDebug(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIFrameBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIInvitation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIInvitationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIPerfCounterLogger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIPerfCounterLoggingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPISessionProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPISharingSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPISharingSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPITcpConnectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPITransportStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPITransportStreamBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPITransportStreamEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIViewer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIVirtualChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIVirtualChannelManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIWindowList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPViewerInputSink(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct RDPENCOMAPI_ATTENDEE_FLAGS(i32);
#[repr(C)]
pub struct RDPSRAPIApplication(i32);
#[repr(C)]
pub struct RDPSRAPIApplicationFilter(i32);
#[repr(C)]
pub struct RDPSRAPIApplicationList(i32);
#[repr(C)]
pub struct RDPSRAPIAttendee(i32);
#[repr(C)]
pub struct RDPSRAPIAttendeeDisconnectInfo(i32);
#[repr(C)]
pub struct RDPSRAPIAttendeeManager(i32);
#[repr(C)]
pub struct RDPSRAPIFrameBuffer(i32);
#[repr(C)]
pub struct RDPSRAPIInvitation(i32);
#[repr(C)]
pub struct RDPSRAPIInvitationManager(i32);
#[repr(C)]
pub struct RDPSRAPISessionProperties(i32);
#[repr(C)]
pub struct RDPSRAPITcpConnectionInfo(i32);
#[repr(C)]
pub struct RDPSRAPIWindow(i32);
#[repr(C)]
pub struct RDPSRAPIWindowList(i32);
#[repr(C)]
pub struct RDPSRAPI_APP_FLAGS(i32);
#[repr(C)]
pub struct RDPSRAPI_KBD_CODE_TYPE(i32);
#[repr(C)]
pub struct RDPSRAPI_KBD_SYNC_FLAG(i32);
#[repr(C)]
pub struct RDPSRAPI_MOUSE_BUTTON_TYPE(i32);
#[repr(C)]
pub struct RDPSRAPI_WND_FLAGS(i32);
#[repr(C)]
pub struct RDPSession(i32);
#[repr(C)]
pub struct RDPTransportStreamBuffer(i32);
#[repr(C)]
pub struct RDPTransportStreamEvents(i32);
#[repr(C)]
pub struct RDPViewer(i32);
#[repr(transparent)]
pub struct _IRDPSessionEvents(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(i32);
#[repr(C)]
pub struct __ReferenceRemainingTypes__(i32);
