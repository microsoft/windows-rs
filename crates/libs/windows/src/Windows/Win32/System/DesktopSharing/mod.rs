#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ATTENDEE_DISCONNECT_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_MIN: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_APP: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_ERR: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_CLI: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_DISCONNECT_REASON_MAX: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
impl ::core::marker::Copy for ATTENDEE_DISCONNECT_REASON {}
impl ::core::clone::Clone for ATTENDEE_DISCONNECT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ATTENDEE_DISCONNECT_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ATTENDEE_DISCONNECT_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for ATTENDEE_DISCONNECT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTENDEE_DISCONNECT_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHANNEL_ACCESS_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_ACCESS_ENUM_NONE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(0i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_ACCESS_ENUM_SENDRECEIVE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(1i32);
impl ::core::marker::Copy for CHANNEL_ACCESS_ENUM {}
impl ::core::clone::Clone for CHANNEL_ACCESS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANNEL_ACCESS_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANNEL_ACCESS_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANNEL_ACCESS_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANNEL_ACCESS_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHANNEL_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_FLAGS_LEGACY: CHANNEL_FLAGS = CHANNEL_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_FLAGS_UNCOMPRESSED: CHANNEL_FLAGS = CHANNEL_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_FLAGS_DYNAMIC: CHANNEL_FLAGS = CHANNEL_FLAGS(4i32);
impl ::core::marker::Copy for CHANNEL_FLAGS {}
impl ::core::clone::Clone for CHANNEL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANNEL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANNEL_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANNEL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANNEL_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHANNEL_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_PRIORITY_LO: CHANNEL_PRIORITY = CHANNEL_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_PRIORITY_MED: CHANNEL_PRIORITY = CHANNEL_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CHANNEL_PRIORITY_HI: CHANNEL_PRIORITY = CHANNEL_PRIORITY(2i32);
impl ::core::marker::Copy for CHANNEL_PRIORITY {}
impl ::core::clone::Clone for CHANNEL_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHANNEL_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CHANNEL_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CHANNEL_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANNEL_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CTRL_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_MIN: CTRL_LEVEL = CTRL_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_INVALID: CTRL_LEVEL = CTRL_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_NONE: CTRL_LEVEL = CTRL_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_VIEW: CTRL_LEVEL = CTRL_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(3i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_REQCTRL_VIEW: CTRL_LEVEL = CTRL_LEVEL(4i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_REQCTRL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(5i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CTRL_LEVEL_MAX: CTRL_LEVEL = CTRL_LEVEL(5i32);
impl ::core::marker::Copy for CTRL_LEVEL {}
impl ::core::clone::Clone for CTRL_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CTRL_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CTRL_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for CTRL_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CTRL_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPAPI_EVENT_ON_BOUNDING_RECT_CHANGED: u32 = 340u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPFILTER_UPDATE: u32 = 322u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_CLOSE: u32 = 317u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_OPEN: u32 = 316u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_UPDATE: u32 = 318u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_CONNECTED: u32 = 301u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_DISCONNECTED: u32 = 302u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_UPDATE: u32 = 303u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_REQUEST: u32 = 309u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_RESPONSE: u32 = 338u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_ERROR: u32 = 304u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_FOCUSRELEASED: u32 = 324u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_PAUSED: u32 = 310u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_RESUMED: u32 = 311u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_DESKTOP_SETTINGS_CHANGED: u32 = 325u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_RECT_CHANGED: u32 = 323u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_CLOSED: u32 = 634u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_DATARECEIVED: u32 = 633u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_SENDCOMPLETED: u32 = 632u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_AUTHENTICATED: u32 = 307u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTED: u32 = 305u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTFAILED: u32 = 308u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_DISCONNECTED: u32 = 306u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_DATARECEIVED: u32 = 314u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_JOIN: u32 = 312u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_LEAVE: u32 = 313u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_SENDCOMPLETED: u32 = 315u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_CLOSE: u32 = 320u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_OPEN: u32 = 319u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_UPDATE: u32 = 321u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_BUTTON_RECEIVED: u32 = 700u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_MOVE_RECEIVED: u32 = 701u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_WHEEL_RECEIVED: u32 = 702u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_ADD_TOUCH_INPUT: u32 = 125u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_BEGIN_TOUCH_FRAME: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CLOSE: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CONNECTTOCLIENT: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CONNECTUSINGTRANSPORTSTREAM: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_CREATE_INVITATION: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_END_TOUCH_FRAME: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_GETFRAMEBUFFERBITS: u32 = 149u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_GETSHAREDRECT: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_OPEN: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_PAUSE: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_REQUEST_COLOR_DEPTH_CHANGE: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_REQUEST_CONTROL: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_RESUME: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SENDCONTROLLEVELCHANGERESPONSE: u32 = 148u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_KEYBOARD_EVENT: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_BUTTON_EVENT: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_MOVE_EVENT: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_WHEEL_EVENT: u32 = 121u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SEND_SYNC_EVENT: u32 = 123u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SETSHAREDRECT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SET_RENDERING_SURFACE: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_SHOW_WINDOW: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STARTREVCONNECTLISTENER: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMCLOSE: u32 = 426u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMOPEN: u32 = 425u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMREADDATA: u32 = 424u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAMSENDDATA: u32 = 423u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAM_ALLOCBUFFER: u32 = 421u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_STREAM_FREEBUFFER: u32 = 422u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_TERMINATE_CONNECTION: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIEWERCONNECT: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIEWERDISCONNECT: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_CREATE: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SEND_DATA: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SET_ACCESS: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPFILTERENABLED: u32 = 219u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPFILTER_ENABLED: u32 = 218u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPFLAGS: u32 = 223u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION: u32 = 211u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION_FILTER: u32 = 215u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPLICATION_LIST: u32 = 217u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_APPNAME: u32 = 214u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEELIMIT: u32 = 235u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEES: u32 = 203u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ATTENDEE_FLAGS: u32 = 230u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CHANNELMANAGER: u32 = 206u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CODE: u32 = 241u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CONINFO: u32 = 231u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CONNECTION_STRING: u32 = 232u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_COUNT: u32 = 244u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_CTRL_LEVEL: u32 = 242u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_DBG_CLX_CMDLINE: u32 = 222u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_DISCONNECTED_STRING: u32 = 237u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_DISPIDVALUE: u32 = 200u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER: u32 = 254u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_BPP: u32 = 253u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_HEIGHT: u32 = 251u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_WIDTH: u32 = 252u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_GROUP_NAME: u32 = 233u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_ID: u32 = 201u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATION: u32 = 205u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATIONITEM: u32 = 221u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_INVITATIONS: u32 = 204u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_LOCAL_IP: u32 = 227u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_LOCAL_PORT: u32 = 226u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PASSWORD: u32 = 234u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PEER_IP: u32 = 229u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PEER_PORT: u32 = 228u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_PROTOCOL_TYPE: u32 = 225u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_REASON: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_REMOTENAME: u32 = 243u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_REVOKED: u32 = 236u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_SESSION_COLORDEPTH: u32 = 239u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_SESSION_PROPERTIES: u32 = 202u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_SHARED: u32 = 220u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_CONTEXT: u32 = 560u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_FLAGS: u32 = 561u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADOFFSET: u32 = 559u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADSIZE: u32 = 558u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORAGE: u32 = 555u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORESIZE: u32 = 562u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_USESMARTSIZING: u32 = 238u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETFLAGS: u32 = 208u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETNAME: u32 = 207u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETPRIORITY: u32 = 209u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWID: u32 = 210u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWNAME: u32 = 213u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOWSHARED: u32 = 212u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WINDOW_LIST: u32 = 216u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const DISPID_RDPSRAPI_PROP_WNDFLAGS: u32 = 224u32;
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIApplication(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplication {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Windows(&self) -> ::windows::core::Result<IRDPSRAPIWindowList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Windows)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIWindowList>(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Shared(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Shared)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShared(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShared)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Flags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Flags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIApplication> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIApplication> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIApplication> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIApplication) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIApplication> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIApplication) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIApplication> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIApplication) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIApplication {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIApplication").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIApplication {
    type Vtable = IRDPSRAPIApplication_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41e7a09d_eb7a_436e_935d_780ca2628324);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIApplication_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Windows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwindowlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Windows: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub Shared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT,
    pub SetShared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIApplicationFilter(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationFilter {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Applications(&self) -> ::windows::core::Result<IRDPSRAPIApplicationList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Applications)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIApplicationList>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Windows(&self) -> ::windows::core::Result<IRDPSRAPIWindowList> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Windows)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIWindowList>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::windows::core::Interface::as_raw(self), newval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIApplicationFilter> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIApplicationFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIApplicationFilter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIApplicationFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIApplicationFilter> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIApplicationFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIApplicationFilter> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIApplicationFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIApplicationFilter> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIApplicationFilter) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIApplicationFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIApplicationFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIApplicationFilter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIApplicationFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIApplicationFilter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIApplicationFilter {
    type Vtable = IRDPSRAPIApplicationFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd20f10ca_6637_4f06_b1d5_277ea7e5160d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIApplicationFilter_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Applications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papplications: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Applications: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Windows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwindows: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Windows: usize,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIApplicationList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIApplicationList {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, item: i32) -> ::windows::core::Result<IRDPSRAPIApplication> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), item, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIApplication>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIApplicationList> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIApplicationList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIApplicationList> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIApplicationList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIApplicationList> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIApplicationList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIApplicationList> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIApplicationList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIApplicationList> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIApplicationList) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIApplicationList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIApplicationList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIApplicationList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIApplicationList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIApplicationList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIApplicationList {
    type Vtable = IRDPSRAPIApplicationList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4b4aeb3_22dc_4837_b3b6_42ea2517849a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIApplicationList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: i32, papplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIAttendee(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendee {
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoteName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RemoteName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ControlLevel(&self) -> ::windows::core::Result<CTRL_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ControlLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CTRL_LEVEL>(result__)
    }
    pub unsafe fn SetControlLevel(&self, pnewval: CTRL_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetControlLevel)(::windows::core::Interface::as_raw(self), pnewval).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invitation(&self) -> ::windows::core::Result<IRDPSRAPIInvitation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Invitation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIInvitation>(result__)
    }
    pub unsafe fn TerminateConnection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TerminateConnection)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Flags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn ConnectivityInfo(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConnectivityInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIAttendee> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIAttendee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIAttendee> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIAttendee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIAttendee> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIAttendee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIAttendee> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIAttendee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIAttendee> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIAttendee) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIAttendee {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIAttendee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIAttendee {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIAttendee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIAttendee").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIAttendee {
    type Vtable = IRDPSRAPIAttendee_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec0671b3_1b78_4b80_a464_9132247543e3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAttendee_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoteName: usize,
    pub ControlLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut CTRL_LEVEL) -> ::windows::core::HRESULT,
    pub SetControlLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnewval: CTRL_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Invitation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invitation: usize,
    pub TerminateConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT,
    pub ConnectivityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIAttendeeDisconnectInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeDisconnectInfo {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Attendee(&self) -> ::windows::core::Result<IRDPSRAPIAttendee> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Attendee)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIAttendee>(result__)
    }
    pub unsafe fn Reason(&self) -> ::windows::core::Result<ATTENDEE_DISCONNECT_REASON> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Reason)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ATTENDEE_DISCONNECT_REASON>(result__)
    }
    pub unsafe fn Code(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Code)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIAttendeeDisconnectInfo> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIAttendeeDisconnectInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIAttendeeDisconnectInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIAttendeeDisconnectInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIAttendeeDisconnectInfo> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIAttendeeDisconnectInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIAttendeeDisconnectInfo> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIAttendeeDisconnectInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIAttendeeDisconnectInfo> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIAttendeeDisconnectInfo) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIAttendeeDisconnectInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIAttendeeDisconnectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIAttendeeDisconnectInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIAttendeeDisconnectInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIAttendeeDisconnectInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIAttendeeDisconnectInfo {
    type Vtable = IRDPSRAPIAttendeeDisconnectInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc187689f_447c_44a1_9c14_fffbb3b7ec17);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAttendeeDisconnectInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Attendee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attendee: usize,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preason: *mut ATTENDEE_DISCONNECT_REASON) -> ::windows::core::HRESULT,
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIAttendeeManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIAttendeeManager {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, id: i32) -> ::windows::core::Result<IRDPSRAPIAttendee> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), id, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIAttendee>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIAttendeeManager> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIAttendeeManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIAttendeeManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIAttendeeManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIAttendeeManager> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIAttendeeManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIAttendeeManager> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIAttendeeManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIAttendeeManager> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIAttendeeManager) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIAttendeeManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIAttendeeManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIAttendeeManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIAttendeeManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIAttendeeManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIAttendeeManager {
    type Vtable = IRDPSRAPIAttendeeManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba3a37e8_33da_4749_8da0_07fa34da7944);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAttendeeManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
pub struct IRDPSRAPIAudioStream(::windows::core::IUnknown);
impl IRDPSRAPIAudioStream {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Start)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Stop)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetBuffer(&self, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbdata), ::core::mem::transmute(pcbdata), ::core::mem::transmute(ptimestamp)).ok()
    }
    pub unsafe fn FreeBuffer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreeBuffer)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IRDPSRAPIAudioStream> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIAudioStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRDPSRAPIAudioStream> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIAudioStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPSRAPIAudioStream> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIAudioStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRDPSRAPIAudioStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRDPSRAPIAudioStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRDPSRAPIAudioStream {}
impl ::core::fmt::Debug for IRDPSRAPIAudioStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIAudioStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIAudioStream {
    type Vtable = IRDPSRAPIAudioStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3e30ef9_89c6_4541_ba3b_19336ac6d31c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIAudioStream_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnperiodinhundrednsintervals: *mut i64) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32, ptimestamp: *mut u64) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
pub struct IRDPSRAPIClipboardUseEvents(::windows::core::IUnknown);
impl IRDPSRAPIClipboardUseEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnPasteFromClipboard<'a, P0>(&self, clipboardformat: u32, pattendee: P0) -> ::windows::core::Result<i16>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).OnPasteFromClipboard)(::windows::core::Interface::as_raw(self), clipboardformat, pattendee.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
}
impl ::core::convert::From<IRDPSRAPIClipboardUseEvents> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIClipboardUseEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRDPSRAPIClipboardUseEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIClipboardUseEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPSRAPIClipboardUseEvents> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIClipboardUseEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRDPSRAPIClipboardUseEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRDPSRAPIClipboardUseEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRDPSRAPIClipboardUseEvents {}
impl ::core::fmt::Debug for IRDPSRAPIClipboardUseEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIClipboardUseEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIClipboardUseEvents {
    type Vtable = IRDPSRAPIClipboardUseEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd559f59a_7a27_4138_8763_247ce5f659a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIClipboardUseEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnPasteFromClipboard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipboardformat: u32, pattendee: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnPasteFromClipboard: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
pub struct IRDPSRAPIDebug(::windows::core::IUnknown);
impl IRDPSRAPIDebug {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCLXCmdLine<'a, P0>(&self, clxcmdline: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetCLXCmdLine)(::windows::core::Interface::as_raw(self), clxcmdline.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CLXCmdLine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CLXCmdLine)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IRDPSRAPIDebug> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRDPSRAPIDebug> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPSRAPIDebug> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRDPSRAPIDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRDPSRAPIDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRDPSRAPIDebug {}
impl ::core::fmt::Debug for IRDPSRAPIDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIDebug {
    type Vtable = IRDPSRAPIDebug_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa1e42b5_496d_4ca4_a690_348dcb2ec4ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIDebug_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCLXCmdLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clxcmdline: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCLXCmdLine: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CLXCmdLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclxcmdline: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CLXCmdLine: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIFrameBuffer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIFrameBuffer {
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Width)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Height(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Height)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Bpp(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Bpp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFrameBufferBits(&self, x: i32, y: i32, width: i32, heigth: i32) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFrameBufferBits)(::windows::core::Interface::as_raw(self), x, y, width, heigth, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIFrameBuffer> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIFrameBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIFrameBuffer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIFrameBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIFrameBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIFrameBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIFrameBuffer> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIFrameBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIFrameBuffer> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIFrameBuffer) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIFrameBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIFrameBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIFrameBuffer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIFrameBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIFrameBuffer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIFrameBuffer {
    type Vtable = IRDPSRAPIFrameBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d67e7d2_b27b_448e_81b3_c6110ed8b4be);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIFrameBuffer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plwidth: *mut i32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plheight: *mut i32) -> ::windows::core::HRESULT,
    pub Bpp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plbpp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFrameBufferBits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: i32, y: i32, width: i32, heigth: i32, ppbits: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFrameBufferBits: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIInvitation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitation {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectionString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConnectionString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GroupName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GroupName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Password(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Password)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn AttendeeLimit(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AttendeeLimit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAttendeeLimit(&self, newval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttendeeLimit)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    pub unsafe fn Revoked(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Revoked)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetRevoked(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRevoked)(::windows::core::Interface::as_raw(self), newval).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIInvitation> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIInvitation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIInvitation> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIInvitation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIInvitation> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIInvitation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIInvitation> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIInvitation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIInvitation> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIInvitation) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIInvitation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIInvitation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIInvitation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIInvitation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIInvitation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIInvitation {
    type Vtable = IRDPSRAPIInvitation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fac1d43_fc51_45bb_b1b4_2b53aa562fa3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIInvitation_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectionString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GroupName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GroupName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Password: usize,
    pub AttendeeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    pub SetAttendeeLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT,
    pub Revoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT,
    pub SetRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIInvitationManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIInvitationManager {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, P0>(&self, item: P0) -> ::windows::core::Result<IRDPSRAPIInvitation>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), item.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIInvitation>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateInvitation<'a, P0, P1, P2>(&self, bstrauthstring: P0, bstrgroupname: P1, bstrpassword: P2, attendeelimit: i32) -> ::windows::core::Result<IRDPSRAPIInvitation>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateInvitation)(::windows::core::Interface::as_raw(self), bstrauthstring.into().abi(), bstrgroupname.into().abi(), bstrpassword.into().abi(), attendeelimit, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIInvitation>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIInvitationManager> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIInvitationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIInvitationManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIInvitationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIInvitationManager> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIInvitationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIInvitationManager> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIInvitationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIInvitationManager> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIInvitationManager) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIInvitationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIInvitationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIInvitationManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIInvitationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIInvitationManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIInvitationManager {
    type Vtable = IRDPSRAPIInvitationManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4722b049_92c3_4c2d_8a65_f7348f644dcf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIInvitationManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppinvitation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateInvitation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrauthstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, attendeelimit: i32, ppinvitation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateInvitation: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
pub struct IRDPSRAPIPerfCounterLogger(::windows::core::IUnknown);
impl IRDPSRAPIPerfCounterLogger {
    pub unsafe fn LogValue(&self, lvalue: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LogValue)(::windows::core::Interface::as_raw(self), lvalue).ok()
    }
}
impl ::core::convert::From<IRDPSRAPIPerfCounterLogger> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIPerfCounterLogger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRDPSRAPIPerfCounterLogger> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIPerfCounterLogger) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPSRAPIPerfCounterLogger> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIPerfCounterLogger) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRDPSRAPIPerfCounterLogger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRDPSRAPIPerfCounterLogger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRDPSRAPIPerfCounterLogger {}
impl ::core::fmt::Debug for IRDPSRAPIPerfCounterLogger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIPerfCounterLogger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIPerfCounterLogger {
    type Vtable = IRDPSRAPIPerfCounterLogger_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x071c2533_0fa4_4e8f_ae83_9c10b4305ab5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIPerfCounterLogger_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub LogValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lvalue: i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
pub struct IRDPSRAPIPerfCounterLoggingManager(::windows::core::IUnknown);
impl IRDPSRAPIPerfCounterLoggingManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLogger<'a, P0>(&self, bstrcountername: P0) -> ::windows::core::Result<IRDPSRAPIPerfCounterLogger>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateLogger)(::windows::core::Interface::as_raw(self), bstrcountername.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIPerfCounterLogger>(result__)
    }
}
impl ::core::convert::From<IRDPSRAPIPerfCounterLoggingManager> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIPerfCounterLoggingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRDPSRAPIPerfCounterLoggingManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIPerfCounterLoggingManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPSRAPIPerfCounterLoggingManager> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIPerfCounterLoggingManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRDPSRAPIPerfCounterLoggingManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRDPSRAPIPerfCounterLoggingManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRDPSRAPIPerfCounterLoggingManager {}
impl ::core::fmt::Debug for IRDPSRAPIPerfCounterLoggingManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIPerfCounterLoggingManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPIPerfCounterLoggingManager {
    type Vtable = IRDPSRAPIPerfCounterLoggingManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a512c86_ac6e_4a8e_b1a4_fcef363f6e64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIPerfCounterLoggingManager_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateLogger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcountername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pplogger: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateLogger: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPISessionProperties(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISessionProperties {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Property<'a, P0>(&self, propertyname: P0) -> ::windows::core::Result<super::Com::VARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Property)(::windows::core::Interface::as_raw(self), propertyname.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn put_Property<'a, P0, P1>(&self, propertyname: P0, newval: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).put_Property)(::windows::core::Interface::as_raw(self), propertyname.into().abi(), newval.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPISessionProperties> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPISessionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPISessionProperties> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPISessionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPISessionProperties> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPISessionProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPISessionProperties> for super::Com::IDispatch {
    fn from(value: IRDPSRAPISessionProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPISessionProperties> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPISessionProperties) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPISessionProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPISessionProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPISessionProperties {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPISessionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPISessionProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPISessionProperties {
    type Vtable = IRDPSRAPISessionProperties_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x339b24f2_9bc0_4f16_9aac_f165433d13d4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPISessionProperties_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Property: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub put_Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newval: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    put_Property: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPISharingSession(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSession {
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetColorDepth(&self, colordepth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorDepth)(::windows::core::Interface::as_raw(self), colordepth).ok()
    }
    pub unsafe fn ColorDepth(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ColorDepth)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IRDPSRAPISessionProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPISessionProperties>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Attendees(&self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Attendees)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIAttendeeManager>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invitations(&self) -> ::windows::core::Result<IRDPSRAPIInvitationManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Invitations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIInvitationManager>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationFilter(&self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ApplicationFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIApplicationFilter>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn VirtualChannelManager(&self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VirtualChannelManager)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIVirtualChannelManager>(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectToClient<'a, P0>(&self, bstrconnectionstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ConnectToClient)(::windows::core::Interface::as_raw(self), bstrconnectionstring.into().abi()).ok()
    }
    pub unsafe fn SetDesktopSharedRect(&self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDesktopSharedRect)(::windows::core::Interface::as_raw(self), left, top, right, bottom).ok()
    }
    pub unsafe fn GetDesktopSharedRect(&self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDesktopSharedRect)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPISharingSession> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPISharingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPISharingSession> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPISharingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPISharingSession> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPISharingSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPISharingSession> for super::Com::IDispatch {
    fn from(value: IRDPSRAPISharingSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPISharingSession> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPISharingSession) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPISharingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPISharingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPISharingSession {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPISharingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPISharingSession").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPISharingSession {
    type Vtable = IRDPSRAPISharingSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeb20886_e470_4cf6_842b_2739c0ec5cfb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPISharingSession_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColorDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colordepth: i32) -> ::windows::core::HRESULT,
    pub ColorDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolordepth: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Attendees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attendees: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Invitations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invitations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub VirtualChannelManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VirtualChannelManager: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectToClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectToClient: usize,
    pub SetDesktopSharedRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::HRESULT,
    pub GetDesktopSharedRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPISharingSession2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPISharingSession2 {
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Open)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetColorDepth(&self, colordepth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetColorDepth)(::windows::core::Interface::as_raw(self), colordepth).ok()
    }
    pub unsafe fn ColorDepth(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ColorDepth)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IRDPSRAPISessionProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPISessionProperties>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Attendees(&self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Attendees)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIAttendeeManager>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invitations(&self) -> ::windows::core::Result<IRDPSRAPIInvitationManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Invitations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIInvitationManager>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationFilter(&self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ApplicationFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIApplicationFilter>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn VirtualChannelManager(&self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.VirtualChannelManager)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIVirtualChannelManager>(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectToClient<'a, P0>(&self, bstrconnectionstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.ConnectToClient)(::windows::core::Interface::as_raw(self), bstrconnectionstring.into().abi()).ok()
    }
    pub unsafe fn SetDesktopSharedRect(&self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetDesktopSharedRect)(::windows::core::Interface::as_raw(self), left, top, right, bottom).ok()
    }
    pub unsafe fn GetDesktopSharedRect(&self, pleft: *mut i32, ptop: *mut i32, pright: *mut i32, pbottom: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDesktopSharedRect)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pleft), ::core::mem::transmute(ptop), ::core::mem::transmute(pright), ::core::mem::transmute(pbottom)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConnectUsingTransportStream<'a, P0, P1, P2>(&self, pstream: P0, bstrgroup: P1, bstrauthenticatedattendeename: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRDPSRAPITransportStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).ConnectUsingTransportStream)(::windows::core::Interface::as_raw(self), pstream.into().abi(), bstrgroup.into().abi(), bstrauthenticatedattendeename.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FrameBuffer(&self) -> ::windows::core::Result<IRDPSRAPIFrameBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).FrameBuffer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIFrameBuffer>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SendControlLevelChangeResponse<'a, P0>(&self, pattendee: P0, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRDPSRAPIAttendee>>,
    {
        (::windows::core::Interface::vtable(self).SendControlLevelChangeResponse)(::windows::core::Interface::as_raw(self), pattendee.into().abi(), requestedlevel, reasoncode).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPISharingSession2> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPISharingSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPISharingSession2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPISharingSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPISharingSession2> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPISharingSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPISharingSession2> for super::Com::IDispatch {
    fn from(value: IRDPSRAPISharingSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPISharingSession2> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPISharingSession2) -> Self {
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
impl ::core::convert::From<IRDPSRAPISharingSession2> for IRDPSRAPISharingSession {
    fn from(value: IRDPSRAPISharingSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPISharingSession2> for &'a IRDPSRAPISharingSession {
    fn from(value: &'a IRDPSRAPISharingSession2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPISharingSession2> for IRDPSRAPISharingSession {
    fn from(value: &IRDPSRAPISharingSession2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IRDPSRAPISharingSession2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPISharingSession2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPISharingSession2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPISharingSession2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPISharingSession2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPISharingSession2 {
    type Vtable = IRDPSRAPISharingSession2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfee4ee57_e3e8_4205_8fb0_8fd1d0675c21);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPISharingSession2_Vtbl {
    pub base__: IRDPSRAPISharingSession_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ConnectUsingTransportStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, bstrgroup: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrauthenticatedattendeename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConnectUsingTransportStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FrameBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FrameBuffer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SendControlLevelChangeResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattendee: *mut ::core::ffi::c_void, requestedlevel: CTRL_LEVEL, reasoncode: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SendControlLevelChangeResponse: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPITcpConnectionInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPITcpConnectionInfo {
    pub unsafe fn Protocol(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Protocol)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn LocalPort(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LocalPort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalIP(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LocalIP)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn PeerPort(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeerPort)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PeerIP(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PeerIP)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPITcpConnectionInfo> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPITcpConnectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPITcpConnectionInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPITcpConnectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPITcpConnectionInfo> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPITcpConnectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPITcpConnectionInfo> for super::Com::IDispatch {
    fn from(value: IRDPSRAPITcpConnectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPITcpConnectionInfo> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPITcpConnectionInfo) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPITcpConnectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPITcpConnectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPITcpConnectionInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPITcpConnectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPITcpConnectionInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPITcpConnectionInfo {
    type Vtable = IRDPSRAPITcpConnectionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf74049a4_3d06_4028_8193_0a8c29bc2452);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITcpConnectionInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Protocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plprotocol: *mut i32) -> ::windows::core::HRESULT,
    pub LocalPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LocalIP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsrlocalip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LocalIP: usize,
    pub PeerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plport: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PeerIP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrip: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PeerIP: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
pub struct IRDPSRAPITransportStream(::windows::core::IUnknown);
impl IRDPSRAPITransportStream {
    pub unsafe fn AllocBuffer(&self, maxpayload: i32) -> ::windows::core::Result<IRDPSRAPITransportStreamBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AllocBuffer)(::windows::core::Interface::as_raw(self), maxpayload, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPITransportStreamBuffer>(result__)
    }
    pub unsafe fn FreeBuffer<'a, P0>(&self, pbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRDPSRAPITransportStreamBuffer>>,
    {
        (::windows::core::Interface::vtable(self).FreeBuffer)(::windows::core::Interface::as_raw(self), pbuffer.into().abi()).ok()
    }
    pub unsafe fn WriteBuffer<'a, P0>(&self, pbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRDPSRAPITransportStreamBuffer>>,
    {
        (::windows::core::Interface::vtable(self).WriteBuffer)(::windows::core::Interface::as_raw(self), pbuffer.into().abi()).ok()
    }
    pub unsafe fn ReadBuffer<'a, P0>(&self, pbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRDPSRAPITransportStreamBuffer>>,
    {
        (::windows::core::Interface::vtable(self).ReadBuffer)(::windows::core::Interface::as_raw(self), pbuffer.into().abi()).ok()
    }
    pub unsafe fn Open<'a, P0>(&self, pcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRDPSRAPITransportStreamEvents>>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), pcallbacks.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IRDPSRAPITransportStream> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPITransportStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRDPSRAPITransportStream> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPITransportStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPSRAPITransportStream> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPITransportStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRDPSRAPITransportStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRDPSRAPITransportStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRDPSRAPITransportStream {}
impl ::core::fmt::Debug for IRDPSRAPITransportStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPITransportStream").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPITransportStream {
    type Vtable = IRDPSRAPITransportStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36cfa065_43bb_4ef7_aed7_9b88a5053036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITransportStream_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AllocBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxpayload: i32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FreeBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallbacks: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
pub struct IRDPSRAPITransportStreamBuffer(::windows::core::IUnknown);
impl IRDPSRAPITransportStreamBuffer {
    pub unsafe fn Storage(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Storage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<*mut u8>(result__)
    }
    pub unsafe fn StorageSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StorageSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn PayloadSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PayloadSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPayloadSize(&self, lval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPayloadSize)(::windows::core::Interface::as_raw(self), lval).ok()
    }
    pub unsafe fn PayloadOffset(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).PayloadOffset)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetPayloadOffset(&self, lretval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPayloadOffset)(::windows::core::Interface::as_raw(self), lretval).ok()
    }
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Flags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFlags(&self, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFlags)(::windows::core::Interface::as_raw(self), lflags).ok()
    }
    pub unsafe fn Context(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Context)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn SetContext<'a, P0>(&self, pcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).SetContext)(::windows::core::Interface::as_raw(self), pcontext.into().abi()).ok()
    }
}
impl ::core::convert::From<IRDPSRAPITransportStreamBuffer> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPITransportStreamBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRDPSRAPITransportStreamBuffer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPITransportStreamBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPSRAPITransportStreamBuffer> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPITransportStreamBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRDPSRAPITransportStreamBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRDPSRAPITransportStreamBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRDPSRAPITransportStreamBuffer {}
impl ::core::fmt::Debug for IRDPSRAPITransportStreamBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPITransportStreamBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPITransportStreamBuffer {
    type Vtable = IRDPSRAPITransportStreamBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c80290_5085_44b0_b460_f865c39cb4a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITransportStreamBuffer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Storage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbstorage: *mut *mut u8) -> ::windows::core::HRESULT,
    pub StorageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plmaxstore: *mut i32) -> ::windows::core::HRESULT,
    pub PayloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPayloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows::core::HRESULT,
    pub PayloadOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plretval: *mut i32) -> ::windows::core::HRESULT,
    pub SetPayloadOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lretval: i32) -> ::windows::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows::core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
pub struct IRDPSRAPITransportStreamEvents(::windows::core::IUnknown);
impl IRDPSRAPITransportStreamEvents {
    pub unsafe fn OnWriteCompleted<'a, P0>(&self, pbuffer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRDPSRAPITransportStreamBuffer>>,
    {
        (::windows::core::Interface::vtable(self).OnWriteCompleted)(::windows::core::Interface::as_raw(self), pbuffer.into().abi())
    }
    pub unsafe fn OnReadCompleted<'a, P0>(&self, pbuffer: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRDPSRAPITransportStreamBuffer>>,
    {
        (::windows::core::Interface::vtable(self).OnReadCompleted)(::windows::core::Interface::as_raw(self), pbuffer.into().abi())
    }
    pub unsafe fn OnStreamClosed(&self, hrreason: ::windows::core::HRESULT) {
        (::windows::core::Interface::vtable(self).OnStreamClosed)(::windows::core::Interface::as_raw(self), hrreason)
    }
}
impl ::core::convert::From<IRDPSRAPITransportStreamEvents> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPITransportStreamEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRDPSRAPITransportStreamEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPITransportStreamEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPSRAPITransportStreamEvents> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPITransportStreamEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRDPSRAPITransportStreamEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRDPSRAPITransportStreamEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRDPSRAPITransportStreamEvents {}
impl ::core::fmt::Debug for IRDPSRAPITransportStreamEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPITransportStreamEvents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRDPSRAPITransportStreamEvents {
    type Vtable = IRDPSRAPITransportStreamEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea81c254_f5af_4e40_982e_3e63bb595276);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPITransportStreamEvents_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnWriteCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void),
    pub OnReadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void),
    pub OnStreamClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT),
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIViewer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIViewer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, P0, P1, P2>(&self, bstrconnectionstring: P0, bstrname: P1, bstrpassword: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), bstrconnectionstring.into().abi(), bstrname.into().abi(), bstrpassword.into().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Attendees(&self) -> ::windows::core::Result<IRDPSRAPIAttendeeManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Attendees)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIAttendeeManager>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invitations(&self) -> ::windows::core::Result<IRDPSRAPIInvitationManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Invitations)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIInvitationManager>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ApplicationFilter(&self) -> ::windows::core::Result<IRDPSRAPIApplicationFilter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ApplicationFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIApplicationFilter>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn VirtualChannelManager(&self) -> ::windows::core::Result<IRDPSRAPIVirtualChannelManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).VirtualChannelManager)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIVirtualChannelManager>(result__)
    }
    pub unsafe fn SetSmartSizing(&self, vbsmartsizing: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSmartSizing)(::windows::core::Interface::as_raw(self), vbsmartsizing).ok()
    }
    pub unsafe fn SmartSizing(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SmartSizing)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn RequestControl(&self, ctrllevel: CTRL_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestControl)(::windows::core::Interface::as_raw(self), ctrllevel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisconnectedText<'a, P0>(&self, bstrdisconnectedtext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SetDisconnectedText)(::windows::core::Interface::as_raw(self), bstrdisconnectedtext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisconnectedText(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DisconnectedText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RequestColorDepthChange(&self, bpp: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestColorDepthChange)(::windows::core::Interface::as_raw(self), bpp).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IRDPSRAPISessionProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPISessionProperties>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartReverseConnectListener<'a, P0, P1, P2>(&self, bstrconnectionstring: P0, bstrusername: P1, bstrpassword: P2) -> ::windows::core::Result<super::super::Foundation::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).StartReverseConnectListener)(::windows::core::Interface::as_raw(self), bstrconnectionstring.into().abi(), bstrusername.into().abi(), bstrpassword.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIViewer> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIViewer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIViewer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIViewer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIViewer> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIViewer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIViewer> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIViewer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIViewer> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIViewer) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIViewer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIViewer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIViewer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIViewer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIViewer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIViewer {
    type Vtable = IRDPSRAPIViewer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6bfcd38_8ce9_404d_8ae8_f31d00c65cb5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIViewer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Attendees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Attendees: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Invitations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invitations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ApplicationFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ApplicationFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub VirtualChannelManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    VirtualChannelManager: usize,
    pub SetSmartSizing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbsmartsizing: i16) -> ::windows::core::HRESULT,
    pub SmartSizing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbsmartsizing: *mut i16) -> ::windows::core::HRESULT,
    pub RequestControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ctrllevel: CTRL_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDisconnectedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdisconnectedtext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDisconnectedText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisconnectedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisconnectedtext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisconnectedText: usize,
    pub RequestColorDepthChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bpp: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub StartReverseConnectListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrconnectionstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrreverseconnectstring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartReverseConnectListener: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIVirtualChannel(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannel {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SendData<'a, P0>(&self, bstrdata: P0, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).SendData)(::windows::core::Interface::as_raw(self), bstrdata.into().abi(), lattendeeid, channelsendflags).ok()
    }
    pub unsafe fn SetAccess(&self, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAccess)(::windows::core::Interface::as_raw(self), lattendeeid, accesstype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Flags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Flags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<CHANNEL_PRIORITY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Priority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<CHANNEL_PRIORITY>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIVirtualChannel> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIVirtualChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIVirtualChannel> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIVirtualChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIVirtualChannel> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIVirtualChannel) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIVirtualChannel> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIVirtualChannel) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIVirtualChannel> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIVirtualChannel) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIVirtualChannel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIVirtualChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIVirtualChannel {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIVirtualChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIVirtualChannel").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIVirtualChannel {
    type Vtable = IRDPSRAPIVirtualChannel_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05e12f95_28b3_4c9a_8780_d0248574a1e0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIVirtualChannel_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SendData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lattendeeid: i32, channelsendflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SendData: usize,
    pub SetAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lattendeeid: i32, accesstype: CHANNEL_ACCESS_ENUM) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plflags: *mut i32) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppriority: *mut CHANNEL_PRIORITY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIVirtualChannelManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIVirtualChannelManager {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item<'a, P0>(&self, item: P0) -> ::windows::core::Result<IRDPSRAPIVirtualChannel>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Com::VARIANT>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), item.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIVirtualChannel>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateVirtualChannel<'a, P0>(&self, bstrchannelname: P0, priority: CHANNEL_PRIORITY, channelflags: u32) -> ::windows::core::Result<IRDPSRAPIVirtualChannel>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateVirtualChannel)(::windows::core::Interface::as_raw(self), bstrchannelname.into().abi(), priority, channelflags, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIVirtualChannel>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIVirtualChannelManager> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIVirtualChannelManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIVirtualChannelManager> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIVirtualChannelManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIVirtualChannelManager> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIVirtualChannelManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIVirtualChannelManager> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIVirtualChannelManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIVirtualChannelManager> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIVirtualChannelManager) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIVirtualChannelManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIVirtualChannelManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIVirtualChannelManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIVirtualChannelManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIVirtualChannelManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIVirtualChannelManager {
    type Vtable = IRDPSRAPIVirtualChannelManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d11c661_5d0d_4ee4_89df_2166ae1fdfed);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIVirtualChannelManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pchannel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateVirtualChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrchannelname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, priority: CHANNEL_PRIORITY, channelflags: u32, ppchannel: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateVirtualChannel: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIWindow(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindow {
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<IRDPSRAPIApplication> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Application)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIApplication>(result__)
    }
    pub unsafe fn Shared(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Shared)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i16>(result__)
    }
    pub unsafe fn SetShared(&self, newval: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetShared)(::windows::core::Interface::as_raw(self), newval).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Show(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Flags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Flags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIWindow> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIWindow> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIWindow> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIWindow> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIWindow> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIWindow) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIWindow {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIWindow").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIWindow {
    type Vtable = IRDPSRAPIWindow_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeafe0f9_c77b_4933_ba9f_a24cddcc27cf);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIWindow_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Application: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Application: usize,
    pub Shared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut i16) -> ::windows::core::HRESULT,
    pub SetShared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Flags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IRDPSRAPIWindowList(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IRDPSRAPIWindowList {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, item: i32) -> ::windows::core::Result<IRDPSRAPIWindow> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), item, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRDPSRAPIWindow>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIWindowList> for ::windows::core::IUnknown {
    fn from(value: IRDPSRAPIWindowList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIWindowList> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPSRAPIWindowList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IRDPSRAPIWindowList> for ::windows::core::IUnknown {
    fn from(value: &IRDPSRAPIWindowList) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IRDPSRAPIWindowList> for super::Com::IDispatch {
    fn from(value: IRDPSRAPIWindowList) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a IRDPSRAPIWindowList> for &'a super::Com::IDispatch {
    fn from(value: &'a IRDPSRAPIWindowList) -> Self {
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
impl ::core::clone::Clone for IRDPSRAPIWindowList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRDPSRAPIWindowList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRDPSRAPIWindowList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRDPSRAPIWindowList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPSRAPIWindowList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for IRDPSRAPIWindowList {
    type Vtable = IRDPSRAPIWindowList_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a05ce44_715a_4116_a189_a118f30a07bd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IRDPSRAPIWindowList_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: i32, pwindow: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
pub struct IRDPViewerInputSink(::windows::core::IUnknown);
impl IRDPViewerInputSink {
    pub unsafe fn SendMouseButtonEvent(&self, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendMouseButtonEvent)(::windows::core::Interface::as_raw(self), buttontype, vbbuttondown, xpos, ypos).ok()
    }
    pub unsafe fn SendMouseMoveEvent(&self, xpos: u32, ypos: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendMouseMoveEvent)(::windows::core::Interface::as_raw(self), xpos, ypos).ok()
    }
    pub unsafe fn SendMouseWheelEvent(&self, wheelrotation: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendMouseWheelEvent)(::windows::core::Interface::as_raw(self), wheelrotation).ok()
    }
    pub unsafe fn SendKeyboardEvent(&self, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendKeyboardEvent)(::windows::core::Interface::as_raw(self), codetype, keycode, vbkeyup, vbrepeat, vbextended).ok()
    }
    pub unsafe fn SendSyncEvent(&self, syncflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SendSyncEvent)(::windows::core::Interface::as_raw(self), syncflags).ok()
    }
    pub unsafe fn BeginTouchFrame(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginTouchFrame)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddTouchInput(&self, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddTouchInput)(::windows::core::Interface::as_raw(self), contactid, event, x, y).ok()
    }
    pub unsafe fn EndTouchFrame(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndTouchFrame)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IRDPViewerInputSink> for ::windows::core::IUnknown {
    fn from(value: IRDPViewerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRDPViewerInputSink> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRDPViewerInputSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRDPViewerInputSink> for ::windows::core::IUnknown {
    fn from(value: &IRDPViewerInputSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRDPViewerInputSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRDPViewerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRDPViewerInputSink {}
impl ::core::fmt::Debug for IRDPViewerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRDPViewerInputSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRDPViewerInputSink {
    type Vtable = IRDPViewerInputSink_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb590853_a6c5_4a7b_8dd4_76b69eea12d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRDPViewerInputSink_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SendMouseButtonEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buttontype: RDPSRAPI_MOUSE_BUTTON_TYPE, vbbuttondown: i16, xpos: u32, ypos: u32) -> ::windows::core::HRESULT,
    pub SendMouseMoveEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpos: u32, ypos: u32) -> ::windows::core::HRESULT,
    pub SendMouseWheelEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wheelrotation: u16) -> ::windows::core::HRESULT,
    pub SendKeyboardEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codetype: RDPSRAPI_KBD_CODE_TYPE, keycode: u16, vbkeyup: i16, vbrepeat: i16, vbextended: i16) -> ::windows::core::HRESULT,
    pub SendSyncEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncflags: u32) -> ::windows::core::HRESULT,
    pub BeginTouchFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddTouchInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contactid: u32, event: u32, x: i32, y: i32) -> ::windows::core::HRESULT,
    pub EndTouchFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RDPENCOMAPI_ATTENDEE_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const ATTENDEE_FLAGS_LOCAL: RDPENCOMAPI_ATTENDEE_FLAGS = RDPENCOMAPI_ATTENDEE_FLAGS(1i32);
impl ::core::marker::Copy for RDPENCOMAPI_ATTENDEE_FLAGS {}
impl ::core::clone::Clone for RDPENCOMAPI_ATTENDEE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDPENCOMAPI_ATTENDEE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RDPENCOMAPI_ATTENDEE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RDPENCOMAPI_ATTENDEE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDPENCOMAPI_ATTENDEE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RDPENCOMAPI_CONSTANTS(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_MAX_CHANNEL_MESSAGE_SIZE: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(1024i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_MAX_CHANNEL_NAME_LEN: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(8i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_MAX_LEGACY_CHANNEL_MESSAGE_SIZE: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(409600i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_ATTENDEE_ID_EVERYONE: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(-1i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_ATTENDEE_ID_HOST: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(0i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_CONN_INTERVAL: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(50i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const CONST_ATTENDEE_ID_DEFAULT: RDPENCOMAPI_CONSTANTS = RDPENCOMAPI_CONSTANTS(-1i32);
impl ::core::marker::Copy for RDPENCOMAPI_CONSTANTS {}
impl ::core::clone::Clone for RDPENCOMAPI_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDPENCOMAPI_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RDPENCOMAPI_CONSTANTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RDPENCOMAPI_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDPENCOMAPI_CONSTANTS").field(&self.0).finish()
    }
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
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RDPSRAPI_APP_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const APP_FLAG_PRIVILEGED: RDPSRAPI_APP_FLAGS = RDPSRAPI_APP_FLAGS(1i32);
impl ::core::marker::Copy for RDPSRAPI_APP_FLAGS {}
impl ::core::clone::Clone for RDPSRAPI_APP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDPSRAPI_APP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_APP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RDPSRAPI_APP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDPSRAPI_APP_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RDPSRAPI_KBD_CODE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_CODE_SCANCODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_CODE_UNICODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(1i32);
impl ::core::marker::Copy for RDPSRAPI_KBD_CODE_TYPE {}
impl ::core::clone::Clone for RDPSRAPI_KBD_CODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDPSRAPI_KBD_CODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_KBD_CODE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RDPSRAPI_KBD_CODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDPSRAPI_KBD_CODE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RDPSRAPI_KBD_SYNC_FLAG(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_SCROLL_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(1i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_NUM_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(2i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_CAPS_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(4i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_KBD_SYNC_FLAG_KANA_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(8i32);
impl ::core::marker::Copy for RDPSRAPI_KBD_SYNC_FLAG {}
impl ::core::clone::Clone for RDPSRAPI_KBD_SYNC_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDPSRAPI_KBD_SYNC_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_KBD_SYNC_FLAG {
    type Abi = Self;
}
impl ::core::fmt::Debug for RDPSRAPI_KBD_SYNC_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDPSRAPI_KBD_SYNC_FLAG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RDPSRAPI_MOUSE_BUTTON_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(5i32);
impl ::core::marker::Copy for RDPSRAPI_MOUSE_BUTTON_TYPE {}
impl ::core::clone::Clone for RDPSRAPI_MOUSE_BUTTON_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDPSRAPI_MOUSE_BUTTON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_MOUSE_BUTTON_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RDPSRAPI_MOUSE_BUTTON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDPSRAPI_MOUSE_BUTTON_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RDPSRAPI_WND_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
pub const WND_FLAG_PRIVILEGED: RDPSRAPI_WND_FLAGS = RDPSRAPI_WND_FLAGS(1i32);
impl ::core::marker::Copy for RDPSRAPI_WND_FLAGS {}
impl ::core::clone::Clone for RDPSRAPI_WND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RDPSRAPI_WND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RDPSRAPI_WND_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RDPSRAPI_WND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDPSRAPI_WND_FLAGS").field(&self.0).finish()
    }
}
pub const RDPSession: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b78f0e6_3e05_4a5b_b2e8_e743a8956b65);
pub const RDPTransportStreamBuffer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d4a1c69_f17f_4549_a699_761c6e6b5c0a);
pub const RDPTransportStreamEvents: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31e3ab20_5350_483f_9dc6_6748665efdeb);
pub const RDPViewer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32be5ed2_5c86_480f_a914_0ff8885a1b3f);
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _IRDPSessionEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _IRDPSessionEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_IRDPSessionEvents> for ::windows::core::IUnknown {
    fn from(value: _IRDPSessionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a _IRDPSessionEvents> for &'a ::windows::core::IUnknown {
    fn from(value: &'a _IRDPSessionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&_IRDPSessionEvents> for ::windows::core::IUnknown {
    fn from(value: &_IRDPSessionEvents) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<_IRDPSessionEvents> for super::Com::IDispatch {
    fn from(value: _IRDPSessionEvents) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::core::convert::From<&'a _IRDPSessionEvents> for &'a super::Com::IDispatch {
    fn from(value: &'a _IRDPSessionEvents) -> Self {
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
impl ::core::clone::Clone for _IRDPSessionEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _IRDPSessionEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _IRDPSessionEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _IRDPSessionEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_IRDPSessionEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _IRDPSessionEvents {
    type Vtable = _IRDPSessionEvents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98a97042_6698_40e9_8efd_b3200990004b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _IRDPSessionEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DesktopSharing\"`*"]
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
impl ::core::marker::Copy for __ReferenceRemainingTypes__ {}
impl ::core::clone::Clone for __ReferenceRemainingTypes__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for __ReferenceRemainingTypes__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("__ReferenceRemainingTypes__")
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
unsafe impl ::windows::core::Abi for __ReferenceRemainingTypes__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for __ReferenceRemainingTypes__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<__ReferenceRemainingTypes__>()) == 0 }
    }
}
impl ::core::cmp::Eq for __ReferenceRemainingTypes__ {}
impl ::core::default::Default for __ReferenceRemainingTypes__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
