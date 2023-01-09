impl ::core::default::Default for CredentialPromptType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CredentialPromptType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialPromptType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for OnlineIdAuthenticator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdAuthenticator {}
impl ::core::fmt::Debug for OnlineIdAuthenticator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdAuthenticator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for OnlineIdServiceTicket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdServiceTicket {}
impl ::core::fmt::Debug for OnlineIdServiceTicket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdServiceTicket").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for OnlineIdServiceTicketRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdServiceTicketRequest {}
impl ::core::fmt::Debug for OnlineIdServiceTicketRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdServiceTicketRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemAuthenticatorForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemAuthenticatorForUser {}
impl ::core::fmt::Debug for OnlineIdSystemAuthenticatorForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemAuthenticatorForUser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemIdentity {}
impl ::core::fmt::Debug for OnlineIdSystemIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemIdentity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemTicketResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemTicketResult {}
impl ::core::fmt::Debug for OnlineIdSystemTicketResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemTicketResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for OnlineIdSystemTicketStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OnlineIdSystemTicketStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemTicketStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for SignOutUserOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for SignOutUserOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignOutUserOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for UserAuthenticationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for UserAuthenticationOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for UserAuthenticationOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationOperation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UserIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserIdentity {}
impl ::core::fmt::Debug for UserIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserIdentity").field(&self.0).finish()
    }
}
