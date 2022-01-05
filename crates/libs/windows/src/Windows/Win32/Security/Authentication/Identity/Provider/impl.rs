pub trait AsyncIAssociatedIdentityProviderImpl: Sized {
    fn Begin_AssociateIdentity();
    fn Finish_AssociateIdentity();
    fn Begin_DisassociateIdentity();
    fn Finish_DisassociateIdentity();
    fn Begin_ChangeCredential();
    fn Finish_ChangeCredential();
}
pub trait AsyncIConnectedIdentityProviderImpl: Sized {
    fn Begin_ConnectIdentity();
    fn Finish_ConnectIdentity();
    fn Begin_DisconnectIdentity();
    fn Finish_DisconnectIdentity();
    fn Begin_IsConnected();
    fn Finish_IsConnected();
    fn Begin_GetUrl();
    fn Finish_GetUrl();
    fn Begin_GetAccountState();
    fn Finish_GetAccountState();
}
pub trait AsyncIIdentityAdviseImpl: Sized {
    fn Begin_IdentityUpdated();
    fn Finish_IdentityUpdated();
}
pub trait AsyncIIdentityAuthenticationImpl: Sized {
    fn Begin_SetIdentityCredential();
    fn Finish_SetIdentityCredential();
    fn Begin_ValidateIdentityCredential();
    fn Finish_ValidateIdentityCredential();
}
pub trait AsyncIIdentityProviderImpl: Sized {
    fn Begin_GetIdentityEnum();
    fn Finish_GetIdentityEnum();
    fn Begin_Create();
    fn Finish_Create();
    fn Begin_Import();
    fn Finish_Import();
    fn Begin_Delete();
    fn Finish_Delete();
    fn Begin_FindByUniqueID();
    fn Finish_FindByUniqueID();
    fn Begin_GetProviderPropertyStore();
    fn Finish_GetProviderPropertyStore();
    fn Begin_Advise();
    fn Finish_Advise();
    fn Begin_UnAdvise();
    fn Finish_UnAdvise();
}
pub trait AsyncIIdentityStoreImpl: Sized {
    fn Begin_GetCount();
    fn Finish_GetCount();
    fn Begin_GetAt();
    fn Finish_GetAt();
    fn Begin_AddToCache();
    fn Finish_AddToCache();
    fn Begin_ConvertToSid();
    fn Finish_ConvertToSid();
    fn Begin_EnumerateIdentities();
    fn Finish_EnumerateIdentities();
    fn Begin_Reset();
    fn Finish_Reset();
}
pub trait AsyncIIdentityStoreExImpl: Sized {
    fn Begin_CreateConnectedIdentity();
    fn Finish_CreateConnectedIdentity();
    fn Begin_DeleteConnectedIdentity();
    fn Finish_DeleteConnectedIdentity();
}
pub trait IAssociatedIdentityProviderImpl: Sized {
    fn AssociateIdentity();
    fn DisassociateIdentity();
    fn ChangeCredential();
}
pub trait IConnectedIdentityProviderImpl: Sized {
    fn ConnectIdentity();
    fn DisconnectIdentity();
    fn IsConnected();
    fn GetUrl();
    fn GetAccountState();
}
pub trait IIdentityAdviseImpl: Sized {
    fn IdentityUpdated();
}
pub trait IIdentityAuthenticationImpl: Sized {
    fn SetIdentityCredential();
    fn ValidateIdentityCredential();
}
pub trait IIdentityProviderImpl: Sized {
    fn GetIdentityEnum();
    fn Create();
    fn Import();
    fn Delete();
    fn FindByUniqueID();
    fn GetProviderPropertyStore();
    fn Advise();
    fn UnAdvise();
}
pub trait IIdentityStoreImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn AddToCache();
    fn ConvertToSid();
    fn EnumerateIdentities();
    fn Reset();
}
pub trait IIdentityStoreExImpl: Sized {
    fn CreateConnectedIdentity();
    fn DeleteConnectedIdentity();
}
