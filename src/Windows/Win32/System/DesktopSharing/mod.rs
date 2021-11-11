#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ATTENDEE_DISCONNECT_REASON(pub i32);
pub const ATTENDEE_DISCONNECT_REASON_MIN: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
pub const ATTENDEE_DISCONNECT_REASON_APP: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
pub const ATTENDEE_DISCONNECT_REASON_ERR: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(1i32);
pub const ATTENDEE_DISCONNECT_REASON_CLI: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
pub const ATTENDEE_DISCONNECT_REASON_MAX: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
impl ::core::convert::From<i32> for ATTENDEE_DISCONNECT_REASON {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ATTENDEE_DISCONNECT_REASON {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHANNEL_ACCESS_ENUM(pub i32);
pub const CHANNEL_ACCESS_ENUM_NONE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(0i32);
pub const CHANNEL_ACCESS_ENUM_SENDRECEIVE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(1i32);
impl ::core::convert::From<i32> for CHANNEL_ACCESS_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CHANNEL_ACCESS_ENUM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHANNEL_FLAGS(pub i32);
pub const CHANNEL_FLAGS_LEGACY: CHANNEL_FLAGS = CHANNEL_FLAGS(1i32);
pub const CHANNEL_FLAGS_UNCOMPRESSED: CHANNEL_FLAGS = CHANNEL_FLAGS(2i32);
pub const CHANNEL_FLAGS_DYNAMIC: CHANNEL_FLAGS = CHANNEL_FLAGS(4i32);
impl ::core::convert::From<i32> for CHANNEL_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CHANNEL_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CHANNEL_PRIORITY(pub i32);
pub const CHANNEL_PRIORITY_LO: CHANNEL_PRIORITY = CHANNEL_PRIORITY(0i32);
pub const CHANNEL_PRIORITY_MED: CHANNEL_PRIORITY = CHANNEL_PRIORITY(1i32);
pub const CHANNEL_PRIORITY_HI: CHANNEL_PRIORITY = CHANNEL_PRIORITY(2i32);
impl ::core::convert::From<i32> for CHANNEL_PRIORITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CHANNEL_PRIORITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CTRL_LEVEL(pub i32);
pub const CTRL_LEVEL_MIN: CTRL_LEVEL = CTRL_LEVEL(0i32);
pub const CTRL_LEVEL_INVALID: CTRL_LEVEL = CTRL_LEVEL(0i32);
pub const CTRL_LEVEL_NONE: CTRL_LEVEL = CTRL_LEVEL(1i32);
pub const CTRL_LEVEL_VIEW: CTRL_LEVEL = CTRL_LEVEL(2i32);
pub const CTRL_LEVEL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(3i32);
pub const CTRL_LEVEL_REQCTRL_VIEW: CTRL_LEVEL = CTRL_LEVEL(4i32);
pub const CTRL_LEVEL_REQCTRL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(5i32);
pub const CTRL_LEVEL_MAX: CTRL_LEVEL = CTRL_LEVEL(5i32);
impl ::core::convert::From<i32> for CTRL_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CTRL_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPAPI_EVENT_ON_BOUNDING_RECT_CHANGED: u32 = 340u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPFILTER_UPDATE: u32 = 322u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_CLOSE: u32 = 317u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_OPEN: u32 = 316u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_UPDATE: u32 = 318u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_CONNECTED: u32 = 301u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_DISCONNECTED: u32 = 302u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_UPDATE: u32 = 303u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_REQUEST: u32 = 309u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_RESPONSE: u32 = 338u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ERROR: u32 = 304u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_FOCUSRELEASED: u32 = 324u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_PAUSED: u32 = 310u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_RESUMED: u32 = 311u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_DESKTOP_SETTINGS_CHANGED: u32 = 325u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_RECT_CHANGED: u32 = 323u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_CLOSED: u32 = 634u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_DATARECEIVED: u32 = 633u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_SENDCOMPLETED: u32 = 632u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_AUTHENTICATED: u32 = 307u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTED: u32 = 305u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTFAILED: u32 = 308u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_DISCONNECTED: u32 = 306u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_DATARECEIVED: u32 = 314u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_JOIN: u32 = 312u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_LEAVE: u32 = 313u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_SENDCOMPLETED: u32 = 315u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_CLOSE: u32 = 320u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_OPEN: u32 = 319u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_UPDATE: u32 = 321u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_BUTTON_RECEIVED: u32 = 700u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_MOVE_RECEIVED: u32 = 701u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_WHEEL_RECEIVED: u32 = 702u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_ADD_TOUCH_INPUT: u32 = 125u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_BEGIN_TOUCH_FRAME: u32 = 124u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_CLOSE: u32 = 101u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_CONNECTTOCLIENT: u32 = 117u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_CONNECTUSINGTRANSPORTSTREAM: u32 = 127u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_CREATE_INVITATION: u32 = 107u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_END_TOUCH_FRAME: u32 = 126u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_GETFRAMEBUFFERBITS: u32 = 149u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_GETSHAREDRECT: u32 = 103u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_OPEN: u32 = 100u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_PAUSE: u32 = 112u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_REQUEST_COLOR_DEPTH_CHANGE: u32 = 115u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_REQUEST_CONTROL: u32 = 108u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_RESUME: u32 = 113u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_SENDCONTROLLEVELCHANGERESPONSE: u32 = 148u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_KEYBOARD_EVENT: u32 = 122u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_BUTTON_EVENT: u32 = 119u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_MOVE_EVENT: u32 = 120u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_WHEEL_EVENT: u32 = 121u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_SYNC_EVENT: u32 = 123u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_SETSHAREDRECT: u32 = 102u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_SET_RENDERING_SURFACE: u32 = 118u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_SHOW_WINDOW: u32 = 114u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_STARTREVCONNECTLISTENER: u32 = 116u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMCLOSE: u32 = 426u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMOPEN: u32 = 425u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMREADDATA: u32 = 424u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMSENDDATA: u32 = 423u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAM_ALLOCBUFFER: u32 = 421u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAM_FREEBUFFER: u32 = 422u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_TERMINATE_CONNECTION: u32 = 106u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_VIEWERCONNECT: u32 = 104u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_VIEWERDISCONNECT: u32 = 105u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_CREATE: u32 = 109u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SEND_DATA: u32 = 110u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SET_ACCESS: u32 = 111u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_APPFILTERENABLED: u32 = 219u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_APPFILTER_ENABLED: u32 = 218u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_APPFLAGS: u32 = 223u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION: u32 = 211u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION_FILTER: u32 = 215u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION_LIST: u32 = 217u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_APPNAME: u32 = 214u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEELIMIT: u32 = 235u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEES: u32 = 203u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEE_FLAGS: u32 = 230u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_CHANNELMANAGER: u32 = 206u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_CODE: u32 = 241u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_CONINFO: u32 = 231u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_CONNECTION_STRING: u32 = 232u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_COUNT: u32 = 244u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_CTRL_LEVEL: u32 = 242u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_DBG_CLX_CMDLINE: u32 = 222u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_DISCONNECTED_STRING: u32 = 237u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_DISPIDVALUE: u32 = 200u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER: u32 = 254u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_BPP: u32 = 253u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_HEIGHT: u32 = 251u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_WIDTH: u32 = 252u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_GROUP_NAME: u32 = 233u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_ID: u32 = 201u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATION: u32 = 205u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATIONITEM: u32 = 221u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATIONS: u32 = 204u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_LOCAL_IP: u32 = 227u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_LOCAL_PORT: u32 = 226u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_PASSWORD: u32 = 234u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_PEER_IP: u32 = 229u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_PEER_PORT: u32 = 228u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_PROTOCOL_TYPE: u32 = 225u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_REASON: u32 = 240u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_REMOTENAME: u32 = 243u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_REVOKED: u32 = 236u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_SESSION_COLORDEPTH: u32 = 239u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_SESSION_PROPERTIES: u32 = 202u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_SHARED: u32 = 220u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_CONTEXT: u32 = 560u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_FLAGS: u32 = 561u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADOFFSET: u32 = 559u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADSIZE: u32 = 558u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORAGE: u32 = 555u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORESIZE: u32 = 562u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_USESMARTSIZING: u32 = 238u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETFLAGS: u32 = 208u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETNAME: u32 = 207u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETPRIORITY: u32 = 209u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWID: u32 = 210u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWNAME: u32 = 213u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWSHARED: u32 = 212u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOW_LIST: u32 = 216u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub const DISPID_RDPSRAPI_PROP_WNDFLAGS: u32 = 224u32;
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIApplication(pub ::windows::core::IUnknown);
impl IRDPSRAPIApplication {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Windows(&self) -> ::windows::core::Result<IRDPSRAPIWindowList> {
        let mut result__: <IRDPSRAPIWindowList as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIWindowList>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Shared(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetShared(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Flags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIApplication {
    type Vtable = IRDPSRAPIApplication_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41e7a09d_eb7a_436e_935d_780ca2628324);
}
impl ::core::convert::From<IRDPSRAPIApplication> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIApplication) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIApplication> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIApplication) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIApplication {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIApplication> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIApplication> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIApplication {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIApplication {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIApplication_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwindowlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newval: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIApplicationFilter(pub ::windows::core::IUnknown);
impl IRDPSRAPIApplicationFilter {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Applications(&self) -> ::windows::core::Result<IRDPSRAPIApplicationList> {
        let mut result__: <IRDPSRAPIApplicationList as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIApplicationList>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Windows(&self) -> ::windows::core::Result<IRDPSRAPIWindowList> {
        let mut result__: <IRDPSRAPIWindowList as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIWindowList>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetEnabled(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIApplicationFilter {
    type Vtable = IRDPSRAPIApplicationFilter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd20f10ca_6637_4f06_b1d5_277ea7e5160d);
}
impl ::core::convert::From<IRDPSRAPIApplicationFilter> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIApplicationFilter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIApplicationFilter> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIApplicationFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIApplicationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIApplicationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIApplicationFilter> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIApplicationFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIApplicationFilter> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIApplicationFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIApplicationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIApplicationFilter {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIApplicationFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, papplications: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwindows: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newval: i16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIApplicationList(pub ::windows::core::IUnknown);
impl IRDPSRAPIApplicationList {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Item(&self, item: i32) -> ::windows::core::Result<IRDPSRAPIApplication> {
        let mut result__: <IRDPSRAPIApplication as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(item), &mut result__).from_abi::<IRDPSRAPIApplication>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIApplicationList {
    type Vtable = IRDPSRAPIApplicationList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4b4aeb3_22dc_4837_b3b6_42ea2517849a);
}
impl ::core::convert::From<IRDPSRAPIApplicationList> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIApplicationList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIApplicationList> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIApplicationList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIApplicationList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIApplicationList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIApplicationList> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIApplicationList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIApplicationList> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIApplicationList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIApplicationList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIApplicationList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIApplicationList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: i32, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIAttendee(pub ::windows::core::IUnknown);
impl IRDPSRAPIAttendee {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn RemoteName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn ControlLevel(&self) -> ::windows::core::Result<CTRL_LEVEL> {
        let mut result__: <CTRL_LEVEL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<CTRL_LEVEL>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetControlLevel(&self, pnewval: CTRL_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pnewval)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Invitation(&self) -> ::windows::core::Result<IRDPSRAPIInvitation> {
        let mut result__: <IRDPSRAPIInvitation as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIInvitation>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn TerminateConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn ConnectivityInfo(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIAttendee {
    type Vtable = IRDPSRAPIAttendee_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec0671b3_1b78_4b80_a464_9132247543e3);
}
impl ::core::convert::From<IRDPSRAPIAttendee> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIAttendee) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIAttendee> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIAttendee) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIAttendee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIAttendee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIAttendee> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIAttendee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIAttendee> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIAttendee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIAttendee {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIAttendee {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAttendee_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut CTRL_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnewval: CTRL_LEVEL) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIAttendeeDisconnectInfo(pub ::windows::core::IUnknown);
impl IRDPSRAPIAttendeeDisconnectInfo {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Attendee(&self) -> ::windows::core::Result<IRDPSRAPIAttendee> {
        let mut result__: <IRDPSRAPIAttendee as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIAttendee>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Reason(&self) -> ::windows::core::Result<ATTENDEE_DISCONNECT_REASON> {
        let mut result__: <ATTENDEE_DISCONNECT_REASON as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ATTENDEE_DISCONNECT_REASON>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Code(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIAttendeeDisconnectInfo {
    type Vtable = IRDPSRAPIAttendeeDisconnectInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc187689f_447c_44a1_9c14_fffbb3b7ec17);
}
impl ::core::convert::From<IRDPSRAPIAttendeeDisconnectInfo> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIAttendeeDisconnectInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIAttendeeDisconnectInfo> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIAttendeeDisconnectInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIAttendeeDisconnectInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIAttendeeDisconnectInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIAttendeeDisconnectInfo> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIAttendeeDisconnectInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIAttendeeDisconnectInfo> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIAttendeeDisconnectInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIAttendeeDisconnectInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIAttendeeDisconnectInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAttendeeDisconnectInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preason: *mut ATTENDEE_DISCONNECT_REASON) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pval: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIAttendeeManager(pub ::windows::core::IUnknown);
impl IRDPSRAPIAttendeeManager {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Item(&self, id: i32) -> ::windows::core::Result<IRDPSRAPIAttendee> {
        let mut result__: <IRDPSRAPIAttendee as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), &mut result__).from_abi::<IRDPSRAPIAttendee>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIAttendeeManager {
    type Vtable = IRDPSRAPIAttendeeManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba3a37e8_33da_4749_8da0_07fa34da7944);
}
impl ::core::convert::From<IRDPSRAPIAttendeeManager> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIAttendeeManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIAttendeeManager> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIAttendeeManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIAttendeeManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIAttendeeManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIAttendeeManager> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIAttendeeManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIAttendeeManager> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIAttendeeManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIAttendeeManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIAttendeeManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAttendeeManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: i32, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIAudioStream(pub ::windows::core::IUnknown);
impl IRDPSRAPIAudioStream {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<i64> {
        let mut result__: <i64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn GetBuffer(&self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppbdata), ::core::mem::transmute(pcbdata), ::core::mem::transmute(ptimestamp)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn FreeBuffer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIAudioStream {
    type Vtable = IRDPSRAPIAudioStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3e30ef9_89c6_4541_ba3b_19336ac6d31c);
}
impl ::core::convert::From<IRDPSRAPIAudioStream> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIAudioStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIAudioStream> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIAudioStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIAudioStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIAudioStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAudioStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pnperiodinhundrednsintervals: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIClipboardUseEvents(pub ::windows::core::IUnknown);
impl IRDPSRAPIClipboardUseEvents {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_System_Com`*"]
    pub unsafe fn OnPasteFromClipboard<'a, Param1: ::windows::core::IntoParam<'a, super::Com::IDispatch>>(&self, clipboardformat: u32, pattendee: Param1) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(clipboardformat), pattendee.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIClipboardUseEvents {
    type Vtable = IRDPSRAPIClipboardUseEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd559f59a_7a27_4138_8763_247ce5f659a8);
}
impl ::core::convert::From<IRDPSRAPIClipboardUseEvents> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIClipboardUseEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIClipboardUseEvents> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIClipboardUseEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIClipboardUseEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIClipboardUseEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIClipboardUseEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clipboardformat: u32, pattendee: ::windows::core::RawPtr, pretval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIDebug(pub ::windows::core::IUnknown);
impl IRDPSRAPIDebug {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn SetCLXCmdLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, clxcmdline: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), clxcmdline.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn CLXCmdLine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIDebug {
    type Vtable = IRDPSRAPIDebug_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa1e42b5_496d_4ca4_a690_348dcb2ec4ad);
}
impl ::core::convert::From<IRDPSRAPIDebug> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIDebug) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIDebug> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIDebug) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIDebug_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clxcmdline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pclxcmdline: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIFrameBuffer(pub ::windows::core::IUnknown);
impl IRDPSRAPIFrameBuffer {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Height(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Bpp(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_System_Com`*"]
    pub unsafe fn GetFrameBufferBits(&self, x: i32, y: i32, width: i32, heigth: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(heigth), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIFrameBuffer {
    type Vtable = IRDPSRAPIFrameBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d67e7d2_b27b_448e_81b3_c6110ed8b4be);
}
impl ::core::convert::From<IRDPSRAPIFrameBuffer> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIFrameBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIFrameBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIFrameBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIFrameBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIFrameBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIFrameBuffer> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIFrameBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIFrameBuffer> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIFrameBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIFrameBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIFrameBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIFrameBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plwidth: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plheight: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plbpp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIInvitation(pub ::windows::core::IUnknown);
impl IRDPSRAPIInvitation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn ConnectionString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn GroupName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn Password(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn AttendeeLimit(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetAttendeeLimit(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Revoked(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetRevoked(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIInvitation {
    type Vtable = IRDPSRAPIInvitation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fac1d43_fc51_45bb_b1b4_2b53aa562fa3);
}
impl ::core::convert::From<IRDPSRAPIInvitation> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIInvitation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIInvitation> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIInvitation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIInvitation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIInvitation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIInvitation> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIInvitation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIInvitation> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIInvitation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIInvitation {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIInvitation {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIInvitation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newval: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newval: i16) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIInvitationManager(pub ::windows::core::IUnknown);
impl IRDPSRAPIInvitationManager {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, item: Param0) -> ::windows::core::Result<IRDPSRAPIInvitation> {
        let mut result__: <IRDPSRAPIInvitation as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), item.into_param().abi(), &mut result__).from_abi::<IRDPSRAPIInvitation>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn CreateInvitation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrauthstring: Param0, bstrgroupname: Param1, bstrpassword: Param2, attendeelimit: i32) -> ::windows::core::Result<IRDPSRAPIInvitation> {
        let mut result__: <IRDPSRAPIInvitation as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bstrauthstring.into_param().abi(), bstrgroupname.into_param().abi(), bstrpassword.into_param().abi(), ::core::mem::transmute(attendeelimit), &mut result__).from_abi::<IRDPSRAPIInvitation>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIInvitationManager {
    type Vtable = IRDPSRAPIInvitationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4722b049_92c3_4c2d_8a65_f7348f644dcf);
}
impl ::core::convert::From<IRDPSRAPIInvitationManager> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIInvitationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIInvitationManager> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIInvitationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIInvitationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIInvitationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIInvitationManager> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIInvitationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIInvitationManager> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIInvitationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIInvitationManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIInvitationManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIInvitationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrauthstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attendeelimit: i32, ppinvitation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIPerfCounterLogger(pub ::windows::core::IUnknown);
impl IRDPSRAPIPerfCounterLogger {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn LogValue(&self, lvalue: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(lvalue)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIPerfCounterLogger {
    type Vtable = IRDPSRAPIPerfCounterLogger_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x071c2533_0fa4_4e8f_ae83_9c10b4305ab5);
}
impl ::core::convert::From<IRDPSRAPIPerfCounterLogger> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIPerfCounterLogger) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIPerfCounterLogger> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIPerfCounterLogger) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIPerfCounterLogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIPerfCounterLogger {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIPerfCounterLogger_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lvalue: i64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIPerfCounterLoggingManager(pub ::windows::core::IUnknown);
impl IRDPSRAPIPerfCounterLoggingManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn CreateLogger<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcountername: Param0) -> ::windows::core::Result<IRDPSRAPIPerfCounterLogger> {
        let mut result__: <IRDPSRAPIPerfCounterLogger as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), bstrcountername.into_param().abi(), &mut result__).from_abi::<IRDPSRAPIPerfCounterLogger>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIPerfCounterLoggingManager {
    type Vtable = IRDPSRAPIPerfCounterLoggingManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a512c86_ac6e_4a8e_b1a4_fcef363f6e64);
}
impl ::core::convert::From<IRDPSRAPIPerfCounterLoggingManager> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIPerfCounterLoggingManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIPerfCounterLoggingManager> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIPerfCounterLoggingManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIPerfCounterLoggingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIPerfCounterLoggingManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIPerfCounterLoggingManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrcountername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplogger: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPISessionProperties(pub ::windows::core::IUnknown);
impl IRDPSRAPISessionProperties {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Property<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn SetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, propertyname: Param0, newval: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), propertyname.into_param().abi(), newval.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPISessionProperties {
    type Vtable = IRDPSRAPISessionProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x339b24f2_9bc0_4f16_9aac_f165433d13d4);
}
impl ::core::convert::From<IRDPSRAPISessionProperties> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPISessionProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPISessionProperties> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPISessionProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPISessionProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPISessionProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPISessionProperties> for super::Com::IDispatch {
    fn from(value: IRDPSRAPISessionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPISessionProperties> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPISessionProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPISessionProperties {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPISessionProperties {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPISessionProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPISharingSession(pub ::windows::core::IUnknown);
impl IRDPSRAPISharingSession {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetColorDepth(&self, colordepth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(colordepth)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn ColorDepth(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IRDPSRAPISessionProperties> {
        let mut result__: <IRDPSRAPISessionProperties as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPISessionProperties>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Attendees(&self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager> {
        let mut result__: <IRDPSRAPIAttendeeManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIAttendeeManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Invitations(&self) -> ::windows::core::Result<IRDPSRAPIInvitationManager> {
        let mut result__: <IRDPSRAPIInvitationManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIInvitationManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn ApplicationFilter(&self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter> {
        let mut result__: <IRDPSRAPIApplicationFilter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIApplicationFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn VirtualChannelManager(&self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager> {
        let mut result__: <IRDPSRAPIVirtualChannelManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIVirtualChannelManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn ConnectToClient<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnectionstring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstrconnectionstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetDesktopSharedRect(&self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn GetDesktopSharedRect(&self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPISharingSession {
    type Vtable = IRDPSRAPISharingSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeb20886_e470_4cf6_842b_2739c0ec5cfb);
}
impl ::core::convert::From<IRDPSRAPISharingSession> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPISharingSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPISharingSession> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPISharingSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPISharingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPISharingSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPISharingSession> for super::Com::IDispatch {
    fn from(value: IRDPSRAPISharingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPISharingSession> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPISharingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPISharingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPISharingSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPISharingSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, colordepth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolordepth: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPISharingSession2(pub ::windows::core::IUnknown);
impl IRDPSRAPISharingSession2 {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_System_Com`*"]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<super::Com::ITypeInfo> {
        let mut result__: <super::Com::ITypeInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(itinfo), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::Com::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(rgsznames), ::core::mem::transmute(cnames), ::core::mem::transmute(lcid), ::core::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Com::EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            ::core::mem::transmute(dispidmember),
            ::core::mem::transmute(riid),
            ::core::mem::transmute(lcid),
            ::core::mem::transmute(wflags),
            ::core::mem::transmute(pdispparams),
            ::core::mem::transmute(pvarresult),
            ::core::mem::transmute(pexcepinfo),
            ::core::mem::transmute(puargerr),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetColorDepth(&self, colordepth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(colordepth)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn ColorDepth(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IRDPSRAPISessionProperties> {
        let mut result__: <IRDPSRAPISessionProperties as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPISessionProperties>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Attendees(&self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager> {
        let mut result__: <IRDPSRAPIAttendeeManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIAttendeeManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Invitations(&self) -> ::windows::core::Result<IRDPSRAPIInvitationManager> {
        let mut result__: <IRDPSRAPIInvitationManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIInvitationManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn ApplicationFilter(&self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter> {
        let mut result__: <IRDPSRAPIApplicationFilter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIApplicationFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn VirtualChannelManager(&self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager> {
        let mut result__: <IRDPSRAPIVirtualChannelManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIVirtualChannelManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn ConnectToClient<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnectionstring: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bstrconnectionstring.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetDesktopSharedRect(&self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(left), ::core::mem::transmute(top), ::core::mem::transmute(right), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn GetDesktopSharedRect(&self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn ConnectUsingTransportStream<'a, Param0: ::windows::core::IntoParam<'a, IRDPSRAPITransportStream>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pstream: Param0, bstrgroup: Param1, bstrauthenticatedattendeename: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), pstream.into_param().abi(), bstrgroup.into_param().abi(), bstrauthenticatedattendeename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn FrameBuffer(&self) -> ::windows::core::Result<IRDPSRAPIFrameBuffer> {
        let mut result__: <IRDPSRAPIFrameBuffer as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIFrameBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SendControlLevelChangeResponse<'a, Param0: ::windows::core::IntoParam<'a, IRDPSRAPIAttendee>>(&self, pattendee: Param0, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), pattendee.into_param().abi(), ::core::mem::transmute(requestedlevel), ::core::mem::transmute(reasoncode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPISharingSession2 {
    type Vtable = IRDPSRAPISharingSession2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfee4ee57_e3e8_4205_8fb0_8fd1d0675c21);
}
impl ::core::convert::From<IRDPSRAPISharingSession2> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPISharingSession2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPISharingSession2> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPISharingSession2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPISharingSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPISharingSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IRDPSRAPISharingSession2> for IRDPSRAPISharingSession {
    fn from(value: IRDPSRAPISharingSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPSRAPISharingSession2> for IRDPSRAPISharingSession {
    fn from(value: &IRDPSRAPISharingSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRDPSRAPISharingSession> for IRDPSRAPISharingSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRDPSRAPISharingSession> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRDPSRAPISharingSession> for &IRDPSRAPISharingSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, IRDPSRAPISharingSession> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPISharingSession2> for super::Com::IDispatch {
    fn from(value: IRDPSRAPISharingSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPISharingSession2> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPISharingSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPISharingSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPISharingSession2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPISharingSession2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, colordepth: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcolordepth: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pstream: ::windows::core::RawPtr, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrauthenticatedattendeename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattendee: ::windows::core::RawPtr, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPITcpConnectionInfo(pub ::windows::core::IUnknown);
impl IRDPSRAPITcpConnectionInfo {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn LocalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn LocalIP(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn PeerPort(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn PeerIP(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPITcpConnectionInfo {
    type Vtable = IRDPSRAPITcpConnectionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf74049a4_3d06_4028_8193_0a8c29bc2452);
}
impl ::core::convert::From<IRDPSRAPITcpConnectionInfo> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPITcpConnectionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPITcpConnectionInfo> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPITcpConnectionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPITcpConnectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPITcpConnectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPITcpConnectionInfo> for super::Com::IDispatch {
    fn from(value: IRDPSRAPITcpConnectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPITcpConnectionInfo> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPITcpConnectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPITcpConnectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPITcpConnectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITcpConnectionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plprotocol: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plport: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbsrlocalip: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plport: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrip: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPITransportStream(pub ::windows::core::IUnknown);
impl IRDPSRAPITransportStream {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn AllocBuffer(&self, maxpayload: i32) -> ::windows::core::Result<IRDPSRAPITransportStreamBuffer> {
        let mut result__: <IRDPSRAPITransportStreamBuffer as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(maxpayload), &mut result__).from_abi::<IRDPSRAPITransportStreamBuffer>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn FreeBuffer<'a, Param0: ::windows::core::IntoParam<'a, IRDPSRAPITransportStreamBuffer>>(&self, pbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn WriteBuffer<'a, Param0: ::windows::core::IntoParam<'a, IRDPSRAPITransportStreamBuffer>>(&self, pbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn ReadBuffer<'a, Param0: ::windows::core::IntoParam<'a, IRDPSRAPITransportStreamBuffer>>(&self, pbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Open<'a, Param0: ::windows::core::IntoParam<'a, IRDPSRAPITransportStreamEvents>>(&self, pcallbacks: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pcallbacks.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPITransportStream {
    type Vtable = IRDPSRAPITransportStream_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36cfa065_43bb_4ef7_aed7_9b88a5053036);
}
impl ::core::convert::From<IRDPSRAPITransportStream> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPITransportStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPITransportStream> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPITransportStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPITransportStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPITransportStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITransportStream_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maxpayload: i32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcallbacks: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPITransportStreamBuffer(pub ::windows::core::IUnknown);
impl IRDPSRAPITransportStreamBuffer {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Storage(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut u8>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn StorageSize(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn PayloadSize(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetPayloadSize(&self, lval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(lval)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn PayloadOffset(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetPayloadOffset(&self, lretval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lretval)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetFlags(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(lflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Context(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, pcontext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPITransportStreamBuffer {
    type Vtable = IRDPSRAPITransportStreamBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c80290_5085_44b0_b460_f865c39cb4a9);
}
impl ::core::convert::From<IRDPSRAPITransportStreamBuffer> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPITransportStreamBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPITransportStreamBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPITransportStreamBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPITransportStreamBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPITransportStreamBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITransportStreamBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppbstorage: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plmaxstore: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plretval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lval: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plretval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lretval: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lflags: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcontext: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPITransportStreamEvents(pub ::windows::core::IUnknown);
impl IRDPSRAPITransportStreamEvents {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn OnWriteCompleted<'a, Param0: ::windows::core::IntoParam<'a, IRDPSRAPITransportStreamBuffer>>(&self, pbuffer: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pbuffer.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn OnReadCompleted<'a, Param0: ::windows::core::IntoParam<'a, IRDPSRAPITransportStreamBuffer>>(&self, pbuffer: Param0) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pbuffer.into_param().abi()))
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn OnStreamClosed(&self, hrreason: ::windows::core::HRESULT) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrreason)))
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPITransportStreamEvents {
    type Vtable = IRDPSRAPITransportStreamEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea81c254_f5af_4e40_982e_3e63bb595276);
}
impl ::core::convert::From<IRDPSRAPITransportStreamEvents> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPITransportStreamEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPITransportStreamEvents> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPITransportStreamEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPITransportStreamEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPITransportStreamEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITransportStreamEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuffer: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbuffer: ::windows::core::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hrreason: ::windows::core::HRESULT),
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIViewer(pub ::windows::core::IUnknown);
impl IRDPSRAPIViewer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnectionstring: Param0, bstrname: Param1, bstrpassword: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrconnectionstring.into_param().abi(), bstrname.into_param().abi(), bstrpassword.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Attendees(&self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager> {
        let mut result__: <IRDPSRAPIAttendeeManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIAttendeeManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Invitations(&self) -> ::windows::core::Result<IRDPSRAPIInvitationManager> {
        let mut result__: <IRDPSRAPIInvitationManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIInvitationManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn ApplicationFilter(&self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter> {
        let mut result__: <IRDPSRAPIApplicationFilter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIApplicationFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn VirtualChannelManager(&self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager> {
        let mut result__: <IRDPSRAPIVirtualChannelManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIVirtualChannelManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetSmartSizing(&self, vbsmartsizing: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(vbsmartsizing)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SmartSizing(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn RequestControl(&self, ctrllevel: CTRL_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(ctrllevel)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn SetDisconnectedText<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdisconnectedtext: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), bstrdisconnectedtext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn DisconnectedText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn RequestColorDepthChange(&self, bpp: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(bpp)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IRDPSRAPISessionProperties> {
        let mut result__: <IRDPSRAPISessionProperties as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPISessionProperties>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn StartReverseConnectListener<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrconnectionstring: Param0, bstrusername: Param1, bstrpassword: Param2) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), bstrconnectionstring.into_param().abi(), bstrusername.into_param().abi(), bstrpassword.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIViewer {
    type Vtable = IRDPSRAPIViewer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6bfcd38_8ce9_404d_8ae8_f31d00c65cb5);
}
impl ::core::convert::From<IRDPSRAPIViewer> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIViewer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIViewer> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIViewer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIViewer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIViewer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIViewer> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIViewer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIViewer> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIViewer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIViewer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIViewer {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIViewer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, vbsmartsizing: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvbsmartsizing: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ctrllevel: CTRL_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdisconnectedtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrdisconnectedtext: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bpp: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrreverseconnectstring: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIVirtualChannel(pub ::windows::core::IUnknown);
impl IRDPSRAPIVirtualChannel {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn SendData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrdata: Param0, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), bstrdata.into_param().abi(), ::core::mem::transmute(lattendeeid), ::core::mem::transmute(channelsendflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetAccess(&self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(lattendeeid), ::core::mem::transmute(accesstype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Priority(&self) -> ::windows::core::Result<CHANNEL_PRIORITY> {
        let mut result__: <CHANNEL_PRIORITY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<CHANNEL_PRIORITY>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIVirtualChannel {
    type Vtable = IRDPSRAPIVirtualChannel_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05e12f95_28b3_4c9a_8780_d0248574a1e0);
}
impl ::core::convert::From<IRDPSRAPIVirtualChannel> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIVirtualChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIVirtualChannel> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIVirtualChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIVirtualChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIVirtualChannel {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIVirtualChannel> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIVirtualChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIVirtualChannel> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIVirtualChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIVirtualChannel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIVirtualChannel {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIVirtualChannel_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plflags: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppriority: *mut CHANNEL_PRIORITY) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIVirtualChannelManager(pub ::windows::core::IUnknown);
impl IRDPSRAPIVirtualChannelManager {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::Com::VARIANT>>(&self, item: Param0) -> ::windows::core::Result<IRDPSRAPIVirtualChannel> {
        let mut result__: <IRDPSRAPIVirtualChannel as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), item.into_param().abi(), &mut result__).from_abi::<IRDPSRAPIVirtualChannel>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn CreateVirtualChannel<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrchannelname: Param0, priority: CHANNEL_PRIORITY, channelflags: u32) -> ::windows::core::Result<IRDPSRAPIVirtualChannel> {
        let mut result__: <IRDPSRAPIVirtualChannel as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), bstrchannelname.into_param().abi(), ::core::mem::transmute(priority), ::core::mem::transmute(channelflags), &mut result__).from_abi::<IRDPSRAPIVirtualChannel>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIVirtualChannelManager {
    type Vtable = IRDPSRAPIVirtualChannelManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d11c661_5d0d_4ee4_89df_2166ae1fdfed);
}
impl ::core::convert::From<IRDPSRAPIVirtualChannelManager> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIVirtualChannelManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIVirtualChannelManager> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIVirtualChannelManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIVirtualChannelManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIVirtualChannelManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIVirtualChannelManager> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIVirtualChannelManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIVirtualChannelManager> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIVirtualChannelManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIVirtualChannelManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIVirtualChannelManager {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIVirtualChannelManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bstrchannelname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIWindow(pub ::windows::core::IUnknown);
impl IRDPSRAPIWindow {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__: <i32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Application(&self) -> ::windows::core::Result<IRDPSRAPIApplication> {
        let mut result__: <IRDPSRAPIApplication as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRDPSRAPIApplication>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Shared(&self) -> ::windows::core::Result<i16> {
        let mut result__: <i16 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SetShared(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(newval)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_DesktopSharing`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Show(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Flags(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIWindow {
    type Vtable = IRDPSRAPIWindow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeafe0f9_c77b_4933_ba9f_a24cddcc27cf);
}
impl ::core::convert::From<IRDPSRAPIWindow> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIWindow) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIWindow> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIWindow> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIWindow> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIWindow {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIWindow {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIWindow_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, papplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, newval: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pretval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPSRAPIWindowList(pub ::windows::core::IUnknown);
impl IRDPSRAPIWindowList {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn Item(&self, item: i32) -> ::windows::core::Result<IRDPSRAPIWindow> {
        let mut result__: <IRDPSRAPIWindow as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(item), &mut result__).from_abi::<IRDPSRAPIWindow>(result__)
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIWindowList {
    type Vtable = IRDPSRAPIWindowList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a05ce44_715a_4116_a189_a118f30a07bd);
}
impl ::core::convert::From<IRDPSRAPIWindowList> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIWindowList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPSRAPIWindowList> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIWindowList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPSRAPIWindowList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPSRAPIWindowList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIWindowList> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIWindowList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIWindowList> for super::Com::IDispatch {
    fn from(value: &IRDPSRAPIWindowList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for IRDPSRAPIWindowList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &IRDPSRAPIWindowList {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIWindowList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, retval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, item: i32, pwindow: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRDPViewerInputSink(pub ::windows::core::IUnknown);
impl IRDPViewerInputSink {
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SendMouseButtonEvent(&self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(buttontype), ::core::mem::transmute(vbbuttondown), ::core::mem::transmute(xpos), ::core::mem::transmute(ypos)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SendMouseMoveEvent(&self, xpos: u32, ypos: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(xpos), ::core::mem::transmute(ypos)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SendMouseWheelEvent(&self, wheelrotation: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(wheelrotation)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SendKeyboardEvent(&self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(codetype), ::core::mem::transmute(keycode), ::core::mem::transmute(vbkeyup), ::core::mem::transmute(vbrepeat), ::core::mem::transmute(vbextended)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn SendSyncEvent(&self, syncflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(syncflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn BeginTouchFrame(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn AddTouchInput(&self, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(contactid), ::core::mem::transmute(event), ::core::mem::transmute(x), ::core::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_System_DesktopSharing`*"]
    pub unsafe fn EndTouchFrame(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IRDPViewerInputSink {
    type Vtable = IRDPViewerInputSink_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb590853_a6c5_4a7b_8dd4_76b69eea12d5);
}
impl ::core::convert::From<IRDPViewerInputSink> for ::windows::core::IUnknown {
    fn from(value: IRDPViewerInputSink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRDPViewerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IRDPViewerInputSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRDPViewerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRDPViewerInputSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPViewerInputSink_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, xpos: u32, ypos: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wheelrotation: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, syncflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RDPENCOMAPI_ATTENDEE_FLAGS(pub i32);
pub const ATTENDEE_FLAGS_LOCAL: RDPENCOMAPI_ATTENDEE_FLAGS = RDPENCOMAPI_ATTENDEE_FLAGS(1i32);
impl ::core::convert::From<i32> for RDPENCOMAPI_ATTENDEE_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RDPENCOMAPI_ATTENDEE_FLAGS {
    type Abi = Self;
}
pub const RDPSRAPIApplication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc116a484_4b25_4b9f_8a54_b934b06e57fa);
pub const RDPSRAPIApplicationFilter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe35ace89_c7e8_427e_a4f9_b9da072826bd);
pub const RDPSRAPIApplicationList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e31c815_7433_4876_97fb_ed59fe2baa22);
pub const RDPSRAPIAttendee: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74f93bb5_755f_488e_8a29_2390108aef55);
pub const RDPSRAPIAttendeeDisconnectInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb47d7250_5bdb_405d_b487_caad9c56f4f8);
pub const RDPSRAPIAttendeeManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7b13a01_f7d4_42a6_8595_12fc8c24e851);
pub const RDPSRAPIFrameBuffer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4f66bcc_538e_4101_951d_30847adb5101);
pub const RDPSRAPIInvitation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49174dc6_0731_4b5e_8ee1_83a63d3868fa);
pub const RDPSRAPIInvitationManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53d9c9db_75ab_4271_948a_4c4eb36a8f2b);
pub const RDPSRAPISessionProperties: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd7594ff_ea2a_4c06_8fdf_132de48b6510);
pub const RDPSRAPITcpConnectionInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe49db3f_ebb6_4278_8ce0_d5455833eaee);
pub const RDPSRAPIWindow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03cf46db_ce45_4d36_86ed_ed28b74398bf);
pub const RDPSRAPIWindowList: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c21e2b8_5dd4_42cc_81ba_1c099852e6fa);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RDPSRAPI_APP_FLAGS(pub i32);
pub const APP_FLAG_PRIVILEGED: RDPSRAPI_APP_FLAGS = RDPSRAPI_APP_FLAGS(1i32);
impl ::core::convert::From<i32> for RDPSRAPI_APP_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_APP_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RDPSRAPI_KBD_CODE_TYPE(pub i32);
pub const RDPSRAPI_KBD_CODE_SCANCODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(0i32);
pub const RDPSRAPI_KBD_CODE_UNICODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(1i32);
impl ::core::convert::From<i32> for RDPSRAPI_KBD_CODE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_KBD_CODE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RDPSRAPI_KBD_SYNC_FLAG(pub i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_SCROLL_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(1i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_NUM_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(2i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_CAPS_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(4i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_KANA_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(8i32);
impl ::core::convert::From<i32> for RDPSRAPI_KBD_SYNC_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_KBD_SYNC_FLAG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RDPSRAPI_MOUSE_BUTTON_TYPE(pub i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(0i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(1i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(2i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(3i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(4i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(5i32);
impl ::core::convert::From<i32> for RDPSRAPI_MOUSE_BUTTON_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_MOUSE_BUTTON_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RDPSRAPI_WND_FLAGS(pub i32);
pub const WND_FLAG_PRIVILEGED: RDPSRAPI_WND_FLAGS = RDPSRAPI_WND_FLAGS(1i32);
impl ::core::convert::From<i32> for RDPSRAPI_WND_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_WND_FLAGS {
    type Abi = Self;
}
pub const RDPSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b78f0e6_3e05_4a5b_b2e8_e743a8956b65);
pub const RDPTransportStreamBuffer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d4a1c69_f17f_4549_a699_761c6e6b5c0a);
pub const RDPTransportStreamEvents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31e3ab20_5350_483f_9dc6_6748665efdeb);
pub const RDPViewer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32be5ed2_5c86_480f_a914_0ff8885a1b3f);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct _IRDPSessionEvents(pub ::windows::core::IUnknown);
impl _IRDPSessionEvents {}
unsafe impl ::windows::core::Interface for _IRDPSessionEvents {
    type Vtable = _IRDPSessionEvents_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98a97042_6698_40e9_8efd_b3200990004b);
}
impl ::core::convert::From<_IRDPSessionEvents> for ::windows::core::IUnknown {
    fn from(value: _IRDPSessionEvents) -> Self {
        value.0
    }
}
impl ::core::convert::From<&_IRDPSessionEvents> for ::windows::core::IUnknown {
    fn from(value: &_IRDPSessionEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for _IRDPSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a _IRDPSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_IRDPSessionEvents> for super::Com::IDispatch {
    fn from(value: _IRDPSessionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_IRDPSessionEvents> for super::Com::IDispatch {
    fn from(value: &_IRDPSessionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for _IRDPSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::Com::IDispatch> for &_IRDPSessionEvents {
    fn into_param(self) -> ::windows::core::Param<'a, super::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IRDPSessionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
);
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(pub i32);
pub const CONST_MAX_CHANNEL_MESSAGE_SIZE: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(1024i32);
pub const CONST_MAX_CHANNEL_NAME_LEN: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(8i32);
pub const CONST_MAX_LEGACY_CHANNEL_MESSAGE_SIZE: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(409600i32);
pub const CONST_ATTENDEE_ID_EVERYONE: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(-1i32);
pub const CONST_ATTENDEE_ID_HOST: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(0i32);
pub const CONST_CONN_INTERVAL: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(50i32);
pub const CONST_ATTENDEE_ID_DEFAULT: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(-1i32);
impl ::core::convert::From<i32> for __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_DesktopSharing`*"]
pub struct __ReferenceRemainingTypes__ {
    pub __ctrlLevel__: CTRL_LEVEL,
    pub __attendeeDisconnectReason__: ATTENDEE_DISCONNECT_REASON,
    pub __channelPriority__: CHANNEL_PRIORITY,
    pub __channelFlags__: CHANNEL_FLAGS,
    pub __channelAccessEnum__: CHANNEL_ACCESS_ENUM,
    pub __rdpencomapiAttendeeFlags__: RDPENCOMAPI_ATTENDEE_FLAGS,
    pub __rdpsrapiWndFlags__: RDPSRAPI_WND_FLAGS,
    pub __rdpsrapiAppFlags__: RDPSRAPI_APP_FLAGS,
}
impl __ReferenceRemainingTypes__ {}
impl ::core::default::Default for __ReferenceRemainingTypes__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for __ReferenceRemainingTypes__ {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("__ReferenceRemainingTypes__")
            .field("__ctrlLevel__", &self.__ctrlLevel__)
            .field("__attendeeDisconnectReason__", &self.__attendeeDisconnectReason__)
            .field("__channelPriority__", &self.__channelPriority__)
            .field("__channelFlags__", &self.__channelFlags__)
            .field("__channelAccessEnum__", &self.__channelAccessEnum__)
            .field("__rdpencomapiAttendeeFlags__", &self.__rdpencomapiAttendeeFlags__)
            .field("__rdpsrapiWndFlags__", &self.__rdpsrapiWndFlags__)
            .field("__rdpsrapiAppFlags__", &self.__rdpsrapiAppFlags__)
            .finish()
    }
}
impl ::core::cmp::PartialEq for __ReferenceRemainingTypes__ {
    fn eq(&self, other: &Self) -> bool {
        self.__ctrlLevel__ == other.__ctrlLevel__ && self.__attendeeDisconnectReason__ == other.__attendeeDisconnectReason__ && self.__channelPriority__ == other.__channelPriority__ && self.__channelFlags__ == other.__channelFlags__ && self.__channelAccessEnum__ == other.__channelAccessEnum__ && self.__rdpencomapiAttendeeFlags__ == other.__rdpencomapiAttendeeFlags__ && self.__rdpsrapiWndFlags__ == other.__rdpsrapiWndFlags__ && self.__rdpsrapiAppFlags__ == other.__rdpsrapiAppFlags__
    }
}
impl ::core::cmp::Eq for __ReferenceRemainingTypes__ {}
unsafe impl ::windows::core::Abi for __ReferenceRemainingTypes__ {
    type Abi = Self;
}
