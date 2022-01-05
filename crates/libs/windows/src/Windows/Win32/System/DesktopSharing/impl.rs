#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIApplicationImpl: Sized + IDispatchImpl {
    fn Windows();
    fn Id();
    fn Shared();
    fn SetShared();
    fn Name();
    fn Flags();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIApplicationFilterImpl: Sized + IDispatchImpl {
    fn Applications();
    fn Windows();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIApplicationListImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIAttendeeImpl: Sized + IDispatchImpl {
    fn Id();
    fn RemoteName();
    fn ControlLevel();
    fn SetControlLevel();
    fn Invitation();
    fn TerminateConnection();
    fn Flags();
    fn ConnectivityInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIAttendeeDisconnectInfoImpl: Sized + IDispatchImpl {
    fn Attendee();
    fn Reason();
    fn Code();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIAttendeeManagerImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
}
pub trait IRDPSRAPIAudioStreamImpl: Sized {
    fn Initialize();
    fn Start();
    fn Stop();
    fn GetBuffer();
    fn FreeBuffer();
}
pub trait IRDPSRAPIClipboardUseEventsImpl: Sized {
    fn OnPasteFromClipboard();
}
pub trait IRDPSRAPIDebugImpl: Sized {
    fn SetCLXCmdLine();
    fn CLXCmdLine();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIFrameBufferImpl: Sized + IDispatchImpl {
    fn Width();
    fn Height();
    fn Bpp();
    fn GetFrameBufferBits();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIInvitationImpl: Sized + IDispatchImpl {
    fn ConnectionString();
    fn GroupName();
    fn Password();
    fn AttendeeLimit();
    fn SetAttendeeLimit();
    fn Revoked();
    fn SetRevoked();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIInvitationManagerImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn CreateInvitation();
}
pub trait IRDPSRAPIPerfCounterLoggerImpl: Sized {
    fn LogValue();
}
pub trait IRDPSRAPIPerfCounterLoggingManagerImpl: Sized {
    fn CreateLogger();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPISessionPropertiesImpl: Sized + IDispatchImpl {
    fn Property();
    fn SetProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPISharingSessionImpl: Sized + IDispatchImpl {
    fn Open();
    fn Close();
    fn SetColorDepth();
    fn ColorDepth();
    fn Properties();
    fn Attendees();
    fn Invitations();
    fn ApplicationFilter();
    fn VirtualChannelManager();
    fn Pause();
    fn Resume();
    fn ConnectToClient();
    fn SetDesktopSharedRect();
    fn GetDesktopSharedRect();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPISharingSession2Impl: Sized + IRDPSRAPISharingSessionImpl + IDispatchImpl {
    fn ConnectUsingTransportStream();
    fn FrameBuffer();
    fn SendControlLevelChangeResponse();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPITcpConnectionInfoImpl: Sized + IDispatchImpl {
    fn Protocol();
    fn LocalPort();
    fn LocalIP();
    fn PeerPort();
    fn PeerIP();
}
pub trait IRDPSRAPITransportStreamImpl: Sized {
    fn AllocBuffer();
    fn FreeBuffer();
    fn WriteBuffer();
    fn ReadBuffer();
    fn Open();
    fn Close();
}
pub trait IRDPSRAPITransportStreamBufferImpl: Sized {
    fn Storage();
    fn StorageSize();
    fn PayloadSize();
    fn SetPayloadSize();
    fn PayloadOffset();
    fn SetPayloadOffset();
    fn Flags();
    fn SetFlags();
    fn Context();
    fn SetContext();
}
pub trait IRDPSRAPITransportStreamEventsImpl: Sized {
    fn OnWriteCompleted();
    fn OnReadCompleted();
    fn OnStreamClosed();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIViewerImpl: Sized + IDispatchImpl {
    fn Connect();
    fn Disconnect();
    fn Attendees();
    fn Invitations();
    fn ApplicationFilter();
    fn VirtualChannelManager();
    fn SetSmartSizing();
    fn SmartSizing();
    fn RequestControl();
    fn SetDisconnectedText();
    fn DisconnectedText();
    fn RequestColorDepthChange();
    fn Properties();
    fn StartReverseConnectListener();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIVirtualChannelImpl: Sized + IDispatchImpl {
    fn SendData();
    fn SetAccess();
    fn Name();
    fn Flags();
    fn Priority();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIVirtualChannelManagerImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn CreateVirtualChannel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIWindowImpl: Sized + IDispatchImpl {
    fn Id();
    fn Application();
    fn Shared();
    fn SetShared();
    fn Name();
    fn Show();
    fn Flags();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRDPSRAPIWindowListImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
}
pub trait IRDPViewerInputSinkImpl: Sized {
    fn SendMouseButtonEvent();
    fn SendMouseMoveEvent();
    fn SendMouseWheelEvent();
    fn SendKeyboardEvent();
    fn SendSyncEvent();
    fn BeginTouchFrame();
    fn AddTouchInput();
    fn EndTouchFrame();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IRDPSessionEventsImpl: Sized + IDispatchImpl {}
