#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_LOGON_TYPE(pub i32);
impl SECURITY_LOGON_TYPE {
    pub const UndefinedLogonType: Self = Self(0);
    pub const Interactive: Self = Self(2);
    pub const Network: Self = Self(3);
    pub const Batch: Self = Self(4);
    pub const Service: Self = Self(5);
    pub const Proxy: Self = Self(6);
    pub const Unlock: Self = Self(7);
    pub const NetworkCleartext: Self = Self(8);
    pub const NewCredentials: Self = Self(9);
    pub const RemoteInteractive: Self = Self(10);
    pub const CachedInteractive: Self = Self(11);
    pub const CachedRemoteInteractive: Self = Self(12);
    pub const CachedUnlock: Self = Self(13);
}
